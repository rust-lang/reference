# Admonitions

[`mdbook-spec`](../tooling/mdbook-spec.md) provides admonitions that use a style similar to GitHub-flavored Markdown. The style name is placed at the beginning of a blockquote, such as:

```markdown
> [!WARNING]
> This is a warning.

> [!NOTE]
> This is a note.

> [!EDITION-2024]
> This is an edition-specific difference.

> [!EXAMPLE]
> This is an example.
```

The color and styling are defined in [`theme/reference.css`](https://github.com/rust-lang/reference/blob/HEAD/theme/reference.css) and the transformation and icons are in [`tools/mdbook-spec/src/admonitions.rs`](https://github.com/rust-lang/reference/blob/HEAD/tools/mdbook-spec/src/admonitions.rs).

See **[Conventions]** in the Reference introduction for a description of how these should be used.

[Conventions]: https://doc.rust-lang.org/nightly/reference/#conventions
