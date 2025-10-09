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
Each source file is interpreted as a sequence of Unicode characters encoded in UTF-8.

r[input.encoding.invalid]
It is an error if the file is not valid UTF-8.

r[input.byte-order-mark]
## Byte order mark removal

If the first character in the sequence is `U+FEFF` ([BYTE ORDER MARK]), it is removed.

r[input.crlf]
## CRLF normalization

Each pair of characters `U+000D` (CR) immediately followed by `U+000A` (LF) is replaced by a single `U+000A` (LF). This happens once, not repeatedly, so after the normalization, there can still exist `U+000D` (CR) immediately followed by `U+000A` (LF) in the input (e.g. if the raw input contained "CR CR LF LF").

Other occurrences of the character `U+000D` (CR) are left in place (they are treated as [whitespace]).

r[input.shebang]
## Shebang removal

r[input.shebang.intro]
A *[shebang]* is an optional line that is typically used in Unix-like systems to specify an interpreter for executing the file.

> [!EXAMPLE]
> <!-- ignore: tests don't like shebang -->
> ```rust,ignore
> #!/usr/bin/env rustx
>
> fn main() {
>     println!("Hello!");
> }
> ```

r[input.shebang.syntax]
```grammar,lexer
@root SHEBANG ->
    `#!` !((WHITESPACE | LINE_COMMENT | BLOCK_COMMENT)* `[`)
    ~LF* (LF | EOF)
```

The shebang starts with the characters `#!` and extends through the first `U+000A` (LF) or through EOF if no LF is present. If the `#!` characters are followed by `[` (ignoring any intervening [comments] or [whitespace]), the line is not considered a shebang (to avoid ambiguity with an [inner attribute]).

r[input.shebang.position]
The shebang may appear immediately at the start of the file or after the optional [byte order mark].

r[input.shebang.removal]
The shebang is removed from the input sequence (and is therefore ignored).

r[input.frontmatter]
## Frontmatter removal

After some [whitespace], [frontmatter] may next appear in the input.

r[input.tokenization]
## Tokenization

The resulting sequence of characters is then converted into tokens as described in the remainder of this chapter.

> [!NOTE]
> The standard library [`include!`] macro applies the following transformations to the file it reads:
>
> - Byte order mark removal.
> - CRLF normalization.
> - Shebang and frontmatter removal when invoked in an item context (as opposed to expression or statement contexts).
>
> The [`include_str!`] and [`include_bytes!`] macros do not apply these transformations.

[inner attribute]: attributes.md
[BYTE ORDER MARK]: https://en.wikipedia.org/wiki/Byte_order_mark#UTF-8
[comments]: comments.md
[Crates and source files]: crates-and-source-files.md
[shebang]: https://en.wikipedia.org/wiki/Shebang_(Unix)
[frontmatter]: frontmatter.md
[whitespace]: whitespace.md
