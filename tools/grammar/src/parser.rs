//! A parser of the ENBF-like grammar.

use super::{Character, Characters, Expression, ExpressionKind, Grammar, Production, RangeLimit};
use std::fmt;
use std::fmt::Display;
use std::path::Path;

struct Parser<'a> {
    input: &'a str,
    index: usize,
}

pub struct Error {
    message: String,
    line: String,
    lineno: usize,
    col: usize,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let lineno = format!("{}", self.lineno);
        let space = " ".repeat(lineno.len() + 1);
        let col = " ".repeat(self.col);
        let line = &self.line;
        let message = &self.message;
        write!(f, "\n{space}|\n{lineno} | {line}\n{space}|{col}^ {message}")
    }
}

macro_rules! bail {
    ($parser:expr, $($arg:tt)*) => {{
        let mut msg = String::new();
        fmt::write(&mut msg, format_args!($($arg)*)).unwrap();
        return Err($parser.error(msg));
    }};
}

type Result<T> = std::result::Result<T, Error>;

pub fn parse_grammar(
    input: &str,
    grammar: &mut Grammar,
    category: &str,
    path: &Path,
) -> Result<()> {
    let mut parser = Parser { input, index: 0 };
    loop {
        let p = parser.parse_production(category, path)?;
        grammar.name_order.push(p.name.clone());
        if let Some(dupe) = grammar.productions.insert(p.name.clone(), p) {
            bail!(parser, "duplicate production {} in grammar", dupe.name);
        }
        parser.take_while(&|ch| ch == '\n');
        if parser.eof() {
            break;
        }
    }
    Ok(())
}

