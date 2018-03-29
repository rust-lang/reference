# `match` expressions

> **<sup>Syntax</sup>**  
> _MatchExpression_ :  
> &nbsp;&nbsp; `match` [_Expression_]<sub>_except struct expression_</sub> `{`  
> &nbsp;&nbsp; &nbsp;&nbsp; [_InnerAttribute_]<sup>\*</sup>  
> &nbsp;&nbsp; &nbsp;&nbsp; _MatchArms_<sup>?</sup>  
> &nbsp;&nbsp; `}`  
>  
> _MatchArms_ :  
> &nbsp;&nbsp; ( _MatchArm_ `=>`
>                             ( [_BlockExpression_] `,`<sup>?</sup>
>                             | [_Expression_] `,` )
>                           )<sup>\*</sup>  
> &nbsp;&nbsp; _MatchArm_ `=>` ( [_BlockExpression_] | [_Expression_] ) `,`<sup>?</sup>  
>  
> _MatchArm_ :  
> &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> _MatchArmPatterns_ _MatchArmGuard_<sup>?</sup>  
>  
> _MatchArmPatterns_ :  
> &nbsp;&nbsp; `|`<sup>?</sup> [_Pattern_] ( `|` [_Pattern_] )<sup>*</sup>  
>  
> _MatchArmGuard_ :  
> &nbsp;&nbsp; `if` [_Expression_]  

A `match` expression branches on a *pattern*. The exact form of matching that
occurs depends on the pattern. See [Patterns] for more details. A `match` 
expression has a *head expression*, which is the value to compare to the
patterns. The type of the patterns must equal the type of the head expression.

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
`ref mut`. See [Identifier Patterns].

Multiple match patterns may be joined with the `|` operator:

```rust
# let x = 9;
let message = match x {
    0 | 1  => "not many",
    2 ..= 9 => "a few",
    _      => "lots"
};

assert_eq!(message, "a few");
```

Please notice that the `2..=9` is a [Range Pattern], not a [Range Expression]
and, thus, only those types of ranges supported by range patterns can be used
in match arms.

A range pattern may not be a sub-range of another range pattern inside the same `match`.

Match patterns can accept _match guards_ to further refine the
criteria for matching a case. Pattern guards appear after the pattern and
consist of a bool-typed expression following the `if` keyword. A pattern guard
may refer to the variables bound within the pattern they follow.

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

[_Expression_]: expressions.html
[_BlockExpression_]: expressions/block-expr.html#block-expressions
[place expression]: expressions.html#place-expressions-and-value-expressions
[value expression]: expressions.html#place-expressions-and-value-expressions
[_InnerAttribute_]: attributes.html
[_OuterAttribute_]: attributes.html
[Range Expression]: expressions/range-expr.html

[_Pattern_]: patterns.html
[Patterns]: patterns.html
[Identifier Patterns]: patterns.html#identifier-patterns
[Struct Patterns]: patterns.html#struct-patterns
[Tuple Struct Patterns]: patterns.html#tuplestruct-patterns
[Tuple Patterns]: patterns.html#tuple-patterns
[Range Pattern]: patterns.html#range-patterns
