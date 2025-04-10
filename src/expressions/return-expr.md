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

An example of a `return` expression:

```rust
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    return b;
}
```
