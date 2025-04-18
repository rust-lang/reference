r[expr.block]
# Block expressions

r[expr.block.syntax]
```grammar,expressions
BlockExpression ->
    `{`
        InnerAttribute*
        Statements?
    `}`

Statements ->
      Statement+
    | Statement+ ExpressionWithoutBlock
    | ExpressionWithoutBlock
```

r[expr.block.intro]
A *block expression*, or *block*, is a control flow expression and anonymous namespace scope for items and variable declarations.

r[expr.block.sequential-evaluation]
As a control flow expression, a block sequentially executes its component non-item declaration statements and then its final optional expression.

r[expr.block.namespace]
As an anonymous namespace scope, item declarations are only in scope inside the block itself and variables declared by `let` statements are in scope from the next statement until the end of the block.
See the [scopes] chapter for more details.

r[expr.block.inner-attributes]
The syntax for a block is `{`, then any [inner attributes], then any number of [statements], then an optional expression, called the final operand, and finally a `}`.

r[expr.block.statements]
Statements are usually required to be followed by a semicolon, with two exceptions:

1. Item declaration statements do not need to be followed by a semicolon.
2. Expression statements usually require a following semicolon except if its outer expression is a flow control expression.

r[expr.block.null-statement]
Furthermore, extra semicolons between statements are allowed, but these semicolons do not affect semantics.

r[expr.block.evaluation]
When evaluating a block expression, each statement, except for item declaration statements, is executed sequentially.

r[expr.block.result]
Then the final operand is executed, if given.

r[expr.block.type]
The type of a block is the type of the final operand, or `()` if the final operand is omitted.

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

> [!NOTE]
> As a control flow expression, if a block expression is the outer expression of an expression statement, the expected type is `()` unless it is followed immediately by a semicolon.

r[expr.block.value]
Blocks are always [value expressions] and evaluate the last operand in value expression context.

> [!NOTE]
> This can be used to force moving a value if really needed. For example, the following example fails on the call to `consume_self` because the struct was moved out of `s` in the block expression.
>
> ```rust,compile_fail
> struct Struct;
>
> impl Struct {
>     fn consume_self(self) {}
>     fn borrow_self(&self) {}
> }
>
> fn move_by_block_expression() {
>     let s = Struct;
>
>     // Move the value out of `s` in the block expression.
>     (&{ s }).borrow_self();
>
>     // Fails to execute because `s` is moved out of.
>     s.consume_self();
> }
> ```

r[expr.block.async]
## `async` blocks

r[expr.block.async.syntax]
```grammar,expressions
AsyncBlockExpression -> `async` `move`? BlockExpression
```

r[expr.block.async.intro]
An *async block* is a variant of a block expression which evaluates to a future.

r[expr.block.async.future-result]
The final expression of the block, if present, determines the result value of the future.

r[expr.block.async.anonymous-type]
Executing an async block is similar to executing a closure expression:
its immediate effect is to produce and return an anonymous type.

r[expr.block.async.future]
Whereas closures return a type that implements one or more of the [`std::ops::Fn`] traits, however, the type returned for an async block implements the [`std::future::Future`] trait.

r[expr.block.async.layout-unspecified]
The actual data format for this type is unspecified.

> [!NOTE]
> The future type that rustc generates is roughly equivalent to an enum with one variant per `await` point, where each variant stores the data needed to resume from its corresponding point.

r[expr.block.async.edition2018]
> [!EDITION-2018]
> Async blocks are only available beginning with Rust 2018.

r[expr.block.async.capture]
### Capture modes

Async blocks capture variables from their environment using the same [capture modes] as closures.
Like closures, when written `async { .. }` the capture mode for each variable will be inferred from the content of the block.
`async move { .. }` blocks however will move all referenced variables into the resulting future.

r[expr.block.async.context]
### Async context

Because async blocks construct a future, they define an **async context** which can in turn contain [`await` expressions].
Async contexts are established by async blocks as well as the bodies of async functions, whose semantics are defined in terms of async blocks.

r[expr.block.async.function]
### Control-flow operators

r[expr.block.async.function.intro]
Async blocks act like a function boundary, much like closures.

r[expr.block.async.function.return-try]
Therefore, the `?` operator and `return` expressions both affect the output of the future, not the enclosing function or other context.
That is, `return <expr>` from within an async block will return the result of `<expr>` as the output of the future.
Similarly, if `<expr>?` propagates an error, that error is propagated as the result of the future.

r[expr.block.async.function.control-flow]
Finally, the `break` and `continue` keywords cannot be used to branch out from an async block.
Therefore the following is illegal:

