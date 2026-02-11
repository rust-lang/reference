# Running tests

There are several different kinds of tests you can run (these are enforced in CI):

- [`cargo xtask test-all`](#all-tests) --- Runs all tests.
- [`mdbook test`](#inline-tests) --- Tests the inline Rust code blocks.
- [`cargo xtask linkcheck`](#linkcheck) --- Validates that Markdown links aren't broken.
- [`cargo xtask style-check`](#style-checks) --- Validates various style checks.
- [Code formatting](#code-formatting) --- Checks that all Rust tooling code is formatted.
- [mdbook-spec tests](#mdbook-spec-tests) --- Internal tests for `mdbook-spec`.

## All tests

```sh
cargo xtask test-all
```

This command runs all the tests listed below.

We recommend running this as a last step before opening a PR. This runs most of the tests required for CI to pass. See [`tools/xtask/src/main.rs`](https://github.com/rust-lang/reference/blob/HEAD/tools/xtask/src/main.rs) for details on what this does.

## Inline tests

```sh
mdbook test
```

This command runs all tests that are inline in the Markdown. Internally, this uses [`rustdoc`](https://doc.rust-lang.org/rustdoc/) to run the tests and supports all the same features. Any code block with the `rust` language will be compiled unless it is ignored. See [Examples] for more.

## Linkcheck

```sh
cargo xtask linkcheck
```

This command verifies that links are not broken. It downloads and uses the [`linkchecker`](https://github.com/rust-lang/rust/tree/main/src/tools/linkchecker) script hosted in the `rust-lang/rust` repository.

This requires a recent nightly installed via `rustup` and the `rust-docs` component.

After compiling the script, it builds the Reference using `mdbook` and then scans all local links to verify that they are valid, particularly between various books. This does not check any network links.

## Style checks

```sh
cargo xtask style-check
```

This uses the [`style-check`](https://github.com/rust-lang/reference/tree/HEAD/tools/style-check) tool to enforce various formatting rules.

## Code formatting

CI uses `cargo fmt --check` to verify that all Rust sources for the tools (such as `mdbook-spec`) are properly formatted. All code must be formatted with `rustfmt`.

## mdbook-spec tests

```sh
cargo test --manifest-path mdbook-spec/Cargo.toml
```

CI runs `cargo test` on `mdbook-spec` to execute any tests for the tool itself.

[Examples]: examples.md
