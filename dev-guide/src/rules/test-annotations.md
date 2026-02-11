# rustc test annotations

Tests in <https://github.com/rust-lang/rust> can be linked to rules in the Reference. The rule will include a link to the tests, and there is also an [appendix] that tracks how the rules are currently linked.

Tests in the `tests` directory can be annotated with the `//@ reference: x.y.z` header to link them to a rule. The header can be specified multiple times if a single file covers multiple rules.

Compiler developers are not expected to add `reference` annotations to tests. However, if they do want to help, their cooperation is welcome. Reference authors and editors are responsible for ensuring every rule has a test associated with it.

The tests are beneficial for reviewers to see the behavior of a rule. They are also a benefit to readers who may want to see examples of particular behaviors. When adding new rules, you should wait until the Reference side is approved before submitting a PR to `rust-lang/rust` (to avoid churn if we decide on different names).

Always annotate with the most specific rule name available. For example, use `asm.rules.reg-not-input` rather than the broader `asm.rules`.

Complete coverage is the goal but is not yet expected.

[appendix]: https://doc.rust-lang.org/nightly/reference/test-summary.html