```rust,compile_fail
loop {
    async move {
        break; // error[E0267]: `break` inside of an `async` block
    }
}
```

r[expr.block.const]
## `const` blocks

r[expr.block.const.syntax]
```grammar,expressions
ConstBlockExpression -> `const` BlockExpression
```

r[expr.block.const.intro]
A *const block* is a variant of a block expression whose body evaluates at compile-time instead of at runtime.

r[expr.block.const.context]
Const blocks allows you to define a constant value without having to define new [constant items], and thus they are also sometimes referred as *inline consts*.
It also supports type inference so there is no need to specify the type, unlike [constant items].

r[expr.block.const.generic-params]
Const blocks have the ability to reference generic parameters in scope, unlike [free][free item] constant items.
They are desugared to constant items with generic parameters in scope (similar to associated constants, but without a trait or type they are associated with).
For example, this code:

```rust
fn foo<T>() -> usize {
    const { std::mem::size_of::<T>() + 1 }
}
```

is equivalent to:

```rust
fn foo<T>() -> usize {
    {
        struct Const<T>(T);
        impl<T> Const<T> {
            const CONST: usize = std::mem::size_of::<T>() + 1;
        }
        Const::<T>::CONST
    }
}
```

r[expr.block.const.evaluation]

If the const block expression is executed at runtime, then the constant is guaranteed to be evaluated, even if its return value is ignored:

```rust
fn foo<T>() -> usize {
    // If this code ever gets executed, then the assertion has definitely
    // been evaluated at compile-time.
    const { assert!(std::mem::size_of::<T>() > 0); }
    // Here we can have unsafe code relying on the type being non-zero-sized.
    /* ... */
    42
}
```

r[expr.block.const.not-executed]

If the const block expression is not executed at runtime, it may or may not be evaluated:
```rust,compile_fail
if false {
    // The panic may or may not occur when the program is built.
    const { panic!(); }
}
```

r[expr.block.unsafe]
## `unsafe` blocks

r[expr.block.unsafe.syntax]
```grammar,expressions
UnsafeBlockExpression -> `unsafe` BlockExpression
```

r[expr.block.unsafe.intro]
_See [`unsafe` blocks] for more information on when to use `unsafe`_.

A block of code can be prefixed with the `unsafe` keyword to permit [unsafe operations].
Examples:

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

r[expr.block.label]
## Labelled block expressions

Labelled block expressions are documented in the [Loops and other breakable expressions] section.

r[expr.block.attributes]
## Attributes on block expressions

r[expr.block.attributes.inner-attributes]
[Inner attributes] are allowed directly after the opening brace of a block expression in the following situations:

* [Function] and [method] bodies.
* Loop bodies ([`loop`], [`while`], and [`for`]).
* Block expressions used as a [statement].
* Block expressions as elements of [array expressions], [tuple expressions],
  [call expressions], and tuple-style [struct] expressions.
* A block expression as the tail expression of another block expression.
<!-- Keep list in sync with expressions.md -->

r[expr.block.attributes.valid]
The attributes that have meaning on a block expression are [`cfg`] and [the lint check attributes].

For example, this function returns `true` on unix platforms and `false` on other platforms.

```rust
fn is_unix_platform() -> bool {
    #[cfg(unix)] { true }
    #[cfg(not(unix))] { false }
}
```

[`await` expressions]: await-expr.md
[`cfg`]: ../conditional-compilation.md
[`for`]: loop-expr.md#iterator-loops
[`loop`]: loop-expr.md#infinite-loops
[`unsafe` blocks]: ../unsafe-keyword.md#unsafe-blocks-unsafe-
[`while`]: loop-expr.md#predicate-loops
[array expressions]: array-expr.md
[call expressions]: call-expr.md
[capture modes]: ../types/closure.md#capture-modes
[constant items]: ../items/constant-items.md
[free item]: ../glossary.md#free-item
[function]: ../items/functions.md
[inner attributes]: ../attributes.md
[method]: ../items/associated-items.md#methods
[mutable reference]: ../types/pointer.md#mutables-references-
[scopes]: ../names/scopes.md
[shared references]: ../types/pointer.md#shared-references-
[statement]: ../statements.md
[statements]: ../statements.md
[struct]: struct-expr.md
[the lint check attributes]: ../attributes/diagnostics.md#lint-check-attributes
[tuple expressions]: tuple-expr.md
[unsafe operations]: ../unsafety.md
[value expressions]: ../expressions.md#place-expressions-and-value-expressions
[Loops and other breakable expressions]: loop-expr.md#labelled-block-expressions
