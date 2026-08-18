//! Renders the grammar to markdown.

use super::RenderCtx;
use crate::grammar::Grammar;
use anyhow::bail;
use grammar::{Character, Expression, ExpressionKind, Production};
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

fn render_expression(expr: &Expression, cx: &RenderCtx, output: &mut String) {
    match &expr.kind {
        ExpressionKind::Grouped(e) => {
            output.push_str("( ");
            render_expression(e, cx, output);
            if !e.last_expr().is_break() {
                output.push(' ');
            }
            output.push(')');
        }
        ExpressionKind::Alt(es) => {
            let mut iter = es.iter().peekable();
            while let Some(e) = iter.next() {
                render_expression(e, cx, output);
                if iter.peek().is_some() {
                    if !e.last_expr().is_break() {
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
                if iter.peek().is_some() && !e.last_expr().is_break() {
                    output.push(' ');
                }
            }
        }
        ExpressionKind::Optional(e) => {
            render_expression(e, cx, output);
            output.push_str("<sup>?</sup>");
        }
        ExpressionKind::NegativeLookahead(e) => {
            output.push('!');
            render_expression(e, cx, output);
        }
        ExpressionKind::Repeat(e) => {
            render_expression(e, cx, output);
            output.push_str("<sup>\\*</sup>");
        }
        ExpressionKind::RepeatPlus(e) => {
            render_expression(e, cx, output);
            output.push_str("<sup>+</sup>");
        }
        ExpressionKind::RepeatRange {
            expr,
            name,
            min,
            max,
            limit,
        } => {
            render_expression(expr, cx, output);
            write!(
                output,
                "<sup>{name}{min}{limit}{max}</sup>",
                name = name.as_ref().map(|n| format!("{n}:")).unwrap_or_default(),
                min = min.map(|v| v.to_string()).unwrap_or_default(),
                max = max.map(|v| v.to_string()).unwrap_or_default(),
            )
            .unwrap();
        }
        ExpressionKind::RepeatRangeNamed(e, name) => {
            render_expression(e, cx, output);
            write!(output, "<sup>{name}</sup>").unwrap();
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
        ExpressionKind::CharacterRange(start, end) => {
            let write_ch = |ch: &Character, output: &mut String| match ch {
                Character::Char(ch) => write!(
                    output,
                    "<span class=\"grammar-literal\">{}</span>",
                    markdown_escape(&ch.to_string())
                )
                .unwrap(),
                Character::Unicode((_, s)) => write!(output, "U+{s}").unwrap(),
            };
            write_ch(start, output);
            output.push('-');
            write_ch(end, output);
        }
        ExpressionKind::NegExpression(e) => {
            output.push('~');
            render_expression(e, cx, output);
        }
        ExpressionKind::Cut(e) => {
            output.push_str("^ ");
            render_expression(e, cx, output);
        }
        ExpressionKind::Unicode((_, s)) => {
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

fn charset_render_markdown(cx: &RenderCtx, set: &[Expression], output: &mut String) {
    output.push_str("\\[");
    let mut iter = set.iter().peekable();
    while let Some(expr) = iter.next() {
        render_expression(expr, cx, output);
        if iter.peek().is_some() {
            output.push(' ');
        }
    }
    output.push(']');
}

/// Escapes characters that markdown would otherwise interpret.
fn markdown_escape(s: &str) -> Cow<'_, str> {
    static ESC_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"[\\`_*\[\](){}'".-]"#).unwrap());
    ESC_RE.replace_all(s, r"\$0")
}

#[cfg(test)]
mod tests {
    use super::*;
    use grammar::RangeLimit;
    use std::collections::HashMap;

    /// Creates a minimal `RenderCtx` for testing.
    fn test_cx() -> RenderCtx {
        RenderCtx {
            md_link_map: HashMap::new(),
            rr_link_map: HashMap::new(),
            for_summary: false,
        }
    }

    /// Renders a single expression to a markdown string.
    fn render(kind: ExpressionKind) -> String {
        let cx = test_cx();
        let expr = Expression::new_kind(kind, 0);
        let mut output = String::new();
        render_expression(&expr, &cx, &mut output);
        output
    }

    // -- Negative lookahead tests --

    #[test]
    fn lookahead_nonterminal() {
        let result = render(ExpressionKind::NegativeLookahead(Box::new(
            Expression::new_kind(ExpressionKind::Nt("CHAR".to_string()), 0),
        )));
        assert!(result.contains("!"), "should contain `!` prefix");
        assert!(
            result.contains("CHAR"),
            "should contain the nonterminal name"
        );
    }

    #[test]
    fn lookahead_terminal() {
        let result = render(ExpressionKind::NegativeLookahead(Box::new(
            Expression::new_kind(ExpressionKind::Terminal("'".to_string()), 0),
        )));
        assert!(result.starts_with("!"), "should start with `!`");
        assert!(
            result.contains("grammar-literal"),
            "should render inner terminal as a grammar literal"
        );
    }

    #[test]
    fn lookahead_charset() {
        let result = render(ExpressionKind::NegativeLookahead(Box::new(
            Expression::new_kind(
                ExpressionKind::Charset(vec![
                    Expression::new_kind(ExpressionKind::Terminal("e".to_string()), 0),
                    Expression::new_kind(ExpressionKind::Terminal("E".to_string()), 0),
                ]),
                0,
            ),
        )));
        assert!(result.starts_with("!"), "should start with `!`");
        assert!(
            result.contains("\\["),
            "should contain escaped opening bracket for charset"
        );
    }

    #[test]
    fn lookahead_grouped() {
        // !( `.` | `_` )
        let inner = ExpressionKind::Grouped(Box::new(Expression::new_kind(
            ExpressionKind::Alt(vec![
                Expression::new_kind(ExpressionKind::Terminal(".".to_string()), 0),
                Expression::new_kind(ExpressionKind::Terminal("_".to_string()), 0),
            ]),
            0,
        )));
        let result = render(ExpressionKind::NegativeLookahead(Box::new(
            Expression::new_kind(inner, 0),
        )));
        assert!(result.starts_with("!("));
        assert!(result.contains("|"));
    }

    // -- Unicode tests --

    #[test]
    fn unicode_4_digit() {
        let result = render(ExpressionKind::Unicode(('\t', "0009".to_string())));
        assert_eq!(result, "U+0009");
    }

    #[test]
    fn unicode_6_digit() {
        let result = render(ExpressionKind::Unicode((
            '\u{10FFFF}',
            "10FFFF".to_string(),
        )));
        assert_eq!(result, "U+10FFFF");
    }

    // -- Charset with Unicode range tests --

    #[test]
    fn charset_unicode_range() {
        let result = render(ExpressionKind::Charset(vec![Expression::new_kind(
            ExpressionKind::CharacterRange(
                Character::Unicode(('\0', "0000".to_string())),
                Character::Unicode(('\u{007F}', "007F".to_string())),
            ),
            0,
        )]));
        assert!(result.contains("\\["));
        assert!(result.contains("U+0000"));
        assert!(result.contains("U+007F"));
        assert!(result.contains("-"));
    }

    #[test]
    fn charset_char_range() {
        let result = render(ExpressionKind::Charset(vec![Expression::new_kind(
            ExpressionKind::CharacterRange(Character::Char('a'), Character::Char('z')),
            0,
        )]));
        assert!(result.contains("\\["));
        assert!(result.contains("grammar-literal"));
        assert!(result.contains("-"));
    }

    #[test]
    fn charset_mixed_range() {
        // [`a`-U+007A]
        let result = render(ExpressionKind::Charset(vec![Expression::new_kind(
            ExpressionKind::CharacterRange(
                Character::Char('a'),
                Character::Unicode(('\u{007A}', "007A".to_string())),
            ),
            0,
        )]));
        assert!(result.contains("grammar-literal"));
        assert!(result.contains("U+007A"));
        assert!(result.contains("-"));
    }

    // -- Cut test --

    #[test]
    fn cut_rendering() {
        let result = render(ExpressionKind::Cut(Box::new(Expression::new_kind(
            ExpressionKind::Nt("Foo".to_string()),
            0,
        ))));
        assert!(result.starts_with("^ "), "cut should render as `^ ` prefix");
        assert!(result.contains("Foo"));
    }

    // -- NegExpression test --

    #[test]
    fn neg_expression_rendering() {
        let result = render(ExpressionKind::NegExpression(Box::new(
            Expression::new_kind(
                ExpressionKind::Charset(vec![Expression::new_kind(
                    ExpressionKind::Terminal("a".to_string()),
                    0,
                )]),
                0,
            ),
        )));
        assert!(
            result.starts_with("~"),
            "neg expression should render as `~` prefix"
        );
    }

    // -- Markdown escape tests --

    #[test]
    fn markdown_escape_backtick() {
        assert_eq!(markdown_escape("`"), "\\`");
    }

    #[test]
    fn markdown_escape_brackets() {
        assert_eq!(markdown_escape("["), "\\[");
        assert_eq!(markdown_escape("]"), "\\]");
    }

    #[test]
    fn markdown_escape_plain() {
        assert_eq!(markdown_escape("abc"), "abc");
    }

    // -- Named repeat range tests --

    #[test]
    fn repeat_range_with_name() {
        // A RepeatRange with a name renders as `<sup>n:1..=255</sup>`.
        let result = render(ExpressionKind::RepeatRange {
            expr: Box::new(Expression::new_kind(ExpressionKind::Nt("x".to_string()), 0)),
            name: Some("n".to_string()),
            min: Some(1),
            max: Some(255),
            limit: RangeLimit::Closed,
        });
        assert!(
            result.contains("<sup>n:1..=255</sup>"),
            "expected <sup>n:1..=255</sup>, got: {result}"
        );
    }

    #[test]
    fn repeat_range_without_name() {
        // A RepeatRange without a name renders with no spurious
        // colon -- just `<sup>2..5</sup>`.
        let result = render(ExpressionKind::RepeatRange {
            expr: Box::new(Expression::new_kind(ExpressionKind::Nt("x".to_string()), 0)),
            name: None,
            min: Some(2),
            max: Some(5),
            limit: RangeLimit::HalfOpen,
        });
        assert!(
            result.contains("<sup>2..5</sup>"),
            "expected <sup>2..5</sup>, got: {result}"
        );
        assert!(
            !result.contains(":"),
            "unnamed range should not contain a colon"
        );
    }

    #[test]
    fn repeat_range_named_reference() {
        // A RepeatRangeNamed renders as `<sup>n</sup>`.
        let result = render(ExpressionKind::RepeatRangeNamed(
            Box::new(Expression::new_kind(ExpressionKind::Nt("x".to_string()), 0)),
            "n".to_string(),
        ));
        assert!(
            result.contains("<sup>n</sup>"),
            "expected <sup>n</sup>, got: {result}"
        );
    }
}
