r[attributes.derive]
# Derive

r[attributes.derive.intro]
The *`derive` attribute* allows new [items] to be automatically generated for
data structures.

r[attributes.derive.syntax]
It uses the [MetaListPaths] syntax to specify a list of
traits to implement or paths to [derive macros] to process.

For example, the following will create an [`impl` item] for the
[`PartialEq`] and [`Clone`] traits for `Foo`, and the type parameter `T` will be
given the `PartialEq` or `Clone` constraints for the appropriate `impl`:

```rust
#[derive(PartialEq, Clone)]
struct Foo<T> {
    a: i32,
    b: T,
}
```

The generated `impl` for `PartialEq` is equivalent to

```rust
# struct Foo<T> { a: i32, b: T }
impl<T: PartialEq> PartialEq for Foo<T> {
    fn eq(&self, other: &Foo<T>) -> bool {
        self.a == other.a && self.b == other.b
    }
}
```

r[attributes.derive.proc-macro]
You can implement `derive` for your own traits through [procedural macros].

r[attributes.derive.automatically_derived]
## The `automatically_derived` attribute

r[attributes.derive.automatically_derived.intro]
The *`automatically_derived` [attribute][attributes]* is used to annotate an [implementation] to indicate that it was automatically created by a `derive` attribute. It has no direct effect, but it may be used by tools and diagnostic lints to detect these automatically generated implementations.

> [!EXAMPLE]
> The following is an example of what the [`Clone`] derive may generate for a struct named `Example`.
> ```rust
> # struct Example;
> #[automatically_derived]
> impl ::core::clone::Clone for Example {
>     #[inline]
>     fn clone(&self) -> Example {
>         Example
>     }
> }
> ```

r[attributes.derive.automatically_derived.syntax]
The `automatically_derived` attribute uses the [MetaWord] syntax and thus does not take any inputs.

r[attributes.derive.automatically_derived.allowed-positions]
The `automatically_derived` attribute may be placed on an [implementation].

> [!NOTE]
> `rustc` currently warns in other positions.

r[attributes.derive.automatically_derived.duplicates]
Duplicate instances of the `automatically_derived` attribute on the same implementation have no effect.

> [!NOTE]
> `rustc` currently warns on subsequent duplicate `automatically_derived` attributes.

r[attributes.derive.automatically_derived.behavior]
The `automatically_derived` attribute has no behavior.

[`impl` item]: ../items/implementations.md
[items]: ../items.md
[derive macros]: ../procedural-macros.md#derive-macros
[implementation]: ../items/implementations.md
[items]: ../items.md
[procedural macros]: ../procedural-macros.md#derive-macros
