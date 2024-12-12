# Comments

r[comments.syntax]

> **<sup>Lexer</sup>**\
> LINE_COMMENT :\
> &nbsp;&nbsp; &nbsp;&nbsp; `//` (~\[`/` `!` `\n`] | `//`) ~`\n`<sup>\*</sup>\
> &nbsp;&nbsp; | `//`
>
> BLOCK_COMMENT :\
> &nbsp;&nbsp; &nbsp;&nbsp; `/*` (~\[`*` `!`] | `**` | _BlockCommentOrDoc_)
>      (_BlockCommentOrDoc_ | ~`*/`)<sup>\*</sup> `*/`\
> &nbsp;&nbsp; | `/**/`\
> &nbsp;&nbsp; | `/***/`
>
> INNER_LINE_DOC :\
> &nbsp;&nbsp; `//!` ~\[`\n` _IsolatedCR_]<sup>\*</sup>
>
> INNER_BLOCK_DOC :\
> &nbsp;&nbsp; `/*!` ( _BlockCommentOrDoc_ | ~\[`*/` _IsolatedCR_] )<sup>\*</sup> `*/`
>
> OUTER_LINE_DOC :\
> &nbsp;&nbsp; `///` (~`/` ~\[`\n` _IsolatedCR_]<sup>\*</sup>)<sup>?</sup>
>
> OUTER_BLOCK_DOC :\
> &nbsp;&nbsp; `/**` (~`*` | _BlockCommentOrDoc_ )
>              (_BlockCommentOrDoc_ | ~\[`*/` _IsolatedCR_])<sup>\*</sup> `*/`
>
> _BlockCommentOrDoc_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; BLOCK_COMMENT\
> &nbsp;&nbsp; | OUTER_BLOCK_DOC\
> &nbsp;&nbsp; | INNER_BLOCK_DOC
>
> _IsolatedCR_ :\
> &nbsp;&nbsp; \\r

## Non-doc comments

r[comments.normal]

Comments follow the general C++ style of line (`//`) and
block (`/* ... */`) comment forms. Nested block comments are supported.

r[comments.normal.tokenization]
Non-doc comments are interpreted as a form of whitespace.

## Doc comments

r[comments.doc]

r[comments.doc.syntax]
Line doc comments beginning with exactly _three_ slashes (`///`), and block
doc comments (`/** ... */`), both outer doc comments, are interpreted as a
special syntax for [`doc` attributes].

r[comments.doc.attributes]
That is, they are equivalent to writing
`#[doc="..."]` around the body of the comment, i.e., `/// Foo` turns into
`#[doc="Foo"]` and `/** Bar */` turns into `#[doc="Bar"]`. They must therefore
appear before something that accepts an outer attribute.

r[comments.doc.inner-syntax]
Line comments beginning with `//!` and block comments `/*! ... */` are
doc comments that apply to the parent of the comment, rather than the item
that follows.

r[comments.doc.inner-attributes]
That is, they are equivalent to writing `#![doc="..."]` around
the body of the comment. `//!` comments are usually used to document
modules that occupy a source file.

r[comments.doc.bare-crs]
The character `U+000D` (CR) is not allowed in doc comments.

> **Note**: It is conventional for doc comments to contain Markdown, as expected by
> `rustdoc`. However, the comment syntax does not respect any internal Markdown.
> ``/** `glob = "*/*.rs";` */`` terminates the comment at the first `*/`, and the
> remaining code would cause a syntax error. This slightly limits the content of
> block doc comments compared to line doc comments.

> **Note**:  The sequence `U+000D` (CR) immediately followed by `U+000A` (LF) would have been previously transformed into a single `U+000A` (LF).

## Examples

```rust
//! A doc comment that applies to the implicit anonymous module of this crate

pub mod outer_module {

    //!  - Inner line doc
    //!! - Still an inner line doc (but with a bang at the beginning)

    /*!  - Inner block doc */
    /*!! - Still an inner block doc (but with a bang at the beginning) */

    //   - Only a comment
    ///  - Outer line doc (exactly 3 slashes)
    //// - Only a comment

    /*   - Only a comment */
    /**  - Outer block doc (exactly) 2 asterisks */
    /*** - Only a comment */

    pub mod inner_module {}

    pub mod nested_comments {
        /* In Rust /* we can /* nest comments */ */ */

        // All three types of block comments can contain or be nested inside
        // any other type:

        /*   /* */  /** */  /*! */  */
        /*!  /* */  /** */  /*! */  */
        /**  /* */  /** */  /*! */  */
        pub mod dummy_item {}
    }

    pub mod degenerate_cases {
        // empty inner line doc
        //!

        // empty inner block doc
        /*!*/

        // empty line comment
        //

        // empty outer line doc
        ///

        // empty block comment
        /**/

        pub mod dummy_item {}

        // empty 2-asterisk block isn't a doc block, it is a block comment
        /***/

    }

    /* The next one isn't allowed because outer doc comments
       require an item that will receive the doc */

    /// Where is my item?
#   mod boo {}
}
```

[`doc` attributes]: ../rustdoc/the-doc-attribute.html
