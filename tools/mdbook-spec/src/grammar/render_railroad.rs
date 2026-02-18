//! Converts a [`Grammar`] to an SVG railroad diagram.

use super::RenderCtx;
use crate::grammar::Grammar;
use anyhow::bail;
use grammar::{Character, Characters, Expression, ExpressionKind, Production, RangeLimit};
use railroad::*;
use regex::Regex;
use std::fmt::Write;
use std::sync::LazyLock;

pub fn render_railroad(
    grammar: &Grammar,
    cx: &RenderCtx,
    names: &[&str],
    output: &mut String,
) -> anyhow::Result<()> {
    for name in names {
        let prod = match grammar.productions.get(*name) {
            Some(p) => p,
            None => bail!("could not find grammar production named `{name}`"),
        };
        render_production(prod, cx, output);
    }
    Ok(())
}

/// The HTML id for the production.
pub fn railroad_id(name: &str, for_summary: bool) -> String {
    if for_summary {
        format!("railroad-summary-{}", name)
    } else {
        format!("railroad-{}", name)
    }
}

fn render_production(prod: &Production, cx: &RenderCtx, output: &mut String) {
    let mut dia = make_diagram(prod, cx, false);
    // If the diagram is very wide, try stacking it to reduce the width.
    // This 900 is somewhat arbitrary based on looking at productions that
    // looked too squished. If your diagram is still too squished,
    // consider adding more rules to shorten it.
    if dia.width() > 900 {
        dia = make_diagram(prod, cx, true);
    }
    writeln!(
        output,
        "<div style=\"width: {width}px; height: auto; max-width: 100%; max-height: 100%\" \
                class=\"railroad-production\" \
                id=\"{id}\">{dia}</div>",
        width = dia.width(),
        id = railroad_id(&prod.name, cx.for_summary),
    )
    .unwrap();
}

fn make_diagram(prod: &Production, cx: &RenderCtx, stack: bool) -> Diagram<Box<dyn Node>> {
    let n = render_expression(&prod.expression, cx, stack);
    let dest = cx
        .md_link_map
        .get(&prod.name)
        .map(|path| path.to_string())
        .unwrap_or_else(|| format!("missing"));
    let seq: Sequence<Box<dyn Node>> =
        Sequence::new(vec![Box::new(SimpleStart), n.unwrap(), Box::new(SimpleEnd)]);
    let vert = VerticalGrid::<Box<dyn Node>>::new(vec![
        Box::new(Link::new(Comment::new(prod.name.clone()), dest)),
        Box::new(seq),
    ]);

    Diagram::new(Box::new(vert))
}

