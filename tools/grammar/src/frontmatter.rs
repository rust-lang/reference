//! This is a temporary hack to include the frontmatter grammar until it is
//! stabilized.
//!
//! This should be removed once FRONTMATTER is added to the Reference.

use crate::{Grammar, parser};
use diagnostics::Diagnostics;
use std::path::Path;

pub fn load_grammar_with_frontmatter(diag: &mut Diagnostics) -> Grammar {
    let mut grammar = super::load_grammar(diag);

    parser::parse_grammar(FRONTMATTER, &mut grammar, "lexer", Path::new("")).unwrap();

    grammar
}

static FRONTMATTER: &str = "⊥ -> CHAR* CHAR

error -> ^ ⊥ // Should be a hard error.

@root FRONTMATTER ->
    WHITESPACE_ONLY_LINE*
    !FRONTMATTER_INVALID
    FRONTMATTER_MAIN

WHITESPACE_ONLY_LINE -> (!LF WHITESPACE)* LF

FRONTMATTER_INVALID -> (!LF WHITESPACE)+ `---` error

FRONTMATTER_MAIN ->
    `-`{n:3..=255} ^ FRONTMATTER_REST

FRONTMATTER_REST ->
    FRONTMATTER_FENCE_START
    FRONTMATTER_LINE*
    FRONTMATTER_FENCE_END

FRONTMATTER_FENCE_START ->
    MAYBE_INFOSTRING_OR_WS LF

FRONTMATTER_FENCE_END ->
    `-`{n} HORIZONTAL_WHITESPACE* ( LF | EOF )

FRONTMATTER_LINE -> !`-`{n} ~[LF CR]* LF

MAYBE_INFOSTRING_OR_WS ->
    HORIZONTAL_WHITESPACE* INFOSTRING? HORIZONTAL_WHITESPACE*

INFOSTRING -> (XID_Start | `_`) ( XID_Continue | `-` | `.` )*

HORIZONTAL_WHITESPACE ->
      U+0009 // Horizontal tab, `'\t'`
    | U+0020 // Space, `' '`
";
