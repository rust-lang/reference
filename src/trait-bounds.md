r[bound]
# Trait and lifetime bounds

r[bound.syntax]
```grammar,miscellaneous
TypeParamBounds -> TypeParamBound ( `+` TypeParamBound )* `+`?

TypeParamBound -> Lifetime | TraitBound | UseBound

TraitBound ->
      ( `?` | ForLifetimes )? TypePath
    | `(` ( `?` | ForLifetimes )? TypePath `)`

LifetimeBounds -> ( Lifetime `+` )* Lifetime?

Lifetime ->
      LIFETIME_OR_LABEL
    | `'static`
    | `'_`

UseBound -> `use` UseBoundGenericArgs

UseBoundGenericArgs ->
      `<` `>`
    | `<` ( UseBoundGenericArg `,`)* UseBoundGenericArg `,`? `>`

UseBoundGenericArg ->
      Lifetime
    | IDENTIFIER
    | `Self`
```

r[bound.intro]
[Trait] and lifetime bounds provide a way for [generic items][generic] to
restrict which types and lifetimes are used as their parameters. Bounds can be
provided on any type in a [where clause]. There are also shorter forms for
certain common cases:

* Bounds written after declaring a [generic parameter][generic]:
  `fn f<A: Copy>() {}` is the same as `fn f<A>() where A: Copy {}`.
* In trait declarations as [supertraits]: `trait Circle : Shape {}` is
  equivalent to `trait Circle where Self : Shape {}`.
* In trait declarations as bounds on [associated types]:
  `trait A { type B: Copy; }` is equivalent to
  `trait A where Self::B: Copy { type B; }`.

r[bound.global]

Bounds which does not use the item's parameters or any higher-ranked lifetimes are considered global.

Global bounds must be satisfiable without relying on any where clauses.

r[bound.satisfaction]

The bounds of an item must be satisfied when using that item.

r[bound.satisfaction.impl]

A trait bound can be satisfied by using an implementation of that trait. An implementation is applicable if
its generic parameters can be instantiated so that its self type and trait arguments are equal to the trait bound
and the where-bounds of the impl can be recursively satisfied.

r[bound.satisfaction.bounds]

While inside of a generic item, trait bounds can be satisfied by using the where-bounds of the current item as the item is able to assume that its bounds are satisfied. For this, the where-bound is then equated with the trait bound that needs to be satisfied.

r[bound.satisfaction.candidate-preference]

> This is purely descriptive. Candidate preference behavior may change in future releases and must not be relied upon for correctness or soundness.

If there are multiple ways to satisfy a trait bound, some groups of candidate are preferred over others. In case a single group has multiple different candidates, the bound remains ambiguous. Candidate preference has the following order
- builtin implementations of `Sized`
- if there are any non-global where-bounds, all where-bounds
- impls
- global where-bounds

```rust
struct Container<T> {
    inner: Vec<T>,
}
impl<T: Clone> for Container<T> {
    fn clone(&self) -> Self {
        Container { inner: self.inner.clone() }
    }
}

fn is_clone<T: Clone>() {}
fn generic_fn<T: Clone>() {
    // `Container<T>: Clone` is satisfied via the impl above. This requires
    // the nested trait bound `T: Clone` which is satisfied via the where-bound.
    is_clone::<Container<T>>();
}
```

r[bound.trait-object]
Trait and lifetime bounds are also used to name [trait objects].

r[bound.sized]
## `?Sized`

`?` is only used to relax the implicit [`Sized`] trait bound for [type parameters] or [associated types].
`?Sized` may not be used as a bound for other types.

r[bound.lifetime]
## Lifetime bounds

r[bound.lifetime.intro]
Lifetime bounds can be applied to types or to other lifetimes.

r[bound.lifetime.outlive-lifetime]
The bound `'a: 'b` is usually read as `'a` *outlives* `'b`.
`'a: 'b` means that `'a` lasts at least as long as `'b`, so a reference `&'a ()` is valid whenever `&'b ()` is valid.

```rust
fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) where 'a: 'b {
    y = x;                      // &'a i32 is a subtype of &'b i32 because 'a: 'b
    let r: &'b &'a i32 = &&0;   // &'b &'a i32 is well formed because 'a: 'b
}
```

r[bound.lifetime.outlive-type]
`T: 'a` means that all lifetime parameters of `T` outlive `'a`.
For example, if `'a` is an unconstrained lifetime parameter, then `i32: 'static` and `&'static str: 'a` are satisfied, but `Vec<&'a ()>: 'static` is not.

r[bound.higher-ranked]
## Higher-ranked trait bounds

r[bound.higher-ranked.syntax]
```grammar,miscellaneous
ForLifetimes -> `for` GenericParams
```

r[bound.higher-ranked.intro]
Trait bounds may be *higher ranked* over lifetimes. These bounds specify a bound
that is true *for all* lifetimes. For example, a bound such as `for<'a> &'a T:
PartialEq<i32>` would require an implementation like

```rust
# struct T;
impl<'a> PartialEq<i32> for &'a T {
    // ...
#    fn eq(&self, other: &i32) -> bool {true}
}
```

and could then be used to compare a `&'a T` with any lifetime to an `i32`.

