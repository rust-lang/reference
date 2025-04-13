//! Converts a [`Grammar`] to an SVG railroad diagram.

use super::{Characters, Expression, ExpressionKind, Production};
use crate::grammar::Grammar;
use anyhow::bail;
use railroad::*;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::LazyLock;

impl Grammar {
    pub fn render_railroad(
        &self,
        names: &[&str],
        link_map: &HashMap<String, String>,
        output: &mut String,
        for_summary: bool,
    ) -> anyhow::Result<()> {
        for name in names {
            let prod = match self.productions.get(*name) {
                Some(p) => p,
                None => bail!("could not find grammar production named `{name}`"),
            };
            prod.render_railroad(link_map, output, for_summary);
        }
        Ok(())
    }
}

/// The HTML id for the production.
pub fn railroad_id(name: &str, for_summary: bool) -> String {
    if for_summary {
        format!("railroad-summary-{}", name)
    } else {
        format!("railroad-{}", name)
    }
}

impl Production {
    fn render_railroad(
        &self,
        link_map: &HashMap<String, String>,
        output: &mut String,
        for_summary: bool,
    ) {
        let mut dia = self.make_diagram(false, link_map);
        // If the diagram is very wide, try stacking it to reduce the width.
        // This 900 is somewhat arbitrary based on looking at productions that
        // looked too squished. If your diagram is still too squished,
        // consider adding more rules to shorten it.
        if dia.width() > 900 {
            dia = self.make_diagram(true, link_map);
        }
        writeln!(
            output,
            "<div style=\"width: {width}px; height: auto; max-width: 100%; max-height: 100%\" \
                class=\"railroad-production\" \
                id=\"{id}\">{dia}</div>",
            width = dia.width(),
            id = railroad_id(&self.name, for_summary),
        )
        .unwrap();
    }

    fn make_diagram(
        &self,
        stack: bool,
        link_map: &HashMap<String, String>,
    ) -> Diagram<Box<dyn Node>> {
        let n = self.expression.render_railroad(stack, link_map, false);
        let seq: Sequence<Box<dyn Node>> =
            Sequence::new(vec![Box::new(SimpleStart), n.unwrap(), Box::new(SimpleEnd)]);
        let vert = VerticalGrid::<Box<dyn Node>>::new(vec![
            Box::new(Comment::new(self.name.clone())),
            Box::new(seq),
        ]);

        Diagram::new(Box::new(vert))
    }
}

