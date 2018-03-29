# Patterns

> **<sup>Syntax</sup>**  
> _Pattern_ :  
> &nbsp;&nbsp; &nbsp;&nbsp; [_LiteralPattern_]  
> &nbsp;&nbsp; | [_WildcardPattern_]  
> &nbsp;&nbsp; | [_RangePattern_]  
> &nbsp;&nbsp; | [_ReferencePattern_]  
> &nbsp;&nbsp; | [_IdentifierPattern_]  
> &nbsp;&nbsp; | [_StructPattern_]  
> &nbsp;&nbsp; | [_TupleStructPattern_]  
> &nbsp;&nbsp; | [_TuplePattern_]  
> &nbsp;&nbsp; | [_SlicePattern_]  
> &nbsp;&nbsp; | [_PathPattern_]  

Patterns in Rust are used to match values against structures and to,
optionally, bind variables to values inside these structures. They are also
used in variable declarations and function/closure parameters, though in these
cases most of the time they are simply used as an identifier that binds to a
value.

For example, the pattern used in:

```rust
# struct Car;
# struct Computer;
# struct Person {
#     name: String,
#     car: Option<Car>,
#     computer: Option<Computer>,
#     age: u8,
# }
# let person = Person {
#     name: String::from("John"),
#     car: Some(Car),
#     computer: None,
#     age: 15,
# };
if let
    Person {
        car: Some(_),
        age: person_age @ 13...19,
        name: ref person_name,
        ..
    } = person
{
    println!("{} has a car and is {} years old.", person_name, person_age);
}
```
does four things:

* Tests if `person` has the `car` field filled with something.
* Tests if the person's `age` field is between 13 and 19, and binds its value to
  the `person_age` variable.
* Binds a reference to the `name` field to the variable `person_name`.
* Ignores the rest of the fields of `person`, i.e., they can have any value and
  are not bound to any variables.

Patterns are used in:

