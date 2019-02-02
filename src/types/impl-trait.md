# Impl trait

> **<sup>Syntax</sup>**\
> _ImplTraitType_ : `impl` [_TypeParamBounds_]
>
> _ImplTraitTypeOneBound_ : `impl` [_TraitBound_]

## Anonymous type parameters

> Note: This section is a placeholder for more comprehensive reference
> material.

> Note: This is often called "impl Trait in argument position".

Functions can declare an argument to be an anonymous type parameter where the
callee must provide a type that has the bounds declared by the anonymous type
parameter and the function can only use the methods available by the trait
bounds of the anonymous type parameter.

They are written as `impl` followed by a set of trait bounds.

## Abstract return types

> Note: This section is a placeholder for more comprehensive reference
> material.

> Note: This is often called "impl Trait in return position".

Functions, except for associated trait functions, can return an abstract
return type. These  types stand in for another concrete type where the
use-site may only use the trait methods declared by the trait bounds of the
type.

They are written as `impl` followed by a set of trait bounds.

[_TraitBound_]: ../trait-bounds.md
[_TypeParamBounds_]: ../trait-bounds.md
