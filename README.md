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

To build the project, follow the steps given below :

Clone the project by downloading the ZIP from the [GitHub page](https://github.com/rust-lang/reference) or
run the following command:

```
git clone https://github.com/rust-lang/reference
```

Change the directory to the downloaded repository:

```sh
cd reference
```
Run the following command to test the code snippets to catch compilation errors:

```shell
mdbook test
```


To generate a local instance of the book, run:

```sh
mdbook build
```

The generated HTML will be in the `book` folder.
