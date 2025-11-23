# rustc test annotations

Tests in <https://github.com/rust-lang/rust> can be linked to rules in the reference. The rule will include a link to the tests, and there is also an [appendix] which tracks how the rules are currently linked.

Tests in the `tests` directory can be annotated with the `//@ reference: x.y.z` header to link it to a rule. The header can be specified multiple times if a single file covers multiple rules.

Compiler developers are not expected to add `reference` annotations to tests. However, if they do want to help, their cooperation is very welcome. Reference authors and editors are responsible for making sure every rule has a test associated with it.

The tests are beneficial for reviewers to see the behavior of a rule. It is also a benefit to readers who may want to see examples of particular behaviors. When adding new rules, you should wait until the reference side is approved before submitting a PR to `rust-lang/rust` (to avoid churn if we decide on different names).

Prefixed rule names should not be used in tests. That is, do not use something like `asm.rules` when there are specific rules like `asm.rules.reg-not-input`.

We are not expecting 100% coverage at any time. Although it would be nice, it is unrealistic due to the sequence things are developed, and resources available.

[appendix]: https://doc.rust-lang.org/nightly/reference/test-summary.html
