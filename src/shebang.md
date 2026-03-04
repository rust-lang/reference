r[shebang]
# Shebang

r[shebang.syntax]
```grammar,lexer
@root SHEBANG ->
    `#!` !((WHITESPACE | LINE_COMMENT | BLOCK_COMMENT)* `[`)
    ~LF* (LF | EOF)
```

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

r[shebang.syntax-description]
The shebang starts with the characters `#!` and extends through the first `U+000A` (LF) or through EOF if no LF is present. If the `#!` characters are followed by `[` (ignoring any intervening [comments] or [whitespace]), the line is not considered a shebang (to avoid ambiguity with an [inner attribute]).

r[shebang.position]
The shebang may appear immediately at the start of the file or after the optional [byte order mark].

[byte order mark]: https://en.wikipedia.org/wiki/Byte_order_mark#UTF-8
[comments]: comments.md
[inner attribute]: attributes.md
[shebang]: https://en.wikipedia.org/wiki/Shebang_(Unix)
[whitespace]: whitespace.md
