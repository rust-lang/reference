# Impl trait

> **<sup>Syntax</sup>**\
> _ImplTraitType_ : `impl` [_TypeParamBounds_]
>
> _ImplTraitTypeOneBound_ : `impl` [_TraitBound_]

`impl Trait` provides ways to specify unnamed but concrete types that
implement a specific trait.
It can appear in two sorts of places: argument position (where it can act as an anonymous type parameter to functions), and return position (where it can act as an abstract return type).

```rust
trait Trait {}
# impl Trait for () {}

// argument position: anonymous type parameter
fn foo(arg: impl Trait) {
}

// return position: abstract return type
fn bar() -> impl Trait {
}
```
## Anonymous type parameters

> Note: This is often called "impl Trait in argument position".
(The term "parameter" is more correct here, but "impl Trait in argument position" is the phrasing used during the development of this feature, and it remains in parts of the implementation.)

Functions can use `impl` followed by a set of trait bounds to declare a parameter as having an anonymous type.
The caller must provide a type that satisfies the bounds declared by the anonymous type parameter, and the function can only use the methods available through the trait bounds of the anonymous type parameter.

For example, these two forms are almost equivalent:

```rust
trait Trait {}

// generic type parameter
fn with_generic_type<T: Trait>(arg: T) {
}

// impl Trait in argument position
fn with_impl_trait(arg: impl Trait) {
}
```

That is, `impl Trait` in argument position is syntactic sugar for a generic type parameter like `<T: Trait>`, except that the type is anonymous and doesn't appear in the [_GenericParams_] list.

> **Note:**
> For function parameters, generic type parameters and `impl Trait` are not exactly equivalent.
> With a generic parameter such as `<T: Trait>`, the caller has the option to explicitly specify the generic argument for `T` at the call site using [_GenericArgs_], for example, `foo::<usize>(1)`.
> Changing a parameter from either one to the other can constitute a breaking change for the callers of a function, since this changes the number of generic arguments.

## Abstract return types

> Note: This is often called "impl Trait in return position".

Functions can use `impl Trait` to return an abstract return type.
These types stand in for another concrete type where the caller may only use the methods declared by the specified `Trait`.
Each possible return value from the function must resolve to the same concrete type.

`impl Trait` in return position allows a function to return an unboxed abstract type.
This is particularly useful with [closures] and iterators.
For example, closures have a unique, un-writable type.
Previously, the only way to return a closure from a function was to use a [trait object]:

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

This could incur performance penalties from heap allocation and dynamic dispatch.
It wasn't possible to fully specify the type of the closure, only to use the `Fn` trait.
That means that the trait object is necessary.
However, with `impl Trait`, it is possible to write this more simply:

```rust
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
```

which also avoids the drawbacks of using a boxed trait object.

Similarly, the concrete types of iterators could become very complex, incorporating the types of all previous iterators in a chain.
Returning `impl Iterator` means that a function only exposes the `Iterator` trait as a bound on its return type, instead of explicitly specifying all of the other iterator types involved.

## Return-position `impl Trait` in traits and trait implementations

Functions in traits may also use `impl Trait` as a syntax for an anonymous associated type.

Every `impl Trait` in the return type of an associated function in a trait is desugared to an anonymous associated type. The return type that appears in the implementation's function signature is used to determine the value of the associated type.

## Capturing

Behind each return-position `impl Trait` abstract type is some hidden concrete type.  For this concrete type to use a generic parameter, that generic parameter must be *captured* by the abstract type.

## Automatic capturing

Return-position `impl Trait` abstract types automatically capture certain of the in-scope generic parameters.  Everywhere, these automatically capture all in-scope type and const generic parameters.

On items of trait impls and trait definitions, these types additionally automatically capture all in-scope generic lifetime parameters, including higher-ranked ones.  On free functions and on associated functions and methods of inherent impls, only the generic lifetime parameters that appear in the bounds of abstract return type are captured.

## Precise capturing

The set of generic parameters captured by a return-position `impl Trait` abstract type may be explicitly controlled with a [`use<..>` bound].  If present, only the generic parameters listed in the `use<..>` bound will be captured.  E.g.:

```rust
fn capture<'a, 'b, T>(x: &'a (), y: T) -> impl Sized + use<'a, T> {
  //                                      ~~~~~~~~~~~~~~~~~~~~~~~
  //                                     Captures `'a` and `T` only.
  (x, y)
}
```

Currently, only one `use<..>` bound may be present in a bounds list, such bounds are not allowed in the signature of items of a trait definition, all in-scope type and const generic parameters must be included, and all lifetime parameters that appear in other bounds of the abstract type must be included.  Within the `use<..>` bound, any lifetime parameters present must appear before all type and const generic parameters, and the elided lifetime (`'_`) may be present if it is otherwise allowed to appear within the `impl Trait` return type.

Because all in-scope type parameters must be included by name, a `use<..>` bound may not be used in the signature of items that use argument-position `impl Trait`, as those items have anonymous type parameters in scope.

## Differences between generics and `impl Trait` in return position

In argument position, `impl Trait` is very similar in semantics to a generic type parameter.
However, there are significant differences between the two in return position.
With `impl Trait`, unlike with a generic type parameter, the function chooses the return type, and the caller cannot choose the return type.

The function:

```rust
# trait Trait {}
fn foo<T: Trait>() -> T {
    // ...
# panic!()
}
```

allows the caller to determine the return type, `T`, and the function returns that type.

The function:

```rust
# trait Trait {}
# impl Trait for () {}
fn foo() -> impl Trait {
    // ...
}
```

doesn't allow the caller to determine the return type.
Instead, the function chooses the return type, but only promises that it will implement `Trait`.

## Limitations

`impl Trait` can only appear as a parameter or return type of a non-`extern` function.
It cannot be the type of a `let` binding, field type, or appear inside a type alias.

[_GenericArgs_]: ../paths.md#paths-in-expressions
[_GenericParams_]: ../items/generics.md
[_TraitBound_]: ../trait-bounds.md
[_TypeParamBounds_]: ../trait-bounds.md
[`use<..>` bound]: ../trait-bounds.md#use-bounds
[closures]: closure.md
[trait object]: trait-object.md
