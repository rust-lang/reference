# Impl trait

> **<sup>Syntax</sup>**\
> _ImplTraitType_ : `impl` [_TypeParamBounds_]
>
> _ImplTraitTypeOneBound_ : `impl` [_TraitBound_]

> **Edition differences**: `impl Trait` is new in the 2018 edition.

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

Functions can use `impl` followed by a set of trait bounds to declare an argument as having an anonymous type.
The caller must provide a type that satisfies the bounds declared by the anonymous type parameter, and the function can only use the methods available through the trait bounds of the anonymous type parameter.

For example, these two forms are almost equivalent:

```rust,ignore
trait Trait {}

// generic type parameter
fn foo<T: Trait>(arg: T) {
}

// impl Trait in argument position
fn foo(arg: impl Trait) {
}
```

That is, `impl Trait` in argument position is syntactic sugar for a generic type parameter like `<T: Trait>`, except that the type is anonymous and doesn't appear in the [_GenericParams_] list.

> **Note:**
> For function arguments, generic type parameters and `impl Trait` are not exactly equivalent.
> With a generic parameter such as `<T: Trait>`, the caller has the option to explicitly specify the generic argument for `T` at the call site using [_GenericArgs_], for example, `foo::<usize>(1)`.
> If `impl Trait` is the type of a function argument, then the caller can't ever specify the type of that argument by using a generic argument.
>
> Therefore, changing the function signature from either one to the other can constitute a breaking change for the callers of a function.

## Abstract return types

> Note: This is often called "impl Trait in return position".

Functions can use `impl Trait` to return an abstract return type.
These types stand in for another concrete type where the caller may only use the methods declared by the specified `Trait`.
Each possible return value from the function must resolve to the same concrete type.

Prior to `impl Trait`, a function could express abstract return types by using [trait objects]:

```rust
trait Trait {}

impl Trait for i32 {}

fn returns_a_trait_object() -> Box<dyn Trait> {
    Box::new(5)
}
```

This has some drawbacks: constructing `Box<T>` involves a heap allocation, and the `dyn Trait` will use dynamic dispatch on its methods.
However, this function only returns one possible type here: the `Box<i32>`.
This means incurring the costs of dynamic dispatch, even though the return type cannot vary.

With `impl Trait`, the code above could be written like this:

```rust
trait Trait {}

impl Trait for i32 {}

fn returns_a_trait_object() -> impl Trait {
    5
}
```

There is no `Box<T>`, no trait object, and no dynamic dispatch.
However, the function can still can obscure the `i32` return type.

With `i32`, this might not seem very useful.
There is one major place in Rust where this is much more useful: closures.

### `impl Trait` and closures

In Rust, [closures] have a unique, un-writable type.
However, they do implement the `Fn` family of traits.
This means that previously, the only way to return a closure from a function was to use a trait object:

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

It wasn't possible to fully specify the type of the closure, only use the `Fn` trait.
That means that the trait object is necessary.
However, with `impl Trait`:

```rust
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
```

It is now possible to return closures by value, just like any other type.

## More details

The above is all you need to know to get going with `impl Trait`, but for some more nitty-gritty details: type parameters and `impl Trait` work slightly differently when they're in argument position versus return position.
Consider this function:

```rust,ignore
fn foo<T: Trait>(x: T) {
```

The caller of this function determines the type, `T`.
This function signature means that the function accepts any type that implements `Trait`."

This version:

```rust,ignore
fn foo<T: Trait>() -> T {
```

is similar, but also different.
The caller determines the return type, `T`, and the function returns it.
Examples of this include the `.parse()` or `.collect()` methods:

```rust,ignore
let x: i32 = "5".parse()?;
let x: u64 = "5".parse()?;
```

Here, `.parse()` has this signature:

```rust,ignore
pub fn parse<F>(&self) -> Result<F, <F as FromStr>::Err> where
    F: FromStr,
```

Same general idea, though with a result type and `FromStr` has an associated type... anyway, you can see how `F` is in the return position here.
So you have the ability to choose.

With `impl Trait`, the function asserts that the return type will implement this trait, but the caller can't know exactly which type.
So with `impl Trait`, unlike with a generic type parameter for the return type, the caller can't choose the return type, and the function itself gets to choose.
If we tried to define parse with `Result<impl F,...` as the return type, it wouldn't work.

### Using `impl Trait` in more places

As previously mentioned, as a start, you will only be able to use `impl Trait` as the argument or return type of a free or inherent function.
However, `impl Trait` can't be used inside implementations of traits, nor can it be used as the type of a let binding or inside a type alias.
Some of these restrictions will eventually be lifted.
For more information, see the [tracking issue on `impl Trait`](https://github.com/rust-lang/rust/issues/34511).

[closures]: closure.md
[_GenericArgs_]: ../paths.md#paths-in-expressions
[_GenericParams_]: ../items/generics.md
[_TraitBound_]: ../trait-bounds.md
[trait objects]: trait-object.md
[_TypeParamBounds_]: ../trait-bounds.md