impl Expression {
    fn render_railroad(
        &self,
        stack: bool,
        link_map: &HashMap<String, String>,
        reverse: bool,
    ) -> Option<Box<dyn Node>> {
        let n: Box<dyn Node> = match &self.kind {
            ExpressionKind::Grouped(e) => {
                // I don't think this needs anything special. The grouped
                // expression is usually an Alt or Optional or something like
                // that which ends up as a distinct railroad node. But I'm not
                // sure.
                e.render_railroad(stack, link_map, reverse)?
            }
            ExpressionKind::Alt(es) => {
                let choices: Vec<_> = es
                    .iter()
                    .map(|e| e.render_railroad(stack, link_map, reverse))
                    .filter_map(|n| n)
                    .collect();
                Box::new(Choice::<Box<dyn Node>>::new(choices))
            }
            ExpressionKind::Sequence(es) => {
                let mut es: Vec<_> = es.iter().collect();
                // For reversing, see ::Repeat for an explanation.
                if reverse {
                    es.reverse();
                }
                let make_seq = |es: &[&Expression]| {
                    let seq: Vec<_> = es
                        .iter()
                        .map(|e| e.render_railroad(stack, link_map, reverse))
                        .filter_map(|n| n)
                        .collect();
                    let seq: Sequence<Box<dyn Node>> = Sequence::new(seq);
                    Box::new(seq)
                };

                // If `stack` is true, split the sequence on Breaks and stack them vertically.
                if stack {
                    // First, trim a Break from the front and back.
                    let es = if matches!(
                        es.first(),
                        Some(e) if e.is_break()
                    ) {
                        &es[1..]
                    } else {
                        &es[..]
                    };
                    let es = if matches!(
                        es.last(),
                        Some(e) if e.is_break()
                    ) {
                        &es[..es.len() - 1]
                    } else {
                        &es[..]
                    };

                    let mut breaks: Vec<_> =
                        es.split(|e| e.is_break()).map(|es| make_seq(es)).collect();
                    // If there aren't any breaks, don't bother stacking.
                    if breaks.len() == 1 {
                        breaks.pop().unwrap()
                    } else {
                        Box::new(Stack::new(breaks))
                    }
                } else {
                    make_seq(&es)
                }
            }
            ExpressionKind::Optional(e) => {
                let n = e.render_railroad(stack, link_map, reverse)?;
                Box::new(Optional::new(n))
            }
            ExpressionKind::Repeat(e) => {
                // Railroad renders everything in the opposite order. However,
                // our grammar is not written that way, so we need to undo the
                // reversal.
                let n = e.render_railroad(stack, link_map, !reverse)?;
                Box::new(Repeat::new(railroad::Empty, n))
            }
            ExpressionKind::RepeatNonGreedy(e) => {
                let n = e.render_railroad(stack, link_map, !reverse)?;
                let r = Box::new(Repeat::new(railroad::Empty, n));
                let lbox = LabeledBox::new(r, Comment::new("non-greedy".to_string()));
                Box::new(lbox)
            }
            ExpressionKind::RepeatPlus(e) => {
                let n = e.render_railroad(stack, link_map, reverse)?;
                Box::new(Repeat::new(n, railroad::Empty))
            }
            ExpressionKind::RepeatPlusNonGreedy(e) => {
                let n = e.render_railroad(stack, link_map, reverse)?;
                let r = Repeat::new(n, railroad::Empty);
                let lbox = LabeledBox::new(r, Comment::new("non-greedy".to_string()));
                Box::new(lbox)
            }
            ExpressionKind::RepeatRange(e, a, b) => {
                let n = e.render_railroad(stack, link_map, reverse)?;
                let cmt = match (a, b) {
                    (Some(a), Some(b)) => format!("repeat between {a} and {b} times"),
                    (None, Some(b)) => format!("repeat at most {b} times"),
                    (Some(a), None) => format!("repeat at least {a} times"),
                    (None, None) => panic!("infinite repeat should use *"),
                };
                let r = Repeat::new(n, Comment::new(cmt));
                Box::new(r)
            }
            ExpressionKind::Nt(nt) => node_for_nt(link_map, nt),
            ExpressionKind::Terminal(t) => Box::new(Terminal::new(t.clone())),
            ExpressionKind::Prose(s) => Box::new(Terminal::new(s.clone())),
            ExpressionKind::Break(_) => return None,
            ExpressionKind::Charset(set) => {
                let ns: Vec<_> = set.iter().map(|c| c.render_railroad(link_map)).collect();
                Box::new(Choice::<Box<dyn Node>>::new(ns))
            }
            ExpressionKind::NegExpression(e) => {
                let n = e.render_railroad(stack, link_map, reverse)?;
                let lbox = LabeledBox::new(n, Comment::new("any character except".to_string()));
                Box::new(lbox)
            }
            ExpressionKind::Unicode(s) => Box::new(Terminal::new(format!("U+{}", s))),
        };
        if let Some(suffix) = &self.suffix {
            let suffix = strip_markdown(suffix);
            let lbox = LabeledBox::new(n, Comment::new(suffix));
            return Some(Box::new(lbox));
        }
        // Note: Footnotes aren't supported. They could be added as a comment
        // on a vertical stack or a LabeledBox or something like that, but I
        // don't feel like bothering.
        Some(n)
    }
}

impl Characters {
    fn render_railroad(&self, link_map: &HashMap<String, String>) -> Box<dyn Node> {
        match self {
            Characters::Named(s) => node_for_nt(link_map, s),
            Characters::Terminal(s) => Box::new(Terminal::new(s.clone())),
            Characters::Range(a, b) => Box::new(Terminal::new(format!("{a}-{b}"))),
        }
    }
}

fn node_for_nt(link_map: &HashMap<String, String>, name: &str) -> Box<dyn Node> {
    let dest = link_map
        .get(name)
        .map(|path| path.to_string())
        .unwrap_or_else(|| format!("missing"));
    let n = NonTerminal::new(name.to_string());
    Box::new(Link::new(n, dest))
}

/// Removes some markdown so it can be rendered as text.
fn strip_markdown(s: &str) -> String {
    // Right now this just removes markdown linkifiers, but more can be added if needed.
    static LINK_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?s)\[([^\]]+)\](?:\[[^\]]*\]|\([^)]*\))?").unwrap());
    LINK_RE.replace_all(s, "$1").to_string()
}
