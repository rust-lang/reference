# `_` expression

> **<sup>Syntax</sup>**\
> _UnderscoreExpression_ :\
> &nbsp;&nbsp; `_`

The underscore expression, denoted with the symbol `_`, is used to signify a
placeholder in a destructuring assignment. It may only appear in the left-hand
side of an assignment.

An example of an `_` expression:

```rust,ignore
let p = (1, 2);
let mut a = 0;
(_, a) = p;
```

[_Expression_]: ../expressions.md
