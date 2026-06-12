r[type.never]
# Never type

r[type.never.intro]
The never type `!` is a type with no values, representing the result of computations that never complete.

> [!EXAMPLE]
> ```rust
> fn foo() -> ! {
>     panic!("This call never returns.");
> }
> ```
>
> ```rust
> unsafe extern "C" {
>     pub safe fn no_return_extern_func() -> !;
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
