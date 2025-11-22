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

## How is this published?

The process for getting the reference content into a [Rust release](https://doc.rust-lang.org/reference/#rust-releases) and on the website is:

1. Changes are merged to this repository.
2. [Triagebot](https://forge.rust-lang.org/triagebot/doc-updates.html) will automatically synchronize this repository to [rust-lang/rust]. This happens every other week. The reference is tracked in [rust-lang/rust] as a [submodule](https://github.com/rust-lang/rust/tree/master/src/doc).
  - This will open a PR on [rust-lang/rust] which needs to be merged, and that can take up to several days.
3. At midnight UTC, whatever is on the default branch of [rust-lang/rust] will be a part of that nightly release, and will be published after a few hours to <https://doc.rust-lang.org/nightly/reference/>.
4. Following Rust's [release process](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html), every 6 weeks, nightly will be promoted to beta (<https://doc.rust-lang.org/beta/reference/>), and then 6 weeks after that it will be promoted to stable (<https://doc.rust-lang.org/stable/reference/>).

[rust-lang/rust]: https://github.com/rust-lang/rust/