Only a higher-ranked bound can be used here, because the lifetime of the reference is shorter than any possible lifetime parameter on the function:

```rust
fn call_on_ref_zero<F>(f: F) where for<'a> F: Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}
```

r[bound.higher-ranked.trait]
Higher-ranked lifetimes may also be specified just before the trait: the only
difference is the [scope][hrtb-scopes] of the lifetime parameter, which extends only to the
end of the following trait instead of the whole bound. This function is
equivalent to the last one.

```rust
fn call_on_ref_zero<F>(f: F) where F: for<'a> Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}
```

r[bound.implied]
## Implied bounds

r[bound.implied.intro]
Lifetime bounds required for types to be well-formed are sometimes inferred.

```rust
fn requires_t_outlives_a<'a, T>(x: &'a T) {}
```

The type parameter `T` is required to outlive `'a` for the type `&'a T` to be well-formed.
This is inferred because the function signature contains the type `&'a T` which is
only valid if `T: 'a` holds.

r[bound.implied.context]
Implied bounds are added for all parameters and outputs of functions. Inside of `requires_t_outlives_a`
you can assume `T: 'a` to hold even if you don't explicitly specify this:

```rust
fn requires_t_outlives_a_not_implied<'a, T: 'a>() {}

fn requires_t_outlives_a<'a, T>(x: &'a T) {
    // This compiles, because `T: 'a` is implied by
    // the reference type `&'a T`.
    requires_t_outlives_a_not_implied::<'a, T>();
}
```

```rust,compile_fail,E0309
# fn requires_t_outlives_a_not_implied<'a, T: 'a>() {}
fn not_implied<'a, T>() {
    // This errors, because `T: 'a` is not implied by
    // the function signature.
    requires_t_outlives_a_not_implied::<'a, T>();
}
```

r[bound.implied.trait]
Only lifetime bounds are implied, trait bounds still have to be explicitly added.
The following example therefore causes an error:

```rust,compile_fail,E0277
use std::fmt::Debug;
struct IsDebug<T: Debug>(T);
// error[E0277]: `T` doesn't implement `Debug`
fn doesnt_specify_t_debug<T>(x: IsDebug<T>) {}
```

r[bound.implied.def]
Lifetime bounds are also inferred for type definitions and impl blocks for any type:

```rust
struct Struct<'a, T> {
    // This requires `T: 'a` to be well-formed
    // which is inferred by the compiler.
    field: &'a T,
}

enum Enum<'a, T> {
    // This requires `T: 'a` to be well-formed,
    // which is inferred by the compiler.
    //
    // Note that `T: 'a` is required even when only
    // using `Enum::OtherVariant`.
    SomeVariant(&'a T),
    OtherVariant,
}

trait Trait<'a, T: 'a> {}

// This would error because `T: 'a` is not implied by any type
// in the impl header.
//     impl<'a, T> Trait<'a, T> for () {}

// This compiles as `T: 'a` is implied by the self type `&'a T`.
impl<'a, T> Trait<'a, T> for &'a T {}
```

r[bound.use]
## Use bounds

Certain bounds lists may include a `use<..>` bound to control which generic parameters are captured by the `impl Trait` [abstract return type].  See [precise capturing] for more details.

[abstract return type]: types/impl-trait.md#abstract-return-types
[arrays]: types/array.md
[associated types]: items/associated-items.md#associated-types
[hrtb-scopes]: names/scopes.md#higher-ranked-trait-bound-scopes
[supertraits]: items/traits.md#supertraits
[generic]: items/generics.md
[higher-ranked lifetimes]: #higher-ranked-trait-bounds
[precise capturing]: types/impl-trait.md#precise-capturing
[slice]: types/slice.md
[Trait]: items/traits.md#trait-bounds
[trait object]: types/trait-object.md
[trait objects]: types/trait-object.md
[type parameters]: types/parameters.md
[where clause]: items/generics.md#where-clauses
