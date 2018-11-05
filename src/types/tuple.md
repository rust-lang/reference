# Tuple types

> **<sup>Syntax</sup>**\
> _TupleType_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; `(` `)`\
> &nbsp;&nbsp; | `(` ( [_Type_] `,` )<sup>+</sup> [_Type_]<sup>?</sup> `)`

A tuple *type* is a heterogeneous product of other types, called the *elements*
of the tuple. It has no nominal name and is instead structurally typed.

Tuple types and values are denoted by listing the types or values of their
elements, respectively, in a parenthesized, comma-separated list.

Because tuple elements don't have a name, they can only be accessed by
pattern-matching or by using `N` directly as a field to access the `N`th
element.

An example of a tuple type and its use:

```rust
type Pair<'a> = (i32, &'a str);
let p: Pair<'static> = (10, "ten");
let (a, b) = p;

assert_eq!(a, 10);
assert_eq!(b, "ten");
assert_eq!(p.0, 10);
assert_eq!(p.1, "ten");
```

For historical reasons and convenience, the tuple type with no elements (`()`)
is often called ‘unit’ or ‘the unit type’.

[_Type_]: types.html#type-expressions
