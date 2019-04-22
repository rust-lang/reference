# Associated Items

*Associated Items* are the items declared in [traits] or defined in
[implementations]. They are called this because they are defined on an associate
type &mdash; the type in the implementation. They are a subset of the kinds of
items you can declare in a module. Specifically, there are [associated
functions] (including methods), [associated types], and [associated constants].

[associated functions]: #associated-functions-and-methods
[associated types]: #associated-types
[associated constants]: #associated-constants

Associated items are useful when the associated item logically is related to the
associating item. For example, the `is_some` method on `Option` is intrinsically
related to Options, so should be associated.

Every associated item kind comes in two varieties: definitions that contain the
actual implementation and declarations that declare signatures for
definitions.

It is the declarations that make up the contract of traits and what it available
on generic types.

## Associated functions and methods

*Associated functions* are [functions] associated with a type.

An *associated function declaration* declares a signature for an associated
function definition. It is written as a function item, except the
function body is replaced with a `;`.

The identifier is the name of the function. The generics, parameter list,
return type, and where clause of the associated function must be the same as the
associated function declarations's.

An *associated function definition* defines a function associated with another
type. It is written the same as a [function item].

An example of a common associated function is a `new` function that returns
a value of the type the associated function is associated with.

```rust
struct Struct {
    field: i32
}

impl Struct {
    fn new() -> Struct {
        Struct {
            field: 0i32
        }
    }
}

fn main () {
    let _struct = Struct::new();
}
```

When the associated function is declared on a trait, the function can also be
called with a [path] that is a path to the trait appended by the name of the
trait. When this happens, it is substituted for `<_ as Trait>::function_name`.

```rust
trait Num {
    fn from_i32(n: i32) -> Self;
}

impl Num for f64 {
    fn from_i32(n: i32) -> f64 { n as f64 }
}

// These 4 are all equivalent in this case.
let _: f64 = Num::from_i32(42);
let _: f64 = <_ as Num>::from_i32(42);
let _: f64 = <f64 as Num>::from_i32(42);
let _: f64 = f64::from_i32(42);
```

### Methods

> _Method_ :\
> &nbsp;&nbsp; [_FunctionQualifiers_] `fn` [IDENTIFIER]&nbsp;[_Generics_]<sup>?</sup>\
> &nbsp;&nbsp; &nbsp;&nbsp; `(` _SelfParam_ (`,` [_FunctionParam_])<sup>\*</sup> `,`<sup>?</sup> `)`\
> &nbsp;&nbsp; &nbsp;&nbsp; [_FunctionReturnType_]<sup>?</sup> [_WhereClause_]<sup>?</sup>\
> &nbsp;&nbsp; &nbsp;&nbsp; [_BlockExpression_]
>
> _SelfParam_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; (`&` | `&` [_Lifetime_])<sup>?</sup> `mut`<sup>?</sup> `self`\
> &nbsp;&nbsp; | `mut`<sup>?</sup> `self` (`:` [_Type_])<sup>?</sup>

Associated functions whose first parameter is named `self` are called *methods*
and may be invoked using the [method call operator], for example, `x.foo()`, as
well as the usual function call notation.

If the type of the `self` parameter is specified, it is limited to one of the
following types:

- `Self`
- `&Self`
- `&mut Self`
- [`Box<Self>`]
- [`Rc<Self>`]
- [`Arc<Self>`]
- [`Pin<P>`] where `P` is one of the above types except `Self`.

The `Self` term can be replaced with the type being implemented.

```rust
# use std::rc::Rc;
# use std::sync::Arc;
# use std::pin::Pin;
struct Example;
impl Example {
    fn by_value(self: Self) {}
    fn by_ref(self: &Self) {}
    fn by_ref_mut(self: &mut Self) {}
    fn by_box(self: Box<Self>) {}
    fn by_rc(self: Rc<Self>) {}
    fn by_arc(self: Arc<Self>) {}
    fn by_pin(self: Pin<&Self>) {}
    fn explicit_type(self: Arc<Example>) {}
    fn with_lifetime<'a>(self: &'a Self) {}
}
```

Shorthand syntax can be used without specifying a type, which have the
following equivalents:

Shorthand             | Equivalent
----------------------|-----------
`self`                | `self: Self`
`&'lifetime self`     | `self: &'lifetime Self`
`&'lifetime mut self` | `self: &'lifetime mut Self`

> Note: Lifetimes can be and usually are elided with this shorthand.

If the `self` parameter is prefixed with `mut`, it becomes a mutable variable,
similar to regular parameters using a `mut` [identifier pattern]. For example:

```rust
trait Changer: Sized {
    fn change(mut self) {}
    fn modify(mut self: Box<Self>) {}
}
```

As an example of methods on a trait, consider the following:

```rust
# type Surface = i32;
# type BoundingBox = i32;
trait Shape {
    fn draw(&self, surface: Surface);
    fn bounding_box(&self) -> BoundingBox;
}
```

This defines a trait with two methods. All values that have [implementations]
of this trait while the trait is in scope can have their `draw` and
`bounding_box` methods called.

