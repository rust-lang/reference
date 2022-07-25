# Loops

> **<sup>Syntax</sup>**\
> _LoopExpression_ :\
> &nbsp;&nbsp; [_LoopLabel_]<sup>?</sup> (\
> &nbsp;&nbsp; &nbsp;&nbsp; &nbsp;&nbsp; [_InfiniteLoopExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_PredicateLoopExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_IteratorLoopExpression_]\
> &nbsp;&nbsp; )

[_LoopLabel_]: #loop-labels
[_InfiniteLoopExpression_]: #infinite-loops
[_PredicateLoopExpression_]: #predicate-loops
[_IteratorLoopExpression_]: #iterator-loops

Rust supports three loop expressions:

*   A [`loop` expression](#infinite-loops) denotes an infinite loop.
*   A [`while` expression](#predicate-loops) loops until a predicate is false.
*   A [`for` expression](#iterator-loops) extracts values from an iterator, looping until the iterator is empty.

All three types of loop support [`break` expressions](#break-expressions), [`continue` expressions](#continue-expressions), and [labels](#loop-labels).
Only `loop` supports [evaluation to non-trivial values](#break-and-loop-values).

## Infinite loops

> **<sup>Syntax</sup>**\
> _InfiniteLoopExpression_ :\
> &nbsp;&nbsp; `loop` [_BlockExpression_]

A `loop` expression repeats execution of its body continuously:
`loop { println!("I live."); }`.

A `loop` expression without an associated `break` expression is diverging and has type [`!`](../types/never.md).
A `loop` expression containing associated [`break` expression(s)](#break-expressions) may terminate, and must have type compatible with the value of the `break` expression(s).

## Predicate loops

> **<sup>Syntax</sup>**\
> [_PredicateLoopExpression_] :\
> &nbsp;&nbsp; `while` _WhileConditions_ [_BlockExpression_]
>
> _WhileConditions_ :\
> &nbsp;&nbsp; _WhileCondition_ ( && _WhileCondition_ )*
>
> _WhileCondition_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; [_Expression_]<sub>_except struct expression_</sub>\
> &nbsp;&nbsp; | `let` [_Pattern_] `=` [_Scrutinee_]

A `while` loop expression allows repeating the evaluation of a block while a set of conditions remain true.
The syntax of a `while` expression is a sequence of one or more condition operands separated by `&&`,
followed by a [_BlockExpression_].

Condition operands must be either an [_Expression_] with a [boolean type] or a conditional `let` match.
If all of the condition operands evaluate to `true` and all of the `let` patterns successfully match their [scrutinee]s,
then the loop body block executes.
After the loop body successfully executes, the condition operands are re-evaluated to determine if the body should be executed again.
If any condition operand evaluates to `false` or any `let` pattern does not match its scrutinee,
the body is not executed and execution continues after the `while` expression.

A `while` expression evaluates to `()`.

An example:

```rust
let mut i = 0;

while i < 10 {
    println!("hello");
    i = i + 1;
}
```

### `while let` patterns

`let` patterns in a `while` condition allow binding new variables into scope when the pattern matches successfully.
The following examples illustrate bindings using `let` patterns:

```rust
let mut x = vec![1, 2, 3];

while let Some(y) = x.pop() {
    println!("y = {}", y);
}

while let _ = 5 {
    println!("Irrefutable patterns are always true");
    break;
}
```

A `while let` loop is equivalent to a `loop` expression containing a [`match` expression] as follows.

<!-- ignore: expansion example -->
```rust,ignore
'label: while let PATS = EXPR {
    /* loop body */
}
```

is equivalent to

<!-- ignore: expansion example -->
```rust,ignore
'label: loop {
    match EXPR {
        PATS => { /* loop body */ },
        _ => break,
    }
}
```

Multiple patterns may be specified with the `|` operator.
This has the same semantics as with `|` in `match` expressions:

```rust
let mut vals = vec![2, 3, 1, 2, 2];
while let Some(v @ 1) | Some(v @ 2) = vals.pop() {
    // Prints 2, 2, then 1
    println!("{}", v);
}
```

As is the case in [`if` expressions], the scrutinee cannot be a [lazy boolean operator expression][_LazyBooleanOperatorExpression_].

### `while` condition chains

Multiple condition operands can be separated with `&&`.
These have the same semantics and restrictions as [`if` condition chains].

The following is an example of chaining multiple expressions, mixing `let` bindings and boolean expressions, and with expressions able to reference pattern bindings from previous expressions:

```rust
fn main() {
    let outer_opt = Some(Some(1i32));

    while let Some(inner_opt) = outer_opt
        && let Some(number) = inner_opt
        && number == 1
    {
        println!("Peek a boo");
        break;
    }
}
```

## Iterator loops

> **<sup>Syntax</sup>**\
> _IteratorLoopExpression_ :\
> &nbsp;&nbsp; `for` [_Pattern_] `in` [_Expression_]<sub>_except struct expression_</sub>
>              [_BlockExpression_]

A `for` expression is a syntactic construct for looping over elements provided by an implementation of `std::iter::IntoIterator`.
If the iterator yields a value, that value is matched against the irrefutable pattern, the body of the loop is executed, and then control returns to the head of the `for` loop.
If the iterator is empty, the `for` expression completes.

An example of a `for` loop over the contents of an array:

```rust
let v = &["apples", "cake", "coffee"];

for text in v {
    println!("I like {}.", text);
}
```

An example of a for loop over a series of integers:

```rust
let mut sum = 0;
for n in 1..11 {
    sum += n;
}
assert_eq!(sum, 55);
```

A `for` loop is equivalent to a `loop` expression containing a [`match` expression] as follows:

<!-- ignore: expansion example -->
```rust,ignore
'label: for PATTERN in iter_expr {
    /* loop body */
}
```

is equivalent to

<!-- ignore: expansion example -->
```rust,ignore
{
    let result = match IntoIterator::into_iter(iter_expr) {
        mut iter => 'label: loop {
            let mut next;
            match Iterator::next(&mut iter) {
                Option::Some(val) => next = val,
                Option::None => break,
            };
            let PATTERN = next;
            let () = { /* loop body */ };
        },
    };
    result
}
```

`IntoIterator`, `Iterator`, and `Option` are always the standard library items here, not whatever those names resolve to in the current scope.
The variable names `next`, `iter`, and `val` are for exposition only, they do not actually have names the user can type.

> **Note**: that the outer `match` is used to ensure that any [temporary values] in `iter_expr` don't get dropped before the loop is finished.
> `next` is declared before being assigned because it results in types being inferred correctly more often.

## Loop labels

> **<sup>Syntax</sup>**\
> _LoopLabel_ :\
> &nbsp;&nbsp; [LIFETIME_OR_LABEL] `:`

A loop expression may optionally have a _label_. The label is written as a lifetime preceding the loop expression, as in `'foo: loop { break 'foo; }`, `'bar: while false {}`, `'humbug: for _ in 0..0 {}`.
If a label is present, then labeled `break` and `continue` expressions nested within this loop may exit out of this loop or return control to its head.
See [break expressions](#break-expressions) and [continue expressions](#continue-expressions).

## `break` expressions

> **<sup>Syntax</sup>**\
> _BreakExpression_ :\
> &nbsp;&nbsp; `break` [LIFETIME_OR_LABEL]<sup>?</sup> [_Expression_]<sup>?</sup>

When `break` is encountered, execution of the associated loop body is immediately terminated, for example:

```rust
let mut last = 0;
for x in 1..100 {
    if x > 12 {
        break;
    }
    last = x;
}
assert_eq!(last, 12);
```

A `break` expression is normally associated with the innermost `loop`, `for` or `while` loop enclosing the `break` expression,
but a [label](#loop-labels) can be used to specify which enclosing loop is affected.
Example:

```rust
'outer: loop {
    while true {
        break 'outer;
    }
}
```

A `break` expression is only permitted in the body of a loop, and has one of the forms `break`, `break 'label` or ([see below](#break-and-loop-values)) `break EXPR` or `break 'label EXPR`.

## `continue` expressions

> **<sup>Syntax</sup>**\
> _ContinueExpression_ :\
> &nbsp;&nbsp; `continue` [LIFETIME_OR_LABEL]<sup>?</sup>

When `continue` is encountered, the current iteration of the associated loop body is immediately terminated, returning control to the loop *head*.
In the case of a `while` loop, the head is the conditional operands controlling the loop.
In the case of a `for` loop, the head is the call-expression controlling the loop.

Like `break`, `continue` is normally associated with the innermost enclosing loop, but `continue 'label` may be used to specify the loop affected.
A `continue` expression is only permitted in the body of a loop.

## `break` and loop values

When associated with a `loop`, a break expression may be used to return a value from that loop, via one of the forms `break EXPR` or `break 'label EXPR`, where `EXPR` is an expression whose result is returned from the `loop`.
For example:

```rust
let (mut a, mut b) = (1, 1);
let result = loop {
    if b > 10 {
        break b;
    }
    let c = a + b;
    a = b;
    b = c;
};
// first number in Fibonacci sequence over 10:
assert_eq!(result, 13);
```

In the case a `loop` has an associated `break`, it is not considered diverging, and the `loop` must have a type compatible with each `break` expression.
`break` without an expression is considered identical to `break` with expression `()`.

[LIFETIME_OR_LABEL]: ../tokens.md#lifetimes-and-loop-labels
[_BlockExpression_]: block-expr.md
[_Expression_]: ../expressions.md
[_LazyBooleanOperatorExpression_]: operator-expr.md#lazy-boolean-operators
[_Pattern_]: ../patterns.md
[_Scrutinee_]: match-expr.md
[`if` condition chains]: if-expr.md#chains-of-conditions
[`if` expressions]: if-expr.md
[`match` expression]: match-expr.md
[boolean type]: ../types/boolean.md
[scrutinee]: ../glossary.md#scrutinee
[temporary values]: ../expressions.md#temporaries
