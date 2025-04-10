r[expr.paren]
# Grouped expressions

r[expr.paren.syntax]
```grammar,expressions
GroupedExpression -> `(` Expression `)`
```

r[expr.paren.intro]
A *parenthesized expression* wraps a single expression, evaluating to that expression.
The syntax for a parenthesized expression is a `(`, then an expression, called the *enclosed operand*, and then a `)`.

r[expr.paren.evaluation]
Parenthesized expressions evaluate to the value of the enclosed operand.

r[expr.paren.place-or-value]
Unlike other expressions, parenthesized expressions are both [place expressions and value expressions][place].
When the enclosed operand is a place expression, it is a place expression and when the enclosed operand is a value expression, it is a value expression.

r[expr.paren.override-precedence]
Parentheses can be used to explicitly modify the precedence order of subexpressions within an expression.

An example of a parenthesized expression:

```rust
let x: i32 = 2 + 3 * 4; // not parenthesized
let y: i32 = (2 + 3) * 4; // parenthesized
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

[place]: ../expressions.md#place-expressions-and-value-expressions
