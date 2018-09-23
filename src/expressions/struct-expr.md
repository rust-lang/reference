# Struct expressions

> **<sup>Syntax</sup>**\
> _StructExpression_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; _StructExprStruct_\
> &nbsp;&nbsp; | _StructExprTuple_\
> &nbsp;&nbsp; | _StructExprUnit_
>
> _StructExprStruct_ :\
> &nbsp;&nbsp; [_PathInExpression_] `{` (_StructExprFields_ | _StructBase_)<sup>?</sup> `}`
>
> _StructExprFields_ :\
> &nbsp;&nbsp; _StructExprField_ (`,` _StructExprField_)<sup>\*</sup> (`,` _StructBase_ | `,`<sup>?</sup>)
>
> _StructExprField_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; [IDENTIFIER]\
> &nbsp;&nbsp; | ([IDENTIFIER] | [TUPLE_INDEX]) `:` [_Expression_]
>
> _StructBase_ : `..` [_Expression_]
>
> _StructExprTuple_ :\
> &nbsp;&nbsp; [_PathInExpression_] `(`\
> &nbsp;&nbsp; &nbsp;&nbsp; ( [_Expression_] (`,` [_Expression_])<sup>\*</sup> `,`<sup>?</sup> )<sup>?</sup>\
> &nbsp;&nbsp; `)`
>
> _StructExprUnit_ : [_PathInExpression_]

A _struct expression_ creates a struct or union value. It consists of a path to a [struct]
or [union] item followed by the values for the fields of the item. There are three forms
of struct expressions: struct, tuple, and unit.

The following are examples of struct expressions:

```rust
# struct Point { x: f64, y: f64 }
# struct NothingInMe { }
# struct TuplePoint(f64, f64);
# mod game { pub struct User<'a> { pub name: &'a str, pub age: u32, pub score: usize } }
# struct Cookie; fn some_fn<T>(t: T) {}
Point {x: 10.0, y: 20.0};
NothingInMe {};
TuplePoint(10.0, 20.0);
TuplePoint { 0: 10.0, 1: 20.0 }; // Results in the same value as the above line
let u = game::User {name: "Joe", age: 35, score: 100_000};
some_fn::<Cookie>(Cookie);
```

## Field struct expression

A struct expression with fields enclosed in curly braces allows you to specify the value
for each individual field in any order. The field name is separated from its value with a
colon.

A value of a [union] type can also be created using this syntax, except that it must
specify exactly one field.

A struct expression can terminate with the syntax `..` followed by an
expression to denote a functional update. The expression following `..` (the
base) must have the same struct type as the new struct type being formed. The
entire expression denotes the result of constructing a new struct (with the
same type as the base expression) with the given values for the fields that
were explicitly specified and the values in the base expression for all other
fields. Just as with all struct expressions, all of the fields of the struct
must be [visible], even those not explicitly named.

```rust
# struct Point3d { x: i32, y: i32, z: i32 }
let base = Point3d {x: 1, y: 2, z: 3};
Point3d {y: 0, z: 10, .. base};
```

Struct expressions with curly braces can't be used directly in the head of a [loop] or an
[if], [if let] or [match] expression. However, struct expressions can be in used in these
situations if they are within another expression, for example inside
[parentheses].

The field names can be decimal integer values to specify indices for constructing tuple
structs. This can be used with base structs to fill out the remaining indices not
specified:

```rust
struct Color(u8, u8, u8);
let c1 = Color(0, 0, 0);  // Typical way of creating a tuple struct.
let c2 = Color{0: 255, 1: 127, 2: 0};  // Specifying fields by index.
let c3 = Color{1: 0, ..c2};  // Fill out all other fields using a base struct.
```

### Struct field init shorthand

When initializing a data structure (struct, enum, union) with named (but not
numbered) fields, it is allowed to write `fieldname` as a shorthand for
`fieldname: fieldname`. This allows a compact syntax with less duplication. Example:

```rust
# struct Point3d { x: i32, y: i32, z: i32 }
# let x = 0;
# let y_value = 0;
# let z = 0;
Point3d { x: x, y: y_value, z: z };
Point3d { x, y: y_value, z };
```

## Tuple struct expression

A struct expression with fields enclosed in parentheses constructs a tuple struct. Though
it is listed here as a specific expression for completeness, it is equivalent to a [call
expression] to the tuple struct's constructor. Example:

```rust
struct Position(i32, i32, i32);
Position(0, 0, 0);  // Typical way of creating a tuple struct.
let c = Position;  // `c` is a function that takes 3 arguments.
let pos = c(8, 6, 7);  // Creates a `Position` value.
```

## Unit struct expression

A unit struct expression is just the path to a unit struct item. This refers to the unit
struct's implicit constant of its value. The unit struct value can also be constructed
with fieldless struct expression. Example:

```rust
struct Gamma;
let a = Gamma;  // Gamma unit value.
let b = Gamma{};  // Exact same value as `a`.
```


[IDENTIFIER]: identifiers.html
[TUPLE_INDEX]: tokens.html#integer-literals
[_Expression_]: expressions.html
[_PathInExpression_]: paths.html#paths-in-expressions
[call expression]: expressions/call-expr.html
[if let]: expressions/if-expr.html#if-let-expressions
[if]: expressions/if-expr.html#if-expressions
[loop]: expressions/loop-expr.html
[match]: expressions/match-expr.html
[parentheses]: http://localhost:3000/expressions/grouped-expr.html
[path]: paths.html
[struct]: items/structs.html
[union]: items/unions.html
[visible]: visibility-and-privacy.html
