# Struct expressions

There are several forms of struct expressions. A _struct expression_ consists
of the [path](paths.html) of a [struct item](items/structs.html), followed by a
brace-enclosed list of zero or more comma-separated name-value pairs, providing
the field values of a new instance of the struct. A field name can be any
[identifier](identifiers.html), and is separated from its value expression by a
colon. In the case of a tuple struct the field names are numbers corresponding
to the position of the field. The numbers must be written in decimal,
containing no underscores and with no leading zeros or integer suffix. A value
of a [union](items/unions.html) type can also be created using this syntax,
except that it must specify exactly one field.

Struct expressions can't be used directly in the head of a [loop] 
or an [if], [if let] or [match] expression. But struct expressions can still be
in used inside parentheses, for example.

[loop]: expressions/loop-expr.html
[if]: expressions/if-expr.html#if-expressions
[if let]: expressions/if-expr.html#if-let-expressions
[match]: expressions/match-expr.html

A _tuple struct expression_ consists of the path of a struct item, followed by
a parenthesized list of one or more comma-separated expressions (in other
words, the path of a struct item followed by a tuple expression). The struct
item must be a tuple struct item.

A _unit-like struct expression_ consists only of the path of a struct item.

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

A struct expression forms a new value of the named struct type. Note that for a
given *unit-like* struct type, this will always be the same value.

A struct expression can terminate with the syntax `..` followed by an
expression to denote a functional update. The expression following `..` (the
base) must have the same struct type as the new struct type being formed. The
entire expression denotes the result of constructing a new struct (with the
same type as the base expression) with the given values for the fields that
were explicitly specified and the values in the base expression for all other
fields. Just as with all struct expressions, all of the fields of the struct
must be [visible](visibility-and-privacy.html), even those not explicitly
named.

```rust
# struct Point3d { x: i32, y: i32, z: i32 }
let base = Point3d {x: 1, y: 2, z: 3};
Point3d {y: 0, z: 10, .. base};
```

## Struct field init shorthand

When initializing a data structure (struct, enum, union) with named (but not
numbered) fields, it is allowed to write `fieldname` as a shorthand for
`fieldname: fieldname`. This allows a compact syntax with less duplication.

Example:

```rust
# struct Point3d { x: i32, y: i32, z: i32 }
# let x = 0;
# let y_value = 0;
# let z = 0;
Point3d { x: x, y: y_value, z: z };
Point3d { x, y: y_value, z };
```
