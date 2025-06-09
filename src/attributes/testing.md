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
The `test` attribute may only be applied to [free functions] that are monomorphic, that take no arguments, and where the return type implements the [`Termination`] trait.

> [!NOTE]
> Some of types that implement the [`Termination`] trait include:
> * `()`
> * `Result<T, E> where T: Termination, E: Debug`

r[attributes.testing.test.duplicates]
Only the first instance of `test` on a function is honored.

> [!NOTE]
> Subsequent `test` attributes are currently ignored and `rustc` warns about these.

<!-- TODO: This is a minor lie. Currently rustc warns that duplicates are ignored, but it then generates multiple test entries with the same name. I would vote for rejecting this in the future. -->

r[attributes.testing.test.stdlib]
The `test` attribute is exported from the standard library prelude as [`std::prelude::v1::test`].

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

> [!EXAMPLE]
> ```rust
> # use std::io;
> # fn setup_the_thing() -> io::Result<i32> { Ok(1) }
> # fn do_the_thing(s: &i32) -> io::Result<()> { Ok(()) }
> #[test]
> fn test_the_thing() -> io::Result<()> {
>     let state = setup_the_thing()?; // expected to succeed
>     do_the_thing(&state)?;          // expected to succeed
>     Ok(())
> }
> ```

r[attributes.testing.ignore]
## The `ignore` attribute

r[attributes.testing.ignore.intro]
The *`ignore` [attribute][attributes]* can be used with the [`test` attribute][attributes.testing.test] to tell the test harness to not execute that function as a test.

> [!EXAMPLE]
> ```rust
> #[test]
> #[ignore]
> fn check_thing() {
>     // …
> }
> ```

> [!NOTE]
> The `rustc` test harness supports the `--include-ignored` flag to force ignored tests to be run.

r[attributes.testing.ignore.syntax]
The `ignore` attribute uses either the [MetaWord] or [MetaNameValueStr] syntax.

r[attributes.testing.ignore.reason]
The [MetaNameValueStr] form of the `ignore` attribute provides a way to specify a reason why the test is ignored.

> [!EXAMPLE]
> ```rust
> #[test]
> #[ignore = "not yet implemented"]
> fn mytest() {
>     // …
> }
> ```

r[attributes.testing.ignore.allowed-positions]
The `ignore` attribute may be applied to functions annotated with the `test` attribute.

> [!NOTE]
> `rustc` currently warns when `ignore` is used in some other situations. This may become an error in the future.

r[attributes.testing.ignore.duplicates]
Only the first instance of `ignore` on a function is honored.

> [!NOTE]
> `rustc` currently ignores duplicate `ignore` attributes. This may become an error in the future.

r[attributes.testing.ignore.behavior]
Ignored tests are still compiled when in test mode, but they are not executed.

r[attributes.testing.should_panic]
## The `should_panic` attribute

r[attributes.testing.should_panic.intro]
The *`should_panic` [attribute][attributes]* changes a [test function][attributes.testing.test] so that it passes only if it panics.

> [!EXAMPLE]
> ```rust
> #[test]
> #[should_panic(expected = "values don't match")]
> fn mytest() {
>     assert_eq!(1, 2, "values don't match");
> }
> ```

r[attributes.testing.should_panic.syntax]
The `should_panic` attribute is specified with one of the following forms:

- [MetaWord]
  > [!EXAMPLE]
  > ```rust
  > #[test]
  > #[should_panic]
  > fn mytest() { panic!("some message"); }
  > ```

- [MetaNameValueStr] --- This indicates that the given string should appear within the panic message.
  > [!EXAMPLE]
  > ```rust
  > #[test]
  > #[should_panic = "some message"]
  > fn mytest() { panic!("some message"); }
  > ```

- [MetaListNameValueStr] --- Specified with the key `expected`. Same behavior as [MetaNameValueStr], just with an explicit key.
  > [!EXAMPLE]
  > ```rust
  > #[test]
  > #[should_panic(expected = "some message")]
  > fn mytest() { panic!("some message"); }
  > ```

r[attributes.testing.should_panic.allowed-positions]
The `should_panic` attribute may be applied to functions annotated with the `test` attribute.

> [!NOTE]
> `rustc` currently warns in some other positions. This may become a hard error in the future.

r[attributes.testing.should_panic.duplicates]
Only the first instance of `should_panic` on a function is honored. Subsequent `should_panic` attributes are ignored.

> [!NOTE]
> `rustc` currently ignores subsequent duplicate `should_panic` attributes. This may become an error in the future.

r[attributes.testing.should_panic.expected]
The string specified with the [MetaNameValueStr] form or the `expected` key in [MetaListNameValueStr] indicates that the string must appear somewhere within the panic message. If the string is not found in the message, then the test will fail.

r[attributes.testing.should_panic.return]
The return type of the test function must be `()`.

[`Termination`]: std::process::Termination
[`report`]: std::process::Termination::report
[`test` conditional compilation option]: ../conditional-compilation.md#test
[attributes]: ../attributes.md
[`ExitCode`]: std::process::ExitCode
[free functions]: ../glossary.md#free-item
