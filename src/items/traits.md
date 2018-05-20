# Traits

A _trait_ describes an abstract interface that types can implement. This
interface consists of [associated items], which come in three varieties:

- [functions](items/associated-items.html#associated-functions-and-methods)
- [types](items/associated-items.html#associated-types)
- [constants](items/associated-items.html#associated-constants)

All traits define an implicit type parameter `Self` that refers to "the type
that is implementing this interface". Traits may also contain additional type
parameters. These type parameters, including `Self`, may be constrained by
other traits and so forth [as usual][generics].

Traits are implemented for specific types through separate [implementations].

Items associated with a trait do not need to be defined in the trait, but they
may be. If the trait provides a definition, then this definition acts as a
default for any implementation which does not override it. If it does not, then
any implementation must provide a definition.

## Trait bounds

Generic items may use traits as [bounds] on their type parameters.

## Generic Traits

Type parameters can be specified for a trait to make it generic. These appear
after the trait name, using the same syntax used in [generic functions].

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
* All supertraits must also be object safe.

## Supertraits

**Supertraits** are traits that are required to be implemented for a type to
implement a specific trait. Furthermore, anywhere a [generic] or [trait object]
is bounded by a trait, it has access to the associated items of its supertraits.

Supertraits are declared by trait bounds on the `Self` type of a trait and
transitively the supertraits of the traits declared in those trait bounds. It is
an error for a trait to be its own supertrait.

The trait with a supertrait is called a **subtrait** of its supertrait.

The following is an example of declaring `Shape` to be a supertrait of `Circle`.

```rust
trait Shape { fn area(&self) -> f64; }
trait Circle : Shape { fn radius(&self) -> f64; }
```

And the following is the same example, except using [where clauses].

```rust
trait Shape { fn area(&self) -> f64; }
trait Circle where Self: Shape { fn radius(&self) -> f64; }
```

This next example gives `radius` a default implementation using the `area`
function from `Shape`.

```rust
# trait Shape { fn area(&self) -> f64; }
trait Circle where Self: Shape {
    fn radius(&self) -> f64 {
        // A = pi * r^2
        // so algebraically,
        // r = sqrt(A / pi)
        (self.area() /std::f64::consts::PI).sqrt()
    }
}
```

This next example calls a supertrait method on a generic parameter.

```rust
# trait Shape { fn area(&self) -> f64; }
# trait Circle : Shape { fn radius(&self) -> f64; }
fn print_area_and_radius<C: Circle>(c: C) {
    // Here we call the area method from the supertrait `Shape` of `Circle`.
    println!("Area: {}", c.area());
    println!("Radius: {}", c.radius());
}
```

Similarly, here is an example of calling supertrait methods on trait objects.

```rust
# trait Shape { fn area(&self) -> f64; }
# trait Circle : Shape { fn radius(&self) -> f64; }
# struct UnitCircle;
# impl Shape for UnitCircle { fn area(&self) -> f64 { std::f64::consts::PI } }
# impl Circle for UnitCircle { fn radius(&self) -> f64 { 1.0 } }
# let circle = UnitCircle;
let circle = Box::new(circle) as Box<dyn Circle>;
let nonsense = circle.radius() * circle.area();
```

[bounds]: trait-bounds.html
[trait object]: types.html#trait-objects
[explicit]: expressions/operator-expr.html#type-cast-expressions
[RFC 255]: https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md
[associated items]: items/associated-items.html
[method]: items/associated-items.html#methods
[implementations]: items/implementations.html
[generics]: items/generics.html
[where clauses]: items/generics.html#where-clauses
[generic functions]: items/functions.html#generic-functions