//! The `rustc_parse`-based tool.

use std::fmt::Write as _;
extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_lexer;
extern crate rustc_parse;
extern crate rustc_session;
extern crate rustc_span;

use parser::{Edition, Node, ParseError};
use rustc_ast::ast::AttrStyle;
use rustc_ast::token::{CommentKind, IdentIsRaw, TokenKind};
use rustc_errors::emitter::HumanReadableErrorType;
use rustc_errors::json::JsonEmitter;
use rustc_errors::{ColorConfig, DiagCtxt};
use rustc_parse::lexer::StripTokens;
use rustc_session::parse::ParseSess;
use rustc_span::FileName;
use rustc_span::fatal_error::FatalError;
use rustc_span::source_map::{FilePathMapping, SourceMap};
use std::io;
use std::io::Write;
use std::ops::Range;
use std::sync::{Arc, Mutex};

struct Shared<T> {
    data: Arc<Mutex<T>>,
}

impl<T: Write> Write for Shared<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.data.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.data.lock().unwrap().flush()
    }
}

fn to_rustc_edition(edition: Edition) -> rustc_span::edition::Edition {
    match edition {
        Edition::Edition2015 => rustc_span::edition::Edition::Edition2015,
        Edition::Edition2018 => rustc_span::edition::Edition::Edition2018,
        Edition::Edition2021 => rustc_span::edition::Edition::Edition2021,
        Edition::Edition2024 => rustc_span::edition::Edition::Edition2024,
    }
}

pub fn tokenize(src: &str, edition: Edition) -> Result<Vec<Node>, ParseError> {
    rustc_span::create_session_globals_then(to_rustc_edition(edition), &[], None, || {
        let source_map = Arc::new(SourceMap::new(FilePathMapping::empty()));
        // TODO: probably not needed?
        // source_map.new_source_file(Path::new("test.rs").to_owned().into(), "".to_owned());
        let output = Arc::new(Mutex::new(Vec::new()));
        let je = JsonEmitter::new(
            Box::new(Shared {
                data: output.clone(),
            }),
            Some(source_map.clone()),
            false, // pretty
            HumanReadableErrorType {
                short: true,
                unicode: true,
            },
            ColorConfig::Never,
        );

        let dcx = DiagCtxt::new(Box::new(je));
        let psess = ParseSess::with_dcx(dcx, source_map);
        // TODO: Use StripTokens::Nothing before frontmatter is
        // stabilized. Use StripTokens::ShebangAndFrontmatter after it is
        // stabilized.
        let strip_tokens = StripTokens::ShebangAndFrontmatter;
        let source = String::from(src);
        let filename = FileName::Custom("internal".into());
        rustc_driver::catch_fatal_errors(|| {
            let mut parser = match rustc_parse::new_parser_from_source_str(
                &psess,
                filename,
                source,
                strip_tokens,
            ) {
                Ok(parser) => parser,
                Err(e) => {
                    for diag in e {
                        diag.emit();
                    }
                    FatalError.raise();
                }
            };
            let mut tokens = Vec::new();
            while parser.token.kind != TokenKind::Eof {
                let source_file = psess
                    .source_map()
                    .lookup_source_file(parser.token.span.lo());
                let start = source_file
                    .original_relative_byte_pos(parser.token.span.lo())
                    .0 as usize;
                let end = source_file
                    .original_relative_byte_pos(parser.token.span.hi())
                    .0 as usize;

                let token = Node::new(to_reference_name(&parser.token.kind), Range { start, end });
                tokens.push(token);
                parser.bump();
            }
            // Unfortunately this is handled outside of normal lexing.
            psess.bad_unicode_identifiers.with_lock(|idents| {
                for (ident, spans) in idents.drain(..) {
                    psess
                        .dcx()
                        .struct_span_err(
                            spans,
                            format!("identifiers cannot contain emoji: {ident}"),
                        )
                        .emit();
                }
            });
            psess.dcx().emit_stashed_diagnostics();
            let diags = diagnostics(&output.lock().unwrap());
            if diags.iter().any(|diag| diag.level.starts_with("error")) {
                FatalError.raise();
            }
            tokens
        })
        .map_err(|_| {
            let mut message = String::new();
            let out = &output.lock().unwrap();
            let diags = diagnostics(out);
            let mut byte_offset = 0;
            for diag in diags {
                write!(message, "error: {}", diag.rendered).unwrap();
                if byte_offset == 0 {
                    byte_offset = diag
                        .spans
                        .iter()
                        .find(|sp| sp.is_primary)
                        .map(|sp| sp.byte_start)
                        .unwrap_or_default();
                }
            }
            ParseError {
                byte_offset: byte_offset as usize,
                message,
            }
        })
    })
}

