# Type and Lifetime Parameters

> **<sup>Syntax</sup>**\
> _Generics_ :\
> &nbsp;&nbsp; `<` _GenericParams_ `>`
>
> _GenericParams_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; _LifetimeParams_\
> &nbsp;&nbsp; | ( _LifetimeParam_ `,` )<sup>\*</sup> _TypeParams_
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
> &nbsp;&nbsp; [_OuterAttribute_]<sup>?</sup> [IDENTIFIER] ( `:` [_TypeParamBounds_]<sup>?</sup> )<sup>?</sup> ( `=` [_Type_] )<sup>?</sup>

Functions, type aliases, structs, enumerations, unions, traits and
implementations may be *parameterized* by types and lifetimes. These parameters
are declared in angle <span class="parenthetical">brackets (`<...>`)</span>,
usually immediately after the name of the item and before its definition. For
implementations, which don't have a name, they are declared directly after the
`impl` keyword. Lifetime parameters must be declared before type parameters.
Some examples of items with type and lifetime parameters:

```rust
fn foo<'a, T>() {}
trait A<U> {}
struct Ref<'a, T> where T: 'a { r: &'a T }
```

[References], [raw pointers], [arrays], [slices][arrays], [tuples] and
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
> &nbsp;&nbsp; `for` `<` [_LifetimeParams_](#type-and-lifetime-parameters) `>`

*Where clauses* provide another way to specify bounds on type and lifetime
parameters as well as a way to specify bounds on types that aren't type
parameters.

Bounds that don't use the item's parameters or higher-ranked lifetimes are
checked when the item is defined. It is an error for such a bound to be false.

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

## Formulae and Free Variables

The syntatic locations a *variable* is allowed is a *formulae*. Formulae for
type and lifetime variables are:

* [Types]
* Type constructors (for example, `Option` in `enum Option<T> { ... }`)
* Lifetimes (only lifetime variables)
* [Items]
* Item constructors (for example, `Into` in `trait Into<T>`)
* [Associated Items]
* [Trait Bounds]
* [Expressions] \(through path expressions with generics)

The free variables of a formulae are those not referenced (*bound*) by a generic
binder in the formulae.

These free variables may be substituted for other types or lifetimes.
A formulae is *concrete*, or *closed*, if there are no free variables in it.
See [Wikipedia][wikipedia free variables] for more.

Examples of generic binders for type and lifetime variables are:

* `for<'a>` in `for<'a> fn(&'a u8)`
* `<T>` in `fn identity<T>(x: T) -> T { x }` for item constructors
* [Traits] and [implementations] implicitly bind `Self`

All type and lifetimes are unbound at the boundaries of items.

Formulae nest. For example, in a function item, the entire function item is a
formulae but so are the types of the parameters of the function. A type or
lifetime variable may be free in one formulae but bound in a containing
formulae. For example, in the function prototype `for<'a> fn foo(a: &'a i32>`,
`'a` is bound by the `for<'a>` but in the formulae of the type
`&'a i32`, `'a` is a free variable.

Furthermore, the same syntax may have free variables when looked at as one
formulae while having no free variables when looked at as another formulae. Most
commonly, the type or item will have a free variable when the type or item
constructor does not. For example, `Option<T>` as a type has `T` as a free
variable but as a type constructor, has no free variables.

For example of free variables and concrete formulae, consider this trait and
function:

```rust
# struct SomeStruct;
#
trait Example<A> {
    fn foo(a: A);

    fn bar<'b>(a: A, b: &'b SomeStruct);

    fn baz();
}

fn quux(a: i32, b: Option<i32>) {
#    unimplemented!("")
    // ...
}
```

In it, the trait `Example<A>` has a free variable `A` since it declares it
itself. Furthermore, `foo` has `A` as a free variable because it uses it as the
type of its first argument. The function `bar` has `A` as a free type variable,
getting `A` from the trait's generic parameters. The lifetime `'b` is bound in
the function constructor `bar` but is free in the type `&'b SomeStruct`. All of
these trait functions also have `Self` as a free variable despite not being
explicitly quantified. The function `quux` is concrete.

For another example, the following table shows the free type variables of
various types. Types without free parameters show "concrete" instead of "none".
Assume that `A`, `E`, and `'a` are defined as generic parameters.

| Type | Free Variables |
| - | - |
| `i32` | concrete |
| `&'a i32` | `'a` |
| `&i32` | *anonymous inferred lifetime of reference* |
| `&'static i32` | concrete |
| `UserDefinedType` | concrete |
| `GenericDefinedType<A>` | `A` |
| `GenericDefinedType<i32>` | concrete |
| `GenericDefinedType<Option<A>>` | `A` |
| `GenericDefinedType<Result<&'a A, E>` | `'a`, `A`, `E` |
| `GenericDefinedType<Option<UserDefinedType>` | concrete |


> Note: In some programming languages and in type theory, concrete types are
> called *ground types*. In mathematical logic, these are called ground
> expressions. See [Wikipedia][wikipedia ground expression] for more.

## Attributes

Generic lifetime and type parameters allow [attributes] on them. There are no
built-in attributes that do anything in this position, although custom derive
attributes may give meaning to it.

This example shows using a custom derive attribute to modify the meaning of a
generic parameter.

```ignore
// Assume that the derive for MyFlexibleClone declared `my_flexible_clone` as
// an attribute it understands.
#[derive(MyFlexibleClone)] struct Foo<#[my_flexible_clone(unbounded)] H> {
    a: *const H
}
```

[IDENTIFIER]: identifiers.html
[LIFETIME_OR_LABEL]: tokens.html#lifetimes-and-loop-labels

[_LifetimeBounds_]: trait-bounds.html
[_Lifetime_]: trait-bounds.html
[_OuterAttribute_]: attributes.html
[_Type_]: types.html#type-expressions
[_TypeParamBounds_]: trait-bounds.html

[arrays]: types/array.html
[associated items]: items/associated-items.html
[attributes]: attributes.html
[expressions]: expression.html
[function pointers]: types/function-pointer.html
[implementations]: items/implementations.html
[items]: items.html
[references]: types/pointer.html#shared-references-
[raw pointers]: types/pointer.html#raw-pointers-const-and-mut
[trait bounds]: trait-bound.html
[trait object]: types/trait-object.html
[traits]: items/traitshtml
[tuples]: types/tuple.html
[types]: types.html
[wikipedia free variables]: https://en.wikipedia.org/wiki/Free_variables_and_bound_variables
[wikipedia ground expression]: https://en.wikipedia.org/wiki/Ground_expression
[`Clone`]: special-types-and-traits.html#clone
[`Copy`]: special-types-and-traits.html#copy
[`Sized`]: special-types-and-traits.html#sized