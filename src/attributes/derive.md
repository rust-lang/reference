<!-- template:attributes -->
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
The `derive` attribute may only be applied to [structs][items.struct], [enums][items.enum], and [unions][items.union].

r[attributes.derive.duplicates]
The `derive` attribute may be used any number of times on an item. All derive macros listed in all attributes are invoked.

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

<!-- template:attributes -->
r[attributes.derive.automatically_derived]
## The `automatically_derived` attribute

r[attributes.derive.automatically_derived.intro]
The *`automatically_derived` [attribute][attributes]* is used to annotate an [implementation] to indicate that it was automatically created by a [derive macro]. It has no direct effect, but it may be used by tools and diagnostic lints to detect these automatically generated implementations.

> [!EXAMPLE]
> Given [`#[derive(Clone)]`][macro@Clone] on `struct Example`, the [derive macro] may produce:
>
> ```rust
> # struct Example;
> #[automatically_derived]
> impl ::core::clone::Clone for Example {
>     #[inline]
>     fn clone(&self) -> Self {
>         Example
>     }
> }
> ```

r[attributes.derive.automatically_derived.syntax]
The `automatically_derived` attribute uses the [MetaWord] syntax.

r[attributes.derive.automatically_derived.allowed-positions]
The `automatically_derived` attribute may only be applied to an [implementation].

> [!NOTE]
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

r[attributes.derive.automatically_derived.duplicates]
Using `automatically_derived` more than once on an implementation has the same effect as using it once.

> [!NOTE]
> `rustc` lints against any use following the first.

r[attributes.derive.automatically_derived.behavior]
The `automatically_derived` attribute has no behavior.

[items]: ../items.md
[derive macro]: macro.proc.derive
[derive macros]: macro.proc.derive
[implementation]: ../items/implementations.md
[items]: ../items.md
[procedural macros]: macro.proc.derive
