//! The proc-macro2 tool.

use parser::lexer::Tokens;
use parser::{Node, ParseError};
use proc_macro2::{Spacing, TokenStream, TokenTree};
use regex::Regex;
use std::ops::Range;
use std::str::FromStr;
use std::sync::LazyLock;

pub fn tokenize(src: &str) -> Result<Vec<Node>, ParseError> {
    let mut tokens = Vec::new();
    let stream = TokenStream::from_str(src).map_err(|e| ParseError {
        byte_offset: 0,
        message: e.to_string(),
    })?;
    tokens_from_ts(src, stream, &mut tokens)?;
    Ok(tokens)
}

// proc-macro2 does not reject literals starting with E.
// We'll need to do that to match behavior.
// https://github.com/dtolnay/proc-macro2/issues/506
static SUFFIX_NO_E: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)
        ^(
          ([0-9][0-9_]*[eE])  # DEC_LITERAL
        | (0b([01]|_)*?[01]([01]|_)*[eE]) # BIN_LITERL
        | (0o([0-7]|_)*?[0-7]([0-7]|_)*[eE]) # OCT_LITERAL
        | ([0-9]([0-9]|_)*\.[0-9]([0-9]|_)*[eE]) # FLOAT_LITERAL
        )
        ",
    )
    .unwrap()
});

static FLOAT_EXPONENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)
        ^
          [0-9]([0-9]|_)*
          (\. [0-9]([0-9]|_)*)?
          [eE] [+-]? ([0-9]|_)*? [0-9] ([0-9]|_)*
        ",
    )
    .unwrap()
});

static NUM_DOT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)
        ^(
          (0b([01]|_)*?[01]([01]|_)*) # BIN_LITERL
        | (0o([0-7]|_)*?[0-7]([0-7]|_)*) # OCT_LITERAL
        | (0x([0-9a-fA-F]|_)*?[0-9a-fA-F]([0-9a-fA-F]|_)*) # HEX_LITERAL
        )
        \.
        ",
    )
    .unwrap()
});

