# Stabilization process

New features and changes to the Rust language usually require an update to the Reference to incorporate the change. This can be done at any time before stabilization, and it is usually better to prepare a PR early (assuming the implementation is not expected to change significantly).

An exception to this process occurs when a language change involves a section of the Reference or language that is undocumented. For example, type inference is currently not documented, so changes to it do not require an update to the Reference.

## Pull request

When opening a PR, please include links to as much information as possible so that reviewers can better understand the change. This includes links to the following, if they exist:

- The tracking issue.
- The `rust-lang/rust` stabilization pull request.
- The stabilization report.
- Background information such as RFCs.
- The files in `rustc` where it is implemented, if it is isolated to a relatively concise part.
- The tests in `rust-lang/rust`.

Some of this information may already exist, such as in the tracking issue, stabilization report, or PR, so there is no need to duplicate it.

## Inline tests

Because the Reference only documents stabilized features, inline tests will fail. We intend to improve this process in the future (see [#1864]).

[#1864]: https://github.com/rust-lang/reference/issues/1864
