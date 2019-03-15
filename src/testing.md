# Testing

## The `test` attribute

The compiler comes with a default test framework. It works by attributing
functions with the `test` attribute. These functions are only compiled when
compiling with the test harness. Like [main], functions annotated with this
attribute must take no arguments, must not declare any
[trait or lifetime bounds], must not have any [where clauses], and its return
type must be one of the following:

* `()`
* `Result<(), E> where E: Error`
<!-- * `!` -->
<!-- * Result<!, E> where E: Error` -->

> Note: The implementation of which return types are allowed is determined by
> the unstable [`Termination`] trait.

<!-- If the previous section needs updating (from "must take no arguments"
  onwards, also update it in the crates-and-source-files.md file -->

> Note: The test harness is ran by passing the `--test` argument to `rustc` or
> using `cargo test`.

Tests that return `()` pass as long as they terminate and do not panic. Tests
that return a `Result` pass as long as they return `Ok(())`. Tests that do not
terminate neither pass nor fail.

```rust
# use std::io;
# fn setup_the_thing() -> io::Result<i32> { Ok(1) }
# fn do_the_thing(s: &i32) -> io::Result<()> { Ok(()) }
#[test]
fn test_the_thing() -> io::Result<()> {
    let state = setup_the_thing()?; // expected to succeed
    do_the_thing(&state)?;          // expected to succeed
    Ok(())
}
```

## The `ignore` attribute

A function annotated with the `test` attribute can also be annotated with the
`ignore` attribute. The *`ignore` attribute* tells the test harness to not
execute that function as a test. It will still only be compiled when compiling
with the test harness.

The `ignore` attribute may optionally be written with the [_MetaNameValueStr_]
syntax to specify a reason why the test is ignored.

```rust
#[test]
#[ignore = "not yet implemented"]
fn mytest() {
    // …
}
```

> **Note**: The `rustc` test harness supports the `--include-ignored` flag to
> force ignored tests to be run.

## The `should_panic` attribute

A function annotated with the `test` attribute that returns `()` can also be
annotated with the `should_panic` attribute. The *`should_panic` attribute*
makes the test only pass if it actually panics.

The `should_panic` attribute may optionally take an input string that must
appear within the panic message. If the string is not found in the message,
then the test will fail. The string may be passed using the
[_MetaNameValueStr_] syntax or the [_MetaListNameValueStr_] syntax with an
`expected` field.

```rust
#[test]
#[should_panic(expected = "values don't match")]
fn mytest() {
    assert_eq!(1, 2, "values don't match");
}
```

[_MetaListNameValueStr_]: attributes.html#meta-item-attribute-syntax
[_MetaNameValueStr_]: attributes.html#meta-item-attribute-syntax
[`Termination`]: ../std/process/trait.Termination.html
[main]: crates-and-source-files.html
[trait or lifetime bounds]: trait-bounds.html
[where clauses]: items/generics.html#where-clauses
