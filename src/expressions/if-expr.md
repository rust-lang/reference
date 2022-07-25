# `if` expressions

## Syntax

> **<sup>Syntax</sup>**\
> _IfExpression_ :\
> &nbsp;&nbsp; `if` _IfConditions_ [_BlockExpression_]\
> &nbsp;&nbsp; (`else` ( [_BlockExpression_] | _IfExpression_ ) )<sup>\?</sup>
>
> _IfConditions_ :\
> &nbsp;&nbsp; _IfCondition_ ( && _IfCondition_ )*
>
> _IfCondition_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; [_Expression_]<sub>_except struct expression_</sub>\
> &nbsp;&nbsp; | `let` [_Pattern_] `=` [_Scrutinee_]

An `if` expression is a conditional branch in program control.
The syntax of an `if` expression is a sequence of one or more condition operands separated by `&&`,
followed by a consequent block, any number of `else if` conditions and blocks, and an optional trailing `else` block.

Condition operands must be either an [_Expression_] with a [boolean type] or a conditional `let` match.
If all of the condition operands evaluate to `true` and all of the `let` patterns successfully match their [scrutinee]s,
the consequent block is executed and any subsequent `else if` or `else` block is skipped.
If any condition operand evaluates to `false` or any `let` pattern does not match its scrutinee,
the consequent block is skipped and any subsequent `else if` condition is evaluated.
If all `if` and `else if` conditions evaluate to `false` then any `else` block is executed.

An `if` expression evaluates to the same value as the executed block, or `()` if no block is evaluated.
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

// `if` can be used as an expression.
let y = if 12 * 15 > 150 {
    "Bigger"
} else {
    "Smaller"
};
assert_eq!(y, "Bigger");
```

## `if let` patterns

`let` patterns in an `if` condition allow binding new variables into scope when the pattern matches successfully.
The following examples illustrate bindings using `let` patterns:

```rust
let dish = ("Ham", "Eggs");

// This body will be skipped because the pattern is refuted.
if let ("Bacon", b) = dish {
    println!("Bacon is served with {}", b);
} else {
    // This block is evaluated instead.
    println!("No bacon will be served");
}

// This body will execute.
if let ("Ham", b) = dish {
    println!("Ham is served with {}", b);
}

if let _ = 5 {
    println!("Irrefutable patterns are always true");
}
```

Multiple patterns may be specified with the `|` operator.
This has the same semantics as with `|` in [`match` expressions]:

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

## Chains of conditions

Multiple condition operands can be separated with `&&`.
Similar to a `&&` [_LazyBooleanOperatorExpression_], each operand is evaluated from left-to-right until an operand evaluates as `false` or a `let` match fails,
in which case the subsequent operands are not evaluated.

The bindings of each pattern are put into scope to be available for the next condition operand and the consequent block.

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
```

The above is equivalent to the following without using chains of conditions:

```rust
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

If any condition operand is a `let` pattern, then none of the condition operands can be a `||` [lazy boolean operator expression][_LazyBooleanOperatorExpression_] due to ambiguity and precedence with the `let` scrutinee.
If a `||` expression is needed, then parentheses can be used. For example:

```rust
# let foo = Some(123);
# let condition1 = true;
# let condition2 = false;
// Parentheses are required here.
if let Some(x) = foo && (condition1 || condition2) { /*...*/ }
```


[_BlockExpression_]: block-expr.md
[_Expression_]: ../expressions.md
[_LazyBooleanOperatorExpression_]: operator-expr.md#lazy-boolean-operators
[_Pattern_]: ../patterns.md
[_Scrutinee_]: match-expr.md
[`match` expressions]: match-expr.md
[boolean type]: ../types/boolean.md
[scrutinee]: ../glossary.md#scrutinee
