r[attributes.derive]
# Derive

r[attributes.derive.intro]
The *`derive` [attribute][attributes]* invokes one or more [derive macros], allowing new [items] to be automatically generated for data structures. You can create `derive` macros with [procedural macros].

> [!EXAMPLE]
> The [`PartialEq`][macro@PartialEq] derive macro emits an [implementation] of [`PartialEq`] for `Foo<T> where T: PartialEq`. The [`Clone`][macro@Clone] derive macro does likewise for [`Clone`].
>
> ```rust
> #[derive(PartialEq, Clone)]
> struct Foo<T> {
>     a: i32,
>     b: T,
> }
> ```
>
> The generated `impl` items are equivalent to:
>
> ```rust
> # struct Foo<T> { a: i32, b: T }
> impl<T: PartialEq> PartialEq for Foo<T> {
>     fn eq(&self, other: &Foo<T>) -> bool {
>         self.a == other.a && self.b == other.b
>     }
> }
>
> impl<T: Clone> Clone for Foo<T> {
>     fn clone(&self) -> Self {
>         Foo { a: self.a.clone(), b: self.b.clone() }
>     }
> }
> ```

r[attributes.derive.syntax]
The `derive` attribute uses the [MetaListPaths] syntax to specify a list of paths to [derive macros] to invoke.

r[attributes.derive.allowed-positions]
The `derive` attribute may be applied to [structs][items.struct], [enums][items.enum], and [unions][items.union].

r[attributes.derive.duplicates]
The `derive` attribute may be specified multiple times on an item, with all derive macros listed in all attributes being invoked.

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

r[attributes.derive.built-in-automatically_derived]
The built-in derives include the [`automatically_derived` attribute][attributes.derive.automatically_derived] on the implementations they generate.

r[attributes.derive.behavior]
During macro expansion, for each element in the list of derives, the corresponding derive macro expands to zero or more [items].

r[attributes.derive.automatically_derived]
## The `automatically_derived` attribute

The *`automatically_derived` attribute* is automatically added to [implementations] created by the `derive` attribute for built-in traits. It has no direct effect, but it may be used by tools and diagnostic lints to detect these automatically generated implementations.

[items]: ../items.md
[derive macros]: ../procedural-macros.md#derive-macros
[implementations]: ../items/implementations.md
[items]: ../items.md
[procedural macros]: ../procedural-macros.md#derive-macros
