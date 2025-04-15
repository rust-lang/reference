r[expr.placeholder]
# `_` expressions

r[expr.placeholder.syntax]
```grammar,expressions
UnderscoreExpression -> `_`
```

r[expr.placeholder.intro]
Underscore expressions, denoted with the symbol `_`, are used to signify a
placeholder in a destructuring assignment.

r[expr.placeholder.lhs-assignment-only]
They may only appear in the left-hand side of an assignment.

r[expr.placeholder.pattern]
Note that this is distinct from the [wildcard pattern](../patterns.md#wildcard-pattern).

Examples of `_` expressions:

```rust
let p = (1, 2);
let mut a = 0;
(_, a) = p;

struct Position {
    x: u32,
    y: u32,
}

Position { x: a, y: _ } = Position{ x: 2, y: 3 };

// unused result, assignment to `_` used to declare intent and remove a warning
_ = 2 + 2;
// triggers unused_must_use warning
// 2 + 2;

// equivalent technique using a wildcard pattern in a let-binding
let _ = 2 + 2;
```
