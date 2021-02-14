# Grouped expressions

> **<sup>Syntax</sup>**\
> _GroupedExpression_ :\
> &nbsp;&nbsp; `(` [_InnerAttribute_]<sup>\*</sup> [_Expression_] `)`

A *parenthesized expression* wrap a single expression, evaluating to that expression.
The syntax for a parenthesized expression is a `(`, then an operand, the *enclosed operand*, and then a `)`.

An expression enclosed in parentheses evaluates to the value of the enclosed operand.

Parentheses can be used to explicitly modify the precedence order of subexpressions within an expression.

An example of a parenthesized expression:

```rust
let x: i32 = 2 + 3 * 4;
let y: i32 = (2 + 3) * 4;
assert_eq!(x, 14);
assert_eq!(y, 20);
```

An example of a necessary use of parentheses is when calling a function pointer that is a member of a struct:

```rust
# struct A {
#    f: fn() -> &'static str
# }
# impl A {
#    fn f(&self) -> &'static str {
#        "The method f"
#    }
# }
# let a = A{f: || "The field f"};
#
assert_eq!( a.f (), "The method f");
assert_eq!((a.f)(), "The field f");
```

## Group expression attributes

[Inner attributes] are allowed directly after the opening parenthesis of a group expression in the same expression contexts as [attributes on block expressions].

[Inner attributes]: ../attributes.md
[_Expression_]: ../expressions.md
[_InnerAttribute_]: ../attributes.md
[attributes on block expressions]: block-expr.md#attributes-on-block-expressions
