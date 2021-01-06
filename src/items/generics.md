# Generic parameters

> **<sup>Syntax</sup>**\
> _Generics_ :\
> &nbsp;&nbsp; `<` _GenericParams_ `>`
>
> _GenericParams_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; _LifetimeParams_\
> &nbsp;&nbsp; | ( _LifetimeParam_ `,` )<sup>\*</sup> _TypeParams_\
> &nbsp;&nbsp; | ( _LifetimeParam_ `,` )<sup>\*</sup> ( _TypeParam_ `,` )<sup>\*</sup> _ConstParams_
>
> _LifetimeParams_ :\
> &nbsp;&nbsp; ( _LifetimeParam_ `,` )<sup>\*</sup> _LifetimeParam_<sup>?</sup>
>
> _LifetimeParam_ :\
> &nbsp;&nbsp; [_OuterAttribute_]<sup>?</sup> [LIFETIME_OR_LABEL]&nbsp;( `:` [_LifetimeBounds_] )<sup>?</sup>
>
> _TypeParams_:\
> &nbsp;&nbsp; ( _TypeParam_ `,` )<sup>\*</sup> _TypeParam_<sup>?</sup>
>
> _TypeParam_ :\
> &nbsp;&nbsp; [_OuterAttribute_]<sup>?</sup> [IDENTIFIER]( `:` [_TypeParamBounds_]<sup>?</sup> )<sup>?</sup> ( `=` [_Type_] )<sup>?</sup>
>
> _ConstParams_:\
> &nbsp;&nbsp; ( _ConstParam_ `,` )<sup>\*</sup> _ConstParam_<sup>?</sup>
>
> _ConstParam_:\
> &nbsp;&nbsp; [_OuterAttribute_]<sup>?</sup> `const` [IDENTIFIER] `:` [_Type_]

Functions, type aliases, structs, enumerations, unions, traits, and
implementations may be *parameterized* by types, constants, and lifetimes. These
parameters are listed in angle <span class="parenthetical">brackets (`<...>`)</span>,
usually immediately after the name of the item and before its definition. For
implementations, which don't have a name, they come directly after `impl`.
The order of generic parameters is restricted to lifetime parameters, then type parameters, and then const parameters.

Some examples of items with type, const, and lifetime parameters:

```rust
fn foo<'a, T>() {}
trait A<U> {}
struct Ref<'a, T> where T: 'a { r: &'a T }
struct InnerArray<T, const N: usize>([T; N]);
```

The only allowed types of const parameters are `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
`i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `char` and `bool`.

Const parameters may only be be used as standalone arguments inside
of [types] and [repeat expressions] but may be freely used elsewhere:

```rust,compile_fail
// ok: standalone argument
fn foo<const N: usize>() -> [u8; N] { todo!() }

// ERROR: generic const operation
fn bar<const N: usize>() -> [u8; N + 1] { todo!() }
```

Unlike type and lifetime parameters, const parameters of types can be used without
being mentioned inside of a parameterized type:

```rust,compile_fail
// ok
struct Foo<const N: usize>;
enum Bar<const M: usize> { A, B }

// ERROR: unused parameter
struct Baz<T>;
struct Biz<'a>;
```

[References], [raw pointers], [arrays], [slices][arrays], [tuples], and
[function pointers] have lifetime or type parameters as well, but are not
referred to with path syntax.

## Where clauses

> **<sup>Syntax</sup>**\
> _WhereClause_ :\
> &nbsp;&nbsp; `where` ( _WhereClauseItem_ `,` )<sup>\*</sup> _WhereClauseItem_ <sup>?</sup>
>
> _WhereClauseItem_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; _LifetimeWhereClauseItem_\
> &nbsp;&nbsp; | _TypeBoundWhereClauseItem_
>
> _LifetimeWhereClauseItem_ :\
> &nbsp;&nbsp; [_Lifetime_] `:` [_LifetimeBounds_]
>
> _TypeBoundWhereClauseItem_ :\
> &nbsp;&nbsp; _ForLifetimes_<sup>?</sup> [_Type_] `:` [_TypeParamBounds_]<sup>?</sup>
>
> _ForLifetimes_ :\
> &nbsp;&nbsp; `for` `<` [_LifetimeParams_](#generic-parameters) `>`

*Where clauses* provide another way to specify bounds on type and lifetime
parameters as well as a way to specify bounds on types that aren't type
parameters.

Bounds that don't use the item's parameters or higher-ranked lifetimes are
checked when the item is defined. It is an error for such a bound to be false.

[`Copy`], [`Clone`], and [`Sized`] bounds are also checked for certain generic
types when defining the item. It is an error to have `Copy` or `Clone` as a
bound on a mutable reference, [trait object] or [slice][arrays] or `Sized` as a
bound on a trait object or slice.

```rust,compile_fail
struct A<T>
where
    T: Iterator,            // Could use A<T: Iterator> instead
    T::Item: Copy,
    String: PartialEq<T>,
    i32: Default,           // Allowed, but not useful
    i32: Iterator,          // Error: the trait bound is not satisfied
    [T]: Copy,              // Error: the trait bound is not satisfied
{
    f: T,
}
```

## Attributes

Generic lifetime and type parameters allow [attributes] on them. There are no
built-in attributes that do anything in this position, although custom derive
attributes may give meaning to it.

This example shows using a custom derive attribute to modify the meaning of a
generic parameter.

<!-- ignore: requires proc macro derive -->
```rust,ignore
// Assume that the derive for MyFlexibleClone declared `my_flexible_clone` as
// an attribute it understands.
#[derive(MyFlexibleClone)]
struct Foo<#[my_flexible_clone(unbounded)] H> {
    a: *const H
}
```

[IDENTIFIER]: ../identifiers.md
[LIFETIME_OR_LABEL]: ../tokens.md#lifetimes-and-loop-labels

[_LifetimeBounds_]: ../trait-bounds.md
[_Lifetime_]: ../trait-bounds.md
[_OuterAttribute_]: ../attributes.md
[_Type_]: ../types.md#type-expressions
[_TypeParamBounds_]: ../trait-bounds.md

[arrays]: ../types/array.md
[const contexts]: ../const_eval.md#const-context
[function pointers]: ../types/function-pointer.md
[raw pointers]: ../types/pointer.md#raw-pointers-const-and-mut
[references]: ../types/pointer.md#shared-references-
[repeat expressions]: ../expressions/array-expr.md
[`Clone`]: ../special-types-and-traits.md#clone
[`Copy`]: ../special-types-and-traits.md#copy
[`Sized`]: ../special-types-and-traits.md#sized
[tuples]: ../types/tuple.md
[trait object]: ../types/trait-object.md
[types]: ../types.md
[attributes]: ../attributes.md
