# `if` and `if let` expressions

## `if` expressions

> **<sup>Syntax</sup>**  
> _IfExpression_ :  
> &nbsp;&nbsp; `if` [_Expression_]<sub>_except struct expression_</sub> [_BlockExpression_]  
> &nbsp;&nbsp; (`else` (
>   [_BlockExpression_]
> | _IfExpression_
> | _IfLetExpression_ ) )<sup>\?</sup>  

An `if` expression is a conditional branch in program control. The form of an
`if` expression is a condition expression, followed by a consequent block, any
number of `else if` conditions and blocks, and an optional trailing `else`
block. The condition expressions must have type `bool`. If a condition
expression evaluates to `true`, the consequent block is executed and any
subsequent `else if` or `else` block is skipped. If a condition expression
evaluates to `false`, the consequent block is skipped and any subsequent `else
if` condition is evaluated. If all `if` and `else if` conditions evaluate to
`false` then any `else` block is executed. An if expression evaluates to the
same value as the executed block, or `()` if no block is evaluated. An `if`
expression must have the same type in all situations.

```rust
# let x = 3;
if x == 4 {
    println!("x is four");
} else if x == 3 {
    println!("x is three");
} else {
    println!("x is something else");
}

let y = if 12 * 15 > 150 {
    "Bigger"
} else {
    "Smaller"
};
assert_eq!(y, "Bigger");
```

## `if let` expressions

> **<sup>Syntax</sup>**  
> _IfLetExpression_ :  
> &nbsp;&nbsp; `if` `let` [_Pattern_] `=` [_Expression_]<sub>_except struct expression_</sub>
>              [_BlockExpression_]  
> &nbsp;&nbsp; (`else` (
>   [_BlockExpression_]
> | _IfExpression_
> | _IfLetExpression_ ) )<sup>\?</sup>  

An `if let` expression is semantically similar to an `if` expression but in
place of a condition expression it expects the keyword `let` followed by a
refutable pattern, an `=` and an expression. If the value of the expression on
the right hand side of the `=` matches the pattern, the corresponding block
will execute, otherwise flow proceeds to the following `else` block if it
exists. Like `if` expressions, `if let` expressions have a value determined by
the block that is evaluated.

```rust
let dish = ("Ham", "Eggs");

// this body will be skipped because the pattern is refuted
if let ("Bacon", b) = dish {
    println!("Bacon is served with {}", b);
} else {
    // This block is evaluated instead.
    println!("No bacon will be served");
}

// this body will execute
if let ("Ham", b) = dish {
    println!("Ham is served with {}", b);
}
```

`if` and `if let` expressions can be intermixed:

```rust
let x = Some(3);
let a = if let Some(1) = x {
    1
} else if x == Some(2) {
    2
} else if let Some(y) = x {
    y
} else {
    -1
};
assert_eq!(a, 3);
```

[_Expression_]: expressions.html
[_BlockExpression_]: expressions/block-expr.html
[_Pattern_]: patterns.html
