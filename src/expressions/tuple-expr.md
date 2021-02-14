# Tuple and tuple indexing expressions

## Tuple expressions

> **<sup>Syntax</sup>**\
> _TupleExpression_ :\
> &nbsp;&nbsp; `(` [_InnerAttribute_]<sup>\*</sup> _TupleElements_<sup>?</sup> `)`
>
> _TupleElements_ :\
> &nbsp;&nbsp; ( [_Expression_] `,` )<sup>+</sup> [_Expression_]<sup>?</sup>

Tuple expressions evaluate into [tuple values][tuple type] with the operands initializing the elements of the tuple.

Tuple expressions are written by listing the [operands] in a parenthesized, comma-separated list. 1-ary tuple expressions require a comma after their operand to be disambiguated with a [parenthetical expression].

The number of operands is the arity of the constructed tuple.
Tuple expressions without operands produce the unit tuple.
For other tuple expressions, the first written operand initializes the 0th element and subsequent operands initializes the next highest element.
For example, in the tuple expression `('a', 'b', 'c')`, `'a'` initializes the value of the 0th element, `'b'` the 1st, and `'c'` the 2nd.

Examples of tuple expressions:

| Expression           | Type         |
| -------------------- | ------------ |
| `()`                 | `()` (unit)  |
| `(0.0, 4.5)`         | `(f64, f64)` |
| `("x".to_string(), )` | `(String, )`  |
| `("a", 4usize, true)`| `(&'static str, usize, bool)` |

### Tuple expression attributes

[Inner attributes] are allowed directly after the opening parenthesis of a tuple expression in the same expression contexts as [attributes on block expressions].

## Tuple indexing expressions

> **<sup>Syntax</sup>**\
> _TupleIndexingExpression_ :\
> &nbsp;&nbsp; [_Expression_] `.` [TUPLE_INDEX]

Tuple indexing expressions evaluate like [field access expressions], but access elements of [tuples][tuple type] or [tuple structs].

Tuple index expressions are written as an operand, `.`, and a tuple index.
The index must be written as a [decimal literal] with no leading zeros, underscores, or suffix.
The operand must have the type of a tuple or tuple struct.
If the tuple index is not an element of the tuple or tuple struct, it is a compiler error.

Examples of tuple indexing expressions:

```rust
let pair = ("a string", 2);
assert_eq!(pair.1, 2);

# struct Point(f32, f32);
let point = Point(1.0, 0.0);
assert_eq!(point.0, 1.0);
assert_eq!(point.1, 0.0);
```

> **Note**: Unlike field access expressions, tuple index expressions can be the function operand of a [call expression] as it cannot be confused with a method call since method names cannot be numbers.

[_Expression_]: ../expressions.md
[_InnerAttribute_]: ../attributes.md
[attributes on block expressions]: block-expr.md#attributes-on-block-expressions
[call expression]: ./call-expr.md
[decimal literal]: ../tokens.md#integer-literals
[field access expressions]: ./field-expr.html#field-access-expressions
[Inner attributes]: ../attributes.md
[operands]: ../expressions.md
[parenthetical expression]: grouped-expr.md
[tuple type]: ../types/tuple.md
[tuple structs]: ../types/struct.md
[TUPLE_INDEX]: ../tokens.md#tuple-index