impl Parser<'_> {
    fn take_while(&mut self, f: &dyn Fn(char) -> bool) -> &str {
        let mut upper = 0;
        let i = self.index;
        let mut ci = self.input[i..].chars();
        while let Some(ch) = ci.next() {
            if !f(ch) {
                break;
            }
            upper += ch.len_utf8();
        }
        self.index += upper;
        &self.input[i..i + upper]
    }

    /// Returns whether or not the given string is next, and advances the head if it is.
    fn take_str(&mut self, s: &str) -> bool {
        if self.input[self.index..].starts_with(s) {
            self.index += s.len();
            true
        } else {
            false
        }
    }

    /// Returns the next byte, or None if eof.
    fn peek(&mut self) -> Option<u8> {
        if self.index >= self.input.len() {
            None
        } else {
            Some(self.input.as_bytes()[self.index])
        }
    }

    fn eof(&mut self) -> bool {
        self.index >= self.input.len()
    }

    /// Expects the next input to be the given string, and advances the head.
    fn expect(&mut self, s: &str, err: &str) -> Result<()> {
        if !self.input[self.index..].starts_with(s) {
            bail!(self, "{err}");
        };
        self.index += s.len();
        Ok(())
    }

    fn error(&mut self, message: String) -> Error {
        let (line, lineno, col) = translate_position(self.input, self.index);
        Error {
            message,
            line: line.to_string(),
            lineno,
            col,
        }
    }

    /// Advances zero or more spaces.
    fn space0(&mut self) -> &str {
        self.take_while(&|ch| ch == ' ')
    }

    fn parse_production(&mut self, category: &str, path: &Path) -> Result<Production> {
        let mut comments = Vec::new();
        while let Ok(comment) = self.parse_comment() {
            self.expect("\n", "expected newline")?;
            comments.push(Expression::new_kind(comment));
            comments.push(Expression::new_kind(ExpressionKind::Break(0)));
        }
        let is_root = self.parse_is_root();
        self.space0();
        let name = self
            .parse_name()
            .ok_or_else(|| self.error("expected production name".to_string()))?;
        self.expect(" ->", "expected -> arrow")?;
        let Some(expression) = self.parse_expression()? else {
            bail!(self, "expected an expression");
        };
        Ok(Production {
            name,
            comments,
            category: category.to_string(),
            expression,
            path: path.to_owned(),
            is_root,
        })
    }

    fn parse_is_root(&mut self) -> bool {
        self.take_str("@root")
    }

    fn parse_name(&mut self) -> Option<String> {
        let name = self.take_while(&|c: char| c.is_alphanumeric() || c == '_');
        if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        }
    }

    fn parse_expression(&mut self) -> Result<Option<Expression>> {
        let mut es = Vec::new();
        loop {
            let Some(e) = self.parse_seq()? else { break };
            es.push(e);
            _ = self.space0();
            if !self.take_str("|") {
                break;
            }
        }
        match es.len() {
            0 => Ok(None),
            1 => Ok(Some(es.pop().unwrap())),
            _ => Ok(Some(Expression::new_kind(ExpressionKind::Alt(es)))),
        }
    }

    fn parse_seq(&mut self) -> Result<Option<Expression>> {
        let mut es = Vec::new();
        loop {
            self.space0();
            if self.peek() == Some(b'^') {
                let cut = self.parse_cut()?;
                es.push(cut);
                break;
            }
            let Some(e) = self.parse_expr1()? else {
                break;
            };
            es.push(e);
        }
        match es.len() {
            0 => Ok(None),
            1 => Ok(Some(es.pop().unwrap())),
            _ => Ok(Some(Expression {
                kind: ExpressionKind::Sequence(es),
                suffix: None,
                footnote: None,
            })),
        }
    }

    /// Parse cut (`^`) operator.
    fn parse_cut(&mut self) -> Result<Expression> {
        self.expect("^", "expected `^`")?;
        let Some(rhs) = self.parse_seq()? else {
            bail!(self, "expected expression after cut operator");
        };
        Ok(Expression {
            kind: ExpressionKind::Cut(Box::new(rhs)),
            suffix: None,
            footnote: None,
        })
    }

    fn parse_expr1(&mut self) -> Result<Option<Expression>> {
        let Some(next) = self.peek() else {
            return Ok(None);
        };

        let kind = if self.take_str("U+") {
            ExpressionKind::Unicode(self.parse_unicode()?)
        } else if self.input[self.index..]
            .chars()
            .next()
            .map(|ch| ch.is_alphanumeric())
            .unwrap_or(false)
        {
            self.parse_nonterminal()
                .expect("first char already checked")
        } else if self.take_str("\n") {
            if self.eof() || self.take_str("\n") {
                return Ok(None);
            }
            let space = self.take_while(&|ch| ch == ' ');
            if space.len() == 0 {
                bail!(self, "expected indentation on next line");
            }
            ExpressionKind::Break(space.len())
        } else if next == b'/' {
            self.parse_comment()?
        } else if next == b'`' {
            self.parse_terminal()?
        } else if next == b'[' {
            self.parse_charset()?
        } else if next == b'<' {
            self.parse_prose()?
        } else if next == b'(' {
            self.parse_grouped()?
        } else if next == b'~' {
            self.parse_neg_expression()?
        } else if next == b'!' {
            self.parse_negative_lookahead()?
        } else {
            return Ok(None);
        };
        let kind = match self.peek() {
            Some(b'?') => self.parse_optional(kind)?,
            Some(b'*') => self.parse_repeat(kind)?,
            Some(b'+') => self.parse_repeat_plus(kind)?,
            Some(b'{') => self.parse_repeat_range(kind)?,
            _ => kind,
        };
        let suffix = self.parse_suffix()?;
        let footnote = self.parse_footnote()?;

        Ok(Some(Expression {
            kind,
            suffix,
            footnote,
        }))
    }

    fn parse_nonterminal(&mut self) -> Option<ExpressionKind> {
        let nt = self.parse_name()?;
        Some(ExpressionKind::Nt(nt))
    }

    /// Parse terminal within backticks.
    fn parse_terminal(&mut self) -> Result<ExpressionKind> {
        Ok(ExpressionKind::Terminal(self.parse_terminal_str()?))
    }

    /// Parse string within backticks.
    fn parse_terminal_str(&mut self) -> Result<String> {
        self.expect("`", "expected opening backtick")?;
        let term = self.take_while(&|x| !['\n', '`'].contains(&x)).to_string();
        if term.is_empty() {
            bail!(self, "expected terminal");
        }
        self.expect("`", "expected closing backtick")?;
        Ok(term)
    }

    /// Parse e.g. `// Single line comment.`.
    fn parse_comment(&mut self) -> Result<ExpressionKind> {
        self.expect("//", "expected `//`")?;
        let text = self.take_while(&|x| x != '\n').to_string();
        Ok(ExpressionKind::Comment(text))
    }

    fn parse_charset(&mut self) -> Result<ExpressionKind> {
        self.expect("[", "expected opening [")?;
        let mut characters = Vec::new();
        loop {
            self.space0();
            let Some(ch) = self.parse_characters()? else {
                break;
            };
            characters.push(ch);
        }
        if characters.is_empty() {
            bail!(self, "expected at least one character in character group");
        }
        self.space0();
        self.expect("]", "expected closing ]")?;
        Ok(ExpressionKind::Charset(characters))
    }

    /// Parse an element of a character class, e.g.
    /// `` `a`-`b` `` | `` `term` `` | `` NonTerminal ``.
    fn parse_characters(&mut self) -> Result<Option<Characters>> {
        if let Some(a) = self.parse_character()? {
            if self.take_str("-") {
                let Some(b) = self.parse_character()? else {
                    bail!(self, "expected character in range");
                };
                Ok(Some(Characters::Range(a, b)))
            } else {
                //~^ Parse terminal in backticks.
                let t = match a {
                    Character::Char(ch) => ch.to_string(),
                    Character::Unicode(_) => bail!(self, "unicode not supported"),
                };
                Ok(Some(Characters::Terminal(t)))
            }
        } else if let Some(name) = self.parse_name() {
            //~^ Parse nonterminal identifier.
            Ok(Some(Characters::Named(name)))
        } else {
            Ok(None)
        }
    }

    fn parse_character(&mut self) -> Result<Option<Character>> {
        if let Some(b'`') = self.peek() {
            let recov = self.index;
            let term = self.parse_terminal_str()?;
            if term.len() > 1 {
                self.index = recov + 1;
                bail!(self, "invalid start terminal in range");
            }
            let ch = term.chars().next().unwrap();
            Ok(Some(Character::Char(ch)))
        } else if self.take_str("U+") {
            Ok(Some(Character::Unicode(self.parse_unicode()?)))
        } else {
            Ok(None)
        }
    }

    /// Parse e.g. `<prose text>`.
    fn parse_prose(&mut self) -> Result<ExpressionKind> {
        self.expect("<", "expected opening `<`")?;
        let text = self.take_while(&|x| !['\n', '>'].contains(&x)).to_string();
        if text.is_empty() {
            bail!(self, "expected prose text");
        }
        self.expect(">", "expected closing `>`")?;
        Ok(ExpressionKind::Prose(text))
    }

    fn parse_grouped(&mut self) -> Result<ExpressionKind> {
        self.expect("(", "expected opening `(`")?;
        self.space0();
        let Some(e) = self.parse_expression()? else {
            bail!(self, "expected expression in parenthesized group");
        };
        self.space0();
        self.expect(")", "expected closing `)`")?;
        Ok(ExpressionKind::Grouped(Box::new(e)))
    }

    fn parse_neg_expression(&mut self) -> Result<ExpressionKind> {
        self.expect("~", "expected ~")?;
        let Some(next) = self.peek() else {
            bail!(self, "expected expression after ~");
        };
        let kind = match next {
            b'[' => self.parse_charset()?,
            b'`' => self.parse_terminal()?,
            _ => self.parse_nonterminal().ok_or_else(|| {
                self.error("expected a charset, terminal, or name after ~ negation".to_string())
            })?,
        };
        Ok(ExpressionKind::NegExpression(box_kind(kind)))
    }

    fn parse_negative_lookahead(&mut self) -> Result<ExpressionKind> {
        self.expect("!", "expected !")?;
        self.space0();
        let Some(e) = self.parse_expr1()? else {
            bail!(self, "expected expression after !");
        };
        Ok(ExpressionKind::NegativeLookahead(Box::new(e)))
    }

    /// Parse e.g. `F00F` after `U+`.
    fn parse_unicode(&mut self) -> Result<(char, String)> {
        let mut xs = Vec::with_capacity(6);
        let mut push_next = || {
            match self.peek() {
                Some(x @ (b'0'..=b'9' | b'A'..=b'F')) => {
                    xs.push(x);
                    self.index += 1;
                }
                _ => bail!(self, "expected 4 uppercase hexadecimal digits after `U+`"),
            }
            Ok(())
        };
        for _ in 0..4 {
            push_next()?;
        }
        for _ in 0..2 {
            if push_next().is_err() {
                break;
            }
        }
        let s = String::from_utf8(xs).unwrap();
        let ch = char::from_u32(u32::from_str_radix(&s, 16).unwrap()).unwrap();
        Ok((ch, s))
    }

    /// Parse `?` after expression.
    fn parse_optional(&mut self, kind: ExpressionKind) -> Result<ExpressionKind> {
        self.expect("?", "expected `?`")?;
        Ok(ExpressionKind::Optional(box_kind(kind)))
    }

    /// Parse `*` | `*?` after expression.
    fn parse_repeat(&mut self, kind: ExpressionKind) -> Result<ExpressionKind> {
        self.expect("*", "expected `*`")?;
        Ok(if self.take_str("?") {
            ExpressionKind::RepeatNonGreedy(box_kind(kind))
        } else {
            ExpressionKind::Repeat(box_kind(kind))
        })
    }

    /// Parse `+` | `+?` after expression.
    fn parse_repeat_plus(&mut self, kind: ExpressionKind) -> Result<ExpressionKind> {
        self.expect("+", "expected `+`")?;
        Ok(if self.take_str("?") {
            ExpressionKind::RepeatPlusNonGreedy(box_kind(kind))
        } else {
            ExpressionKind::RepeatPlus(box_kind(kind))
        })
    }

    /// Parse `{a..b}` | `{a..=b}` after expression.
    fn parse_repeat_range(&mut self, kind: ExpressionKind) -> Result<ExpressionKind> {
        self.expect("{", "expected `{`")?;
        let min = self.take_while(&|x| x.is_ascii_digit());
        let Ok(min) = (!min.is_empty()).then(|| min.parse::<u32>()).transpose() else {
            bail!(self, "malformed range start");
        };
        self.expect("..", "expected `..` or `..=`")?;
        let limit = if self.take_str("=") {
            RangeLimit::Closed
        } else {
            RangeLimit::HalfOpen
        };
        let max = self.take_while(&|x| x.is_ascii_digit());
        let Ok(max) = (!max.is_empty()).then(|| max.parse::<u32>()).transpose() else {
            bail!(self, "malformed range end");
        };
        match (min, max, limit) {
            (Some(min), Some(max), _) if max < min => {
                bail!(self, "range {min}{limit}{max} is malformed")
            }
            (Some(min), Some(max), RangeLimit::HalfOpen) if max <= min => {
                bail!(self, "half-open range maximum must be greater than minimum")
            }
            (None, Some(0), RangeLimit::HalfOpen) => {
                bail!(self, "half-open range `..0` is empty")
            }
            (_, None, RangeLimit::Closed) => bail!(self, "closed range must have an upper bound"),
            _ => {}
        }
        self.expect("}", "expected `}`")?;
        Ok(ExpressionKind::RepeatRange {
            expr: box_kind(kind),
            min,
            max,
            limit,
        })
    }

    fn parse_suffix(&mut self) -> Result<Option<String>> {
        if !self.take_str(" _") {
            return Ok(None);
        }
        let mut in_backtick = false;
        let start = self.index;
        loop {
            let Some(next) = self.peek() else {
                bail!(self, "failed to find end of _ suffixed text");
            };
            self.index += 1;
            match next {
                b'\n' => bail!(self, "failed to find end of _ suffixed text"),
                b'`' => in_backtick = !in_backtick,
                b'_' if !in_backtick => {
                    if self
                        .peek()
                        .map(|b| matches!(b, b'\n' | b' '))
                        .unwrap_or(true)
                    {
                        break;
                    }
                }
                _ => {}
            }
        }
        Ok(Some(self.input[start..self.index - 1].to_string()))
    }

    /// Parse footnote reference, e.g. `[^id]`.
    fn parse_footnote(&mut self) -> Result<Option<String>> {
        if !self.take_str("[^") {
            return Ok(None);
        }
        let id = self.take_while(&|x| !['\n', ']'].contains(&x)).to_string();
        if id.is_empty() {
            bail!(self, "expected footnote id");
        }
        self.expect("]", "expected closing `]`")?;
        Ok(Some(id))
    }
}

