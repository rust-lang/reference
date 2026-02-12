# Stabilization process

New features and changes to the Rust language usually require an update to the Reference to incorporate the change. This can be done at any time before stabilization, and it is usually better to prepare a PR early (assuming the implementation is not expected to change significantly).

An exception to this process occurs when a language change involves a part of the language that is undocumented or a section of the Reference that is incomplete. For example, type inference is currently not documented, so changes to the details of type inference do not require an update to the Reference.

However, when a new feature introduces a rule that can be stated independently of the undocumented material, that rule should still be documented. In this case, add the new rule to the relevant placeholder section. When the section is eventually filled out, the rule will be incorporated into the complete text.

## Pull request

When opening a PR, please include links to as much information as possible so that reviewers can better understand the change. This includes links to the following, if they exist:

- The tracking issue.
- The `rust-lang/rust` stabilization pull request.
- The stabilization report.
- Background information such as RFCs.
- The files in `rustc` where it is implemented, if it is isolated to a relatively concise part.
- The tests in `rust-lang/rust`.

Always link to the tracking issue and, if applicable, the stabilization PR. Beyond those, information that already appears in the tracking issue, stabilization report, or PR does not need to be duplicated in the PR description.

## Inline tests

If a PR documents a newly stabilized feature, its inline tests will fail until the stabilization PR is merged and a new nightly compiler is available. We intend to improve this process in the future (see [#1864]).

[#1864]: https://github.com/rust-lang/reference/issues/1864
