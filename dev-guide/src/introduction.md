# Introduction

Thank you for your interest in contributing to **The Rust Reference**. This document provides an overview of how to contribute to the Reference and serves as a guide for editors and reviewers.

There are a few ways of helping with the Reference: critiquing the Reference, editing the Reference, fixing incorrect information, adding examples and glossary entries, and documenting new or otherwise undocumented features in Rust.

We encourage you to read the [introduction] of the Reference to familiarize yourself with the kind of content the Reference is expected to contain and the conventions it uses.

## Critiquing the Reference

This is the easiest way to contribute. As you read the Reference, if you find something confusing, incorrect, or missing, then you can file an issue against the Reference explaining your concerns.

## Editing the Reference

Typos and incorrect links get through from time to time. Should you find them, we welcome PRs to fix them.

## Adding examples and glossary entries

Examples are great. Many people will only read examples and ignore the prose. Ideally, every facet of every feature should have an example.

Likewise, the Reference has a glossary. It doesn't need to explain everything or contain every possible definition, but it does need to be expanded upon from its current state. Ideally, entries in the glossary should link to the associated documentation.

## Adding documentation

There are a lot of features that are not documented at all or are documented poorly. This is the hardest but most valuable task. Pick an unassigned issue from the [issue tracker] and write about it.

While writing, you may find it handy to have a [playground] open to test out what you are documenting.

Feel free to take information from the standard library and Rustonomicon as appropriate.

Note that we don't write documentation for purely library features, such as threads and IO, and we don't write about Rust in the future. Documentation is written as if the current stable release of Rust is the last release. The `master` branch of the Reference corresponds to what is **stable** on the `main` branch ("nightly") of [rust-lang/rust]. If you want to write about Rust in the future, you want **[The Unstable Book][unstable]**.

[introduction]: https://doc.rust-lang.org/nightly/reference/introduction.html
[issue tracker]: https://github.com/rust-lang/reference/issues
[playground]: https://play.rust-lang.org/
[rust-lang/rust]: https://github.com/rust-lang/rust/
[unstable]: https://doc.rust-lang.org/nightly/unstable-book/
