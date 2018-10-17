# Tuple and tuple indexing expressions

## Tuple expressions

> **<sup>Syntax</sup>**\
> _TupleExpression_ :\
> &nbsp;&nbsp; `(` [_InnerAttribute_]<sup>\*</sup> _TupleElements_<sup>?</sup> `)`
>
> _TupleElements_ :\
> &nbsp;&nbsp; ( [_Expression_] `,` )<sup>+</sup> [_Expression_]<sup>?</sup>

Tuples are written by enclosing zero or more comma-separated expressions in
parentheses. They are used to create [tuple-typed](types/tuple.html)
values.

```rust
(0.0, 4.5);
("a", 4usize, true);
();
```

You can disambiguate a single-element tuple from a value in parentheses with a
comma:

```rust
(0,); // single-element tuple
(0); // zero in parentheses
```

### Tuple expression attributes

[Inner attributes] are allowed directly after the opening parenthesis of a
tuple expression in the same expression contexts as [attributes on block
expressions].

## Tuple indexing expressions

> **<sup>Syntax</sup>**\
> _TupleIndexingExpression_ :\
> &nbsp;&nbsp; [_Expression_] `.` [TUPLE_INDEX]

[Tuples](types/tuple.html) and [struct tuples](items/structs.html) can be
indexed using the number corresponding to the position of the field. The index
must be written as a [decimal literal](tokens.html#integer-literals) with no
underscores or suffix. Tuple indexing expressions also differ from field
expressions in that they can unambiguously be called as a function. In all
other aspects they have the same behavior.

```rust
# struct Point(f32, f32);
let pair = (1, 2);
assert_eq!(pair.1, 2);
let unit_x = Point(1.0, 0.0);
assert_eq!(unit_x.0, 1.0);
```

[Inner attributes]: attributes.html
[TUPLE_INDEX]: tokens.html#integer-literals
[_Expression_]: expressions.html
[_InnerAttribute_]: attributes.html
[attributes on block expressions]: expressions/block-expr.html#attributes-on-block-expressions
