r[expr.struct]
# Struct expressions

r[expr.struct.syntax]
```grammar,expressions
StructExpression ->
    PathInExpression `{` (StructExprFields | StructBase)? `}`

StructExprFields ->
    StructExprField (`,` StructExprField)* (`,` StructBase | `,`?)

StructExprField ->
    OuterAttribute*
    (
        IDENTIFIER
      | (IDENTIFIER | TUPLE_INDEX) `:` Expression
    )

StructBase -> `..` Expression
```

r[expr.struct.intro]
A *struct expression* creates a struct, enum, or union value.
It consists of a path to a [struct], [enum variant], or [union] item followed by the values for the fields of the item.

The following are examples of struct expressions:

```rust
# struct Point { x: f64, y: f64 }
# struct NothingInMe { }
# mod game { pub struct User<'a> { pub name: &'a str, pub age: u32, pub score: usize } }
# enum Enum { Variant {} }
Point {x: 10.0, y: 20.0};
NothingInMe {};
let u = game::User {name: "Joe", age: 35, score: 100_000};
Enum::Variant {};
```

> [!NOTE]
> Tuple structs and tuple enum variants are typically instantiated using a [call expression][expr.call] referring to the [constructor in the value namespace][items.struct.tuple]. These are distinct from a struct expression using curly braces referring to the constructor in the type namespace.
>
> ```rust
> struct Position(i32, i32, i32);
> Position(0, 0, 0);  // Typical way of creating a tuple struct.
> let c = Position;  // `c` is a function that takes 3 arguments.
> let pos = c(8, 6, 7);  // Creates a `Position` value.
>
> enum Version { Triple(i32, i32, i32) };
> Version::Triple(0, 0, 0);
> let f = Version::Triple;
> let ver = f(8, 6, 7);
> ```
>
> The last segment of the call path cannot refer to a type alias:
>
> ```rust
> trait Tr { type T; }
> impl<T> Tr for T { type T = T; }
>
> struct Tuple();
> enum Enum { Tuple() }
>
> // <Unit as Tr>::T(); // causes an error -- `::T` is a type, not a value
> <Enum as Tr>::T::Tuple(); // OK
> ```
>
> ----
>
> Unit structs and unit enum variants are typically instantiated using a [path expression][expr.path] referring to the [constant in the value namespace][items.struct.unit].
>
> ```rust
> struct Gamma;
> // Gamma unit value, referring to the const in the value namespace.
> let a = Gamma;
> // Exact same value as `a`, but constructed using a struct expression
> // referring to the type namespace.
> let b = Gamma {};
>
> enum ColorSpace { Oklch }
> let c = ColorSpace::Oklch;
> let d = ColorSpace::Oklch {};
> ```

r[expr.struct.field]
## Field struct expression

r[expr.struct.field.intro]
A struct expression with fields enclosed in curly braces allows you to specify the value for each individual field in any order.
The field name is separated from its value with a colon.

r[expr.struct.field.union-constraint]
A value of a [union] type can only be created using this syntax, and it must specify exactly one field.

r[expr.struct.update]
## Functional update syntax

r[expr.struct.update.intro]
A struct expression that constructs a value of a struct type can terminate with the syntax `..` followed by an expression to denote a functional update.

r[expr.struct.update.base-same-type]
The expression following `..` (the base) must have the same struct type as the new struct type being formed.

r[expr.struct.update.fields]
The entire expression uses the given values for the fields that were specified and moves or copies the remaining fields from the base expression.

r[expr.struct.update.visibility-constraint]
As with all struct expressions, all of the fields of the struct must be [visible], even those not explicitly named.

```rust
# struct Point3d { x: i32, y: i32, z: i32 }
let mut base = Point3d {x: 1, y: 2, z: 3};
let y_ref = &mut base.y;
Point3d {y: 0, z: 10, .. base}; // OK, only base.x is accessed
drop(y_ref);
```

r[expr.struct.brace-restricted-positions]
Struct expressions can't be used directly in a [loop] or [if] expression's head, or in the [scrutinee] of an [if let] or [match] expression.
However, struct expressions can be used in these situations if they are within another expression, for example inside [parentheses].

r[expr.struct.tuple-field]
The field names can be decimal integer values to specify indices for constructing tuple structs.
This can be used with base structs to fill out the remaining indices not specified:

```rust
struct Color(u8, u8, u8);
let c1 = Color(0, 0, 0);  // Typical way of creating a tuple struct.
let c2 = Color{0: 255, 1: 127, 2: 0};  // Specifying fields by index.
let c3 = Color{1: 0, ..c2};  // Fill out all other fields using a base struct.
```

r[expr.struct.field.named]
### Struct field init shorthand

When initializing a data structure (struct, enum, union) with named (but not numbered) fields, it is allowed to write `fieldname` as a shorthand for `fieldname: fieldname`.
This allows a compact syntax with less duplication.
For example:

```rust
# struct Point3d { x: i32, y: i32, z: i32 }
# let x = 0;
# let y_value = 0;
# let z = 0;
Point3d { x: x, y: y_value, z: z };
Point3d { x, y: y_value, z };
```

[enum variant]: ../items/enumerations.md
[if let]: if-expr.md#if-let-patterns
[if]: if-expr.md#if-expressions
[loop]: loop-expr.md
[match]: match-expr.md
[parentheses]: grouped-expr.md
[struct]: ../items/structs.md
[union]: ../items/unions.md
[visible]: ../visibility-and-privacy.md
[scrutinee]: ../glossary.md#scrutinee