fn render_expression(expr: &Expression, cx: &RenderCtx, stack: bool) -> Option<Box<dyn Node>> {
    let mut state;
    let mut state_ref = &expr.kind;
    let n: Box<dyn Node> = 'l: loop {
        state_ref = 'cont: {
            break 'l match state_ref {
                // Render grouped nodes and `e{1..1}` repeats directly.
                ExpressionKind::Grouped(e)
                | ExpressionKind::RepeatRange {
                    expr: e,
                    min: Some(1),
                    max: Some(1),
                    limit: RangeLimit::Closed,
                } => render_expression(e, cx, stack)?,
                ExpressionKind::Alt(es) => {
                    let choices: Vec<_> = es
                        .iter()
                        .map(|e| render_expression(e, cx, stack))
                        .filter_map(|n| n)
                        .collect();
                    Box::new(Choice::<Box<dyn Node>>::new(choices))
                }
                ExpressionKind::Sequence(es) => {
                    let es: Vec<_> = es.iter().collect();
                    let make_seq = |es: &[&Expression]| {
                        let seq: Vec<_> = es
                            .iter()
                            .map(|e| render_expression(e, cx, stack))
                            .filter_map(|n| n)
                            .collect();
                        if seq.is_empty() {
                            return None;
                        }
                        let seq: Sequence<Box<dyn Node>> = Sequence::new(seq);
                        Some(Box::new(seq))
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

                        let mut breaks: Vec<_> = es
                            .split(|e| e.is_break())
                            .flat_map(|es| make_seq(es))
                            .collect();
                        // If there aren't any breaks, don't bother stacking.
                        match breaks.len() {
                            0 => return None,
                            1 => breaks.pop().unwrap(),
                            _ => Box::new(Stack::new(breaks)),
                        }
                    } else {
                        make_seq(&es)?
                    }
                }
                ExpressionKind::NegativeLookahead(e) => {
                    let forward = render_expression(e, cx, stack)?;
                    let lbox =
                        LabeledBox::new(forward, Comment::new("not followed by".to_string()));
                    Box::new(lbox)
                }
                // Treat `e?` and `e{..=1}` / `e{0..=1}` equally.
                ExpressionKind::Optional(e)
                | ExpressionKind::RepeatRange {
                    expr: e,
                    min: None | Some(0),
                    max: Some(1),
                    limit: RangeLimit::Closed,
                } => {
                    let n = render_expression(e, cx, stack)?;
                    Box::new(Optional::new(n))
                }
                // Treat `e*` and `e{..}` / `e{0..}` equally.
                ExpressionKind::Repeat(e)
                | ExpressionKind::RepeatRange {
                    expr: e,
                    min: None | Some(0),
                    max: None,
                    limit: RangeLimit::HalfOpen,
                } => {
                    let n = render_expression(e, cx, stack)?;
                    Box::new(Optional::new(Repeat::new(n, railroad::Empty)))
                }
                ExpressionKind::RepeatNonGreedy(e) => {
                    let n = render_expression(e, cx, stack)?;
                    let r = Box::new(Optional::new(Repeat::new(n, railroad::Empty)));
                    let lbox = LabeledBox::new(r, Comment::new("non-greedy".to_string()));
                    Box::new(lbox)
                }
                // Treat `e+` and `e{1..}` equally.
                ExpressionKind::RepeatPlus(e)
                | ExpressionKind::RepeatRange {
                    expr: e,
                    min: Some(1),
                    max: None,
                    limit: RangeLimit::HalfOpen,
                } => {
                    let n = render_expression(e, cx, stack)?;
                    Box::new(Repeat::new(n, railroad::Empty))
                }
                ExpressionKind::RepeatPlusNonGreedy(e) => {
                    let n = render_expression(e, cx, stack)?;
                    let r = Repeat::new(n, railroad::Empty);
                    let lbox = LabeledBox::new(r, Comment::new("non-greedy".to_string()));
                    Box::new(lbox)
                }
                // For `e{..=0}` / `e{0..=0}` or `e{..1}` / `e{0..1}` render an empty node.
                ExpressionKind::RepeatRange { max: Some(0), .. }
                | ExpressionKind::RepeatRange {
                    max: Some(1),
                    limit: RangeLimit::HalfOpen,
                    ..
                } => Box::new(railroad::Empty),
                // Treat `e{..b}` / `e{0..b}` / `e{..=b}` / `e{0..=b}` as
                // `(e{1..=b})?` (or `(e{1..b})?` for half-open).
                ExpressionKind::RepeatRange {
                    expr: e,
                    min: None | Some(0),
                    max: Some(b @ 2..),
                    limit,
                } => {
                    state = ExpressionKind::Optional(Box::new(Expression::new_kind(
                        ExpressionKind::RepeatRange {
                            expr: e.clone(),
                            min: Some(1),
                            max: Some(*b),
                            limit: *limit,
                        },
                    )));
                    break 'cont &state;
                }
                // Render `e{1..b}` / `e{1..=b}` directly.
                ExpressionKind::RepeatRange {
                    expr: e,
                    min: Some(1),
                    max: Some(b @ 2..),
                    limit,
                } => {
                    let n = render_expression(e, cx, stack)?;
                    let more = match limit {
                        RangeLimit::HalfOpen => b - 2,
                        RangeLimit::Closed => b - 1,
                    };
                    let cmt = format!("at most {more} more times");
                    let r = Repeat::new(n, Comment::new(cmt));
                    Box::new(r)
                }
                // A half-open range where min >= max is empty (e.g.,
                // `e{2..2}` means zero repetitions).
                ExpressionKind::RepeatRange {
                    min: Some(a),
                    max: Some(b),
                    limit: RangeLimit::HalfOpen,
                    ..
                } if b <= a => Box::new(railroad::Empty),

                // Decompose ranges with min >= 2 into a fixed prefix
                // and a remainder:
                // - `e{a..}` as `e{0..a-1} e{1..}`
                // - `e{a..=b}` as `e{0..a-1} e{1..=b-(a-1)}`
                // - `e{a..b}` as `e{0..a-1} e{1..b-(a-1)}`
                ExpressionKind::RepeatRange {
                    expr: e,
                    min: Some(a @ 2..),
                    max: b @ None,
                    limit,
                }
                | ExpressionKind::RepeatRange {
                    expr: e,
                    min: Some(a @ 2..),
                    max: b @ Some(_),
                    limit,
                } => {
                    let mut es = Vec::<Expression>::new();
                    for _ in 0..(a - 1) {
                        es.push(*e.clone());
                    }
                    es.push(Expression::new_kind(ExpressionKind::RepeatRange {
                        expr: e.clone(),
                        min: Some(1),
                        max: b.map(|x| x - (a - 1)),
                        limit: *limit,
                    }));
                    state = ExpressionKind::Sequence(es);
                    break 'cont &state;
                }
                ExpressionKind::RepeatRange {
                    max: None,
                    limit: RangeLimit::Closed,
                    ..
                } => unreachable!("closed range must have upper bound"),
                ExpressionKind::Nt(nt) => node_for_nt(cx, nt),
                ExpressionKind::Terminal(t) => Box::new(Terminal::new(t.clone())),
                ExpressionKind::Prose(s) => Box::new(Terminal::new(s.clone())),
                ExpressionKind::Break(_) => return None,
                ExpressionKind::Comment(_) => return None,
                ExpressionKind::Charset(set) => {
                    let ns: Vec<_> = set.iter().map(|c| render_characters(c, cx)).collect();
                    Box::new(Choice::<Box<dyn Node>>::new(ns))
                }
                ExpressionKind::NegExpression(e) => {
                    let n = render_expression(e, cx, stack)?;
                    let ch = node_for_nt(cx, "CHAR");
                    Box::new(Except::new(Box::new(ch), n))
                }
                ExpressionKind::Cut(e) => {
                    let rhs = render_expression(e, cx, stack)?;
                    let lbox = LabeledBox::new(rhs, Comment::new("no backtracking".to_string()));
                    Box::new(lbox)
                }
                ExpressionKind::Unicode((_, s)) => Box::new(Terminal::new(format!("U+{}", s))),
            };
        }
    };
    if let Some(suffix) = &expr.suffix {
        let suffix = strip_markdown(suffix);
        let lbox = LabeledBox::new(n, Comment::new(suffix));
        return Some(Box::new(lbox));
    }
    // Note: Footnotes aren't supported. They could be added as a comment
    // on a vertical stack or a LabeledBox or something like that, but I
    // don't feel like bothering.
    Some(n)
}

