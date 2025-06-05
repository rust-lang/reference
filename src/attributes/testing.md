r[attributes.testing]
# Testing attributes

The following [attributes] are used for specifying functions for performing
tests. Compiling a crate in "test" mode enables building the test functions
along with a test harness for executing the tests. Enabling the test mode also
enables the [`test` conditional compilation option].

r[attributes.testing.test]
## The `test` attribute

r[attributes.testing.test.intro]
The *`test` [attribute][attributes]* marks a function to be executed as a test.

> [!EXAMPLE]
> ```rust
> # pub fn add(left: u64, right: u64) -> u64 { left + right }
>
> #[test]
> fn it_works() {
>     let result = add(2, 2);
>     assert_eq!(result, 4);
> }
> ```

r[attributes.testing.test.syntax]
The `test` attribute uses the [MetaWord] syntax and thus does not take any inputs.

r[attributes.testing.test.allowed-positions]
The `test` attribute may only be applied to [free functions] that are monomorphic, that take no arguments, and the return type must implement the [`Termination`] trait.

> [!NOTE]
> Some of types that implement the [`Termination`] trait include:
> * `()`
> * `Result<T, E> where T: Termination, E: Debug`

r[attributes.testing.test.duplicates]
Only the first instance of `test` on an function is honored. Subsequent `test` attributes are ignored.

> [!NOTE]
> `rustc` currently warns on duplicate `test` attributes.

<!-- TODO: This is a minor lie. Currently rustc warns that duplicates are ignored, but it then generates multiple test entries with the same name. I would vote for rejecting this in the future. -->

r[attributes.testing.test.enabled]
These functions are only compiled when in test mode.

> [!NOTE]
> The test mode is enabled by passing the `--test` argument to `rustc` or using `cargo test`.

r[attributes.testing.test.success]
The test harness calls the returned value's [`report`] method, and classifies the test as passed or failed depending on whether the resulting [`ExitCode`] represents successful termination.
In particular:
* Tests that return `()` pass as long as they terminate and do not panic.
* Tests that return a `Result<(), E>` pass as long as they return `Ok(())`.
* Tests that return `ExitCode::SUCCESS` pass, and tests that return `ExitCode::FAILURE` fail.
* Tests that do not terminate neither pass nor fail.

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

r[attributes.testing.ignore]
## The `ignore` attribute

r[attributes.testing.ignore.intro]
A function annotated with the `test` attribute can also be annotated with the
`ignore` attribute. The *`ignore` attribute* tells the test harness to not
execute that function as a test. It will still be compiled when in test mode.

r[attributes.testing.ignore.syntax]
The `ignore` attribute may optionally be written with the [MetaNameValueStr]
syntax to specify a reason why the test is ignored.

```rust
#[test]
#[ignore = "not yet implemented"]
fn mytest() {
    // …
}
```

> [!NOTE]
> The `rustc` test harness supports the `--include-ignored` flag to force ignored tests to be run.

r[attributes.testing.should_panic]
## The `should_panic` attribute

r[attributes.testing.should_panic.intro]
A function annotated with the `test` attribute that returns `()` can also be
annotated with the `should_panic` attribute.

r[attributes.testing.should_panic.behavior]
The *`should_panic` attribute*
makes the test only pass if it actually panics.

r[attributes.testing.should_panic.syntax]
The `should_panic` attribute may optionally take an input string that must
appear within the panic message. If the string is not found in the message,
then the test will fail. The string may be passed using the
[MetaNameValueStr] syntax or the [MetaListNameValueStr] syntax with an
`expected` field.

```rust
#[test]
#[should_panic(expected = "values don't match")]
fn mytest() {
    assert_eq!(1, 2, "values don't match");
}
```

[`Termination`]: std::process::Termination
[`report`]: std::process::Termination::report
[`test` conditional compilation option]: ../conditional-compilation.md#test
[attributes]: ../attributes.md
[`ExitCode`]: std::process::ExitCode
[free functions]: ../glossary.md#free-item