fn box_kind(kind: ExpressionKind) -> Box<Expression> {
    Box::new(Expression {
        kind,
        suffix: None,
        footnote: None,
    })
}

/// Helper to translate a byte index to a `(line, line_no, col_no)` (1-based).
fn translate_position(input: &str, index: usize) -> (&str, usize, usize) {
    if input.is_empty() {
        return ("", 0, 0);
    }
    let index = index.min(input.len());

    let mut line_start = 0;
    let mut line_number = 0;
    for line in input.lines() {
        let line_end = line_start + line.len();
        if index >= line_start && index <= line_end {
            let column_number = index - line_start + 1;
            return (line, line_number + 1, column_number);
        }
        line_start = line_end + 1;
        line_number += 1;
    }
    ("", line_number + 1, 0)
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_grammar, translate_position};
    use crate::{Character, Characters, ExpressionKind, Grammar, RangeLimit};
    use std::path::Path;

    #[test]
    fn test_translate() {
        assert_eq!(translate_position("", 0), ("", 0, 0));
        assert_eq!(translate_position("test", 0), ("test", 1, 1));
        assert_eq!(translate_position("test", 3), ("test", 1, 4));
        assert_eq!(translate_position("test", 4), ("test", 1, 5));
        assert_eq!(translate_position("test\ntest2", 4), ("test", 1, 5));
        assert_eq!(translate_position("test\ntest2", 5), ("test2", 2, 1));
        assert_eq!(translate_position("test\ntest2\n", 11), ("", 3, 0));
    }

    fn parse(input: &str) -> Result<Grammar, String> {
        let mut grammar = Grammar::default();
        parse_grammar(input, &mut grammar, "test", Path::new("test.md"))
            .map_err(|e| e.to_string())?;
        Ok(grammar)
    }

    #[test]
    fn test_cut() {
        let input = "Rule -> A ^ B | C";
        let grammar = parse(input).unwrap();
        grammar.productions.get("Rule").unwrap();
    }

    #[test]
    fn test_cut_captures() {
        let input = "Rule -> A ^ B C | D";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        // The top-level expression is an alternation: (A ^ B C) | D.
        let ExpressionKind::Alt(alts) = &rule.expression.kind else {
            panic!("expected Alt, got {:?}", rule.expression.kind);
        };
        assert_eq!(alts.len(), 2);
        // First alternative is a sequence: A, Cut(Sequence(B, C)).
        let ExpressionKind::Sequence(seq) = &alts[0].kind else {
            panic!("expected Sequence, got {:?}", alts[0].kind);
        };
        assert_eq!(seq.len(), 2);
        assert!(matches!(&seq[0].kind, ExpressionKind::Nt(n) if n == "A"));
        // The cut captures the rest of the sequence (B and C).
        let ExpressionKind::Cut(cut_inner) = &seq[1].kind else {
            panic!("expected Cut, got {:?}", seq[1].kind);
        };
        let ExpressionKind::Sequence(cut_seq) = &cut_inner.kind else {
            panic!("expected Sequence inside Cut, got {:?}", cut_inner.kind);
        };
        assert_eq!(cut_seq.len(), 2);
        assert!(matches!(&cut_seq[0].kind, ExpressionKind::Nt(n) if n == "B"));
        assert!(matches!(&cut_seq[1].kind, ExpressionKind::Nt(n) if n == "C"));
        // Second alternative is just D.
        assert!(matches!(&alts[1].kind, ExpressionKind::Nt(n) if n == "D"));
    }

    #[test]
    fn test_cut_fail_trailing() {
        let input = "Rule -> A ^";
        let err = parse(input).unwrap_err();
        assert!(err.contains("expected expression after cut operator"));
    }

    /// Extract the `RepeatRange` fields from a single-production
    /// grammar whose rule body is a repeat-range expression.
    fn repeat_range(input: &str) -> (Option<u32>, Option<u32>, RangeLimit) {
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("A").unwrap();
        let ExpressionKind::RepeatRange {
            min, max, limit, ..
        } = rule.expression.kind
        else {
            panic!("expected RepeatRange, got {:?}", rule.expression.kind);
        };
        (min, max, limit)
    }

    // -- Valid ranges -----------------------------------------------

    #[test]
    fn test_range_half_open() {
        let (min, max, limit) = repeat_range("A -> x{2..5}");
        assert_eq!(min, Some(2));
        assert_eq!(max, Some(5));
        assert!(matches!(limit, RangeLimit::HalfOpen));
    }

    #[test]
    fn test_range_half_open_no_min() {
        let (min, max, limit) = repeat_range("A -> x{..5}");
        assert_eq!(min, None);
        assert_eq!(max, Some(5));
        assert!(matches!(limit, RangeLimit::HalfOpen));
    }

    #[test]
    fn test_range_half_open_no_max() {
        let (min, max, limit) = repeat_range("A -> x{2..}");
        assert_eq!(min, Some(2));
        assert_eq!(max, None);
        assert!(matches!(limit, RangeLimit::HalfOpen));
    }

    #[test]
    fn test_range_half_open_unbounded() {
        let (min, max, limit) = repeat_range("A -> x{..}");
        assert_eq!(min, None);
        assert_eq!(max, None);
        assert!(matches!(limit, RangeLimit::HalfOpen));
    }

    #[test]
    fn test_range_closed() {
        let (min, max, limit) = repeat_range("A -> x{2..=5}");
        assert_eq!(min, Some(2));
        assert_eq!(max, Some(5));
        assert!(matches!(limit, RangeLimit::Closed));
    }

    #[test]
    fn test_range_closed_no_min() {
        let (min, max, limit) = repeat_range("A -> x{..=5}");
        assert_eq!(min, None);
        assert_eq!(max, Some(5));
        assert!(matches!(limit, RangeLimit::Closed));
    }

    // -- Invalid ranges ---------------------------------------------

    #[test]
    fn test_range_err_max_less_than_min() {
        let err = parse("A -> x{3..2}").unwrap_err();
        assert!(
            err.contains("malformed"),
            "expected malformed error, got: {err}"
        );
    }

    #[test]
    fn test_range_err_empty_exclusive_equal() {
        let err = parse("A -> x{2..2}").unwrap_err();
        assert!(
            err.contains("half-open range maximum must be greater"),
            "expected empty-exclusive error, got: {err}"
        );
    }

    #[test]
    fn test_range_err_empty_exclusive_zero() {
        let err = parse("A -> x{0..0}").unwrap_err();
        assert!(
            err.contains("half-open range maximum must be greater"),
            "expected empty-exclusive error, got: {err}"
        );
    }

    #[test]
    fn test_range_err_closed_no_upper() {
        let err = parse("A -> x{..=}").unwrap_err();
        assert!(
            err.contains("closed range must have an upper bound"),
            "expected closed-needs-upper error, got: {err}"
        );
    }

    #[test]
    fn test_range_err_closed_no_upper_with_min() {
        let err = parse("A -> x{2..=}").unwrap_err();
        assert!(
            err.contains("closed range must have an upper bound"),
            "expected closed-needs-upper error, got: {err}"
        );
    }

    #[test]
    fn test_range_err_half_open_zero_max() {
        let err = parse("A -> x{..0}").unwrap_err();
        assert!(
            err.contains("half-open range `..0` is empty"),
            "expected half-open-zero error, got: {err}"
        );
    }

    // -- Valid edge cases -------------------------------------------

    #[test]
    fn test_range_closed_exact() {
        // `x{2..=2}` means exactly 2 — not empty.
        let (min, max, limit) = repeat_range("A -> x{2..=2}");
        assert_eq!(min, Some(2));
        assert_eq!(max, Some(2));
        assert!(matches!(limit, RangeLimit::Closed));
    }

    #[test]
    fn test_range_half_open_zero_to_one() {
        // `x{0..1}` means exactly 0 repetitions (the half-open
        // range contains only 0).
        let (min, max, limit) = repeat_range("A -> x{0..1}");
        assert_eq!(min, Some(0));
        assert_eq!(max, Some(1));
        assert!(matches!(limit, RangeLimit::HalfOpen));
    }

    // --- Negative lookahead tests ---

    #[test]
    fn lookahead_simple_nonterminal() {
        let input = "Rule -> !Foo";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::NegativeLookahead(inner) = &rule.expression.kind else {
            panic!("expected NegativeLookahead, got {:?}", rule.expression.kind);
        };
        assert!(matches!(&inner.kind, ExpressionKind::Nt(n) if n == "Foo"));
    }

    #[test]
    fn lookahead_terminal() {
        let input = "Rule -> !`'` Foo";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Sequence(seq) = &rule.expression.kind else {
            panic!("expected Sequence, got {:?}", rule.expression.kind);
        };
        assert_eq!(seq.len(), 2);
        let ExpressionKind::NegativeLookahead(inner) = &seq[0].kind else {
            panic!("expected NegativeLookahead, got {:?}", seq[0].kind);
        };
        assert!(matches!(&inner.kind, ExpressionKind::Terminal(t) if t == "'"));
        assert!(matches!(&seq[1].kind, ExpressionKind::Nt(n) if n == "Foo"));
    }

    #[test]
    fn lookahead_charset() {
        let input = "Rule -> ![`e` `E`] SUFFIX";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Sequence(seq) = &rule.expression.kind else {
            panic!("expected Sequence, got {:?}", rule.expression.kind);
        };
        assert_eq!(seq.len(), 2);
        let ExpressionKind::NegativeLookahead(inner) = &seq[0].kind else {
            panic!("expected NegativeLookahead, got {:?}", seq[0].kind);
        };
        let ExpressionKind::Charset(chars) = &inner.kind else {
            panic!("expected Charset inside lookahead, got {:?}", inner.kind);
        };
        assert_eq!(chars.len(), 2);
        assert!(matches!(&chars[0], Characters::Terminal(t) if t == "e"));
        assert!(matches!(&chars[1], Characters::Terminal(t) if t == "E"));
    }

    #[test]
    fn lookahead_grouped() {
        let input = "Rule -> !(`.` | `_` | XID_Start)";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::NegativeLookahead(inner) = &rule.expression.kind else {
            panic!("expected NegativeLookahead, got {:?}", rule.expression.kind);
        };
        let ExpressionKind::Grouped(grouped) = &inner.kind else {
            panic!("expected Grouped inside lookahead, got {:?}", inner.kind);
        };
        let ExpressionKind::Alt(alts) = &grouped.kind else {
            panic!("expected Alt inside Grouped, got {:?}", grouped.kind);
        };
        assert_eq!(alts.len(), 3);
        assert!(matches!(&alts[0].kind, ExpressionKind::Terminal(t) if t == "."));
        assert!(matches!(&alts[1].kind, ExpressionKind::Terminal(t) if t == "_"));
        assert!(matches!(&alts[2].kind, ExpressionKind::Nt(n) if n == "XID_Start"));
    }

    #[test]
    fn lookahead_in_sequence_middle() {
        let input = "Rule -> A !B C";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Sequence(seq) = &rule.expression.kind else {
            panic!("expected Sequence, got {:?}", rule.expression.kind);
        };
        assert_eq!(seq.len(), 3);
        assert!(matches!(&seq[0].kind, ExpressionKind::Nt(n) if n == "A"));
        let ExpressionKind::NegativeLookahead(inner) = &seq[1].kind else {
            panic!("expected NegativeLookahead, got {:?}", seq[1].kind);
        };
        assert!(matches!(&inner.kind, ExpressionKind::Nt(n) if n == "B"));
        assert!(matches!(&seq[2].kind, ExpressionKind::Nt(n) if n == "C"));
    }

    #[test]
    fn lookahead_in_repetition() {
        let input = "Rule -> (!A B)*";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Repeat(rep) = &rule.expression.kind else {
            panic!("expected Repeat, got {:?}", rule.expression.kind);
        };
        let ExpressionKind::Grouped(grouped) = &rep.kind else {
            panic!("expected Grouped inside Repeat, got {:?}", rep.kind);
        };
        let ExpressionKind::Sequence(seq) = &grouped.kind else {
            panic!("expected Sequence inside Grouped, got {:?}", grouped.kind);
        };
        assert_eq!(seq.len(), 2);
        assert!(matches!(&seq[0].kind, ExpressionKind::NegativeLookahead(_)));
        assert!(matches!(&seq[1].kind, ExpressionKind::Nt(n) if n == "B"));
    }

    #[test]
    fn lookahead_in_alternation() {
        let input = "Rule -> !A B | C";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Alt(alts) = &rule.expression.kind else {
            panic!("expected Alt, got {:?}", rule.expression.kind);
        };
        assert_eq!(alts.len(), 2);
        let ExpressionKind::Sequence(seq) = &alts[0].kind else {
            panic!("expected Sequence, got {:?}", alts[0].kind);
        };
        assert_eq!(seq.len(), 2);
        assert!(matches!(&seq[0].kind, ExpressionKind::NegativeLookahead(_)));
        assert!(matches!(&seq[1].kind, ExpressionKind::Nt(n) if n == "B"));
        assert!(matches!(&alts[1].kind, ExpressionKind::Nt(n) if n == "C"));
    }

    #[test]
    fn lookahead_fail_trailing() {
        let input = "Rule -> !";
        let err = parse(input).unwrap_err();
        assert!(err.contains("expected expression after !"));
    }

    // --- Unicode tests ---

    #[test]
    fn unicode_4_digit() {
        let input = "Rule -> U+0009";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Unicode((ch, s)) = &rule.expression.kind else {
            panic!("expected Unicode, got {:?}", rule.expression.kind);
        };
        assert_eq!(*ch, '\t');
        assert_eq!(s, "0009");
    }

    #[test]
    fn unicode_5_digit() {
        let input = "Rule -> U+E0000";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Unicode((ch, s)) = &rule.expression.kind else {
            panic!("expected Unicode, got {:?}", rule.expression.kind);
        };
        assert_eq!(*ch, '\u{E0000}');
        assert_eq!(s, "E0000");
    }

    #[test]
    fn unicode_6_digit() {
        let input = "Rule -> U+10FFFF";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Unicode((ch, s)) = &rule.expression.kind else {
            panic!("expected Unicode, got {:?}", rule.expression.kind);
        };
        assert_eq!(*ch, '\u{10FFFF}');
        assert_eq!(s, "10FFFF");
    }

    #[test]
    fn unicode_in_alternation() {
        let input = "Rule -> U+0009 | U+000A";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Alt(alts) = &rule.expression.kind else {
            panic!("expected Alt, got {:?}", rule.expression.kind);
        };
        assert_eq!(alts.len(), 2);
        assert!(matches!(
            &alts[0].kind,
            ExpressionKind::Unicode((ch, _)) if *ch == '\t'
        ));
        assert!(matches!(
            &alts[1].kind,
            ExpressionKind::Unicode((ch, _)) if *ch == '\n'
        ));
    }

    // --- Character / charset range tests ---

    #[test]
    fn charset_unicode_range() {
        let input = "Rule -> [U+0000-U+007F]";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Charset(chars) = &rule.expression.kind else {
            panic!("expected Charset, got {:?}", rule.expression.kind);
        };
        assert_eq!(chars.len(), 1);
        let Characters::Range(a, b) = &chars[0] else {
            panic!("expected Range, got {:?}", chars[0]);
        };
        assert!(matches!(a, Character::Unicode((ch, _)) if *ch == '\0'));
        assert!(matches!(
            b,
            Character::Unicode((ch, _)) if *ch == '\u{7F}'
        ));
    }

    #[test]
    fn charset_char_range() {
        let input = "Rule -> [`a`-`z`]";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Charset(chars) = &rule.expression.kind else {
            panic!("expected Charset, got {:?}", rule.expression.kind);
        };
        assert_eq!(chars.len(), 1);
        let Characters::Range(a, b) = &chars[0] else {
            panic!("expected Range, got {:?}", chars[0]);
        };
        assert!(matches!(a, Character::Char(ch) if *ch == 'a'));
        assert!(matches!(b, Character::Char(ch) if *ch == 'z'));
    }

    #[test]
    fn charset_mixed_range() {
        let input = "Rule -> [`a`-U+007A]";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Charset(chars) = &rule.expression.kind else {
            panic!("expected Charset, got {:?}", rule.expression.kind);
        };
        assert_eq!(chars.len(), 1);
        let Characters::Range(a, b) = &chars[0] else {
            panic!("expected Range, got {:?}", chars[0]);
        };
        assert!(matches!(a, Character::Char(ch) if *ch == 'a'));
        assert!(matches!(
            b,
            Character::Unicode((ch, _)) if *ch == 'z'
        ));
    }

    #[test]
    fn charset_multiple_unicode_ranges() {
        let input = "Rule -> [U+0000-U+D7FF U+E000-U+10FFFF]";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Charset(chars) = &rule.expression.kind else {
            panic!("expected Charset, got {:?}", rule.expression.kind);
        };
        assert_eq!(chars.len(), 2);
        let Characters::Range(a1, b1) = &chars[0] else {
            panic!("expected Range, got {:?}", chars[0]);
        };
        assert!(matches!(a1, Character::Unicode((ch, _)) if *ch == '\0'));
        assert!(matches!(b1, Character::Unicode((ch, _)) if *ch == '\u{D7FF}'));
        let Characters::Range(a2, b2) = &chars[1] else {
            panic!("expected Range, got {:?}", chars[1]);
        };
        assert!(matches!(a2, Character::Unicode((ch, _)) if *ch == '\u{E000}'));
        assert!(matches!(b2, Character::Unicode((ch, _)) if *ch == '\u{10FFFF}'));
    }

    #[test]
    fn charset_terminals_and_named() {
        let input = "Rule -> [`a` `b` Foo]";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Charset(chars) = &rule.expression.kind else {
            panic!("expected Charset, got {:?}", rule.expression.kind);
        };
        assert_eq!(chars.len(), 3);
        assert!(matches!(&chars[0], Characters::Terminal(t) if t == "a"));
        assert!(matches!(&chars[1], Characters::Terminal(t) if t == "b"));
        assert!(matches!(&chars[2], Characters::Named(n) if n == "Foo"));
    }

    // --- Negative lookahead combined with charset ---

    #[test]
    fn lookahead_charset_with_named_and_terminals() {
        // Pattern from tokens.md: ![`'` `\` LF CR TAB] ASCII
        let input = "Rule -> ![`x` `y` LF] Foo";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Sequence(seq) = &rule.expression.kind else {
            panic!("expected Sequence, got {:?}", rule.expression.kind);
        };
        assert_eq!(seq.len(), 2);
        let ExpressionKind::NegativeLookahead(inner) = &seq[0].kind else {
            panic!("expected NegativeLookahead, got {:?}", seq[0].kind);
        };
        let ExpressionKind::Charset(chars) = &inner.kind else {
            panic!("expected Charset, got {:?}", inner.kind);
        };
        assert_eq!(chars.len(), 3);
        assert!(matches!(&chars[0], Characters::Terminal(t) if t == "x"));
        assert!(matches!(&chars[1], Characters::Terminal(t) if t == "y"));
        assert!(matches!(&chars[2], Characters::Named(n) if n == "LF"));
    }

    // --- Negative lookahead combined with Unicode ---

    #[test]
    fn lookahead_charset_with_unicode_range() {
        let input = "Rule -> ![U+0000-U+007F] Foo";
        let grammar = parse(input).unwrap();
        let rule = grammar.productions.get("Rule").unwrap();
        let ExpressionKind::Sequence(seq) = &rule.expression.kind else {
            panic!("expected Sequence, got {:?}", rule.expression.kind);
        };
        let ExpressionKind::NegativeLookahead(inner) = &seq[0].kind else {
            panic!("expected NegativeLookahead, got {:?}", seq[0].kind);
        };
        let ExpressionKind::Charset(chars) = &inner.kind else {
            panic!("expected Charset, got {:?}", inner.kind);
        };
        assert_eq!(chars.len(), 1);
        let Characters::Range(a, b) = &chars[0] else {
            panic!("expected Range, got {:?}", chars[0]);
        };
        assert!(matches!(a, Character::Unicode((ch, _)) if *ch == '\0'));
        assert!(matches!(
            b,
            Character::Unicode((ch, _)) if *ch == '\u{7F}'
        ));
    }
}
