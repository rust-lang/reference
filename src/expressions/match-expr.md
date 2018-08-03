# `match` expressions

> **<sup>Syntax</sup>**\
> _MatchExpression_ :\
> &nbsp;&nbsp; `match` [_Expression_]<sub>_except struct expression_</sub> `{`\
> &nbsp;&nbsp; &nbsp;&nbsp; [_InnerAttribute_]<sup>\*</sup>\
> &nbsp;&nbsp; &nbsp;&nbsp; _MatchArms_<sup>?</sup>\
> &nbsp;&nbsp; `}`
>
> _MatchArms_ :\
> &nbsp;&nbsp; ( _MatchArm_ `=>`
>                             ( [_BlockExpression_] `,`<sup>?</sup>
>                             | [_Expression_] `,` )
>                           )<sup>\*</sup>\
> &nbsp;&nbsp; _MatchArm_ `=>` ( [_BlockExpression_] | [_Expression_] ) `,`<sup>?</sup>
>
> _MatchArm_ :\
> &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> _MatchArmPatterns_ _MatchArmGuard_<sup>?</sup>
>
> _MatchArmPatterns_ :\
> &nbsp;&nbsp; `|`<sup>?</sup> _Pattern_ ( `|` _Pattern_ )<sup>\*</sup>
>
> _MatchArmGuard_ :\
> &nbsp;&nbsp; `if` [_Expression_]

A *`match` expression* branches on a pattern. The exact form of matching that
occurs depends on the pattern. *Patterns* consist of some combination of
literals, destructured arrays or enum constructors, structs and tuples,
variable binding specifications, wildcards (`..`), and placeholders (`_`). A
`match` expression has a *head expression*, which is the value to compare to
the patterns. The type of the patterns must equal the type of the head
expression.

A `match` behaves differently depending on whether or not the head expression
is a [place expression or value expression][place expression].
If the head expression is a [value expression], it is first evaluated into a
temporary location, and the resulting value is sequentially compared to the
patterns in the arms until a match is found. The first arm with a matching
pattern is chosen as the branch target of the `match`, any variables bound by
the pattern are assigned to local variables in the arm's block, and control
enters the block.

When the head expression is a [place expression], the match does not allocate a
temporary location; however, a by-value binding may copy or move from the
memory location.
When possible, it is preferable to match on place expressions, as the lifetime
of these matches inherits the lifetime of the place expression rather than being
restricted to the inside of the match.

An example of a `match` expression:

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
```

Patterns that bind variables default to binding to a copy or move of the
matched value (depending on the matched value's type). This can be changed to
bind to a reference by using the `ref` keyword, or to a mutable reference using
`ref mut`.

Patterns can be used to *destructure* structs, enums, and tuples. Destructuring
breaks a value up into its component pieces. The syntax used is the same as
when creating such values. When destructing a data structure with named (but
not numbered) fields, it is allowed to write `fieldname` as a shorthand for
`fieldname: fieldname`. In a pattern whose head expression has a `struct`,
`enum` or `tupl` type, a placeholder (`_`) stands for a *single* data field,
whereas a wildcard `..` stands for *all* the fields of a particular variant.

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

Patterns can also dereference pointers by using the `&`, `&mut` and `box`
symbols, as appropriate. For example, these two matches on `x: &i32` are
equivalent:

```rust
let int_reference = &3;

let a = match *int_reference { 0 => "zero", _ => "some" };
let b = match int_reference { &0 => "zero", _ => "some" };

assert_eq!(a, b);
```

Subpatterns can also be bound to variables by the use of the syntax `variable @
subpattern`. For example:

```rust
let x = 1;

match x {
    e @ 1 ... 5 => println!("got a range element {}", e),
    _ => println!("anything"),
}
```

Multiple match patterns may be joined with the `|` operator. An inclusive range
of values may be specified with `..=`. For example:

```rust
# let x = 9;
let message = match x {
    0 | 1  => "not many",
    2 ..= 9 => "a few",
    _      => "lots"
};

assert_eq!(message, "a few");
```

Other forms of [range] \(e.g `..` for an exclusive range, or any range with one or
both endpoints left unspecified) are not supported in matches. The
syntax `...` is also accepted for inclusive ranges in patterns only, for
backwards compatibility.

Range patterns only work with [`char`] and [numeric types]. A range pattern may
not be a sub-range of another range pattern inside the same `match`.

Slice patterns can match both arrays of fixed size and slices of dynamic size.
```rust
// Fixed size
let arr = [1, 2, 3];
match arr {
    [1, _, _] => "starts with one",
    [a, b, c] => "starts with something else",
};
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

Finally, match arms can accept *pattern guards* to further refine the
criteria for matching a case. Pattern guards appear after the pattern and
consist of a bool-typed expression following the `if` keyword. A pattern guard
may refer to the variables bound within the pattern they follow.

When the pattern matches successfully, the pattern guard expression is executed.
If the expression is truthy, the pattern is successfully matched against.
Otherwise, the next pattern, including other matches with the `|` operator in
the same arm, is tested.

```rust
# let maybe_digit = Some(0);
# fn process_digit(i: i32) { }
# fn process_other(i: i32) { }
let message = match maybe_digit {
    Some(x) if x < 10 => process_digit(x),
    Some(x) => process_other(x),
    None => panic!(),
};
```

> Note: Multiple matches using the `|` operator can cause the pattern guard and
> and side effects it has to execute multiple times. For example:
>
> ```rust
> use std::cell::Cell;
> fn main() {
>     let i : Cell<i32> = Cell::new(0);
>     match 1 {
>         1 | _ if { i.set(i.get() + 1); false } => {}
>         _ => {}
>     }
>     assert_eq!(i.get(), 2);
> }
> ```

## Attributes on match arms

Outer attributes are allowed on match arms. The only attributes that have
meaning on match arms are [`cfg`], `cold`, and the [lint check attributes].

[_Expression_]: expressions.html
[_BlockExpression_]: expressions/block-expr.html#block-expressions
[place expression]: expressions.html#place-expressions-and-value-expressions
[value expression]: expressions.html#place-expressions-and-value-expressions
[`char`]: types.html#textual-types
[numeric types]: types.html#numeric-types
[_InnerAttribute_]: attributes.html
[_OuterAttribute_]: attributes.html
[`cfg`]: attributes.html#conditional-compilation
[lint check attributes]: attributes.html#lint-check-attributes
[range]: expressions/range-expr.html
