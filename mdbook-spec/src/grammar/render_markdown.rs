//! Renders the grammar to markdown.

use super::{Characters, Expression, ExpressionKind, Production, RenderCtx};
use crate::grammar::Grammar;
use anyhow::bail;
use regex::Regex;
use std::borrow::Cow;
use std::fmt::Write;
use std::sync::LazyLock;

impl Grammar {
    pub fn render_markdown(
        &self,
        cx: &RenderCtx,
        names: &[&str],
        output: &mut String,
    ) -> anyhow::Result<()> {
        let mut iter = names.into_iter().peekable();
        while let Some(name) = iter.next() {
            let Some(prod) = self.productions.get(*name) else {
                bail!("could not find grammar production named `{name}`");
            };
            prod.render_markdown(cx, output);
            if iter.peek().is_some() {
                output.push_str("\n");
            }
        }
        Ok(())
    }
}

/// The HTML id for the production.
pub fn markdown_id(name: &str, for_summary: bool) -> String {
    if for_summary {
        format!("grammar-summary-{}", name)
    } else {
        format!("grammar-{}", name)
    }
}

impl Production {
    fn render_markdown(&self, cx: &RenderCtx, output: &mut String) {
        let dest = cx
            .rr_link_map
            .get(&self.name)
            .map(|path| path.to_string())
            .unwrap_or_else(|| format!("missing"));
        write!(
            output,
            "<span class=\"grammar-text grammar-production\" id=\"{id}\" \
               onclick=\"show_railroad()\"\
             >\
               [{name}]({dest})\
             </span> → ",
            id = markdown_id(&self.name, cx.for_summary),
            name = self.name,
        )
        .unwrap();
        self.expression.render_markdown(cx, output);
        output.push('\n');
    }
}

impl Expression {
    /// Returns the last [`ExpressionKind`] of this expression.
    fn last(&self) -> &ExpressionKind {
        match &self.kind {
            ExpressionKind::Alt(es) | ExpressionKind::Sequence(es) => es.last().unwrap().last(),
            ExpressionKind::Grouped(_)
            | ExpressionKind::Optional(_)
            | ExpressionKind::Repeat(_)
            | ExpressionKind::RepeatNonGreedy(_)
            | ExpressionKind::RepeatPlus(_)
            | ExpressionKind::RepeatPlusNonGreedy(_)
            | ExpressionKind::RepeatRange(_, _, _)
            | ExpressionKind::Nt(_)
            | ExpressionKind::Terminal(_)
            | ExpressionKind::Prose(_)
            | ExpressionKind::Break(_)
            | ExpressionKind::Charset(_)
            | ExpressionKind::NegExpression(_)
            | ExpressionKind::Unicode(_) => &self.kind,
        }
    }

    fn render_markdown(&self, cx: &RenderCtx, output: &mut String) {
        match &self.kind {
            ExpressionKind::Grouped(e) => {
                output.push_str("( ");
                e.render_markdown(cx, output);
                if !matches!(e.last(), ExpressionKind::Break(_)) {
                    output.push(' ');
                }
                output.push(')');
            }
            ExpressionKind::Alt(es) => {
                let mut iter = es.iter().peekable();
                while let Some(e) = iter.next() {
                    e.render_markdown(cx, output);
                    if iter.peek().is_some() {
                        if !matches!(e.last(), ExpressionKind::Break(_)) {
                            output.push(' ');
                        }
                        output.push_str("| ");
                    }
                }
            }
            ExpressionKind::Sequence(es) => {
                let mut iter = es.iter().peekable();
                while let Some(e) = iter.next() {
                    e.render_markdown(cx, output);
                    if iter.peek().is_some() && !matches!(e.last(), ExpressionKind::Break(_)) {
                        output.push(' ');
                    }
                }
            }
            ExpressionKind::Optional(e) => {
                e.render_markdown(cx, output);
                output.push_str("<sup>?</sup>");
            }
            ExpressionKind::Repeat(e) => {
                e.render_markdown(cx, output);
                output.push_str("<sup>\\*</sup>");
            }
            ExpressionKind::RepeatNonGreedy(e) => {
                e.render_markdown(cx, output);
                output.push_str("<sup>\\* (non-greedy)</sup>");
            }
            ExpressionKind::RepeatPlus(e) => {
                e.render_markdown(cx, output);
                output.push_str("<sup>+</sup>");
            }
            ExpressionKind::RepeatPlusNonGreedy(e) => {
                e.render_markdown(cx, output);
                output.push_str("<sup>+ (non-greedy)</sup>");
            }
            ExpressionKind::RepeatRange(e, a, b) => {
                e.render_markdown(cx, output);
                write!(
                    output,
                    "<sup>{}..{}</sup>",
                    a.map(|v| v.to_string()).unwrap_or_default(),
                    b.map(|v| v.to_string()).unwrap_or_default(),
                )
                .unwrap();
            }
            ExpressionKind::Nt(nt) => {
                let dest = cx.md_link_map.get(nt).map_or("missing", |d| d.as_str());
                write!(output, "<span class=\"grammar-text\">[{nt}]({dest})</span>").unwrap();
            }
            ExpressionKind::Terminal(t) => {
                write!(
                    output,
                    "<span class=\"grammar-literal\">{}</span>",
                    markdown_escape(t)
                )
                .unwrap();
            }
            ExpressionKind::Prose(s) => {
                write!(output, "<span class=\"grammar-text\">\\<{s}\\></span>").unwrap();
            }
            ExpressionKind::Break(indent) => {
                output.push_str("\\\n");
                output.push_str(&"&nbsp;".repeat(*indent));
            }
            ExpressionKind::Charset(set) => charset_render_markdown(cx, set, output),
            ExpressionKind::NegExpression(e) => {
                output.push('~');
                e.render_markdown(cx, output);
            }
            ExpressionKind::Unicode(s) => {
                output.push_str("U+");
                output.push_str(s);
            }
        }
        if let Some(suffix) = &self.suffix {
            write!(output, "<sub class=\"grammar-text\">{suffix}</sub>").unwrap();
        }
        if !cx.for_summary {
            if let Some(footnote) = &self.footnote {
                // The `ZeroWidthSpace` is to avoid conflicts with markdown link
                // references.
                write!(output, "&ZeroWidthSpace;[^{footnote}]").unwrap();
            }
        }
    }
}

fn charset_render_markdown(cx: &RenderCtx, set: &[Characters], output: &mut String) {
    output.push_str("\\[");
    let mut iter = set.iter().peekable();
    while let Some(chars) = iter.next() {
        chars.render_markdown(cx, output);
        if iter.peek().is_some() {
            output.push(' ');
        }
    }
    output.push(']');
}

impl Characters {
    fn render_markdown(&self, cx: &RenderCtx, output: &mut String) {
        match self {
            Characters::Named(s) => {
                let dest = cx.md_link_map.get(s).map_or("missing", |d| d.as_str());
                write!(output, "[{s}]({dest})").unwrap();
            }
            Characters::Terminal(s) => write!(
                output,
                "<span class=\"grammar-literal\">{}</span>",
                markdown_escape(s)
            )
            .unwrap(),
            Characters::Range(a, b) => write!(
                output,
                "<span class=\"grammar-literal\">{a}\
                 </span>-<span class=\"grammar-literal\">{b}</span>"
            )
            .unwrap(),
        }
    }
}

/// Escapes characters that markdown would otherwise interpret.
fn markdown_escape(s: &str) -> Cow<'_, str> {
    static ESC_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"[\\`_*\[\](){}'".-]"#).unwrap());
    ESC_RE.replace_all(s, r"\$0")
}