```rust
# type Surface = i32;
# type BoundingBox = i32;
# trait Shape {
#     fn draw(&self, surface: Surface);
#     fn bounding_box(&self) -> BoundingBox;
# }
#
struct Circle {
    // ...
}

impl Shape for Circle {
    // ...
#   fn draw(&self, _: Surface) {}
#   fn bounding_box(&self) -> BoundingBox { 0i32 }
}

# impl Circle {
#     fn new() -> Circle { Circle{} }
# }
#
let circle_shape = Circle::new();
let bounding_box = circle_shape.bounding_box();
```

> **Edition Differences**: In the 2015 edition, it is possible to declare trait
> methods with anonymous parameters (e.g. `fn foo(u8)`). This is deprecated and
> an error as of the 2018 edition. All parameters must have an argument name.

## Associated Types

*Associated types* are [type aliases] associated with another type. Associated
types cannot be defined in [inherent implementations] nor can they be given a
default implementation in traits.

An *associated type declaration* declares a signature for associated type
definitions. It is written as `type`, then an [identifier], and
finally an optional list of trait bounds.

The identifier is the name of the declared type alias. The optional trait bounds
must be fulfilled by the implementations of the type alias.

An *associated type definition* defines a type alias on another type. It is
written as `type`, then an [identifier], then an `=`, and finally a [type].

If a type `Item` has an associated type `Assoc` from a trait `Trait`, then
`<Item as Trait>::Assoc` is a type that is an alias of the type specified in the
associated type definition. Furthermore, if `Item` is a type parameter, then
`Item::Assoc` can be used in type parameters.

```rust
trait AssociatedType {
    // Associated type declaration
    type Assoc;
}

struct Struct;

struct OtherStruct;

impl AssociatedType for Struct {
    // Associated type definition
    type Assoc = OtherStruct;
}

impl OtherStruct {
    fn new() -> OtherStruct {
        OtherStruct
    }
}

fn main() {
    // Usage of the associated type to refer to OtherStruct as <Struct as AssociatedType>::Assoc
    let _other_struct: OtherStruct = <Struct as AssociatedType>::Assoc::new();
}
```

### Associated Types Container Example

Consider the following example of a `Container` trait. Notice that the type is
available for use in the method signatures:

```rust
trait Container {
    type E;
    fn empty() -> Self;
    fn insert(&mut self, elem: Self::E);
}
```

In order for a type to implement this trait, it must not only provide
implementations for every method, but it must specify the type `E`. Here's an
implementation of `Container` for the standard library type `Vec`:

```rust
# trait Container {
#     type E;
#     fn empty() -> Self;
#     fn insert(&mut self, elem: Self::E);
# }
impl<T> Container for Vec<T> {
    type E = T;
    fn empty() -> Vec<T> { Vec::new() }
    fn insert(&mut self, x: T) { self.push(x); }
}
```

## Associated Constants

*Associated constants* are [constants] associated with a type.

An *associated constant declaration* declares a signature for associated
constant definitions. It is written as `const`, then an identifier,
then `:`, then a type, finished by a `;`.

The identifier is the name of the constant used in the path. The type is the
type that the definition has to implement.

An *associated constant definition* defines a constant associated with a
type. It is written the same as a [constant item].

### Associated Constants Examples

A basic example:

```rust
trait ConstantId {
    const ID: i32;
}

struct Struct;

impl ConstantId for Struct {
    const ID: i32 = 1;
}

fn main() {
    assert_eq!(1, Struct::ID);
}
```

Using default values:

```rust
trait ConstantIdDefault {
    const ID: i32 = 1;
}

struct Struct;
struct OtherStruct;

impl ConstantIdDefault for Struct {}

impl ConstantIdDefault for OtherStruct {
    const ID: i32 = 5;
}

fn main() {
    assert_eq!(1, Struct::ID);
    assert_eq!(5, OtherStruct::ID);
}
```

[_BlockExpression_]: expressions/block-expr.html
[_FunctionParam_]: items/functions.html
[_FunctionQualifiers_]: items/functions.html
[_FunctionReturnType_]: items/functions.html
[_Generics_]: items/generics.html
[_Lifetime_]: trait-bounds.html
[_Type_]: types.html#type-expressions
[_WhereClause_]: items/generics.html#where-clauses
[`Arc<Self>`]: special-types-and-traits.html#arct
[`Box<Self>`]: special-types-and-traits.html#boxt
[`Pin<P>`]: special-types-and-traits.html#pinp
[`Rc<Self>`]: special-types-and-traits.html#rct
[traits]: items/traits.html
[type aliases]: items/type-aliases.html
[inherent implementations]: items/implementations.html#inherent-implementations
[identifier]: identifiers.html
[identifier pattern]: patterns.html#identifier-patterns
[implementations]: items/implementations.html
[type]: types.html#type-expressions
[constants]: items/constant-items.html
[constant item]: items/constant-items.html
[functions]: items/functions.html
[function item]: types/function-item.html
[method call operator]: expressions/method-call-expr.html
[path]: paths.html
