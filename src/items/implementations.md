# Implementations

> **<sup>Syntax</sup>**\
> _Implementation_ :\
> &nbsp;&nbsp; _InherentImpl_ | _TraitImpl_
>
> _InherentImpl_ :\
> &nbsp;&nbsp; `impl` [_Generics_]<sup>?</sup>&nbsp;[_Type_]&nbsp;[_WhereClause_]<sup>?</sup> `{`\
> &nbsp;&nbsp; &nbsp;&nbsp; [_InnerAttribute_]<sup>\*</sup>\
> &nbsp;&nbsp; &nbsp;&nbsp; _InherentImplItem_<sup>\*</sup>\
> &nbsp;&nbsp; `}`
>
> _InherentImplItem_ :\
> &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> (\
> &nbsp;&nbsp; &nbsp;&nbsp; &nbsp;&nbsp; [_MacroInvocationSemi_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | ( [_Visibility_]<sup>?</sup> ( [_ConstantItem_] | [_Function_] | [_Method_] ) )\
> &nbsp;&nbsp; )
>
> _TraitImpl_ :\
> &nbsp;&nbsp; `unsafe`<sup>?</sup> `impl` [_Generics_]<sup>?</sup> `!`<sup>?</sup>
>              [_TypePath_] `for` [_Type_]\
> &nbsp;&nbsp; [_WhereClause_]<sup>?</sup>\
> &nbsp;&nbsp; `{`\
> &nbsp;&nbsp; &nbsp;&nbsp; [_InnerAttribute_]<sup>\*</sup>\
> &nbsp;&nbsp; &nbsp;&nbsp; _TraitImplItem_<sup>\*</sup>\
> &nbsp;&nbsp; `}`
>
> _TraitImplItem_ :\
> &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> (\
> &nbsp;&nbsp; &nbsp;&nbsp; &nbsp;&nbsp; [_MacroInvocationSemi_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | ( [_Visibility_]<sup>?</sup> ( [_TypeAlias_] | [_ConstantItem_] | [_Function_] | [_Method_] ) )\
> &nbsp;&nbsp; )

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

Inherent implementations associate the contained items to the
implementing type.  Inherent implementations can contain [associated
functions] (including methods) and [associated constants]. They cannot
contain associated type aliases.

The [path] to an associated item is any path to the implementing type,
followed by the associated item's identifier as the final path
component.

A type can also have multiple inherent implementations. An implementing type
must be defined within the same crate as the original type definition.

``` rust
pub mod color {
    pub struct Color(pub u8, pub u8, pub u8);

    impl Color {
        pub const WHITE: Color = Color(255, 255, 255);
    }
}

mod values {
    use super::color::Color;
    impl Color {
        pub fn red() -> Color {
            Color(255, 0, 0)
        }
    }
}

pub use self::color::Color;
fn main() {
    // Actual path to the implementing type and impl in the same module.
    color::Color::WHITE;

    // Impl blocks in different modules are still accessed through a path to the type.
    color::Color::red();

    // Re-exported paths to the implementing type also work.
    Color::red();

    // Does not work, because use in `values` is not pub.
    // values::Color::red();
}
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

[Unsafe traits] require the trait implementation to begin with the `unsafe`
keyword.

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

[_ConstantItem_]: constant-items.md
[_Function_]: functions.md
[_Generics_]: generics.md
[_InnerAttribute_]: ../attributes.md
[_MacroInvocationSemi_]: ../macros.md#macro-invocation
[_Method_]: associated-items.md#methods
[_OuterAttribute_]: ../attributes.md
[_TypeAlias_]: type-aliases.md
[_TypePath_]: ../paths.md#paths-in-types
[_Type_]: ../types.md#type-expressions
[_Visibility_]: ../visibility-and-privacy.md
[_WhereClause_]: generics.md#where-clauses
[trait]: traits.md
[associated functions]: associated-items.md#associated-functions-and-methods
[associated constants]: associated-items.md#associated-constants
[attributes]: ../attributes.md
[`cfg`]: ../conditional-compilation.md
[`deprecated`]: ../attributes/diagnostics.md#the-deprecated-attribute
[`doc`]: ../../rustdoc/the-doc-attribute.html
[path]: ../paths.md
[the lint check attributes]: ../attributes/diagnostics.md#lint-check-attributes
[Unsafe traits]: traits.md#unsafe-traits