fn render_characters(chars: &Characters, cx: &RenderCtx) -> Box<dyn Node> {
    match chars {
        Characters::Named(s) => node_for_nt(cx, s),
        Characters::Terminal(s) => Box::new(Terminal::new(s.clone())),
        Characters::Range(a, b) => {
            let mut s = String::new();
            let write_ch = |ch: &Character, output: &mut String| match ch {
                Character::Char(ch) => output.push(*ch),
                Character::Unicode((_, s)) => write!(output, "U+{s}").unwrap(),
            };
            write_ch(a, &mut s);
            s.push('-');
            write_ch(b, &mut s);
            Box::new(Terminal::new(s))
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

#[cfg(test)]
mod tests {
    use super::*;
    use grammar::{Character, Characters, Expression, ExpressionKind, RangeLimit};

    /// Render an expression to an SVG string fragment.
    fn render_to_svg(expr: &Expression) -> Option<String> {
        let cx = RenderCtx::for_test();
        let node = render_expression(expr, &cx, false)?;
        let svg = node.draw(0, 0, svg::HDir::LTR);
        Some(svg.to_string())
    }

    /// Build a `RepeatRange` expression wrapping a nonterminal `e`.
    fn range_expr(min: Option<u32>, max: Option<u32>, limit: RangeLimit) -> Expression {
        Expression::new_kind(ExpressionKind::RepeatRange {
            expr: Box::new(Expression::new_kind(ExpressionKind::Nt("e".to_string()))),
            min,
            max,
            limit,
        })
    }

    // -- RepeatRange tests --

    #[test]
    fn test_empty_exclusive_equal() {
        // `e{2..2}` (half-open, min == max) renders as empty.
        let expr = range_expr(Some(2), Some(2), RangeLimit::HalfOpen);
        let svg = render_to_svg(&expr).unwrap();
        // An empty node produces a minimal SVG path with no
        // nonterminal content.
        assert!(
            !svg.contains("nonterminal"),
            "expected empty rendering for e{{2..2}}, got: {svg}"
        );
    }

    #[test]
    fn test_empty_inverted() {
        // `e{3..1}` (half-open, max < min) renders as empty.
        let expr = range_expr(Some(3), Some(1), RangeLimit::HalfOpen);
        let svg = render_to_svg(&expr).unwrap();
        assert!(
            !svg.contains("nonterminal"),
            "expected empty rendering for e{{3..1}}, got: {svg}"
        );
    }

    #[test]
    fn test_closed_exact_one() {
        // `e{1..=1}` renders as a single `e` (no repeat).
        let expr = range_expr(Some(1), Some(1), RangeLimit::Closed);
        let svg = render_to_svg(&expr).unwrap();
        assert!(
            svg.contains("nonterminal"),
            "expected nonterminal for e{{1..=1}}, got: {svg}"
        );
        // Should not contain "more times" (no repeat comment).
        assert!(
            !svg.contains("more times"),
            "e{{1..=1}} should not show a repeat comment"
        );
    }

    #[test]
    fn test_closed_range() {
        // `e{2..=4}` renders with repeat indicators.
        let expr = range_expr(Some(2), Some(4), RangeLimit::Closed);
        let svg = render_to_svg(&expr).unwrap();
        assert!(
            svg.contains("nonterminal"),
            "expected nonterminal for e{{2..=4}}, got: {svg}"
        );
        assert!(
            svg.contains("more times"),
            "e{{2..=4}} should show a repeat comment"
        );
    }

    #[test]
    fn test_closed_optional() {
        // `e{..=1}` renders as optional.
        let expr = range_expr(None, Some(1), RangeLimit::Closed);
        let svg = render_to_svg(&expr).unwrap();
        assert!(
            svg.contains("nonterminal"),
            "expected nonterminal for e{{..=1}}, got: {svg}"
        );
    }

    // -- Negative lookahead tests --

    #[test]
    fn lookahead_nonterminal() {
        let expr = Expression::new_kind(ExpressionKind::NegativeLookahead(Box::new(
            Expression::new_kind(ExpressionKind::Nt("CHAR".to_string())),
        )));
        let svg = render_to_svg(&expr).unwrap();
        assert!(
            svg.contains("not followed by"),
            "should contain the 'not followed by' label"
        );
        assert!(svg.contains("CHAR"), "should contain the nonterminal name");
    }

    #[test]
    fn lookahead_terminal() {
        let expr = Expression::new_kind(ExpressionKind::NegativeLookahead(Box::new(
            Expression::new_kind(ExpressionKind::Terminal("CR".to_string())),
        )));
        let svg = render_to_svg(&expr).unwrap();
        assert!(svg.contains("not followed by"));
        assert!(svg.contains("CR"));
    }

    #[test]
    fn lookahead_charset() {
        let expr = Expression::new_kind(ExpressionKind::NegativeLookahead(Box::new(
            Expression::new_kind(ExpressionKind::Charset(vec![
                Characters::Terminal("e".to_string()),
                Characters::Terminal("E".to_string()),
            ])),
        )));
        let svg = render_to_svg(&expr).unwrap();
        assert!(svg.contains("not followed by"));
        assert!(svg.contains("e"));
        assert!(svg.contains("E"));
    }

    // -- Unicode tests --

    #[test]
    fn unicode_4_digit() {
        let expr = Expression::new_kind(ExpressionKind::Unicode(('\t', "0009".to_string())));
        let svg = render_to_svg(&expr).unwrap();
        assert!(svg.contains("U+0009"), "should render Unicode code point");
    }

    #[test]
    fn unicode_6_digit() {
        let expr = Expression::new_kind(ExpressionKind::Unicode((
            '\u{10FFFF}',
            "10FFFF".to_string(),
        )));
        let svg = render_to_svg(&expr).unwrap();
        assert!(svg.contains("U+10FFFF"));
    }

    // -- Charset with ranges --

    #[test]
    fn charset_unicode_range() {
        let expr = Expression::new_kind(ExpressionKind::Charset(vec![Characters::Range(
            Character::Unicode(('\0', "0000".to_string())),
            Character::Unicode(('\u{007F}', "007F".to_string())),
        )]));
        let svg = render_to_svg(&expr).unwrap();
        assert!(svg.contains("U+0000"));
        assert!(svg.contains("U+007F"));
    }

    #[test]
    fn charset_char_range() {
        let expr = Expression::new_kind(ExpressionKind::Charset(vec![Characters::Range(
            Character::Char('a'),
            Character::Char('z'),
        )]));
        let svg = render_to_svg(&expr).unwrap();
        assert!(svg.contains("a"));
        assert!(svg.contains("z"));
    }

    // -- Cut test --

    #[test]
    fn cut_rendering() {
        let expr = Expression::new_kind(ExpressionKind::Cut(Box::new(Expression::new_kind(
            ExpressionKind::Nt("Foo".to_string()),
        ))));
        let svg = render_to_svg(&expr).unwrap();
        assert!(
            svg.contains("no backtracking"),
            "cut should render with 'no backtracking' label"
        );
        assert!(svg.contains("Foo"));
    }

    // -- NegExpression test --

    #[test]
    fn neg_expression_rendering() {
        let expr = Expression::new_kind(ExpressionKind::NegExpression(Box::new(
            Expression::new_kind(ExpressionKind::Charset(vec![Characters::Terminal(
                "a".to_string(),
            )])),
        )));
        let svg = render_to_svg(&expr).unwrap();
        assert!(
            svg.contains("with the exception of"),
            "neg expression should have exception label"
        );
    }
}
