r[frontmatter]
# Frontmatter

r[frontmatter.syntax]
```grammar,lexer
@root FRONTMATTER ->
    WHITESPACE_ONLY_LINE*
    !FRONTMATTER_INVALID
    FRONTMATTER_MAIN

WHITESPACE_ONLY_LINE -> (!LF WHITESPACE)* LF

FRONTMATTER_INVALID -> (!LF WHITESPACE)+ `---` ^ ⊥

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
```

r[frontmatter.intro]
Frontmatter is an optional section of metadata whose syntax allows external tools to read it without parsing Rust.

> [!EXAMPLE]
> <!-- ignore: test runner doesn't support frontmatter -->
> ```rust,ignore
> #!/usr/bin/env cargo
> --- cargo
> package.edition = 2024
> ---
>
> fn main() {}
> ```

r[frontmatter.position]
Frontmatter may appear at the start of the file (after the optional [byte order mark]) or after a [shebang]. In either case, it may be preceded by [whitespace].

r[frontmatter.fence]
Frontmatter must start and end with a *fence*. Each fence must start at the beginning of a line. The opening fence must consist of at least 3 and no more than 255 hyphens (`-`). The closing fence must have exactly the same number of hyphens as the opening fence. The hyphens of either fence may be followed by [horizontal whitespace].

r[frontmatter.infostring]
The opening fence, after optional [horizontal whitespace], may be followed by an infostring that identifies the format or purpose of the body. An infostring may be followed by horizontal whitespace.

r[frontmatter.body]
No line in the body may start with a sequence of hyphens (`-`) equal to or longer than the opening fence. The body may not contain carriage returns.

[byte order mark]: https://en.wikipedia.org/wiki/Byte_order_mark#UTF-8
[horizontal whitespace]: grammar-HORIZONTAL_WHITESPACE
[shebang]: input-format.md#shebang-removal
[whitespace]: whitespace.md
