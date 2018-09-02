# Block expressions

> **<sup>Syntax</sup>**\
> _BlockExpression_ :\
> &nbsp;&nbsp; `{`\
> &nbsp;&nbsp; &nbsp;&nbsp; [_InnerAttribute_]<sup>\*</sup>\
> &nbsp;&nbsp; &nbsp;&nbsp; _Statements_<sup>\*</sup>\
> &nbsp;&nbsp; &nbsp;&nbsp; [_Expression_]<sup>?</sup>\
> &nbsp;&nbsp; `}`
>
> _Statements_ :\
> &nbsp;&nbsp; ( `;`\
> &nbsp;&nbsp; | [_ItemDeclaration_]\
> &nbsp;&nbsp; | [_LetStatement_] ;\
> &nbsp;&nbsp; | [_NonControlFlowExpressionStatement_][expression statement] ;\
> &nbsp;&nbsp; | [_FlowControlExpressionStatement_][expression statement] ;<sup>?</sup>\
> &nbsp;&nbsp; )<sup>\*</sup>

A *block expression*, or *block*, is a control flow expression and anonymouse
namespace scope for items and variable declarations. As a control flow
expression, a block sequentially executes its component non-item declaration
statements and then its final optional expression. As an anonymous namespace
scope, item declarations are only in scope inside the block itself and variables
declared by `let` statements are in scope from the next statement until the end
of the block.

Blocks are written as `{`, then any [inner attributes], then [statements],
then an optional expression, and finally a `}`. Statements are usually required
to be followed a semicolon, with two exceptions. Item declaration statements do
not need to be followed by a semicolon. Expression statements usually require
a following semicolon except if its outer expression is a flow control
expression. Furthermore, extra semicolons between statements are allowed, but
these semicolons do not affect semantics.

> Note: The semicolon following a statement is not a part of the statement
> itself. They are invalid when using the `stmt` macro matcher.

When evaluating a block expression, each statement, except for item declaration
statements, is executed sequentially. Then the final expression is executed,
if given.

The type of a block is the type of the final expression, or `()` if the final
expression is omitted.

```rust
# fn fn_call() {}
let _: () = {
    fn_call();
};

let five: i32 = {
    fn_call();
    5
};

assert_eq!(5, five);
```

> Note: As a control flow expression, if a block expression is the outer
> expression of an expression statement, the expected type is `()` unless it
> is followed immediately by a semicolon.

Blocks are always [value expressions] and evaluate the last expression in 
value expression context. This can be used to force moving a value if really
needed. For example, the following example fails on the call to `consume_self`
because the struct was moved out of `s` in the block expression.

```rust,compile_fail
struct Struct;

impl Struct {
    fn consume_self(self) {}
    fn borrow_self(&self) {}
}

fn move_by_block_expression() {
    let s = Struct;

    // Move the value out of `s` in the block expreesion.
    (&{ s }).borrow_self();

    // Fails to execute because `s` is moved out of.
    s.consume_self();
}
```

## `unsafe` blocks

> **<sup>Syntax</sup>**\
> _UnsafeBlockExpression_ :\
> &nbsp;&nbsp; `unsafe` _BlockExpression_

_See [`unsafe` block](unsafe-blocks.html) for more information on when to use `unsafe`_

A block of code can be prefixed with the `unsafe` keyword to permit [unsafe
operations]. Examples:

```rust
unsafe {
    let b = [13u8, 17u8];
    let a = &b[0] as *const u8;
    assert_eq!(*a, 13);
    assert_eq!(*a.offset(1), 17);
}

# unsafe fn an_unsafe_fn() -> i32 { 10 }
let a = unsafe { an_unsafe_fn() };
```

## Attributes on block expressions

Block expressions allow [outer attributes] and [inner attributes] directly after
the opening brace when the block expression is the outer expression of an
[expression statement] or the final expression of another block expression. The
attributes that have meaning on a block expression are [`cfg`] and [the lint
check attributes].

For example, this function returns `true` on unix platforms and `false` on other
platforms.

```rust
fn is_unix_platform() -> bool {
    #[cfg(unix)] { true }
    #[cfg(not(unix))] { false }
}
```

[_InnerAttribute_]: attributes.html
[_ItemDeclaration_]: items.html
[_LetStatement_]: statements.html#let-statements
[expression statement]: statements.html#expression-statements
[_Expression_]: expressions.html
[expression]: expressions.html
[statements]: statements.html
[value expressions]: expressions.html#place-expressions-and-value-expressions
[outer attributes]: attributes.html
[inner attributes]: attributes.html
[expression statement]: statements.html#expression-statements
[`cfg`]: conditional-compilation.html
[the lint check attributes]: attributes.html#lint-check-attributes
[unsafe operations]: unsafety.html
