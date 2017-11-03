# Implementations

An _implementation_ is an item that associates items with an *implementing type*.

There are two types of implementations: Bare implementations and [trait] 
implementations.

Implementations are defined with the keyword `impl`.

## Bare Implementations

A bare implementation is defined as the sequence of the `impl` keyword, generic
type declarations, a path to a nomial type, a where clause, and a bracketed
set of associable items.

The nominal type is called the *implementing type* and the associable items are
the *associated items* to the implementing type.

Bare implementations associate the associated items to the implementing type.

The associated item has a path of a path to the implementing type followed by
the associate item's path component.

Bare implementations cannot contain associated type aliases.

A type can have multiple bare implementations.

The implementing type must be defined within the same crate.

```rust
struct Point {x: i32, y: i32}

impl Point {
    fn log(&self) {
        println!("Point is at ({}, {})", self.x, self.y);
    }
}

let my_point = Point {x: 10, y:11};
my_point.log();
```

## Trait Implementations

A *trait implementation* is defined like a bare implementation except that
the optional generic type declarations is followed by a trait followed
by the keyword `for`. <!-- To understand this, you have to back-reference to
the previous section. :( -->

The trait is known as the *implemented trait*.

The implementing type implements the implemented trait.

A trait implementation must define all non-default associated items declared
by the implemented trait, may redefine default associated items defined by the 
implemented trait trait, and cannot define any other items.

The path to the associated items is `<` followed by a path to the implementing
type followed by `as` followed by a path to the trait followed by `>` as a path
component followed by the associated item's path component.

```rust
# #[derive(Copy, Clone)]
# struct Point {x: f64, y: f64};
# type Surface = i32;
# struct BoundingBox {x: f64, y: f64, width: f64, height: f64};
# trait Shape { fn draw(&self, Surface); fn bounding_box(&self) -> BoundingBox; }
# fn do_draw_circle(s: Surface, c: Circle) { }
struct Circle {
    radius: f64,
    center: Point,
}

impl Copy for Circle {}

impl Clone for Circle {
    fn clone(&self) -> Circle { *self }
}

impl Shape for Circle {
    fn draw(&self, s: Surface) { do_draw_circle(s, *self); }
    fn bounding_box(&self) -> BoundingBox {
        let r = self.radius;
        BoundingBox {
            x: self.center.x - r,
            y: self.center.y - r,
            width: 2.0 * r,
            height: 2.0 * r,
        }
    }
}
```

### Trait Implementation Coherence

A trait implementation is consider incoherent if either the orphan check fails
or there are overlapping implementation instaces. 

Two trait implementations overlap when there is a non-empty intersection of the
traits the implementation is for, the implementations can be instantiated with
the same type. <!-- This is probably wrong? Source: No two implementations can 
be instantiable with the same set of types for the input type parameters. -->

The `Orphan Check` states that every trait implementation must meet either of
the following conditions:

1. The trait being implemented is defined in the same crate.

2. At least one of either `Self` or a generic type parameter of the trait must
   meet the following grammar, where `C` is a nominal type defined
   within the containing crate:

   ```ignore
    T = C
      | &T
      | &mut T
      | Box<T>
   ```

## Generic Implementations

An implementation can take type and lifetime parameters, which can be used in
the rest of the implementation. Type parameters declared for an implementation
must be used at least once in either the trait or the implementing type of an
implementation. Implementation parameters are written directly after the `impl`
keyword.

```rust
# trait Seq<T> { fn dummy(&self, _: T) { } }
impl<T> Seq<T> for Vec<T> {
    /* ... */
}
impl Seq<bool> for u32 {
    /* Treat the integer as a sequence of bits */
}
```


[trait]: items/traits.html