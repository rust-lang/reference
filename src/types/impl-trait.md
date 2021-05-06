# Impl trait

> **<sup>Syntax</sup>**\
> _ImplTraitType_ : `impl` [_TypeParamBounds_]
>
> _ImplTraitTypeOneBound_ : `impl` [_TraitBound_]

`impl Trait` is the new way to specify unnamed but concrete types that
implement a specific trait.
There are two places you can put it: argument position, and return position.

```rust,ignore
trait Trait {}

// argument position
fn foo(arg: impl Trait) {
}

// return position
fn foo() -> impl Trait {
}
```
## Anonymous type parameters

> Note: This is often called "impl Trait in argument position".

Functions can declare an argument to be an anonymous type parameter where the
caller must provide a type that has the bounds declared by the anonymous type
parameter and the function can only use the methods available by the trait
bounds of the anonymous type parameter.

They are written as `impl` followed by a set of trait bounds.
In argument position, this feature is quite simple.
These two forms are almost the same:

```rust,ignore
trait Trait {}

fn foo<T: Trait>(arg: T) {
}

fn foo(arg: impl Trait) {
}
```

That is, it's a slightly shorter syntax for a generic type parameter.
It means, "`arg` is an argument that takes any type that implements the `Trait` trait."

However, there's also an important technical difference between `T: Trait` and `impl Trait` here.
When you write the former, you can specify the type of `T` at the call site with turbo-fish syntax as with `foo::<usize>(1)`.
In the case of `impl Trait`, if it is used anywhere in the function definition, then you can't use turbo-fish at all.
Therefore, you should be mindful that changing both from and to `impl Trait` can constitute a breaking change for the users of your code.

## Abstract return types

> Note: This is often called "impl Trait in return position".

Functions, except for associated trait functions, can return an abstract
return type. These  types stand in for another concrete type where the
use-site may only use the trait methods declared by the trait bounds of the
type.

They are written as `impl` followed by a set of trait bounds.

Before `impl Trait`, you could do this with trait objects:

```rust
trait Trait {}

impl Trait for i32 {}

fn returns_a_trait_object() -> Box<dyn Trait> {
    Box::new(5)
}
```

However, this has some overhead: the `Box<T>` means that there's a heap allocation here, and this will use dynamic dispatch.
See the `dyn Trait` section for an explanation of this syntax.
But we only ever return one possible thing here, the `Box<i32>`.
This means that we're paying for dynamic dispatch, even though we don't use it!

With `impl Trait`, the code above could be written like this:

```rust
trait Trait {}

impl Trait for i32 {}

fn returns_a_trait_object() -> impl Trait {
    5
}
```

Here, we have no `Box<T>`, no trait object, and no dynamic dispatch.
But we still can obscure the `i32` return type.

With `i32`, this isn't super useful.
But there's one major place in Rust where this is much more useful: closures.

### `impl Trait` and closures

> If you need to catch up on closures, check out [their chapter in the
> book](https://doc.rust-lang.org/book/second-edition/ch13-01-closures.html).

In Rust, closures have a unique, un-writable type.
They do implement the `Fn` family of traits, however.
This means that previously, the only way to return a closure from a function was to use a trait object:

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

You couldn't write the type of the closure, only use the `Fn` trait.
That means that the trait object is necessary. However, with `impl Trait`:

```rust
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
```

We can now return closures by value, just like any other type!

## More details

The above is all you need to know to get going with `impl Trait`, but for some more nitty-gritty details: type parameters and `impl Trait` work slightly differently when they're in argument position versus return position.
Consider this function:

```rust,ignore
fn foo<T: Trait>(x: T) {
```

When you call it, you set the type, `T`.
"you" being the caller here.
This signature says "I accept any type that implements `Trait`."
("any type" == universal in the jargon)

This version:

```rust,ignore
fn foo<T: Trait>() -> T {
```

is similar, but also different.
You, the caller, provide the type you want, `T`, and then the function returns it.
You can see this in Rust today with things like parse or collect:

```rust,ignore
let x: i32 = "5".parse()?;
let x: u64 = "5".parse()?;
```

Here, `.parse` has this signature:

```rust,ignore
pub fn parse<F>(&self) -> Result<F, <F as FromStr>::Err> where
    F: FromStr,
```

Same general idea, though with a result type and `FromStr` has an associated type... anyway, you can see how `F` is in the return position here.
So you have the ability to choose.

With `impl Trait`, you're saying "hey, some type exists that implements this trait, but I'm not gonna tell you what it is."
So now, the caller can't choose, and the function itself gets to choose.
If we tried to define parse with `Result<impl F,...` as the return type, it wouldn't work.

### Using `impl Trait` in more places

As previously mentioned, as a start, you will only be able to use `impl Trait` as the argument or return type of a free or inherent function.
However, `impl Trait` can't be used inside implementations of traits, nor can it be used as the type of a let binding or inside a type alias.
Some of these restrictions will eventually be lifted.
For more information, see the [tracking issue on `impl Trait`](https://github.com/rust-lang/rust/issues/34511).

[_TraitBound_]: ../trait-bounds.md
[_TypeParamBounds_]: ../trait-bounds.md
