# Implementations

An _implementation_ is an item that associates items with an _implementing type_.
Implementations are defined with the keyword `impl` and contain functions
that belong to an instance of the type that is being implemented or to the
type statically.

There are two types of implementations:

- inherent implementations
- [trait] implementations

## Inherent Implementations

An inherent implementation is defined as the sequence of the `impl` keyword,
generic type declarations, a path to a nominal type, a where clause, and a
bracketed set of associable items.

The nominal type is called the _implementing type_ and the associable items are
the _associated items_ to the implementing type.

Inherent implementations associate the contained items to the implementing type.
The associated item has a path of a path to the implementing type followed by
the associate item's path component. Inherent implementations cannot contain
associated type aliases.

A type can also have multiple inherent implementations. Although the
implementing type must be defined within the same crate.

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

A _trait implementation_ is defined like an inherent implementation except that
the optional generic type declarations is followed by a [trait] followed
by the keyword `for`. Followed by a path to a nominal type.
<!-- To understand this, you have to back-reference to the previous section. :( -->

The trait is known as the _implemented trait_. The implementing type
implements the implemented trait.

A trait implementation must define all non-default associated items declared
by the implemented trait, may redefine default associated items defined by the
implemented trait, and cannot define any other items.

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

A trait implementation is considered incoherent if either the orphan check fails
or there are overlapping implementation instances.

Two trait implementations overlap when there is a non-empty intersection of the
traits the implementation is for, the implementations can be instantiated with
the same type. <!-- This is probably wrong? Source: No two implementations can
be instantiable with the same set of types for the input type parameters. -->

The `Orphan Check` states that every trait implementation must meet either of
the following conditions:

1.  The trait being implemented is defined in the same crate.

2.  At least one of either `Self` or a generic type parameter of the trait must
    meet the following grammar, where `C` is a nominal type defined
    within the containing crate:

    ```ignore
     T = C
       | &C
       | &mut C
       | Box<C>
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

## Attributes on Implementations

Implementations may contain outer [attributes] before the `impl` keyword and
inner [attributes] inside the brackets that contain the associated items. Inner
attributes must come before any associated items. That attributes that have
meaning here are [`cfg`], [`deprecated`], [`doc`], and [the lint check
attributes].

[trait]: items/traits.html
[attributes]: attributes.html
[`cfg`]: attributes.html#conditional-compilation
[`deprecated`]: attributes.html#deprecation
[`doc`]: attributes.html#documentation
[the lint check attributes]: attributes.html#lint-check-attributes
