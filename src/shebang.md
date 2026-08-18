r[shebang]
# Shebang

r[shebang.intro]
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

r[shebang.syntax]
```grammar,lexer
@root SHEBANG ->
    `#!` !((WHITESPACE | LINE_COMMENT | SHEBANG_BLOCK_COMMENT)* `[`)
    ~LF* (LF | EOF)

SHEBANG_BLOCK_COMMENT ->
    `/*` !(`!` | `*` ![`*` `/`])
      ( SHEBANG_NESTED_BLOCK_COMMENT | (!(`*/` | `/*`) CHAR) )*
    `*/`

SHEBANG_NESTED_BLOCK_COMMENT ->
    `/*`
      ( SHEBANG_NESTED_BLOCK_COMMENT | (!(`*/` | `/*`) CHAR) )*
    `*/`
```

r[shebang.syntax-description]
The shebang starts with the characters `#!` and extends through the first `U+000A` (LF) or through EOF if no LF is present. If the `#!` characters are followed by `[` (ignoring any intervening [whitespace] or [non-doc comments]), the line is not considered a shebang (to avoid ambiguity with an [inner attribute]).

> [!NOTE]
> Doc comments are not ignored when determining whether `[` follows the `#!` characters. For example, `#! /*! */ [allow(unused)]` at the start of a file is a shebang, not an inner attribute, because `/*! */` is a doc comment. Likewise, text following `#!` that resembles an unterminated block comment, as in `#!/*`, does not cause an error; the line is a shebang.

r[shebang.position]
The shebang may appear immediately at the start of the file or after the optional [byte order mark].

[byte order mark]: https://en.wikipedia.org/wiki/Byte_order_mark#UTF-8
[inner attribute]: attributes.md
[non-doc comments]: comments.normal
[shebang]: https://en.wikipedia.org/wiki/Shebang_(Unix)
[whitespace]: whitespace.md
