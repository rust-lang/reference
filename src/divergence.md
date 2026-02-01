r[divergence]
# Divergence

r[divergence.intro]
A *diverging expression* is an expression that never completes normal execution.

```rust
fn diverges() -> ! {
    panic!("This function never returns!");
}

fn example() {
    let x: i32 = diverges(); // This line never completes.
    println!("This is never printed: {x}");
}
```

See the following rules for specific expression divergence behavior:

- [expr.block.diverging] --- Block expressions.
- [expr.if.diverging] --- `if` expressions.
- [expr.loop.block-labels.type] --- Labeled block expressions with `break`.
- [expr.loop.break-value.diverging] --- `loop` expressions with `break`.
- [expr.loop.break.diverging] --- `break` expressions.
- [expr.loop.continue.diverging] --- `continue` expressions.
- [expr.loop.infinite.diverging] --- Infinite `loop` expressions.
- [expr.match.diverging] --- `match` expressions.
- [expr.match.empty] --- Empty `match` expressions.
- [expr.return.diverging] --- `return` expressions.
- [type.never.constraint] --- Function calls returning `!`.

> [!NOTE]
> The [`panic!`] macro and related panic-generating macros like [`unreachable!`] also have the type [`!`] and are diverging.

r[divergence.never]
Any expression of type [`!`] is a diverging expression. However, diverging expressions are not limited to type [`!`]; expressions of other types may also diverge (e.g., `Some(loop {})` has type `Option<!>`).

> [!NOTE]
> Though `!` is considered an uninhabited type, a type being uninhabited is not sufficient for it to diverge.
>
> ```rust,compile_fail,E0308
> enum Empty {}
> fn make_never() -> ! {loop{}}
> fn make_empty() -> Empty {loop{}}
>
> fn diverging() -> ! {
>     // This has a type of `!`.
>     // So, the entire function is considered diverging.
>     make_never();
>     // OK: The type of the body is `!` which matches the return type.
> }
> fn not_diverging() -> ! {
>     // This type is uninhabited.
>     // However, the entire function is not considered diverging.
>     make_empty();
>     // ERROR: The type of the body is `()` but expected type `!`.
> }
> ```

> [!NOTE]
> Divergence can propagate to the surrounding block. See [expr.block.diverging].

r[divergence.fallback]
## Fallback

If a type to be inferred is only unified with diverging expressions, then that type will be inferred to be [`!`].

> [!EXAMPLE]
> ```rust,compile_fail,E0277
> fn foo() -> i32 { 22 }
> match foo() {
>     // ERROR: The trait bound `!: Default` is not satisfied.
>     4 => Default::default(),
>     _ => return,
> };
> ```

> [!EDITION-2024]
> Before the 2024 edition, the type was inferred to instead be `()`.

> [!NOTE]
> Importantly, type unification may happen *structurally*, so the fallback `!` may be part of a larger type. The following compiles:
>
> ```rust
> fn foo() -> i32 { 22 }
> // This has the type `Option<!>`, not `!`
> match foo() {
>     4 => Default::default(),
>     _ => Some(return),
> };
> ```

<!-- TODO: This last point should likely should be moved to a more general "type inference" section discussing generalization + unification. -->

[`!`]: type.never
