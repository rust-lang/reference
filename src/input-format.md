# Input format

r[input]

r[input.intro]
This chapter describes how a source file is interpreted as a sequence of tokens.

See [Crates and source files] for a description of how programs are organised into files.

## Source encoding

r[input.encoding]

r[input.encoding.utf8]
Each source file is interpreted as a sequence of Unicode characters encoded in UTF-8.

r[input.encoding.invalid]
It is an error if the file is not valid UTF-8.

## Byte order mark removal

r[input.byte-order-mark]

If the first character in the sequence is `U+FEFF` ([BYTE ORDER MARK]), it is removed.

## CRLF normalization

r[input.crlf]

Each pair of characters `U+000D` (CR) immediately followed by `U+000A` (LF) is replaced by a single `U+000A` (LF).

Other occurrences of the character `U+000D` (CR) are left in place (they are treated as [whitespace]).

## Shebang removal

r[input.shebang]

r[input.shebang.intro]
If the remaining sequence begins with the characters `#!`, the characters up to and including the first `U+000A` (LF) are removed from the sequence.

For example, the first line of the following file would be ignored:

<!-- ignore: tests don't like shebang -->
```rust,ignore
#!/usr/bin/env rustx

fn main() {
    println!("Hello!");
}
```

r[input.shebang.inner-attribute]
As an exception, if the `#!` characters are followed (ignoring intervening [comments] or [whitespace]) by a `[` token, nothing is removed.
This prevents an [inner attribute] at the start of a source file being removed.

> **Note**: The standard library [`include!`] macro applies byte order mark removal, CRLF normalization, and shebang removal to the file it reads. The [`include_str!`] and [`include_bytes!`] macros do not.

## Tokenization

r[input.tokenization]

The resulting sequence of characters is then converted into tokens as described in the remainder of this chapter.

[inner attribute]: attributes.md
[BYTE ORDER MARK]: https://en.wikipedia.org/wiki/Byte_order_mark#UTF-8
[comments]: comments.md
[Crates and source files]: crates-and-source-files.md
[_shebang_]: https://en.wikipedia.org/wiki/Shebang_(Unix)
[whitespace]: whitespace.md
