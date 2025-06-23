r[attributes.limits]
# Limits

The following [attributes] affect compile-time limits.

r[attributes.limits.recursion_limit]
## The `recursion_limit` attribute

r[attributes.limits.recursion_limit.intro]
The *`recursion_limit` [attribute][attributes]* sets the maximum depth for potentially infinitely-recursive compile-time operations like macro expansion or auto-dereference.

> [!EXAMPLE]
> ```rust,compile_fail
> #![recursion_limit = "4"]
>
> macro_rules! a {
>     () => { a!(1); };
>     (1) => { a!(2); };
>     (2) => { a!(3); };
>     (3) => { a!(4); };
>     (4) => { };
> }
>
> // This fails to expand because it requires a recursion depth greater than 4.
> a!{}
> ```

> [!EXAMPLE]
> ```rust,compile_fail
> #![recursion_limit = "1"]
>
> // This fails because it requires two recursive steps to auto-dereference.
> (|_: &u8| {})(&&&1);
> ```

> [!NOTE]
> The default recursion limit in `rustc` is 128.

r[attributes.limits.recursion_limit.syntax]
The `recursion_limit` attribute uses the [MetaNameValueStr] syntax to specify the recursion depth. The value in the string must be a non-negative integer.

r[attributes.limits.recursion_limit.allowed-positions]
The `recursion_limit` attribute may only be applied to the crate root.

> [!NOTE]
> `rustc` currently warns in other positions, but this may be rejected in the future.

r[attributes.limits.recursion_limit.duplicates]
Only the first instance of `recursion_limit` on an item is honored. Subsequent `recursion_limit` attributes are ignored.

> [!NOTE]
> `rustc` currently warns on following duplicate `recursion_limit` attributes. This may become an error in the future.

r[attributes.limits.type_length_limit]
## The `type_length_limit` attribute

> [!NOTE]
> This limit is only enforced when the nightly `-Zenforce-type-length-limit` flag is active.
>
> For more information, see <https://github.com/rust-lang/rust/pull/127670>.

r[attributes.limits.type_length_limit.intro]
The *`type_length_limit` attribute* limits the maximum number of type
substitutions made when constructing a concrete type during monomorphization.

r[attributes.limits.type_length_limit.syntax]
It is applied at the [crate] level, and uses the [MetaNameValueStr] syntax
to set the limit based on the number of type substitutions.

> [!NOTE]
> The default in `rustc` is 1048576.

```rust,ignore
#![type_length_limit = "4"]

fn f<T>(x: T) {}

// This fails to compile because monomorphizing to
// `f::<((((i32,), i32), i32), i32)>` requires more than 4 type elements.
f(((((1,), 2), 3), 4));
```

[attributes]: ../attributes.md
[crate]: ../crates-and-source-files.md
