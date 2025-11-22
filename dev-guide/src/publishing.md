# Publishing process

The process for getting the Reference content into a [Rust release](https://doc.rust-lang.org/reference/#rust-releases) and on the website is:

1. Changes are merged to this repository.
2. [Triagebot](https://forge.rust-lang.org/triagebot/doc-updates.html) will automatically synchronize this repository to [rust-lang/rust]. This happens every other week. The reference is tracked in [rust-lang/rust] as a [submodule](https://github.com/rust-lang/rust/tree/master/src/doc).
  - This will open a PR on [rust-lang/rust] which needs to be merged, and that can take up to several days.
3. At midnight UTC, whatever is on the default branch of [rust-lang/rust] will be a part of that nightly release, and will be published after a few hours to <https://doc.rust-lang.org/nightly/reference/>.
4. Following Rust's [release process](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html), every 6 weeks, nightly will be promoted to beta (<https://doc.rust-lang.org/beta/reference/>), and then 6 weeks after that it will be promoted to stable (<https://doc.rust-lang.org/stable/reference/>).

[rust-lang/rust]: https://github.com/rust-lang/rust/
