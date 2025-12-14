//! Renders the grammar to markdown.

use super::RenderCtx;
use crate::grammar::Grammar;
use anyhow::bail;
use grammar::{Characters, Expression, ExpressionKind, Production};
use regex::Regex;
use std::borrow::Cow;
use std::fmt::Write;
use std::sync::LazyLock;

pub fn render_markdown(
    grammar: &Grammar,
    cx: &RenderCtx,
    names: &[&str],
    output: &mut String,
) -> anyhow::Result<()> {
    let mut iter = names.into_iter().peekable();
    while let Some(name) = iter.next() {
        let Some(prod) = grammar.productions.get(*name) else {
            bail!("could not find grammar production named `{name}`");
        };
        render_production(prod, cx, output);
        if iter.peek().is_some() {
            output.push_str("\n");
        }
    }
    Ok(())
}

/// The HTML id for the production.
pub fn markdown_id(name: &str, for_summary: bool) -> String {
    if for_summary {
        format!("grammar-summary-{}", name)
    } else {
        format!("grammar-{}", name)
    }
}

fn render_production(prod: &Production, cx: &RenderCtx, output: &mut String) {
    let dest = cx
        .rr_link_map
        .get(&prod.name)
        .map(|path| path.to_string())
        .unwrap_or_else(|| format!("missing"));
    for expr in &prod.comments {
        render_expression(expr, cx, output);
    }
    write!(
        output,
        "<span class=\"grammar-text grammar-production\" id=\"{id}\" \
           onclick=\"show_railroad()\"\
         >\
           [{name}]({dest})\
         </span> → ",
        id = markdown_id(&prod.name, cx.for_summary),
        name = prod.name,
    )
    .unwrap();
    render_expression(&prod.expression, cx, output);
    output.push('\n');
}

/// Returns the last [`ExpressionKind`] of this expression.
fn last_expr(expr: &Expression) -> &ExpressionKind {
    match &expr.kind {
        ExpressionKind::Alt(es) | ExpressionKind::Sequence(es) => last_expr(es.last().unwrap()),
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
        | ExpressionKind::Comment(_)
        | ExpressionKind::Charset(_)
        | ExpressionKind::NegExpression(_)
        | ExpressionKind::Cut(_)
        | ExpressionKind::Unicode(_) => &expr.kind,
    }
}

fn render_expression(expr: &Expression, cx: &RenderCtx, output: &mut String) {
    match &expr.kind {
        ExpressionKind::Grouped(e) => {
            output.push_str("( ");
            render_expression(e, cx, output);
            if !matches!(last_expr(e), ExpressionKind::Break(_)) {
                output.push(' ');
            }
            output.push(')');
        }
        ExpressionKind::Alt(es) => {
            let mut iter = es.iter().peekable();
            while let Some(e) = iter.next() {
                render_expression(e, cx, output);
                if iter.peek().is_some() {
                    if !matches!(last_expr(e), ExpressionKind::Break(_)) {
                        output.push(' ');
                    }
                    output.push_str("| ");
                }
            }
        }
        ExpressionKind::Sequence(es) => {
            let mut iter = es.iter().peekable();
            while let Some(e) = iter.next() {
                render_expression(e, cx, output);
                if iter.peek().is_some() && !matches!(last_expr(e), ExpressionKind::Break(_)) {
                    output.push(' ');
                }
            }
        }
        ExpressionKind::Optional(e) => {
            render_expression(e, cx, output);
            output.push_str("<sup>?</sup>");
        }
        ExpressionKind::Repeat(e) => {
            render_expression(e, cx, output);
            output.push_str("<sup>\\*</sup>");
        }
        ExpressionKind::RepeatNonGreedy(e) => {
            render_expression(e, cx, output);
            output.push_str("<sup>\\* (non-greedy)</sup>");
        }
        ExpressionKind::RepeatPlus(e) => {
            render_expression(e, cx, output);
            output.push_str("<sup>+</sup>");
        }
        ExpressionKind::RepeatPlusNonGreedy(e) => {
            render_expression(e, cx, output);
            output.push_str("<sup>+ (non-greedy)</sup>");
        }
        ExpressionKind::RepeatRange(e, a, b) => {
            render_expression(e, cx, output);
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
        ExpressionKind::Comment(s) => {
            write!(output, "<span class=\"grammar-comment\">// {s}</span>").unwrap();
        }
        ExpressionKind::Charset(set) => charset_render_markdown(cx, set, output),
        ExpressionKind::NegExpression(e) => {
            output.push('~');
            render_expression(e, cx, output);
        }
        ExpressionKind::Cut(e) => {
            output.push_str("^ ");
            render_expression(e, cx, output);
        }
        ExpressionKind::Unicode(s) => {
            output.push_str("U+");
            output.push_str(s);
        }
    }
    if let Some(suffix) = &expr.suffix {
        write!(output, "<sub class=\"grammar-text\">{suffix}</sub>").unwrap();
    }
    if !cx.for_summary {
        if let Some(footnote) = &expr.footnote {
            // The `ZeroWidthSpace` is to avoid conflicts with markdown link
            // references.
            write!(output, "&ZeroWidthSpace;[^{footnote}]").unwrap();
        }
    }
}

fn charset_render_markdown(cx: &RenderCtx, set: &[Characters], output: &mut String) {
    output.push_str("\\[");
    let mut iter = set.iter().peekable();
    while let Some(chars) = iter.next() {
        render_characters(chars, cx, output);
        if iter.peek().is_some() {
            output.push(' ');
        }
    }
    output.push(']');
}

fn render_characters(chars: &Characters, cx: &RenderCtx, output: &mut String) {
    match chars {
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

/// Escapes characters that markdown would otherwise interpret.
fn markdown_escape(s: &str) -> Cow<'_, str> {
    static ESC_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"[\\`_*\[\](){}'".-]"#).unwrap());
    ESC_RE.replace_all(s, r"\$0")
}
