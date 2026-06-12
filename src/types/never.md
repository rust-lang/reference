r[type.never]
# Never type

r[type.never.intro]
The never type `!` is a type with no values, representing the result of computations that never complete.

> [!EXAMPLE]
> ```rust
> fn foo() -> ! {
>     loop {}
> }
> ```
>
> ```rust
> unsafe extern "C" {
>     pub safe fn no_return_extern_func() -> !;
> }
> ```
>
> ```rust,no_run
> let _: ! = loop {};
> ```
>
> ```rust
> fn always_ok() -> Result<u32, !> {
>     Ok(42)
> }
> ```
>
> ```rust
> # use std::str::FromStr;
> struct Anything(String);
>
> impl FromStr for Anything {
>     type Err = !;
>
>     fn from_str(s: &str) -> Result<Self, !> {
>         Ok(Anything(s.to_owned()))
>     }
> }
>
> let Ok(s) = Anything::from_str("example");
> ```

r[type.never.syntax]
```grammar,types
NeverType -> `!`
```

r[type.never.coercion]
Expressions of type `!` can be coerced into any other type.

> [!NOTE]
> The standard library type [`Infallible`] is a type alias for `!`.

[`Infallible`]: core::convert::Infallible