fn tokens_from_ts(src: &str, ts: TokenStream, output: &mut Vec<Node>) -> Result<(), ParseError> {
    let trees: Vec<TokenTree> = ts.into_iter().collect();
    let mut i = 0;
    while i < trees.len() {
        let tt = &trees[i];
        let span = tt.span();
        let mut range = span.byte_range();

        // For OUTER_LINE_DOC and CRLF input, the range ends up pointing at
        // the CR. Adjust this to match the other tools.
        let s_range = &src[range.clone()];
        if (s_range.starts_with("///") || s_range.starts_with("//!")) && s_range.ends_with('\r') {
            range.end -= 1;
        }

        match tt {
            TokenTree::Ident(_) => {
                // proc-macro2 does not reject Edition 2021 reserved prefixes.
                // https://github.com/dtolnay/proc-macro2/issues/534
                if src[range.end..].chars().next() == Some('"')
                    && !matches!(&src[range.clone()], "b" | "c" | "r" | "br" | "cr")
                {
                    return Err(ParseError {
                        message: "RESERVED_TOKEN_DOUBLE_QUOTE".to_string(),
                        byte_offset: range.end,
                    });
                }
                if src[range.end..].chars().next() == Some('\'')
                    && !matches!(&src[range.clone()], "b")
                {
                    return Err(ParseError {
                        message: "RESERVED_TOKEN_SINGLE_QUOTE".to_string(),
                        byte_offset: range.end,
                    });
                }

                if src[range.end..].chars().next() == Some('#') {
                    return Err(ParseError {
                        message: "RESERVED_TOKEN_POUND".to_string(),
                        byte_offset: range.start,
                    });
                }

                i += 1;
                let name = if src[range.start..].starts_with("r#") {
                    "RAW_IDENTIFIER"
                } else {
                    "IDENTIFIER_OR_KEYWORD"
                }
                .to_string();
                output.push(Node::new(name, range));
            }
            TokenTree::Punct(p) => {
                // In order to be consistent with rustc which uses joined tokens,
                // this looks to join multiple punctuation tokens.

                // s accumulates the punctuation string.
                let mut s = p.as_char().to_string();
                i += 1;
                let mut current_spacing = p.spacing();

                // Try to consume subsequent punctuation if it is joint and
                // forms a valid operator.
                while current_spacing == Spacing::Joint && i < trees.len() {
                    match &trees[i] {
                        TokenTree::Punct(next_p) => {
                            s.push(next_p.as_char());
                            if is_valid_punctuation(&s) {
                                range.end = next_p.span().byte_range().end;
                                current_spacing = next_p.spacing();
                                i += 1;
                            } else {
                                s.pop();
                                break;
                            }
                        }
                        TokenTree::Ident(ident) => {
                            // lifetime
                            s.push_str(&ident.to_string());
                            range.end = ident.span().byte_range().end;

                            // For some reason, proc-macro2 doesn't seem to fail
                            // when it sees IDENT'IDENT.
                            if i >= 2 {
                                let prev_tt = &trees[i - 2];
                                let prev_range = prev_tt.span().byte_range();
                                if let TokenTree::Ident(_) = prev_tt
                                    && prev_range.end == range.start
                                {
                                    return Err(ParseError {
                                        message: "RESERVED_TOKEN_SINGLE_QUOTE".to_string(),
                                        byte_offset: prev_range.start,
                                    });
                                }
                            }
                            i += 1;
                            break;
                        }
                        _ => break,
                    }
                }

                // https://github.com/dtolnay/proc-macro2/issues/535
                if s == "#"
                    && i < trees.len()
                    && let TokenTree::Literal(lit) = &trees[i]
                    && lit.to_string().starts_with('"')
                    && trees[i].span().byte_range().start == range.start + 1
                {
                    return Err(ParseError {
                        message: "RESERVED_GUARDED_STRING_LITERAL".to_string(),
                        byte_offset: range.start,
                    });
                }

                let name = if s.starts_with('\'') && s.len() > 1 {
                    "LIFETIME_TOKEN"
                } else {
                    "PUNCTUATION"
                }
                .to_string();
                output.push(Node::new(name, range));
            }
            TokenTree::Literal(lit) => {
                let s = lit.to_string();
                if SUFFIX_NO_E.is_match(&s) && !FLOAT_EXPONENT.is_match(&s) {
                    return Err(ParseError {
                        message: "bad E suffix".to_string(),
                        byte_offset: range.start,
                    });
                }

                // https://github.com/dtolnay/proc-macro2/issues/531
                if [
                    "'''", "'\r'", "'\n'", "'\t'", "b'''", "b'\r'", "b'\n'", "b'\t'",
                ]
                .iter()
                .any(|p| s.starts_with(p))
                {
                    return Err(ParseError {
                        message: "invalid byte or char literal".to_string(),
                        byte_offset: range.start,
                    });
                }

                // https://github.com/dtolnay/proc-macro2/issues/532
                if matches!(s.as_bytes().last_chunk::<2>(), Some(b"'_" | b"\"_" | b"#_")) {
                    return Err(ParseError {
                        message: "underscore suffix not allowed".to_string(),
                        byte_offset: range.start,
                    });
                }

                // https://github.com/dtolnay/proc-macro2/issues/533
                let s_rest = &src[range.start..];
                if let Some(m) = NUM_DOT.find(s_rest) {
                    let next = src[range.start + m.len()..].chars().next();
                    if !matches!(next, Some('.' | '_'))
                        && !next
                            .map(|ch| unicode_ident::is_xid_start(ch))
                            .unwrap_or(false)
                    {
                        return Err(ParseError {
                            message: "reserved bin/oct/hex literal followed by .".to_string(),
                            byte_offset: range.start,
                        });
                    }
                }

                output.push(Node::new(lit_to_reference(&s), range));
                i += 1;
            }
            TokenTree::Group(group) => {
                let delim = group.delimiter();
                let delim_str = match &delim {
                    proc_macro2::Delimiter::Parenthesis => "(",
                    proc_macro2::Delimiter::Brace => "{",
                    proc_macro2::Delimiter::Bracket => "[",
                    proc_macro2::Delimiter::None => "",
                };
                if !delim_str.is_empty() {
                    output.push(Node::new(
                        "PUNCTUATION".to_string(),
                        group.span_open().byte_range(),
                    ));
                }
                tokens_from_ts(src, group.stream(), output)?;
                if !delim_str.is_empty() {
                    let close_delim = match delim_str {
                        "(" => ")",
                        "{" => "}",
                        "[" => "]",
                        _ => "",
                    };
                    let mut range = group.span_close().byte_range();
                    // proc-macro2's CRLF handling ends up with a range pointing
                    // at the CR instead of the byte before.
                    if &src[range.clone()] == "\r" && close_delim == "]" {
                        range.start -= 1;
                        range.end -= 1;
                        // After shifting back one byte we may now be inside a
                        // multi-byte UTF-8 character. Walk start back further
                        // until we're on a char boundary, then set end to the
                        // end of that character.
                        while range.start > 0 && !src.is_char_boundary(range.start) {
                            range.start -= 1;
                        }
                        range.end = range.start
                            + src[range.start..]
                                .chars()
                                .next()
                                .map_or(1, |c| c.len_utf8());
                    }
                    output.push(Node::new("PUNCTUATION".to_string(), range));
                }
                i += 1;
            }
        }
    }
    Ok(())
}

