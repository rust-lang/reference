r[type.never]
# Never type

r[type.never.intro]
The never type `!` is a type with no values, representing computations that never complete, also known as [diverging][divergence] computations.

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
> // This does not need to check for the `Err` variant because
> // `FromStr::Err` is the never type.
> let Ok(s) = Anything::from_str("example");
> ```

r[type.never.syntax]
```grammar,types
NeverType -> `!`
```

r[type.never.coercion]
Expressions of type `!` can be coerced into any type.

> [!NOTE]
> The standard library type [`Infallible`] is a type alias for `!`.

r[type.never.layout]
The `!` type has a size of 0 and an alignment of 1.

[`Infallible`]: core::convert::Infallible