fn to_reference_name(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Eq
        | TokenKind::Lt
        | TokenKind::Le
        | TokenKind::EqEq
        | TokenKind::Ne
        | TokenKind::Ge
        | TokenKind::Gt
        | TokenKind::AndAnd
        | TokenKind::OrOr
        | TokenKind::Bang
        | TokenKind::Tilde
        | TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Star
        | TokenKind::Slash
        | TokenKind::Percent
        | TokenKind::Caret
        | TokenKind::And
        | TokenKind::Or
        | TokenKind::Shl
        | TokenKind::Shr
        | TokenKind::PlusEq
        | TokenKind::MinusEq
        | TokenKind::StarEq
        | TokenKind::SlashEq
        | TokenKind::PercentEq
        | TokenKind::CaretEq
        | TokenKind::AndEq
        | TokenKind::OrEq
        | TokenKind::ShlEq
        | TokenKind::ShrEq
        | TokenKind::At
        | TokenKind::Dot
        | TokenKind::DotDot
        | TokenKind::DotDotDot
        | TokenKind::DotDotEq
        | TokenKind::Comma
        | TokenKind::Semi
        | TokenKind::Colon
        | TokenKind::PathSep
        | TokenKind::RArrow
        | TokenKind::LArrow
        | TokenKind::FatArrow
        | TokenKind::Pound
        | TokenKind::Dollar
        | TokenKind::Question
        | TokenKind::SingleQuote
        | TokenKind::OpenParen
        | TokenKind::CloseParen
        | TokenKind::OpenBrace
        | TokenKind::CloseBrace
        | TokenKind::OpenBracket
        | TokenKind::CloseBracket => "PUNCTUATION",
        TokenKind::OpenInvisible(_) | TokenKind::CloseInvisible(_) => {
            panic!("unexpected invisible token")
        }
        TokenKind::Literal(lit) => match lit.kind {
            rustc_ast::token::LitKind::Bool => "IDENTIFIER_OR_KEYWORD",
            rustc_ast::token::LitKind::Byte => "BYTE_LITERAL",
            rustc_ast::token::LitKind::Char => "CHAR_LITERAL",
            rustc_ast::token::LitKind::Integer => "INTEGER_LITERAL",
            rustc_ast::token::LitKind::Float => "FLOAT_LITERAL",
            rustc_ast::token::LitKind::Str => "STRING_LITERAL",
            rustc_ast::token::LitKind::StrRaw(_) => "RAW_STRING_LITERAL",
            rustc_ast::token::LitKind::ByteStr => "BYTE_STRING_LITERAL",
            rustc_ast::token::LitKind::ByteStrRaw(_) => "RAW_BYTE_STRING_LITERAL",
            rustc_ast::token::LitKind::CStr => "C_STRING_LITERAL",
            rustc_ast::token::LitKind::CStrRaw(_) => "RAW_C_STRING_LITERAL",
            // Diagnostics handle this below.
            rustc_ast::token::LitKind::Err(_) => "Literal Error",
        },
        TokenKind::Ident(_, IdentIsRaw::No) => "IDENTIFIER_OR_KEYWORD",
        TokenKind::Ident(_, IdentIsRaw::Yes) => "RAW_IDENTIFIER",
        TokenKind::NtIdent(..) => panic!("unexpected NtIdent"),
        TokenKind::Lifetime(..) => "LIFETIME_TOKEN",
        TokenKind::NtLifetime(..) => panic!("unexpected NtLifetime"),
        TokenKind::DocComment(CommentKind::Line, AttrStyle::Inner, ..) => "INNER_LINE_DOC",
        TokenKind::DocComment(CommentKind::Line, AttrStyle::Outer, ..) => "OUTER_LINE_DOC",
        TokenKind::DocComment(CommentKind::Block, AttrStyle::Inner, ..) => "INNER_BLOCK_DOC",
        TokenKind::DocComment(CommentKind::Block, AttrStyle::Outer, ..) => "OUTER_BLOCK_DOC",
        TokenKind::Eof => panic!("unexpected EOF"),
    }
    .to_string()
}

fn diagnostics(output: &[u8]) -> Vec<Diagnostic> {
    let json = std::str::from_utf8(output).unwrap();
    json.lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

#[derive(serde::Deserialize)]
struct Diagnostic {
    rendered: String,
    level: String,
    spans: Vec<DiagSpan>,
}

#[derive(serde::Deserialize)]
struct DiagSpan {
    is_primary: bool,
    byte_start: u32,
}

pub fn normalize(tokens: &[Node]) -> Result<Vec<Node>, ParseError> {
    let new_ts = tokens
        .iter()
        // rustc_parse does not retain comments.
        .filter(|token| !matches!(token.name.as_str(), "LINE_COMMENT" | "BLOCK_COMMENT"))
        .cloned()
        .collect();
    Ok(new_ts)
}
