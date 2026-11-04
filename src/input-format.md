r[input]
# Input format

r[input.syntax]
```grammar,lexer
CHAR -> [U+0000-U+D7FF U+E000-U+10FFFF] // a Unicode scalar value

ASCII -> [U+0000-U+007F]

NUL -> U+0000

EOF -> !CHAR  // End of file or input
```

r[input.intro]
This chapter describes how a source file is interpreted as a sequence of tokens.

See [Crates and source files] for a description of how programs are organised into files.

r[input.encoding]
## Source encoding

r[input.encoding.utf8]
Each source file is interpreted as a sequence of Unicode characters encoded in UTF-8. Information on this encoding can be found at <https://www.unicode.org/versions/latest/core-spec/chapter-3/#G31703>.

r[input.encoding.invalid]
It is an error if the file is not valid UTF-8.

r[input.encoding.unicode-version]
Unicode is an evolving standard, which Rust tracks. The exact version used by the Rust language is delegated to the standard library, documented at [`char::UNICODE_VERSION`].

r[input.byte-order-mark]
## Byte order mark removal

If the first character in the sequence is `U+FEFF` ([BYTE ORDER MARK]), it is removed.

r[input.crlf]
## CRLF normalization

Each pair of characters `U+000D` (CR) immediately followed by `U+000A` (LF) is replaced by a single `U+000A` (LF). This happens once, not repeatedly, so after the normalization, there can still exist `U+000D` (CR) immediately followed by `U+000A` (LF) in the input (e.g. if the raw input contained "CR CR LF LF").

Other occurrences of the character `U+000D` (CR) are left in place (they are treated as [whitespace]).

r[input.shebang]
## Shebang removal

r[input.shebang.removal]
If a [shebang] is present, it is removed from the input sequence (and is therefore ignored).

r[input.tokenization]
## Tokenization

The resulting sequence of characters is then converted into tokens as described in the remainder of this chapter.

> [!NOTE]
> The standard library [`include!`] macro applies the following transformations to the file it reads:
>
> - Byte order mark removal.
> - CRLF normalization.
> - Shebang removal when invoked in an item context (as opposed to expression or statement contexts).
>
> The [`include_str!`] and [`include_bytes!`] macros do not apply these transformations.

[BYTE ORDER MARK]: https://en.wikipedia.org/wiki/Byte_order_mark#UTF-8
[Crates and source files]: crates-and-source-files.md
[shebang]: shebang.md
[whitespace]: whitespace.md
[`char::UNICODE_VERSION`]: https://doc.rust-lang.org/nightly/std/primitive.char.html#associatedconstant.UNICODE_VERSION
