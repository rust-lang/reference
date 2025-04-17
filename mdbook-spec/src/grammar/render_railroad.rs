//! Converts a [`Grammar`] to an SVG railroad diagram.

use super::{Characters, Expression, ExpressionKind, Production, RenderCtx};
use crate::grammar::Grammar;
use anyhow::bail;
use railroad::*;
use regex::Regex;
use std::fmt::Write;
use std::sync::LazyLock;

impl Grammar {
    pub fn render_railroad(
        &self,
        cx: &RenderCtx,
        names: &[&str],
        output: &mut String,
    ) -> anyhow::Result<()> {
        for name in names {
            let prod = match self.productions.get(*name) {
                Some(p) => p,
                None => bail!("could not find grammar production named `{name}`"),
            };
            prod.render_railroad(cx, output);
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
    fn render_railroad(&self, cx: &RenderCtx, output: &mut String) {
        let mut dia = self.make_diagram(cx, false);
        // If the diagram is very wide, try stacking it to reduce the width.
        // This 900 is somewhat arbitrary based on looking at productions that
        // looked too squished. If your diagram is still too squished,
        // consider adding more rules to shorten it.
        if dia.width() > 900 {
            dia = self.make_diagram(cx, true);
        }
        writeln!(
            output,
            "<div style=\"width: {width}px; height: auto; max-width: 100%; max-height: 100%\" \
                class=\"railroad-production\" \
                id=\"{id}\">{dia}</div>",
            width = dia.width(),
            id = railroad_id(&self.name, cx.for_summary),
        )
        .unwrap();
    }

    fn make_diagram(&self, cx: &RenderCtx, stack: bool) -> Diagram<Box<dyn Node>> {
        let n = self.expression.render_railroad(cx, stack);
        let dest = cx
            .md_link_map
            .get(&self.name)
            .map(|path| path.to_string())
            .unwrap_or_else(|| format!("missing"));
        let seq: Sequence<Box<dyn Node>> =
            Sequence::new(vec![Box::new(SimpleStart), n.unwrap(), Box::new(SimpleEnd)]);
        let vert = VerticalGrid::<Box<dyn Node>>::new(vec![
            Box::new(Link::new(Comment::new(self.name.clone()), dest)),
            Box::new(seq),
        ]);

        Diagram::new(Box::new(vert))
    }
}

impl Expression {
    fn render_railroad(&self, cx: &RenderCtx, stack: bool) -> Option<Box<dyn Node>> {
        let mut state;
        let mut state_ref = &self.kind;
        let n: Box<dyn Node> = 'l: loop {
            state_ref = 'cont: {
                break 'l match state_ref {
                    // Render grouped nodes and `e{1..1}` repeats directly.
                    ExpressionKind::Grouped(e)
                    | ExpressionKind::RepeatRange(e, Some(1), Some(1)) => {
                        e.render_railroad(cx, stack)?
                    }
                    ExpressionKind::Alt(es) => {
                        let choices: Vec<_> = es
                            .iter()
                            .map(|e| e.render_railroad(cx, stack))
                            .filter_map(|n| n)
                            .collect();
                        Box::new(Choice::<Box<dyn Node>>::new(choices))
                    }
                    ExpressionKind::Sequence(es) => {
                        let es: Vec<_> = es.iter().collect();
                        let make_seq = |es: &[&Expression]| {
                            let seq: Vec<_> = es
                                .iter()
                                .map(|e| e.render_railroad(cx, stack))
                                .filter_map(|n| n)
                                .collect();
                            let seq: Sequence<Box<dyn Node>> = Sequence::new(seq);
                            Box::new(seq)
                        };

                        // If `stack` is true, split the sequence on Breaks and
                        // stack them vertically.
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
                    // Treat `e?` and `e{..1}` / `e{0..1}` equally.
                    ExpressionKind::Optional(e)
                    | ExpressionKind::RepeatRange(e, None | Some(0), Some(1)) => {
                        let n = e.render_railroad(cx, stack)?;
                        Box::new(Optional::new(n))
                    }
                    // Treat `e*` and `e{..}` / `e{0..}` equally.
                    ExpressionKind::Repeat(e)
                    | ExpressionKind::RepeatRange(e, None | Some(0), None) => {
                        let n = e.render_railroad(cx, stack)?;
                        Box::new(Optional::new(Repeat::new(n, railroad::Empty)))
                    }
                    ExpressionKind::RepeatNonGreedy(e) => {
                        let n = e.render_railroad(cx, stack)?;
                        let r = Box::new(Optional::new(Repeat::new(n, railroad::Empty)));
                        let lbox = LabeledBox::new(r, Comment::new("non-greedy".to_string()));
                        Box::new(lbox)
                    }
                    // Treat `e+` and `e{1..}` equally.
                    ExpressionKind::RepeatPlus(e)
                    | ExpressionKind::RepeatRange(e, Some(1), None) => {
                        let n = e.render_railroad(cx, stack)?;
                        Box::new(Repeat::new(n, railroad::Empty))
                    }
                    ExpressionKind::RepeatPlusNonGreedy(e) => {
                        let n = e.render_railroad(cx, stack)?;
                        let r = Repeat::new(n, railroad::Empty);
                        let lbox = LabeledBox::new(r, Comment::new("non-greedy".to_string()));
                        Box::new(lbox)
                    }
                    // For `e{a..0}` render an empty node.
                    ExpressionKind::RepeatRange(_, _, Some(0)) => Box::new(railroad::Empty),
                    // Treat `e{..b}` / `e{0..b}` as `(e{1..b})?`.
                    ExpressionKind::RepeatRange(e, None | Some(0), Some(b @ 2..)) => {
                        state = ExpressionKind::Optional(Box::new(Expression::new_kind(
                            ExpressionKind::RepeatRange(e.clone(), Some(1), Some(*b)),
                        )));
                        break 'cont &state;
                    }
                    // Render `e{1..b}` directly.
                    ExpressionKind::RepeatRange(e, Some(1), Some(b @ 2..)) => {
                        let n = e.render_railroad(cx, stack)?;
                        let cmt = format!("at most {b} more times", b = b - 1);
                        let r = Repeat::new(n, Comment::new(cmt));
                        Box::new(r)
                    }
                    // Treat `e{a..}` as `e{a-1..a-1} e{1..}` and `e{a..b}` as
                    // `e{a-1..a-1} e{1..b-(a-1)}`, and treat `e{x..x}` for some
                    // `x` as a sequence of `e` nodes of length `x`.
                    ExpressionKind::RepeatRange(e, Some(a @ 2..), b) => {
                        let mut es = Vec::<Expression>::new();
                        for _ in 0..(a - 1) {
                            es.push(*e.clone());
                        }
                        es.push(Expression::new_kind(ExpressionKind::RepeatRange(
                            e.clone(),
                            Some(1),
                            b.map(|x| x - (a - 1)),
                        )));
                        state = ExpressionKind::Sequence(es);
                        break 'cont &state;
                    }
                    ExpressionKind::Nt(nt) => node_for_nt(cx, nt),
                    ExpressionKind::Terminal(t) => Box::new(Terminal::new(t.clone())),
                    ExpressionKind::Prose(s) => Box::new(Terminal::new(s.clone())),
                    ExpressionKind::Break(_) => return None,
                    ExpressionKind::Charset(set) => {
                        let ns: Vec<_> = set.iter().map(|c| c.render_railroad(cx)).collect();
                        Box::new(Choice::<Box<dyn Node>>::new(ns))
                    }
                    ExpressionKind::NegExpression(e) => {
                        let n = e.render_railroad(cx, stack)?;
                        let ch = node_for_nt(cx, "CHAR");
                        Box::new(Except::new(Box::new(ch), n))
                    }
                    ExpressionKind::Unicode(s) => Box::new(Terminal::new(format!("U+{}", s))),
                };
            }
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
    fn render_railroad(&self, cx: &RenderCtx) -> Box<dyn Node> {
        match self {
            Characters::Named(s) => node_for_nt(cx, s),
            Characters::Terminal(s) => Box::new(Terminal::new(s.clone())),
            Characters::Range(a, b) => Box::new(Terminal::new(format!("{a}-{b}"))),
        }
    }
}

fn node_for_nt(cx: &RenderCtx, name: &str) -> Box<dyn Node> {
    let dest = cx
        .rr_link_map
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

struct Except {
    inner: LabeledBox<Box<dyn Node>, Box<dyn Node>>,
}

impl Except {
    fn new(inner: Box<dyn Node>, label: Box<dyn Node>) -> Self {
        let grid = Box::new(VerticalGrid::new(vec![
            Box::new(Comment::new("⚠️ with the exception of".to_owned())) as Box<dyn Node>,
            label,
        ])) as Box<dyn Node>;
        let mut this = Self {
            inner: LabeledBox::new(inner, grid),
        };
        this.inner
            .attr("class".to_owned())
            .or_default()
            .push_str(" exceptbox");
        this
    }
}

impl Node for Except {
    fn entry_height(&self) -> i64 {
        self.inner.entry_height()
    }

    fn height(&self) -> i64 {
        self.inner.height()
    }

    fn width(&self) -> i64 {
        self.inner.width()
    }

    fn draw(&self, x: i64, y: i64, h_dir: svg::HDir) -> svg::Element {
        self.inner.draw(x, y, h_dir)
    }
}
