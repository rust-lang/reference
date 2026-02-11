# Markdown

There are automated checks for some of these rules. Run [`cargo xtask style-check`] to run them locally.

## Formatting style

- Use [ATX-style headings][atx] (not Setext) with [sentence case].
- Do not use tabs; use only spaces.
- Files must end with a newline.
- Lines must not end with spaces. Double spaces have semantic meaning but can be invisible. Use a trailing backslash if you need a hard line break.
- If possible, avoid double blank lines.
- Do not wrap long lines. This helps with reviewing diffs of the source.
- Use [smart punctuation] instead of Unicode characters. For example, use `---` for an em dash instead of the Unicode character. Characters such as the em dash can be difficult to see in a fixed-width editor, and some editors may not have easy methods to enter such characters.
- See [Admonitions] for formatting callouts such as notes, edition differences, and warnings.

## Code blocks

- Do not use indented code blocks; use fenced code blocks with 3+ backticks instead.
- Code blocks should have an explicit language tag.

## Links

See [Links] for more information about linking.

- Links to other chapters should be relative and use the `.md` extension.
- Links to other rust-lang books that are published with the Reference should also be relative so that the linkchecker can validate them. See [outside book links].
- Links to the standard library should use rustdoc-style links as described in [standard library links].
- Prefer reference links, with shortcut reference links where appropriate. Place sorted link reference definitions at the bottom of the file, or at the bottom of a section if there is an unusually large number of links specific to that section.

    ```markdown
    Example of shortcut link: [enumerations]
    Example of reference link with label: [block expression][block]

    [block]: expressions/block-expr.md
    [enumerations]: types/enum.md
    ```

[`cargo xtask style-check`]: ../tests.md#style-checks
[Admonitions]: admonitions.md
[atx]: https://spec.commonmark.org/0.31.2/#atx-headings
[Links]: ../links.md
[outside book links]: ../links.md#outside-book-links
[sentence case]: https://apastyle.apa.org/style-grammar-guidelines/capitalization/sentence-case
[smart punctuation]: https://rust-lang.github.io/mdBook/format/markdown.html#smart-punctuation
[standard library links]: ../links.md#standard-library-links
