# Grouped expressions

> **<sup>Syntax</sup>**\
> _GroupedExpression_ :\
> &nbsp;&nbsp; `(` [_InnerAttribute_]<sup>\*</sup> [_Expression_] `)`

An expression enclosed in parentheses evaluates to the result of the enclosed
expression. Parentheses can be used to explicitly specify evaluation order
within an expression.

An example of a parenthesized expression:

```rust
let x: i32 = 2 + 3 * 4;
let y: i32 = (2 + 3) * 4;
assert_eq!(x, 14);
assert_eq!(y, 20);
```

An example of a necessary use of parentheses is when calling a function pointer
that is a member of a struct:

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

[Inner attributes] are allowed directly after the opening parenthesis of a
group expression in the same expression contexts as [attributes on block
expressions].

[Inner attributes]: attributes.html
[_Expression_]: expressions.html
[_InnerAttribute_]: attributes.html
[attributes on block expressions]: expressions/block-expr.html#attributes-on-block-expressions
