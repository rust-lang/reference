//! Built-in test cases.
//!
//! The initial idea with this was to collect a broad set of tests that
//! exercise all of the grammar rules. However, that's a tall order as it
//! would end up being quite large. I instead started leaning on the
//! permutation-based tests to more exhaustively cover the grammar. However,
//! there's less certainty using that mechanism, and it also makes it harder
//! to iterate on a single grammar rule. This may still be useful, but would
//! take some considerable work to make it useful.

macro_rules! cases {
    ($($name:path => $($s:literal)+)+) => {
        pub static LEX_CASES: &[(&str, &[&str])] = &[
            $(
                (stringify!($name), &[ $($s),* ]),
            )+
        ];
    };
}

cases! {
    empty =>
        ""

    comment::line_comment =>
        "// line comment"
        "////"
        "//// this is a comment"
        "//\n"
    comment::block_comment =>
        "/* block comment */"
    comment::inner_line_doc =>
        "//! inner line doc"
    comment::inner_block_doc =>
        "/*! inner block doc */"
    comment::outer_line_doc =>
        "/// outer line doc"
        "///"
        "///\n"
        "///abc\n"
        "/// ☃"
    comment::outer_block_doc =>
        "/** outer block doc */"
    comment::cr_starting_block_doc =>
        "/**\r CR starting block doc comment */"
    comment::cr_starting_inner_block_doc =>
        "/*!\r CR starting inner block doc comment */"

    comment::block::nested_cr1 =>
        "/* /**\r*/ */"
    comment::block::nested_cr2 =>
        "/* /*!\r*/ */"
    comment::block::nested_cr3 =>
        "/* /** x\r y */ */"
    comment::block::nested_cr4 =>
        "/** /*\r*/ */"
    comment::block::nested_cr5 =>
        "/*! /*\r*/ */"
    comment::block::nested_cr6 =>
        "/** /* x\r y */ */"
    comment::block::nested_cr7 =>
        "/** /* /*\r*/ */ */"
    comment::block::nested_cr8 =>
        "/* /* /**\r*/ */ */"

    reserved::pounds =>
        "##"
        "###"
        "####"
        "#####"

    raw_identifier =>
        "r#fn"
    char =>
        "'x'"
    string =>
        "\"string\""
    string::continuation::bare_carriage =>
        "\"string\\\n\n\r\tcontinuation\""

    raw_string =>
        "r\"raw string\""
        "r#\"raw string\"#"
        "r#\"\"\"#"
    byte =>
        "b'x'"
    byte_string =>
        "b\"byte\""
    raw_byte_string =>
        "br\"raw byte\""
        "br#\"raw byte\"#"
    c_string =>
        "c\"c str\""
    raw_c_string =>
        "cr\"raw c str\""
        "cr#\"raw c str\"#"
    float =>
        "1.2"
    integer =>
        "123"
    lifetime =>
        "'a"
    punctuation =>
        "!"
    identifier =>
        "ident"
        "fn"

    shebang::doc_comment =>
        "#! /** doc */ [attr]\n"
}
