r[divergence]
# Divergence

r[divergence.intro]
Divergence is the state where a particular section of code could never be encountered at runtime. Importantly, while there are certain language constructs that immediately produce a _diverging expression_ of the type [`!`](./types/never.md), divergence can also propogate to the surrounding block.

Any expression of type [`!`](./types/never.md) is a _diverging expression_, but there are also diverging expressions which are not of type `!` (e.g. `Some(panic!())`).

r[divergence.diverging-expressions]
## Producing diverging expressions

r[divergence.diverging-expressions.unconditional]
The following language constructs unconditonally produce a _diverging expression_ of the type [`!`](./types/never.md):

* [A call to a function returning `!`.](./types/never.md#r-type.never.constraint)
* [A `loop` expression with no corresponding break.](./expressions/loop-expr.md#r-expr.loop.infinite.diverging)
* [A `break` expression](./expressions/loop-expr.md#r-expr.loop.break.type)
* [A `continue` expression](./expressions/loop-expr.md#r-expr.loop.continue.type)
* [A `return` expression](./expressions/return-expr.md#r-expr.return.type)
* [A `match` expression with no arms](./expressions/match-expr.md#r-expr.match.type.diverging.empty)
* [A `block` expression that it itself is diverging.](../expressions/block-expr.md#r-expr.block.type.diverging)

r[divergence.diverging-expressions.conditional]
In a control flow expression, if all arms diverge, then the entire expression also diverges.

r[divergence.fallback]
## Fallback
If a type to be inferred is only unified with diverging expressions, then that type will be inferred to be `!`.

The following fails to compile because `!` does not implement `Debug`:
```rust,compile_fail,E0277
fn foo() -> i32 { 22 }
match foo() {
    4 => Default::default(),
    _ => return,
};
```

> [!EDITION-2024]
> Before the 2024 edition, the type was inferred to instead be `()`.

Importantly, type unification may happen *structurally*, so the fallback `!` may be part of a larger type. The following compiles:
```rust
fn foo() -> i32 { 22 }
// This has the type `Option<!>`, not `!`
match foo() {
    4 => Default::default(),
    _ => Some(return),
};
```

<!-- TODO: This last point should likely should be moved to a more general "type inference" section discussing generalization + unification. -->
