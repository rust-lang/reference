# Tuple types

> **<sup>Syntax</sup>**\
> _TupleType_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; `(` `)`\
> &nbsp;&nbsp; | `(` ( [_Type_] `,` )<sup>+</sup> [_Type_]<sup>?</sup> `)`

The *tuple type* is a structural type[^1] for heterogeneous lists of other
types. Each entry in the list is an *element*[^2] of the tuple. The position of
the element makes it the *nth element* using zero (`0`) as the initial index.

The number of elements determines the arity of the tuple. A tuple with `n`
elements is called an `n-ary tuple`. For example, a tuple with 2 elements is a
2-ary tuple.

For convenience and historical reasons, the tuple type with no elements (`()`)
is often called *unit* or *the unit type*. It's one value is also called *unit*
or *the unit value*.

Tuple types are written by listing the types of their elements in a
parenthesized, comma-separated list. 1-ary tuples require a comma after their
element type to be disambiguated with a [parenthesized type].

Some examples of tuple types:

* `()` (unit)
* `(f64, f64)`
* `(String, i32)`
* `(i32, String)` (different type from the previous example)
* `(i32, f64, Vec<String>, Option<bool>)`

Values of this type are constructed using a [tuple expression]. Furthermore,
various expressions will produce the unit value if there is no other meaningful
value for it to evaluate to. Tuple elements can be accessed by either a [tuple
index expression] or [pattern matching].

[^1]: Structural types are always equivalent if their internal types are
    equivalent. For a nominal version of tuples, see [tuple structs].
[^2]: Element is equivalent to field, except numerical indexes instead of
    identifiers

[_Type_]: ../types.md#type-expressions
[parenthesized type]: ../types.md#parenthesized-types
[pattern matching]: ../patterns.md#tuple-patterns
[tuple expression]: ../expressions/tuple-expr.md#tuple-expressions
[tuple index expression]: ../expressions/tuple-expr.md#tuple-indexing-expressions
[tuple structs]: ./struct.md