* [`let` declarations](statements.html#let-statements)
* [Function](items.html#functions) and [closure](expressions.html#closure-expressions)
  parameters
* [`match` expressions](expressions.html#match-expressions)
* [`if let` expressions](expressions.html#if-let-expressions)
* [`while let` expressions](expressions.html#while-let-loops)
* Inside other patterns

## Destructuring

Patterns can be used to *destructure* structs, enums, and tuples. Destructuring
breaks a value up into its component pieces. The syntax used is almost the same as
when creating such values. When destructing a data structure with named (but
not numbered) fields, it is allowed to write `fieldname` as a shorthand for
`fieldname: fieldname`. In a pattern whose head expression has a `struct`,
`enum` or `tupl` type, a placeholder (`_`) stands for a *single* data field,
whereas a wildcard `..` stands for *all* the remaining fields of a particular variant.

```rust
# enum Message {
#     Quit,
#     WriteString(String),
#     Move { x: i32, y: i32 },
#     ChangeColor(u8, u8, u8),
# }
# let message = Message::Quit;
match message {
    Message::Quit => println!("Quit"),
    Message::WriteString(write) => println!("{}", &write),
    Message::Move{ x, y: 0 } => println!("move {} horizontally", x),
    Message::Move{ .. } => println!("other move"),
    Message::ChangeColor { 0: red, 1: green, 2: _ } => {
        println!("color change, red: {}, green: {}", red, green);
    }
};
```

## Refutability

A pattern is said to be *Refutable* when it **has the possibily of not being matched**
by the value it is being matched against. *Irrefutable* patterns, on the other hand,
always match the value they are being matched against. Examples:

```rust
let (x, y) = (1, 2);               // "(x, y)" is an irrefutable pattern

if let (a, 3) = (1, 2) {           // "(a, 3)" is refutable, and will not match
    panic!("Shouldn't reach here");
} else if let (a, 4) = (3, 4) {    // "(a, 4)" is refutable, and will match
    println!("Matched ({}, 4)", a);
}
```

## Literal patterns

> **<sup>Syntax</sup>**  
> _LiteralPattern_ :  
> &nbsp;&nbsp; &nbsp;&nbsp; [BOOLEAN_LITERAL]  
> &nbsp;&nbsp; | [CHAR_LITERAL]  
> &nbsp;&nbsp; | [BYTE_LITERAL]  
> &nbsp;&nbsp; | [STRING_LITERAL]  
> &nbsp;&nbsp; | [RAW_STRING_LITERAL]  
> &nbsp;&nbsp; | [BYTE_STRING_LITERAL]  
> &nbsp;&nbsp; | [RAW_BYTE_STRING_LITERAL]  
> &nbsp;&nbsp; | `-`<sup>?</sup> [INTEGER_LITERAL]  
> &nbsp;&nbsp; | `-`<sup>?</sup> [FLOAT_LITERAL]  

[BOOLEAN_LITERAL]: tokens.html#boolean-literals
[CHAR_LITERAL]: tokens.html#character-literals
[BYTE_LITERAL]: tokens.html#byte-literals
[STRING_LITERAL]: tokens.html#string-literals
[RAW_STRING_LITERAL]: tokens.html#raw-string-literals
[BYTE_STRING_LITERAL]: tokens.html#byte-string-literals
[RAW_BYTE_STRING_LITERAL]: tokens.html#raw-byte-string-literals
[INTEGER_LITERAL]: tokens.html#integer-literals
[FLOAT_LITERAL]: tokens.html#floating-point-literals

_Literal patterns_ match exactly the value they represent. Since negative numbers are
not literals in Rust, literal patterns also accept an optional minus sign before the
literal.

Floating-point literals are currently accepted, but due to the complexity of comparing
them, they are going to be forbidden on literal patterns in a future version of Rust (see
[issue #41620](https://github.com/rust-lang/rust/issues/41620)).

Literal patterns are always refutable.

Examples:

```rust
for i in -2..5 {
    match i {
        -1 => println!("It's minus one"),
        1 => println!("It's a one"),
        2|4 => println!("It's either a two or a four"),
        _ => println!("Matched none of the arms"),
    }
}
```

## Wildcard pattern

> **<sup>Syntax</sup>**  
> _WildcardPattern_ :  
> &nbsp;&nbsp; `_`

The _wildcard pattern_ matches any value. It is used to ignore values when they don't
matter. Inside other patterns it matches a single data field (as opposed to the `..`
which matches the remaining fields).

Examples: 

```rust
# let x = 20;
let (a, _) = (10, x);   // the x is always matched by _
# assert_eq!(a, 10);

// ignore a function/closure param
let real_part = |a: f64, _: f64| { a };

// ignore a field from a struct
# struct RGBA {
#    r: f32,
#    g: f32,
#    b: f32,
#    a: f32,
# }
# let color = RGBA{r: 0.4, g: 0.1, b: 0.9, a: 0.5};
let RGBA{r: red, g: green, b: blue, a: _} = color;
# assert_eq!(color.r, red);
# assert_eq!(color.g, green);
# assert_eq!(color.b, blue);

// accept any Some, with any value
# let x = Some(10);
if let Some(_) = x {}
```

The wildcard pattern is always irrefutable.

## Range patterns

> **<sup>Syntax</sup>**  
> _RangePattern_ :  
> &nbsp;&nbsp;  _RangePatternBound_ `...` _RangePatternBound_  
>  
> _RangePatternBound_ :  
> &nbsp;&nbsp; &nbsp;&nbsp; [CHAR_LITERAL]  
> &nbsp;&nbsp; | [BYTE_LITERAL]  
> &nbsp;&nbsp; | `-`<sup>?</sup> [INTEGER_LITERAL]  
> &nbsp;&nbsp; | `-`<sup>?</sup> [FLOAT_LITERAL]  
> &nbsp;&nbsp; | [_PathInExpression_]  
> &nbsp;&nbsp; | [_QualifiedPathInExpression_]  

[_PathInExpression_]: paths.html
[_QualifiedPathInExpression_]: paths.html

Range patterns match values that are within the closed range defined by its lower and
upper bounds. For example, a pattern `'m'...'p'` will match only the values `'m'`, `'n'`,
`'o'`, and `'p'`. The bounds can be literals or paths that point to constant values.

A pattern a `...` b must always have a &le; b. Thus, it is not possible to have a range
pattern `10...0`, for example.

Range patterns only work on scalar types. The accepted types are:

* Integer types (u8, i8, u16, i16, usize, isize, etc.).
* Character types (char).
* Floating point types (f32 and f64). This is being deprecated and will not be available
  in a future version of Rust (see
  [issue #41620](https://github.com/rust-lang/rust/issues/41620)).

Examples:

```rust
# let c = 'f';
let valid_variable = match c {
    'a'...'z' => true,
    'A'...'Z' => true,
    'α'...'ω' => true,
    _ => false,
};

# let ph = 10;
println!("{}", match ph {
    0...6 => "acid",
    7 => "neutral",
    8...14 => "base",
    _ => unreachable!(),
});

// using paths to constants:
# const TROPOSPHERE_MIN : u8 = 6;
# const TROPOSPHERE_MAX : u8 = 20;
# 
# const STRATOSPHERE_MIN : u8 = TROPOSPHERE_MAX + 1;
# const STRATOSPHERE_MAX : u8 = 50;
# 
# const MESOSPHERE_MIN : u8 = STRATOSPHERE_MAX + 1;
# const MESOSPHERE_MAX : u8 = 85;
# 
# let altitude = 70;
# 
println!("{}", match altitude {
    TROPOSPHERE_MIN...TROPOSPHERE_MAX => "troposphere",
    STRATOSPHERE_MIN...STRATOSPHERE_MAX => "stratosphere",
    MESOSPHERE_MIN...MESOSPHERE_MAX => "mesosphere",
    _ => "outer space, maybe",
});

# pub mod binary {
#     pub const MEGA : u64 = 1024*1024;
#     pub const GIGA : u64 = 1024*1024*1024;
# }
# let n_items = 20_832_425;
# let bytes_per_item = 12;
if let size @ binary::MEGA...binary::GIGA = n_items * bytes_per_item {
    println!("It fits and occupies {} bytes", size);
}

# trait MaxValue {
#     const MAX: u64;
# }
# impl MaxValue for u8 {
#     const MAX: u64 = (1 << 8) - 1;
# }
# impl MaxValue for u16 {
#     const MAX: u64 = (1 << 16) - 1;
# }
# impl MaxValue for u32 {
#     const MAX: u64 = (1 << 32) - 1;
# }
// using qualified paths:
println!("{}", match 0xfacade {
    0 ... <u8 as MaxValue>::MAX => "fits in a u8",
    0 ... <u16 as MaxValue>::MAX => "fits in a u16",
    0 ... <u32 as MaxValue>::MAX => "fits in a u32",
    _ => "too big",
});

```

Range patterns are always refutable, even when they cover the complete set
of possible values of a type. For example, `0u8...255u8` is refutable even though
it covers all possible values of `u8`.

## Reference patterns

> **<sup>Syntax</sup>**  
> _ReferencePattern_ :  
> &nbsp;&nbsp; (`&`|`&&`) `mut`<sup>?</sup> _Pattern_  

Reference patterns dereference the pointers that are being matched
and, thus, borrow them.

For example, these two matches on `x: &i32` are equivalent:

```rust
# let x = &3;
let y = match *x { 0 => "zero", _ => "some" };
let z = match x { &0 => "zero", _ => "some" };

assert_eq!(y, z);
```

The grammar production for reference patterns has to match the token `&&`
because it is a token by itself, not two `&` tokens.

Reference patterns are always irrefutable.

## Identifier patterns

> **<sup>Syntax</sup>**  
> _IdentifierPattern_ :  
> &nbsp;&nbsp; &nbsp;&nbsp; `mut`<sup>?</sup> IDENTIFIER (`@` [_Pattern_] ) <sup>?</sup>  
> &nbsp;&nbsp; | `ref` `mut`<sup>?</sup> IDENTIFIER (`@` [_Pattern_] ) <sup>?</sup>

_Identifier patterns_ bind the value they match to a **previously undeclared** variable.

Patterns that consist of only an identifier, possibly with a `mut`, like
`variable`, `x`, and `y` below:

```rust
let mut variable = 10;
fn sum(x: i32, y: i32) -> i32 {
#    x + y
# }
```

match any value and bind it to that identifier. This is the most commonly
used pattern in variable declarations and function/closure parameters.

To bind non-trivial patterns to a variable, the use of the syntax `variable @
subpattern` is needed. For example:

```rust
let x = 2;

match x {
    e @ 1 ... 5 => println!("got a range element {}", e),
    _ => println!("anything"),
}
```

binds to `e` the value 2 (not the entire range: the range here is a range subpattern).

By default, identifier patterns bind a variable to a copy of or move from the
matched value (depending whether the matched value implements the [Copy trait]).
This can be changed to bind to a reference by using the `ref` keyword,
or to a mutable reference using `ref mut`. For example:

```rust
# let a = Some(10);
match a {
    None => (),
    Some(value) => (),
}

match a {
    None => (),
    Some(ref value) => (),
}
```

in the first match expression, the value is copied (or moved). In the second match,
a reference to the same memory location is bound to the variable value. This syntax is
needed because in destructuring subpatterns we can't apply the `&` operator to
the value's fields. For example:

```rust,compile_fail
# struct Person {
#    name: String,
#    age: u8,
# }
# let value = Person{ name: String::from("John"), age: 23 };
if let Person{& name: person_name, age: 18...150} = value { }
```

is not valid. What we must do is:

```rust
# struct Person {
#    name: String,
#    age: u8,
# }
# let value = Person{ name: String::from("John"), age: 23 };
if let Person{name: ref person_name, age: 18...150} = value { }
```

Thus, `ref` is not something that is being matched against. Its objective is
exclusively to make the matched binding a reference, instead of potentially
copying or moving what was matched.

## Struct patterns

> **<sup>Syntax</sup>**  
> _StructPattern_ :  
> &nbsp;&nbsp; _Path_ `{`  
> &nbsp;&nbsp; &nbsp;&nbsp; _StructPatternElements_ <sup>?</sup>  
> &nbsp;&nbsp; `}`  
>  
> _StructPatternElements_ :  
> &nbsp;&nbsp; &nbsp;&nbsp; _StructPatternFields_ (`,` | `,` _StructPatternEtCetera_)<sup>?</sup>  
> &nbsp;&nbsp; | _StructPatternEtCetera_  
>  
> _StructPatternFields_ :  
> &nbsp;&nbsp; _StructPatternField_ (`,` _StructPatternField_) <sup>\*</sup>  
>  
> _StructPatternField_ :  
> &nbsp;&nbsp; _OuterAttribute_ <sup>\*</sup>  
> &nbsp;&nbsp; (  
> &nbsp;&nbsp; &nbsp;&nbsp; &nbsp;&nbsp; INTEGER_LITERAL `:` [_Pattern_]  
> &nbsp;&nbsp; &nbsp;&nbsp; | IDENTIFIER `:` [_Pattern_]  
> &nbsp;&nbsp; &nbsp;&nbsp; | `ref`<sup>?</sup> `mut`<sup>?</sup> IDENTIFIER  
> &nbsp;&nbsp; )  
>  
> _StructPatternEtCetera_ :  
> &nbsp;&nbsp; _OuterAttribute_ <sup>\*</sup>  
> &nbsp;&nbsp; `..`  

Struct patterns match struct values that match all criteria defined by its subpatterns.
They are also used to [destructure](destructuring) a struct.

On a struct pattern, the fields are referenced by name, index (in the case of tuples
structs) or ignored by use of `..`:

```rust
# struct Point {
#     x: u32,
#     y: u32,
# }
# let s = Point {x: 1, y: 1};
# 
match s {
    Point {x: 10, y: 20} => (),
    Point {y: 10, x: 20} => (),    // order doesn't matter
    Point {x: 10, ..} => (),
    Point {..} => (),
}

# struct PointTuple (
#     u32,
#     u32,
# );
# let t = PointTuple(1, 2);
# 
match t {
    PointTuple {0: 10, 1: 20} => (),
    PointTuple {1: 10, 0: 20} => (),   // order doesn't matter
    PointTuple {0: 10, ..} => (),
    PointTuple {..} => (),
}
```

If `..` is not used, it is required to match all fields:

```rust
# struct Struct {
#    a: i32,
#    b: char,
#    c: bool,
# }
# let mut struct_value = Struct{a: 10, b: 'X', c: false};
# 
match struct_value {
    Struct{a: 10, b: 'X', c: false} => (),
    Struct{a: 10, b: 'X', ref c} => (),
    Struct{a: 10, b: 'X', ref mut c} => (),
    Struct{a: 10, b: 'X', c: _} => (),
    Struct{a: _, b: _, c: _} => (),
}
```

The `ref` and/or `mut` _IDENTIFIER_ syntax matches any value and binds it to
a variable with the same name as the given field. 

```rust
# struct Struct {
#    a: i32,
#    b: char,
#    c: bool,
# }
# let struct_value = Struct{a: 10, b: 'X', c: false};
# 
let Struct{a: x, b: y, c: z} = struct_value;          // destructure all fields
```

A struct pattern is refutable when one of its subpatterns is refutable.

## TupleStruct patterns

> **<sup>Syntax</sup>**  
> _TupleStructPattern_ :  
> &nbsp;&nbsp; _Path_ `(` _TupleStructItems_ `)`  
>  
> _TupleStructItems_ :  
> &nbsp;&nbsp; &nbsp;&nbsp; [_Pattern_]&nbsp;( `,` [_Pattern_] )<sup>\*</sup> `,`<sup>?</sup>  
> &nbsp;&nbsp; | ([_Pattern_] `,`)<sup>\*</sup> `..` ( (`,` [_Pattern_])<sup>+</sup> `,`<sup>?</sup> )<sup>?</sup>  

TupleStruct patterns match tuple struct and enum values that match all criteria defined
by its subpatterns. They are also used to [destructure](destructuring) a tuple struct or
enum value.

A TupleStruct pattern is refutable when one of its subpatterns is refutable.

## Tuple patterns

> **<sup>Syntax</sup>**  
> _TuplePattern_ :<a name="tuple-pattern-syntax"></a>  
> &nbsp;&nbsp; `(` _TupplePatternItems_<sup>?</sup> `)`  
>  
> _TuplePatternItems_ :  
> &nbsp;&nbsp; &nbsp;&nbsp; [_Pattern_] `,`  
> &nbsp;&nbsp; | [_Pattern_]&nbsp;(`,` [_Pattern_])<sup>+</sup> `,`<sup>?</sup>  
> &nbsp;&nbsp; | ([_Pattern_] `,`)<sup>\*</sup> `..` ( (`,` [_Pattern_])<sup>+</sup> `,`<sup>?</sup> )<sup>?</sup>  

Tuple patterns match tuple values that match all criteria defined by its subpatterns.
They are also used to [destructure](destructuring) a tuple.

This pattern is refutable when one of its subpatterns is refutable.

## Slice patterns

> **<sup>Syntax</sup>**  
> _SlicePattern_ :  
> &nbsp;&nbsp; `[` [_Pattern_] \(`,` [_Pattern_])<sup>\*</sup> `,`<sup>?</sup> `]`

Slice patterns can match both arrays of fixed size and slices of dynamic size.
```rust
// Fixed size
let arr = [1, 2, 3];
match arr {
    [1, _, _] => "starts with one",
    [a, b, c] => "starts with something else",
}
```
```rust
// Dynamic size
let v = vec![1, 2, 3];
match v[..] {
    [a, b] => { /* this arm will not apply because the length doesn't match */ }
    [a, b, c] => { /* this arm will apply */ }
    _ => { /* this wildcard is required, since we don't know length statically */ }
}
```

## Path patterns

> **<sup>Syntax</sup>**  
> _PathPattern_ :  
> &nbsp;&nbsp; &nbsp;&nbsp; _PathForExpression_  
> &nbsp;&nbsp; | _QualifiedPathForExpression_

_Path patterns_ are patterns that refer either to constant values or
to structs or enum variants that have no fields.

Unqualified path patterns can refer to:

* enum variants
* structs
* constants
* associated constants

Qualified path patterns can only refer to associated constants.

Path patterns are irrefutable when they refer to constants or structs.
They are refutable when the refer to enum variants.

[_Pattern_]: #patterns
[_LiteralPattern_]: #literal-patterns
[_WildcardPattern_]: #wildcard-pattern
[_RangePattern_]: #range-patterns
[_ReferencePattern_]: #reference-patterns
[_IdentifierPattern_]: #identifier-patterns
[_TupleStructPattern_]: #tuplestruct-patterns
[_StructPattern_]: #struct-patterns
[_TuplePattern_]: #tuple-patterns
[_SlicePattern_]: #slice-patterns
[_PathPattern_]: #path-patterns

[Copy trait]: special-types-and-traits.html#copy
