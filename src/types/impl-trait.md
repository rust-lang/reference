# Impl trait

> **<sup>Syntax</sup>**\
> _ImplTraitType_ : `impl` [_TypeParamBounds_]
>
> _ImplTraitTypeOneBound_ : `impl` [_TraitBound_]

`impl Trait` provides ways to specify unnamed but concrete types that
implement a specific trait.
It can appear in a few places:

* argument position (where it can act as an anonymous type parameter to functions),
* return position (where it can act as an abstract return type),
* in type aliases (where it acts as an abstract type), and
* in associated types (where it acts as an abstract type).

```rust
#![feature(type_alias_impl_trait)]
trait Trait {}
# impl Trait for () {}

// argument position: anonymous type parameter
fn foo(arg: impl Trait) {
}

// return position: abstract return type
fn bar() -> impl Trait {
}

// type alias: abstract type
type Foo = impl Trait;
fn baz() -> Foo {
    let x: Foo = ();
}
```
## Anonymous type parameters

> Note: This is often called "impl Trait in argument position".
(The term "parameter" is more correct here, but "impl Trait in argument position" is the phrasing used during the development of this feature, and it remains in parts of the implementation.)

Functions can use `impl` followed by a set of trait bounds to declare a parameter as having an anonymous type.
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
> For function parameters, generic type parameters and `impl Trait` are not exactly equivalent.
> With a generic parameter such as `<T: Trait>`, the caller has the option to explicitly specify the generic argument for `T` at the call site using [_GenericArgs_], for example, `foo::<usize>(1)`.
> If `impl Trait` is the type of *any* function parameter, then the caller can't ever provide any generic arguments when calling that function.
This includes generic arguments for the return type or any const generics.
>
> Therefore, changing the function signature from either one to the other can constitute a breaking change for the callers of a function.

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

### Differences between generics and `impl Trait` in return position

In argument position, `impl Trait` is very similar in semantics to a generic type parameter.
However, there are significant differences between the two in return position.
With `impl Trait`, unlike with a generic type parameter, the function chooses the return type, and the caller cannot choose the return type.

The function:

```rust,ignore
fn foo<T: Trait>() -> T {
```

allows the caller to determine the return type, `T`, and the function returns that type.

The function:

```rust,ignore
fn foo() -> impl Trait {
```

doesn't allow the caller to determine the return type.
Instead, the function chooses the return type, but only promises that it will implement `Trait`.


## Type Alias Impl Trait

In contrast to `impl Trait`s in function return types, type aliases can be used in more places than just return types and the same type alias can be used multiple times.

```rust,ignore
fn foo() -> impl Trait {
    value_of_type_that_implements_Trait
}

// The above function can be changed to the
// following without a change in behaviour for callers.

type Foo = impl Trait;
fn foo() -> Foo {
    value_of_type_that_implements_Trait
}
```

When such a type alias is used as the return type of multiple functions, all functions must use the same hidden type.
This is similar to how all code paths in a function with a return-position-impl-trait must return the same type:

```rust,ignore
fn foo() -> impl Debug {
    if true {
        return 42;
    }
    "42" // ERROR: another return site returns an `i32`
}

type Bar = impl Debug;
fn bar() -> Bar {
    42
}
fn boo() -> Bar {
    "42" // ERROR: another function uses `i32` for `Bar`
}
```

### Argument position

Without knowing the hidden type, we can still use the opaque type and use its trait bounds. In this case we can use the `Debug` trait to render it:

```rust,ignore
fn bop(x: Bar) {
    println!("{x:?}");
}
```

Note that this is very different from using `impl Trait` in argument position, as there is no anonymous generic parameter introduced.

Binding a hidden type works in both directions, not just assigning a hidden type value to the opaque type, but also reading an opaque type into a hidden type value:

```rust,ignore
fn bup(x: Bar) {
    let x: i32 = x;
}
```

