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
Frontmatter is an optional section for content intended for external tools without requiring these tools to have full knowledge of the Rust grammar.

```rust
#!/usr/bin/env cargo
---
[dependencies]
fastrand = "2"
---

fn main() {
    let num = fastrand::i32(..);
    println!("{num}");
}
```

r[frontmatter.document]
Frontmatter may only be preceded by a [shebang] and [whitespace].

r[frontmatter.fence]
The delimiters are referred to as a *fence*. The opening and closing fences must be at the start of a line. They must be a matching pair of hyphens (`-`), from 3 to 255. A fence may be followed by horizontal whitespace.

r[frontmatter.infostring]
Following the opening fence may be an infostring for identifying the intention of the contained content. An infostring may be followed by horizontal whitespace.

r[frontmatter.body]
The body of the frontmatter may contain any content except for a line starting with as many or more hyphens (`-`) than in the fences or carriage returns.

[shebang]: input-format.md#shebang-removal
[whitespace]: whitespace.md
