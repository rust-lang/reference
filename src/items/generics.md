# Type and Lifetime Parameters

> **<sup>Syntax</sup>**  
> _Generics_ :  
> &nbsp;&nbsp; `<` _GenericParams_ `>`  
>  
> _GenericParams_ :  
> &nbsp;&nbsp; &nbsp;&nbsp; _LifetimeParams_  
> &nbsp;&nbsp; | ( _LifetimeParam_ `,` )<sup>\*</sup> _TypeParams_  
>  
> _LifetimeParams_ :  
> &nbsp;&nbsp; ( _LifetimeParam_ `,` )<sup>\*</sup> _LifetimeParam_<sup>?</sup>  
>  
> _LifetimeParam_ :  
> &nbsp;&nbsp; [LIFETIME_OR_LABEL] `:` [_LifetimeBounds_]<sup>?</sup>  
>  
> _TypeParams_:  
> &nbsp;&nbsp; ( _TypeParam_ `,` )<sup>\*</sup> _TypeParam_ <sup>?</sup>  
>  
> _TypeParam_ :  
> &nbsp;&nbsp; [IDENTIFIER] ( `:` [_TypeParamBounds_] )<sup>?</sup> ( `=` [_Type_] )<sup>?</sup>  

Functions, type aliases, structs, enumerations, unions, traits and
implementations may be *parameterized* by types and lifetimes. These parameters
are listed in angle <span class="parenthetical">brackets (`<...>`)</span>,
usually immediattely after and before its definition the name of the item. For
implementations, which don't have a name, they come directly after `impl`.
Lifetime parameters must be declared before type parameters. Some examples of
items with type and lifetime parameters:

```rust
fn foo<'a, T>() {}
trait A<U> {}
struct Ref<'a, T> where T: 'a { r: &'a T }
```

[References], [raw pointers], [arrays], [slices][arrays], [tuples] and
[function pointers] have lifetime or type parameters as well, but are not
refered to with path syntax.

## Where clauses

> **<sup>Syntax</sup>**  
> _WhereClause_ :  
> &nbsp;&nbsp; `where` ( _WhereClauseItem_ `,` )<sup>\*</sup> _WhereClauseItem_ <sup>?</sup>  
>  
> _WhereClauseItem_ :  
> &nbsp;&nbsp; &nbsp;&nbsp; _LifetimeWhereClauseItem_  
> &nbsp;&nbsp; | _TypeBoundWhereClauseItem_  
>  
> _LifetimeWhereClauseItem_ :  
> &nbsp;&nbsp; [_Lifetime_] `:` [_LifetimeBounds_]  
>  
> _TypeBoundWhereClauseItem_ :  
> &nbsp;&nbsp; _ForLifetimes_<sup>?</sup> [_Type_] `:` [_TypeParamBounds_]<sup>?</sup>  
>  
> _ForLifetimes_ :  
> &nbsp;&nbsp; `for` `<` [_LifetimeParams_](#type-and-lifetime-parameters) `>`  

*Where clauses* provide an another way to specify bounds on type and lifetime
parameters as well as a way to specify bounds on types that aren't type
parameters.

Bounds that don't use the item's parameters are checked when the item is
defined. It is an error for such a bound to be false.

[`Copy`], [`Clone`] and [`Sized`] bounds are also checked for certain generic
types when defining the item. It is an error to have `Copy` or `Clone`as a
bound on a mutable reference, [trait object] or [slice][arrays] or `Sized` as a
bound on a trait object or slice.

```rust,ignore
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

[IDENTIFIER]: identifiers.html
[LIFETIME_OR_LABEL]: tokens.html#lifetimes-and-loop-labels

[_LifetimeBounds_]: trait-bounds.html
[_Lifetime_]: trait-bounds.html
[_Type_]: types.html
[_TypeParamBounds_]: trait-bounds.html

[arrays]: types.html#array-and-slice-types
[function pointers]: types.html#function-pointer-types
[references]: types.html#shared-references-
[raw pointers]: types.html#raw-pointers-const-and-mut
[`Clone`]: special-types-and-traits.html#clone
[`Copy`]: special-types-and-traits.html#copy
[`Sized`]: special-types-and-traits.html#sized
[tuples]: types.html#tuple-types
[trait object]: types.html#trait-objects

[path]: ../paths.html
[Trait]: traits.html#trait-bounds
[_TypePath_]: paths.html
