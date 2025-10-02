r[frontmatter]
# Frontmatter

r[frontmatter.syntax]
```grammar,lexer
FRONTMATTER ->
      FRONTMATTER_FENCE INFOSTRING? LF
      (FRONTMATTER_LINE LF )*
      FRONTMATTER_FENCE[^matched-fence] LF

FRONTMATTER_FENCE -> `---` `-`*

INFOSTRING -> (XID_Start | `_`) ( XID_Continue | `-` | `.` )*

FRONTMATTER_LINE -> (~INVALID_FRONTMATTER_LINE_START (~INVALID_FRONTMATTER_LINE_CONTINUE)*)?

INVALID_FRONTMATTER_LINE_START -> (FRONTMATTER_FENCE[^escaped-fence] | LF)

INVALID_FRONTMATTER_LINE_CONTINUE -> LF
```

[^matched-fence]: The closing fence must have the same number of `-` as the opening fence
[^escaped-fence]: A `FRONTMATTER_FENCE` at the beginning of a `FRONTMATTER_LINE` is only invalid if it has the same or more `-` as the `FRONTMATTER_FENCE`

Frontmatter is an optional section for content intended for external tools without requiring these tools to have full knowledge of the Rust grammar.

r[frontmatter.document]
Frontmatter may only be preceded by a [shebang] and whitespace.

r[frontmatter.fence]
The delimiters are referred to as a "fence."
The opening and closing fences must be at the start of a line.
They must be a matching pair of three or more hyphens (`-`).

r[frontmatter.infostring]
Following the opening fence may be an infostring for identifying the intention of the contained content.
An infostring may be preceded by non-newline whitespace.

r[frontmatter.body]
The body of the frontmatter may contain any content except for a line starting with as many or more hyphens (`-`) than in the fences.

[shebang]: input-format.md#shebang-removal
