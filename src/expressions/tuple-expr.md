r[expr.tuple]
# Tuple and tuple indexing expressions

## Tuple expressions

r[expr.tuple.syntax]
```grammar,expressions
TupleExpression -> `(` TupleElements? `)`

TupleElements -> ( Expression `,` )+ Expression?
```

r[expr.tuple.result]
A *tuple expression* constructs [tuple values][tuple type].

r[expr.tuple.intro]
The syntax for tuple expressions is a parenthesized, comma separated list of expressions, called the *tuple initializer operands*.

r[expr.tuple.unary-tuple-restriction]
1-ary tuple expressions require a comma after their tuple initializer operand to be disambiguated with a [parenthetical expression].

r[expr.tuple.value]
Tuple expressions are a [value expression] that evaluate into a newly constructed value of a tuple type.

r[expr.tuple.type]
The number of tuple initializer operands is the arity of the constructed tuple.

r[expr.tuple.unit]
Tuple expressions without any tuple initializer operands produce the unit tuple.

r[expr.tuple.fields]
For other tuple expressions, the first written tuple initializer operand initializes the field `0` and subsequent operands initializes the next highest field.
For example, in the tuple expression `('a', 'b', 'c')`, `'a'` initializes the value of the field `0`, `'b'` field `1`, and `'c'` field `2`.

Examples of tuple expressions and their types:

| Expression           | Type         |
| -------------------- | ------------ |
| `()`                 | `()` (unit)  |
| `(0.0, 4.5)`         | `(f64, f64)` |
| `("x".to_string(), )` | `(String, )`  |
| `("a", 4usize, true)`| `(&'static str, usize, bool)` |

r[expr.tuple-index]
## Tuple indexing expressions

r[expr.tuple-index.syntax]
```grammar,expressions
TupleIndexingExpression -> Expression `.` TUPLE_INDEX
```

r[expr.tuple-index.intro]
A *tuple indexing expression* accesses fields of [tuples][tuple type] and [tuple structs][tuple struct].

The syntax for a tuple index expression is an expression, called the *tuple operand*, then a `.`, then finally a tuple index.

r[expr.tuple-index.index-syntax]
The syntax for the *tuple index* is a [decimal literal] with no leading zeros, underscores, or suffix.
For example `0` and `2` are valid tuple indices but not `01`, `0_`, nor `0i32`.

r[expr.tuple-index.required-type]
The type of the tuple operand must be a [tuple type] or a [tuple struct].

r[expr.tuple-index.index-name-operand]
The tuple index must be a name of a field of the type of the tuple operand.

r[expr.tuple-index.result]
Evaluation of tuple index expressions has no side effects beyond evaluation of its tuple operand.
As a [place expression], it evaluates to the location of the field of the tuple operand with the same name as the tuple index.

Examples of tuple indexing expressions:

```rust
// Indexing a tuple
let pair = ("a string", 2);
assert_eq!(pair.1, 2);

// Indexing a tuple struct
# struct Point(f32, f32);
let point = Point(1.0, 0.0);
assert_eq!(point.0, 1.0);
assert_eq!(point.1, 0.0);
```

> [!NOTE]
> Unlike field access expressions, tuple index expressions can be the function operand of a [call expression] as it cannot be confused with a method call since method names cannot be numbers.

> [!NOTE]
> Although arrays and slices also have elements, you must use an [array or slice indexing expression] or a [slice pattern] to access their elements.

[array or slice indexing expression]: array-expr.md#array-and-slice-indexing-expressions
[call expression]: ./call-expr.md
[decimal literal]: ../tokens.md#integer-literals
[field access expressions]: ./field-expr.html#field-access-expressions
[operands]: ../expressions.md
[parenthetical expression]: grouped-expr.md
[place expression]: ../expressions.md#place-expressions-and-value-expressions
[slice pattern]: ../patterns.md#slice-patterns
[tuple type]: ../types/tuple.md
[tuple struct]: ../types/struct.md
[value expression]: ../expressions.md#place-expressions-and-value-expressions