This does not "reveal" the hidden type. It binds an explicitly known `i32` type as the hidden type of `Bar` and will error if that's not the hidden type everywhere else, too.

As a last usage, you can avoid binding any hidden types and just use the type-alias-impl-trait by just forwarding it elsewhere:

```rust,ignore
fn burp(x: Bar) -> Bar {
    x
}
```

### Binding types

You can also use type-alias-impl-trait for the type
of local variables, constants, statics, ...

```rust,ignore
let x: Bar = 42;
const X: Bar = 42;
static Y: Bar = 42;
```

### Nested in other types

You can use type-alias-impl-trait in other types:

```rust,ignore
struct MyStruct {
    bar: Bar,
}
```

and use it just like other uses of `Bar`:

```rust,ignore
fn foo(my_struct: &MyStruct) {
    println!("{:?}", my_struct.bar);
}
fn new() -> MyStruct {
    MyStruct {
        bar: 42
    }
}
```

### Usage in trait impls

Since type-alias-impl-trait can be referenced anywhere a type alias could be, this also means you can use them in `impl` blocks:

```rust,ignore
type Foo = impl Trait;

impl Bar for Foo {}
```

There's a huge caveat though: now it's possible for there to be an impl for an opaque type *and* its hidden type:

```rust,ignore
type Foo = impl Trait;
fn foo() -> Foo {}

impl Bar for Foo {}
impl Bar for () {} // ERROR conflicts with `impl Bar for Foo`
```

This check is *not* done by revealing the hidden type, but by checking whether a type could be a hidden type for that specific opaque type. So the following program is legal:

```rust
#![feature(type_alias_impl_trait)]
trait Trait {}
impl Trait for () {}
type Foo = impl Trait;
fn foo() -> Foo {}

trait Bar {}
impl Bar for Foo {}
impl Bar for i32 {}
```

This is legal, because `i32` could not possibly be a hidden type of `Foo`, because it doesn't implement `Trait` wich is a requirement for all hypothetical hidden types of `Foo`.

### Associated types

Associated types can also use `impl Trait`:

```rust,ignore
impl Deref for MyType {
    type Target = impl Trait;
    fn deref(&self) -> &Self::Target {
        &self.field
    }
}
```

While this example is fairly artificial, the real benefit is when you have unnameable types like `async` blocks:

```rust,ignore
impl IntoFuture for MyType {
    type Output = ();
    type Future = impl Future<Output = ()>;
    fn into_future(self) -> Self::Future {
        async move {
            // do stuff here
        }
    }
}
```

This way you do not need to write burdensome `Future` impls yourself. Similarly with complex `Iterator` implementations.

### Defining scope

Similar to return-position-impl-trait, you can only bind a hidden type of a type-alias-impl-trait within a specific "scope" (henceforth called "defining scope").
The defining scope of a return-position-impl-trait is the function's body, excluding other items nested within that function's body (we may want to relax that restriction on return-position-impl-trait in the future).

The defining scope of a type-alias-impl-trait is the scope in which it was defined. So usually a module and all its child items, but it can also be a function body, const initializer and similar scopes that can define items.

Any use of the type-alias-impl-trait within the defining scope will become a **defining use** (meaning it binds a hidden type), if the type is coerced to or from, equated with, or subtyped with any other concrete type.
Usages that rely solely on the trait bounds of the type are not considered defining.
Similarly, usages that just pass a value of a type-alias-impl-trait around into other places of the type-alias-impl-trait type are not considered defining.

## Limitations

`impl Trait` can only appear as a parameter or return type of a free or inherent function, within a type alias or within an associated type.
It cannot appear inside implementations of traits, nor can it be the type of a let binding.

[closures]: closure.md
[_GenericArgs_]: ../paths.md#paths-in-expressions
[_GenericParams_]: ../items/generics.md
[_TraitBound_]: ../trait-bounds.md
[trait object]: trait-object.md
[_TypeParamBounds_]: ../trait-bounds.md
