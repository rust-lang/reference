# Trait and lifetime bounds

r[bound]

r[bound.syntax]
> **<sup>Syntax</sup>**\
> _TypeParamBounds_ :\
> &nbsp;&nbsp; _TypeParamBound_ ( `+` _TypeParamBound_ )<sup>\*</sup> `+`<sup>?</sup>
>
> _TypeParamBound_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; _Lifetime_ | _TraitBound_ | _UseBound_
>
> _TraitBound_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; ( `?` |
> [_ForLifetimes_](#higher-ranked-trait-bounds) )<sup>?</sup> [_TypePath_]\
> &nbsp;&nbsp; | `(` ( `?` |
> [_ForLifetimes_](#higher-ranked-trait-bounds) )<sup>?</sup> [_TypePath_] `)`
>
> _LifetimeBounds_ :\
> &nbsp;&nbsp; ( _Lifetime_ `+` )<sup>\*</sup> _Lifetime_<sup>?</sup>
>
> _Lifetime_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; [LIFETIME_OR_LABEL]\
> &nbsp;&nbsp; | `'static`
>
> _UseBound_ :\
> &nbsp;&nbsp; `use` _UseBoundGenericArgs_
>
> _UseBoundGenericArgs_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; `<` `>` \
> &nbsp;&nbsp; | `<` \
> &nbsp;&nbsp; &nbsp;&nbsp; ( _UseBoundGenericArg_ `,`)<sup>\*</sup> \
> &nbsp;&nbsp; &nbsp;&nbsp; _UseBoundGenericArg_ `,`<sup>?</sup> \
> &nbsp;&nbsp; &nbsp;&nbsp; `>`
>
> _UseBoundGenericArg_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; _Lifetime_ \
> &nbsp;&nbsp; | [IDENTIFIER][] \
> &nbsp;&nbsp; | `Self`

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

r[bound.satisfaction]
Bounds on an item must be satisfied when using the item. When type checking and
borrow checking a generic item, the bounds can be used to determine that a
trait is implemented for a type. For example, given `Ty: Trait`

* In the body of a generic function, methods from `Trait` can be called on `Ty`
  values. Likewise associated constants on the `Trait` can be used.
* Associated types from `Trait` can be used.
* Generic functions and types with a `T: Trait` bounds can be used with `Ty`
  being used for `T`.

```rust
# type Surface = i32;
trait Shape {
    fn draw(&self, surface: Surface);
    fn name() -> &'static str;
}

fn draw_twice<T: Shape>(surface: Surface, sh: T) {
    sh.draw(surface);           // Can call method because T: Shape
    sh.draw(surface);
}

fn copy_and_draw_twice<T: Copy>(surface: Surface, sh: T) where T: Shape {
    let shape_copy = sh;        // doesn't move sh because T: Copy
    draw_twice(surface, sh);    // Can use generic function because T: Shape
}

struct Figure<S: Shape>(S, S);

fn name_figure<U: Shape>(
    figure: Figure<U>,          // Type Figure<U> is well-formed because U: Shape
) {
    println!(
        "Figure of two {}",
        U::name(),              // Can use associated function
    );
}
```

r[bound.trivial]
Bounds that don't use the item's parameters or [higher-ranked lifetimes] are checked when the item is defined.
It is an error for such a bound to be false.

r[bound.special]
[`Copy`], [`Clone`], and [`Sized`] bounds are also checked for certain generic types when using the item, even if the use does not provide a concrete type.
It is an error to have `Copy` or `Clone` as a bound on a mutable reference, [trait object], or [slice].
It is an error to have `Sized` as a bound on a trait object or slice.

```rust,compile_fail
struct A<'a, T>
where
    i32: Default,           // Allowed, but not useful
    i32: Iterator,          // Error: `i32` is not an iterator
    &'a mut T: Copy,        // (at use) Error: the trait bound is not satisfied
    [T]: Sized,             // (at use) Error: size cannot be known at compilation
{
    f: &'a T,
}
struct UsesA<'a, T>(A<'a, T>);
```

r[bound.trait-object]
Trait and lifetime bounds are also used to name [trait objects].

## `?Sized`

r[bound.sized]

`?` is only used to relax the implicit [`Sized`] trait bound for [type parameters] or [associated types].
`?Sized` may not be used as a bound for other types.

## Lifetime bounds

r[bound.lifetime]

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

In the following text, the words "inferred" and "implied" refer to similar lifetime bounds that may be omitted.
These different words appear in different contexts in existing documentation, due to implementation details.
From the programmer's perspective, they both refer to a category of lifetime bounds that are not required to be explicitly written because the compiler can derive them from other information.

For functions and trait implementations, some lifetime bounds are implied because inputs to functions and trait implementations are assumed to be well-formed.
For the purposes of determining implied bounds on functions, both the function parameter types and the function return type are considered to be inputs.

For example, for a function parameter `x: &'a T` to be well-formed, `T: 'a` must be satisfied (referents must outlive any references to them), so it is not necessary to explicitly write that lifetime bound on the function definition.
On the other hand, removing the bound `where 'a: 'b` from the above example results in an error, because the constructed reference type exists only in the function body and is not among the inputs to `f`:

```rust,compile_fail
fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) {
    // This is NOT well formed, because the bound `'a: 'b` is missing;
    // therefore, `&'b &'a i32` could possibly outlive `&'a i32`.
    let r: &'b &'a i32 = &&0;   // error
}
```

For the purpose of determining implied bounds on trait implementations, the implementing type (the `T` in `impl Trait for T`) is considered to be an input, as are generic parameters on the trait definition.
Parameters of the trait implementation, which come directly after the `impl` keyword, are not otherwise considered to be inputs.
For example, for the `Vec<T>` implementation below, `'a` and `Vec<T>` are inputs to the trait, but `T` and `&'a T` are not:

```rust
trait MakeRef<'a> { type Type; }

impl<'a, T> MakeRef<'a> for Vec<T>
    where T: 'a     // Required because `&'a T` is not an input to the trait
{
    type Type = &'a T;
}

// `T: 'a` is implied: `&'a T` (implementing type) is an input to the trait
impl<'a, T> MakeRef<'a> for &'a T {
    type Type = &'a T;
}
```

For struct definitions, certain lifetime bounds are inferred due to requirements for the types of struct fields to be well-formed.
These inferred lifetime bounds do not have to be explicitly written on the struct definition.

For example, a struct containing a field with a reference such as `&'a T` must satisfy `T: 'a` to be well-formed:

```rust
struct RefToSlice1<'a, T>(&'a [T]);             // inferred
struct RefToSlice2<'a, T>(&'a [T]) where T: 'a; // explicit
// References to references have additional requirements that can be inferred.
struct DoubleRef1<'a, 'b, T>(&'a &'b T);                        // inferred
struct DoubleRef2<'a, 'b, T>(&'a &'b T) where 'b: 'a, T: 'b;    // explicit
```

A reference to a struct such as `Bar<'a, T>` can also result in inferred lifetime bounds derived from the definition of `Bar`.
The explicit `T: 'a` bound in the `where` clause on `Bar` causes the bound `U: 'b` to be inferred on `Foo1`:

```rust
struct Foo1<'b, U>(Bar<'b, U>);             // inferred
struct Foo2<'b, U>(Bar<'b, U>) where U: 'b; // explicit

struct Bar<'a, T>(&'a (), T) where T: 'a;
```

For associated type references such as `<T as MakeRef<'a>>::Type`, only `T` itself is checked to be well-formed, and no lifetime bounds are inferred based on the lifetime requirements of the associated type `MakeRef<'a>::Type`.
An explicit bound is still required to ensure that any lifetime requirements of the associated type are met:

```rust
trait MakeRef<'a> { type Type; }

impl<'a, T> MakeRef<'a> for Vec<T>
    where T: 'a
{
    type Type = &'a T;
}

struct UsesMakeRef<'a, T>
    where T: 'a   // Not inferred: only `Vec<T>` is checked to be well-formed
{
    foo: <Vec<T> as MakeRef<'a>>::Type
}
```

In contrast, `<T as Iterator>::Item: 'a` is inferred below, because `'a` is part of the type of the reference, not part of the associated type `<T as Iterator>::Item`:

```rust
struct RefAssocType1<'a, T: Iterator>(&'a T::Item);     // inferred
struct RefAssocType2<'a, T: Iterator>(&'a T::Item)
    where <T as Iterator>::Item: 'a;                    // explicit
```

## Higher-ranked trait bounds

r[bound.higher-ranked]

r[bound.higher-ranked.syntax]
> _ForLifetimes_ :\
> &nbsp;&nbsp; `for` [_GenericParams_]

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

## Implied bounds

r[bound.implied]

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

## Use bounds

r[bound.use]

Certain bounds lists may include a `use<..>` bound to control which generic parameters are captured by the `impl Trait` [abstract return type].  See [precise capturing] for more details.

[IDENTIFIER]: identifiers.html
[LIFETIME_OR_LABEL]: tokens.md#lifetimes-and-loop-labels
[_GenericParams_]: items/generics.md
[_TypePath_]: paths.md#paths-in-types
[`Clone`]: special-types-and-traits.md#clone
[`Copy`]: special-types-and-traits.md#copy
[`Sized`]: special-types-and-traits.md#sized

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
