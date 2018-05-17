# Traits

A _trait_ describes an abstract interface that types can implement. This
interface consists of [associated items], which come in three varieties:

- [functions](items/associated-items.html#associated-functions-and-methods)
- [types](items/associated-items.html#associated-types)
- [constants](items/associated-items.html#associated-constants)

All traits define an implicit type parameter `Self` that refers to "the type
that is implementing this interface". Traits may also contain additional type
parameters. These type parameters (including `Self`) may be constrained by
other traits and so forth as usual.

Traits are implemented for specific types through separate [implementations].

Items associated with a trait do not need to be defined in the trait, but they
may be. If the trait provides a definition, then this definition acts as a
default for any implementation which does not override it. If it does not, then
any implementation must provide a definition.

## Trait bounds

Generic items may use traits as [bounds] on their type parameters.

## Generic Traits

Type parameters can be specified for a trait to make it generic. These appear
after the trait name, using the same syntax used in [generic
functions](items/functions.html#generic-functions).

```rust
trait Seq<T> {
    fn len(&self) -> u32;
    fn elt_at(&self, n: u32) -> T;
    fn iter<F>(&self, F) where F: Fn(T);
}
```

## Object Safety

Object safe traits can be the base trait of a [trait object]. A trait is
*object safe* if it has the following qualities (defined in [RFC 255]):

* It must not require `Self: Sized`
* All associated functions must either have a `where Self: Sized` bound, or
    * Not have any type parameters (although lifetime parameters are allowed),
      and
    * Be a [method] that does not use `Self` except in the type of the receiver.
* It must not have any associated constants.

## Supertraits

Trait bounds on `Self` are considered "supertraits". These are required to be
acyclic. Supertraits are somewhat different from other constraints in that
they affect what methods are available in the vtable when the trait is used as
a [trait object]. Consider the following example:

```rust
trait Shape { fn area(&self) -> f64; }
trait Circle : Shape { fn radius(&self) -> f64; }
```

The syntax `Circle : Shape` means that types that implement `Circle` must also
have an implementation for `Shape`. Multiple supertraits are separated by `+`,
`trait Circle : Shape + PartialEq { }`. In an implementation of `Circle` for a
given type `T`, methods can refer to `Shape` methods, since the typechecker
checks that any type with an implementation of `Circle` also has an
implementation of `Shape`:

```rust
struct Foo;

trait Shape { fn area(&self) -> f64; }
trait Circle : Shape { fn radius(&self) -> f64; }
impl Shape for Foo {
    fn area(&self) -> f64 {
        0.0
    }
}
impl Circle for Foo {
    fn radius(&self) -> f64 {
        println!("calling area: {}", self.area());

        0.0
    }
}

let c = Foo;
c.radius();
```

In type-parameterized functions, methods of the supertrait may be called on
values of subtrait-bound type parameters. Referring to the previous example of
`trait Circle : Shape`:

```rust
# trait Shape { fn area(&self) -> f64; }
# trait Circle : Shape { fn radius(&self) -> f64; }
fn radius_times_area<T: Circle>(c: T) -> f64 {
    // `c` is both a Circle and a Shape
    c.radius() * c.area()
}
```

Likewise, supertrait methods may also be called on trait objects.

```rust
# trait Shape { fn area(&self) -> f64; }
# trait Circle : Shape { fn radius(&self) -> f64; }
# impl Shape for i32 { fn area(&self) -> f64 { 0.0 } }
# impl Circle for i32 { fn radius(&self) -> f64 { 0.0 } }
# let mycircle = 0i32;
let mycircle = Box::new(mycircle) as Box<Circle>;
let nonsense = mycircle.radius() * mycircle.area();
```

[bounds]: trait-bounds.html
[trait object]: types.html#trait-objects
[explicit]: expressions/operator-expr.html#type-cast-expressions
[RFC 255]: https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md
[associated items]: items/associated-items.html
[method]: items/associated-items.html#methods
[implementations]: items/implementations.html