# The Rust Language Reference

This document is the primary reference for the Rust programming language.

This document is not normative. It may include details that are specific
to `rustc` itself, and should not be taken as a specification for the
Rust language. We intend to produce such a document someday, but this is
what we have for now.

## Dependencies

- rustc (the Rust compiler).
- mdbook (use `cargo install mdbook` to install it).

## Build steps

First, go to the repository folder and test the code snippets to catch
compilation errors:

```bash
cd reference
mdbook test
```

And then generate the book:

```bash
mdbook build
```

The generated HTML will be in the `docs` folder.
