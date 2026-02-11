# Examples

## Example code blocks

Code examples should use code blocks with triple backticks. The language should always be specified (such as `rust`).

```rust
println!("Hello!");
```

See the [mdBook supported languages] for a list of supported languages.

## rustdoc attributes

Rust examples are [tested via rustdoc] and should include the appropriate annotations:

- `edition2015`, `edition2018`, etc. --- Use if it is edition-specific (see `book.toml` for the default).
- `no_run` --- The example should compile successfully but should not be executed.
- `should_panic` --- The example should compile and run but produce a panic.
- `compile_fail` --- The example is expected to fail to compile.
- `ignore` --- The example shouldn't be built or tested. This should be avoided if possible. Usually, this is only necessary when the testing framework does not support it (such as external crates, modules, or a proc-macro) or when it contains pseudocode that is not valid Rust. An HTML comment, such as `<!-- ignore: requires extern crate -->`, should be placed before the example to explain why it is ignored.
- `Exxxx` --- If the example is expected to fail to compile with a specific error code, include that code so that `rustdoc` checks that the expected code is used.

See the [rustdoc documentation] for more detail.

## Combining examples

When demonstrating success cases, multiple cases may be included in a single code block. For failure cases, however, each example must appear in a separate code block so that the tests can ensure that each case indeed fails with the appropriate error code.

## Testing examples

The Rust code blocks are tested in CI. You can verify that the samples pass by running [`mdbook test`].

[`mdbook test`]: tests.md#inline-tests
[mdBook supported languages]: https://rust-lang.github.io/mdBook/format/theme/syntax-highlighting.html#supported-languages
[rustdoc documentation]: https://doc.rust-lang.org/rustdoc/documentation-tests.html
[tested via rustdoc]: tests.md#inline-tests
