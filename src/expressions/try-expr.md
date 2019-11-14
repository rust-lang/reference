# `try` expressions

> **<sup>Syntax</sup>**\
> _TryExpression_ :\
> &nbsp;&nbsp; `try` _BlockExpression_

A *try expression* executes its associated block while limiting the effect of the [question mark operator](./operator-expr.md#the-question-mark-operator) to itself.

The effect of this can be seen in the following example:

```rust,edition2018
fn some_two() -> Option<u8> {
	Some(2)
}

fn some_three() -> Option<u8> {
	Some(3)
}

let sum: Option<u8> = try {
	some_two()? + some_three()?
};

assert_eq!(sum, Some(5));
```

This is useful when you want to coalesce multiple potential `Err` values into a single `Result`, or multiple `None` values into a single `Option`.

```rust,edition2018,ignore
let result = try {
	foo()?.bar()?.baz()?
};
```

> **Edition differences**: Try expressions are only available beginning with
> Rust 2018.
