r[attributes.derive]
# Derive

r[attributes.derive.intro]
The *`derive` [attribute][attributes]* allows new [items] to be automatically generated for data structures. You can implement custom `derive` macros through [procedural macros].

> [!EXAMPLE]
> The following example will create an [`impl` item] for the [`PartialEq`] and [`Clone`] traits for `Foo`, and the type parameter `T` will be given the `PartialEq` or `Clone` constraints for the appropriate `impl`:
>
> ```rust
> #[derive(PartialEq, Clone)]
> struct Foo<T> {
>     a: i32,
>     b: T,
> }
> ```
>
> The generated `impl` for `PartialEq` is equivalent to
>
> ```rust
> # struct Foo<T> { a: i32, b: T }
> impl<T: PartialEq> PartialEq for Foo<T> {
>     fn eq(&self, other: &Foo<T>) -> bool {
>         self.a == other.a && self.b == other.b
>     }
> }
> ```

r[attributes.derive.syntax]
The `derive` attribute uses the [MetaListPaths] syntax to specify a list of paths to [derive macros] to process.

r[attributes.derive.allowed-positions]
The `derive` attribute may be applied to [structs][items.struct], [enums][items.enum], and [unions][items.union].

r[attributes.derive.duplicates]
The `derive` attribute may be specified multiple times on an item, with all entries from all attributes being processed.

r[attributes.derive.stdlib]
The `derive` attribute is exported in the standard library prelude as [`core::prelude::v1::derive`].

r[attributes.derive.built-in]
Built-in derives are defined in the [language prelude][names.preludes.lang]. The list of built-in derives are:

- [`Clone`]
- [`Copy`]
- [`Debug`]
- [`Default`]
- [`Eq`]
- [`Hash`]
- [`Ord`]
- [`PartialEq`]
- [`PartialOrd`]

r[attributes.derive.behavior]
During macro expansion, for each element in the list of derives, the corresponding derive macro expands to zero or more [items].

r[attributes.derive.automatically_derived]
## The `automatically_derived` attribute

The *`automatically_derived` attribute* is automatically added to
[implementations] created by the `derive` attribute for built-in traits. It
has no direct effect, but it may be used by tools and diagnostic lints to
detect these automatically generated implementations.

[`impl` item]: ../items/implementations.md
[items]: ../items.md
[derive macros]: ../procedural-macros.md#derive-macros
[implementations]: ../items/implementations.md
[items]: ../items.md
[procedural macros]: ../procedural-macros.md#derive-macros
