# The Rust Language Reference

This document is the primary reference for the Rust programming language.

## Contributor docs

There are several pages for those working on the reference:

* [Authoring guide](https://github.com/rust-lang/reference/blob/master/docs/authoring.md): Guidelines for writing content.
* [Review policy](https://github.com/rust-lang/reference/blob/master/docs/review-policy.md): Guidelines for reviewers.
* [Grammar](https://github.com/rust-lang/reference/blob/master/docs/grammar.md): How the grammar syntax works.
* [Attribute template](https://github.com/rust-lang/reference/blob/master/docs/attribute-template.md): The standard template for documenting an attribute.

## Running tests

There are several different kinds of tests you can run (these are enforced on CI):

* `mdbook test`: This will run the inline Rust codeblocks (internally it uses `rustdoc` to do this).
* `cargo xtask style-check`: This will validate some style checks (see [authoring guide](docs/authoring.md)).
* `cargo xtask linkcheck`: This will validate that markdown links aren't broken.
* `cargo xtask test-all`: Runs all tests.
