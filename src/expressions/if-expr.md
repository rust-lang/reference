# `if` expressions

## Syntax

The same syntax is used by `if`, `if let` and chains of expressions.

> **<sup>Syntax</sup>**\
> _IfExpression_ :\
> &nbsp;&nbsp; `if` _IfLetList_ [_BlockExpression_]\
> &nbsp;&nbsp; ( `else` _IfLetList_ [_BlockExpression_] )<sup>\?</sup>
>
> _IfLet_ :\
>  &nbsp;&nbsp; [_Expression_]<sub>_except struct expression_</sub>
> | `let` [_Pattern_] `=` [_Expression_]<sub>_except struct expression_</sub>
>
> _IfLetList_ :\
> &nbsp;&nbsp; _IfLet_ ( && _IfLet_ )*

## `if`

An `if` expression is a conditional branch in program control.
The syntax of an `if` expression is a condition operand, followed by a consequent block, any number of `else if` conditions and blocks, and an optional trailing `else` block.
The condition operands must have the [boolean type].
If a condition operand evaluates to `true`, the consequent block is executed and any subsequent `else if` or `else` block is skipped.
If a condition operand evaluates to `false`, the consequent block is skipped and any subsequent `else if` condition is evaluated.
If all `if` and `else if` conditions evaluate to `false` then any `else` block is executed.
An if expression evaluates to the same value as the executed block, or `()` if no block is evaluated.
An `if` expression must have the same type in all situations.

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

## `if let`

An `if let` expression is semantically similar to an `if` expression but in place of a condition operand it expects the keyword `let` followed by a pattern, an `=` and a [scrutinee] operand.
If the value of the scrutinee matches the pattern, the corresponding block will execute.
Otherwise, flow proceeds to the following `else` block if it exists.
Like `if` expressions, `if let` expressions have a value determined by the block that is evaluated.

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

if let _ = 5 {
    println!("Irrefutable patterns are always true");
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

Multiple patterns may be specified with the `|` operator. This has the same semantics as with `|` in `match` expressions:

```rust
enum E {
    X(u8),
    Y(u8),
    Z(u8),
}
let v = E::Y(12);
if let E::X(n) | E::Y(n) = v {
    assert_eq!(n, 12);
}
```

The expression cannot be a [lazy boolean operator expression][_LazyBooleanOperatorExpression_].
Scrutinee expressions also cannot be a [lazy boolean operator expression][_LazyBooleanOperatorExpression_] due to ambiguity and precedence with [chains of expressions][_ChainsOfExpressions_].

## Chains of expressions

The following is an example of chaining multiple expressions, mixing `let` bindings and boolean expressions, and with expressions able to reference pattern bindings from previous expressions:

```rust
fn single() {
    let outer_opt = Some(Some(1i32));

    if let Some(inner_opt) = outer_opt
        && let Some(number) = inner_opt
        && number == 1
    {
        println!("Peek a boo");
    }
}

The above is equivalent to the following without using expression chains:

fn nested() {
    let outer_opt = Some(Some(1i32));

    if let Some(inner_opt) = outer_opt {
        if let Some(number) = inner_opt {
            if number == 1 {
                println!("Peek a boo");
            }
        }
    }
}
```

In other words, chains are equivalent to nested [`if let` expressions]:

<!-- ignore: expansion example -->
```rust,ignore
if let PAT0 = EXPR0 && let PAT1 = EXPR1 {
    /* body */
} else {
    /* else */
}
```

is equivalent to

<!-- ignore: expansion example -->
```rust,ignore
if let PAT0 = EXPR0 {
    if let PAT1 = EXPR1 {
        /* body */
    }
    else {
        /* else */
    }
} else {
    /* else */
}
```

[_BlockExpression_]: block-expr.md
[_ChainsOfExpressions_]: #chains-of-expressions
[_Expression_]: ../expressions.md
[_LazyBooleanOperatorExpression_]: operator-expr.md#lazy-boolean-operators
[_Pattern_]: ../patterns.md
[_Scrutinee_]: match-expr.md
[`match` expression]: match-expr.md
[boolean type]: ../types/boolean.md
[scrutinee]: ../glossary.md#scrutinee
