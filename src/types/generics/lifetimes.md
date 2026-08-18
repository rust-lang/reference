r[generics.lifetimes]
# Generic lifetimes

r[generics.lifetimes.intro]
A *lifetime parameter* is a generic parameter for a lifetime.

r[generics.lifetimes.at-least-once]
[Structs], [enumerations], and [unions] must use each of their lifetime parameters at least once in their fields or variants.

> [!EXAMPLE]
> ```rust,compile_fail
> // ERROR: lifetime parameter `'a` is never used
> struct Foo<'a>;
>
> // ERROR: lifetime parameter `'a` is never used
> enum Bar<'a> { A }
> ```
>
> ```rust
> // OK: `'a` appears in a field.
> struct Ref<'a, T> {
>     r: &'a T,
> }
> ```

[enumerations]: items.enum
[structs]: items.struct
[unions]: items.union