fn lit_to_reference(lit: &str) -> String {
    if lit.starts_with("cr") {
        "RAW_C_STRING_LITERAL".to_string()
    } else if lit.starts_with('c') {
        "C_STRING_LITERAL".to_string()
    } else if lit.starts_with("br") {
        "RAW_BYTE_STRING_LITERAL".to_string()
    } else if lit.starts_with("b'") {
        "BYTE_LITERAL".to_string()
    } else if lit.starts_with("b\"") {
        "BYTE_STRING_LITERAL".to_string()
    } else if lit.starts_with("r\"") || lit.starts_with("r#") {
        "RAW_STRING_LITERAL".to_string()
    } else if lit.starts_with('\'') {
        "CHAR_LITERAL".to_string()
    } else if lit.starts_with('"') {
        "STRING_LITERAL".to_string()
    } else if lit.starts_with("0x") || lit.starts_with("0o") || lit.starts_with("0b") {
        "INTEGER_LITERAL".to_string()
    } else if lit.contains('.') {
        "FLOAT_LITERAL".to_string()
    } else if lit.contains('e') || lit.contains('E') {
        // Could be float with exponent or integer with suffix containing 'e'/'E'
        // Check if there's a valid float exponent pattern
        if lit
            .bytes()
            .position(|b| b == b'e' || b == b'E')
            .map(|pos| {
                lit.as_bytes()
                    .get(pos - 1)
                    .map_or(false, |&ch| ch.is_ascii_digit() || ch == b'_')
            })
            .unwrap_or(false)
        {
            "FLOAT_LITERAL".to_string()
        } else {
            "INTEGER_LITERAL".to_string()
        }
    } else {
        "INTEGER_LITERAL".to_string()
    }
}

fn is_valid_punctuation(s: &str) -> bool {
    matches!(
        s,
        "..."
            | "..="
            | "<<="
            | ">>="
            | "!="
            | "%="
            | "&&"
            | "&="
            | "*="
            | "+="
            | "-="
            | "->"
            | ".."
            | "/="
            | "::"
            | "<-"
            | "<<"
            | "<="
            | "=="
            | "=>"
            | ">="
            | ">>"
            | "^="
            | "|="
            | "||"
    )
}

pub fn normalize(
    pm2_result: Result<Vec<Node>, ParseError>,
    reference_result: Result<Tokens, ParseError>,
    src: &str,
) -> (Result<Vec<Node>, ParseError>, Result<Vec<Node>, ParseError>) {
    let reference_result =
        reference_result.map(|tokens| normalize_reference_tokens(tokens.tokens, src));
    let pm2_result = match (&pm2_result, &reference_result) {
        (Ok(_), Err(e)) => {
            // For some reason, proc-macro2 treats NBSP as whitespace.
            if src[e.byte_offset..].chars().next() == Some('\u{a0}') {
                Err(ParseError {
                    message: "unexpected NBSP whitespace".to_string(),
                    byte_offset: e.byte_offset,
                })
            } else {
                pm2_result
            }
        }
        _ => pm2_result,
    };
    (pm2_result, reference_result)
}

fn normalize_reference_tokens(tokens: Vec<Node>, src: &str) -> Vec<Node> {
    let len = tokens.len();
    tokens
        .into_iter()
        .filter(|token| !matches!(token.name.as_str(), "LINE_COMMENT" | "BLOCK_COMMENT"))
        .fold(Vec::with_capacity(len), |mut acc, token| {
            // proc-macro2 does not handle ## reserved tokens (treats them as individual punctuation)
            // https://github.com/dtolnay/proc-macro2/issues/535
            if token.name == "RESERVED_TOKEN" && src[token.range.clone()].chars().all(|c| c == '#')
            {
                let count = token.range.len();
                for i in 0..count {
                    acc.push(Node::new(
                        String::from("PUNCTUATION"), // #
                        Range {
                            start: token.range.start + i,
                            end: token.range.start + i + 1,
                        },
                    ));
                }
                return acc;
            }
            // proc-macro2 converts doc comments into doc attributes.
            match &*token.name {
                "OUTER_LINE_DOC" | "INNER_LINE_DOC" | "OUTER_BLOCK_DOC" | "INNER_BLOCK_DOC" => {
                    acc.push(Node::new(
                        String::from("PUNCTUATION"), // #
                        token.range.clone(),
                    ));
                    if token.name.starts_with("INNER") {
                        acc.push(Node::new(
                            String::from("PUNCTUATION"), // !
                            token.range.clone(),
                        ));
                    }
                    acc.push(Node::new(
                        String::from("PUNCTUATION"), // [
                        Range {
                            start: token.range.start,
                            end: token.range.start + 1,
                        },
                    ));
                    acc.push(Node::new(
                        String::from("IDENTIFIER_OR_KEYWORD"),
                        token.range.clone(),
                    ));
                    acc.push(Node::new(
                        String::from("PUNCTUATION"), // =
                        token.range.clone(),
                    ));
                    acc.push(Node::new(
                        String::from("STRING_LITERAL"),
                        token.range.clone(),
                    ));
                    acc.push(Node::new(
                        String::from("PUNCTUATION"), // ]
                        Range {
                            start: token.range.end
                                - src[..token.range.end]
                                    .chars()
                                    .next_back()
                                    .unwrap()
                                    .len_utf8(),
                            end: token.range.end,
                        },
                    ));
                }
                _ => acc.push(token),
            }
            acc
        })
}
