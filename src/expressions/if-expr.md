r[expr.if]
# `if` expressions

r[expr.if.syntax]
```grammar,expressions
IfExpression ->
    `if` Conditions BlockExpression
    (`else` ( BlockExpression | IfExpression ) )?

Conditions ->
      Expression _except [StructExpression]_
    | LetChain

LetChain -> LetChainCondition ( `&&` LetChainCondition )*

LetChainCondition ->
      Expression _except [ExcludedConditions]_
    | OuterAttribute* `let` Pattern `=` Scrutinee _except [ExcludedConditions]_

@root ExcludedConditions ->
      StructExpression
    | LazyBooleanExpression
    | RangeExpr
    | RangeFromExpr
    | RangeInclusiveExpr
    | AssignmentExpression
    | CompoundAssignmentExpression
```
<!-- TODO: The struct exception above needs clarification, see https://github.com/rust-lang/reference/issues/1808
     The chain grammar could use some work, see https://github.com/rust-lang/reference/issues/1811
-->

r[expr.if.intro]
The syntax of an `if` expression is a sequence of one or more condition operands separated by `&&`,
followed by a consequent block, any number of `else if` conditions and blocks, and an optional trailing `else` block.

r[expr.if.condition]
Condition operands must be either an [Expression] with a [boolean type] or a conditional `let` match.

r[expr.if.condition-true]
If all of the condition operands evaluate to `true` and all of the `let` patterns successfully match their [scrutinee]s,
the consequent block is executed and any subsequent `else if` or `else` block is skipped.

r[expr.if.else-if]
If any condition operand evaluates to `false` or any `let` pattern does not match its scrutinee,
the consequent block is skipped and any subsequent `else if` condition is evaluated.

r[expr.if.else]
If all `if` and `else if` conditions evaluate to `false` then any `else` block is executed.

r[expr.if.result]
An `if` expression evaluates to the same value as the executed block, or `()` if no block is evaluated.

r[expr.if.type]
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

r[expr.if.let]
## `if let` patterns

r[expr.if.let.intro]
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

r[expr.if.let.or-pattern]
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

r[expr.if.chains]
## Chains of conditions

r[expr.if.chains.intro]
Multiple condition operands can be separated with `&&`.

r[expr.if.chains.order]
Similar to a `&&` [LazyBooleanExpression], each operand is evaluated from left-to-right until an operand evaluates as `false` or a `let` match fails,
in which case the subsequent operands are not evaluated.

r[expr.if.chains.bindings]
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

r[expr.if.chains.or]
If any condition operand is a `let` pattern, then none of the condition operands can be a `||` [lazy boolean operator expression][expr.bool-logic] due to ambiguity and precedence with the `let` scrutinee.
If a `||` expression is needed, then parentheses can be used. For example:

```rust
# let foo = Some(123);
# let condition1 = true;
# let condition2 = false;
// Parentheses are required here.
if let Some(x) = foo && (condition1 || condition2) { /*...*/ }
```

r[expr.if.edition2024]
> [!EDITION-2024]
> Before the 2024 edition, let chains are not supported. That is, the [LetChain] grammar is not allowed in an `if` expression.

[`match` expressions]: match-expr.md
[boolean type]: ../types/boolean.md
[scrutinee]: ../glossary.md#scrutinee

<script>
(function() {
    var fragments = {
        "#if-let-expressions": "if-expr.html#if-let-patterns",
    };
    var target = fragments[window.location.hash];
    if (target) {
        var url = window.location.toString();
        var base = url.substring(0, url.lastIndexOf('/'));
        window.location.replace(base + "/" + target);
    }
})();
</script>
