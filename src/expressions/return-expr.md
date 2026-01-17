r[expr.return]
# `return` expressions

r[expr.return.syntax]
```grammar,expressions
ReturnExpression -> `return` Expression?
```

r[expr.return.intro]
Return expressions are denoted with the keyword `return`.

r[expr.return.behavior]
Evaluating a `return` expression moves its argument into the designated output location for the current function call, destroys the current function activation frame, and transfers control to the caller frame.

r[expr.return.diverging]
A `return` expression is [diverging] and has a type of [`!`].

An example of a `return` expression:

```rust
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    return b;
}
```

[`!`]: type.never
[diverging]: divergence
