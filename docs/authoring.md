# Authoring Guide

This document serves as a guide for editors and reviewers. Some conventions and content guidelines are specified in the [introduction].

[introduction]: ../src/introduction.md

### Linkcheck

To verify that links are not broken, run `cargo xtask linkcheck`.

### Running all tests

As a last step before opening a PR, it is recommended to run `cargo xtask test-all`. This will go through and run most of the tests that are required for CI to pass. See `xtask/src/main.rs` for what all this does.

## Style

Idioms and styling:

* Use American English spelling.
* Use Oxford commas.
* Avoid slashes for alternatives ("program/binary"); use conjunctions or rewrite it ("program or binary").
* Avoid qualifying something as "in Rust"; the entire reference is about Rust.

## Content guidelines

The following are guidelines for the content of the reference.

### Targets

The reference does not document which targets exist, or the properties of specific targets. The reference may refer to *platforms* or *target properties* where required by the language. Some examples:

* Conditional-compilation keys like `target_os` are specified to exist, but not what their values must be.
* The `windows_subsystem` attribute specifies that it only works on Windows platforms.
* Inline assembly and the `target_feature` attribute specify the architectures that are supported.

### Editions

The main text and flow should document only the current edition. Whenever there is a difference between editions, the differences should be called out with an edition block, such as:

```markdown
r[foo.bar.edition2021]
> [!EDITION-2021]
> Describe what changed in 2021.
```
