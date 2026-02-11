# mdbook-spec

[`mdbook-spec`] is an mdBook preprocessor that adds features to the Reference. It provides:

- Parsing and generation of [grammar diagrams].
  - [Automatic grammar production links].
  - Generation of the [grammar summary appendix].
- [Automatic standard library links].
- Handling of [rule names].
  - Validation of the names.
  - Converting rule names to links.
  - [Automatic rule link references].
  - Generation of [links to rule tests].
  - Generation of the [test summary].
- Support for [admonitions].

## Environment variables

There are a few environment variables that `mdbook-spec` uses, described in **[Building the Reference]**:

- [`SPEC_RELATIVE`] --- Can be set to link external books to the live site.
- [`SPEC_DENY_WARNINGS`] --- Whether warnings should be treated as errors.
- [`SPEC_RUST_ROOT`] --- The path to a checkout of the [`rust-lang/rust`] GitHub repository. This is used for test linking.

[`mdbook-spec`]: https://github.com/rust-lang/reference/tree/HEAD/tools/mdbook-spec
[`rust-lang/rust`]: https://github.com/rust-lang/rust
[`SPEC_DENY_WARNINGS`]: building.md#SPEC_DENY_WARNINGS
[`SPEC_RELATIVE`]: building.md#SPEC_RELATIVE
[`SPEC_RUST_ROOT`]: building.md#SPEC_RUST_ROOT
[admonitions]: ../formatting/admonitions.md
[Automatic grammar production links]: ../grammar.md#automatic-linking
[Automatic rule link references]: ../links.md#rule-links
[Automatic standard library links]: ../links.md#standard-library-links
[Building the Reference]: building.md
[grammar diagrams]: ../grammar.md
[grammar summary appendix]: https://doc.rust-lang.org/nightly/reference/grammar.html
[links to rule tests]: ../rules/test-annotations.md
[rule names]: ../rules/index.md
[test summary]: https://doc.rust-lang.org/nightly/reference/test-summary.html
