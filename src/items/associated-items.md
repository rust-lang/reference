# Associated Items

*Associated Items* are the items declared in [traits] or defined in
[implementations]. They are called this because they are defined on an associate
type &mdash; the type in the implementation. They are a subset of the kinds of
items you can declare in a module. Specifically, there are [associated
functions], [associated types], and [associated constants].

[associated functions]: #associated-functions-and-methods
[associated types]: #associated-types
[associated constants]: #associated-constants

Associated items are useful when the associated item logically is related to the
associating item. For example, the `is_some` method on `Option` is intrinsically
related to Options, so should be associated.

Associated items are also the contract that traits have. 

## Associated functions and methods

*Associated functions* are [functions] associated with a type.

An *associated function declaration* is written as `fn`, then an [identifier]
then optional generics, then `(` then a parameter list, then `)`, then
optionally `->` and a type, then an optional where clause, then finally a `;`.

The identifier if the name of the function. The generics declares types for
usage in the rest of the function declaration. The generics, parameter list,
return type, and where clause must be the same in the associated function
definition.

An *associated function definiton* is written as an associated function
declaration, but instead of a `;`, there is a [block] that evaluates to the
return type.

An example of a common associated function is the `new` function that returns
a value of the type the associated function is associated with.

```rust
struct Struct {
    field: i32;
}

impl Struct {
    fn new() -> Struct {
        Struct {
            field: 0i32
        }
    }
}
```

When the associated function is declared on a trait, the function can be called
on the trait. When this happens, it is substituted for
`<_ as Trait>::function_name`. 

```rust
trait Num {
    fn from_i32(n: i32) -> Self;
}
impl Num for f64 {
    fn from_i32(n: i32) -> f64 { n as f64 }
}
let x: f64 = Num::from_i32(42);
let x: f64 = f64::from_i32(42);
```

Associated functions whose first parameter is named `self` are called *methods*
and may be invoked using the [method call operator], for example, `x.foo()`, as
well as the usual function call notation.

When the first parameter is named `self`, the following shorthands may be used:

* `self` -> `self: Self`
* `&self` -> `self: &Self`
* `&mut self` -> `&mut Self`
* `Box<self>` -> `self: Box<Self>`

Consider the following trait:

```rust
# type Surface = i32;
# type BoundingBox = i32;
trait Shape {
    fn draw(&self, Surface);
    fn bounding_box(&self) -> BoundingBox;
}
```

This defines a trait with two methods. All values that have [implementations]
of this trait in scope can have their `draw` and `bounding_box` methods called.

```rust
# type Surface = i32;
# type BoundingBox = i32;
# trait Shape {
#     fn draw(&self, Surface);
#     fn bounding_box(&self) -> BoundingBox;
# }

struct Circle {
    // ...
}

impl Shape for Circle {
    // ...
#   fn draw(&self, Surface) -> {}
#   fn bounding_box(&self) -> BoundingBox { 0i32; }
}

# impl Box {
#     fn new() -> Circle { Circle{} }
}

let circle_shape = Circle::new();
let bounding_box = circle_shape.bounding_box();
```

## Associated Types

*Associated types* are [type aliases] associated with another type. Associated
types cannot be defined in [inherent implementations] nor can they be given a
default implementation in traits.

An *associated type declaration* is written as `type`, then an [identifier], and
finally an optional trait bounds.

The identifier is the name of the declared type alias. The optional trait bounds
must be fulfilled by the implementations of the type alias.

An *associated type definition* is written as `type`, then an [identifier], then
an `=`, and finally a [type].

If an item `Item` has an associated type `Assoc`, then `Item::Assoc` is a type
that is an alias of the type specified in the associated type definition.

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
    // Usage of the associated type to refer to OtherStruct as Struct::Assoc
    let _other_struct: OtherStruct = Struct::Assoc::new();
}
```

### Associated Types Container Example

Consider the following example of a `Container` trait. Notice how the type is
available for use in the method signatures:

```rust
trait Container {
    type E;
    fn empty() -> Self;
    fn insert(&mut self, Self::E);
}
```

In order for a type to implement this trait, it must not only provide
implementations for every method, but it must specify the type `E`. Here's an
implementation of `Container` for the standard library type `Vec`:

```rust
# trait Container {
#     type E;
#     fn empty() -> Self;
#     fn insert(&mut self, Self::E);
# }
impl<T> Container for Vec<T> {
    type E = T;
    fn empty() -> Vec<T> { Vec::new() }
    fn insert(&mut self, x: T) { self.push(x); }
}
```

## Associated Constants

*Associated constants* are [constants] associated with a type.

An *associated constant declaration* is written as `const`, then an identifier,
then `:`, then a type, finished by a `;`.

The identifier is the name of the constant used in the path. The type is the
type that the definition has to implement.

An *associated constant definition* is written as a declaraction, but between
the type and the `;`, there is an `=` followed by a [constant expression].

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

[trait]: items/traits.html
[type aliases]: items/type-aliases.html
[inherent implementations]: items/implementations.html#inherent-implementations
[identifier]: identifiers.html
[trait object]: types.html#trait-objects
[implementations]: items/implementations.html
[type]: types.html
[constants]: items/constants.html
[constant expression]: expressions.html#constant-expressions
[functions]: items/functions.html
[method call operator]: expressions/method-call-expr.html
[block]: expressions/block-expr.html