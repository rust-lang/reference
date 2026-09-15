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

- [asm.diverging.naked_asm] --- `naked_asm!`.
- [asm.diverging.noreturn] --- `noreturn` in `asm!`.
- [expr.arith-logic.diverging] --- Arithmetic and logic expressions.
- [expr.array.diverging] --- Array expressions.
- [expr.array.index.diverging] --- Index expressions.
- [expr.as.diverging] --- `as` expressions.
- [expr.assign.diverging] --- Assignment expressions.
- [expr.await.diverging] --- `.await` expressions.
- [expr.block.async.diverging] --- Async block expressions.
- [expr.block.diverging] --- Block expressions.
- [expr.bool-logic.diverging] --- Lazy boolean expressions.
- [expr.borrow.diverging] --- Borrow expressions.
- [expr.call.diverging] --- Call expressions.
- [expr.closure.diverging] --- Closure expressions.
- [expr.cmp.diverging] --- Comparison expressions.
- [expr.compound-assign.diverging] --- Compound assignment expressions.
- [expr.deref.diverging] --- Dereference expressions.
- [expr.field.diverging] --- Field expressions.
- [expr.if.diverging] --- `if` expressions.
- [expr.literal.diverging] --- Literal expressions.
- [expr.loop.block-labels.type] --- Labeled block expressions with `break`.
- [expr.loop.break-value.diverging] --- `loop` expressions with `break`.
- [expr.loop.break.diverging] --- `break` expressions.
- [expr.loop.continue.diverging] --- `continue` expressions.
- [expr.loop.for.diverging] --- `for` expressions.
- [expr.loop.infinite.diverging] --- Infinite `loop` expressions.
- [expr.loop.while.diverging] --- `while` expressions.
- [expr.match.diverging-arms] and [expr.match.diverging-scrutinee] --- `match` expressions.
- [expr.match.empty] --- Empty `match` expressions.
- [expr.method.diverging] --- Method call expressions.
- [expr.negate.diverging] --- Negation expressions.
- [expr.paren.diverging] --- Parenthesized expressions.
- [expr.path.diverging] --- Path expressions.
- [expr.placeholder.diverging] --- Underscore expressions.
- [expr.range.diverging] --- Range expressions.
- [expr.return.diverging] --- `return` expressions.
- [expr.struct.diverging] --- Struct expressions.
- [expr.try.diverging] --- Try propagation expressions.
- [expr.tuple-index.diverging] --- Tuple indexing expressions.
- [expr.tuple.diverging] --- Tuple expressions.
- [statement.let.diverging] --- `let` statements.

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

r[divergence.place-read]
## Place expression reads for never types

r[divergence.place-read.intro]
When a [place expression] is used in a context that calls for a value, the place is *read*. However, certain contexts take the place itself without reading its value. In these contexts, when the place's type is the [never type], then the expression is not considered to diverge.

r[divergence.place-read.non-place]
A non-place expression always constitutes a read for divergence calculation.

r[divergence.place-read.not-read]
A place expression constitutes a read for divergence calculation **except** for the following contexts:

- As the operand of a [borrow expression] (`&place` or `&mut place`).
- As the scrutinee of a [`match` expression], when not every arm's pattern constitutes a read (see [divergence.place-read.patterns]).
- As the initializer of a [`let` condition], when the pattern does not constitute a read.
- As the initializer of a [`let` statement], when the pattern does not constitute a read.

> [!EXAMPLE]
> The following compile-fail examples are structured around [the behavior of block divergence][expr.block.diverging]. Each function returns `!`. Examples that contain a diverging place expression that is **not** read from cause the final type of the body to be `()`, which fails to type check. If the diverging place expression is read from, then every code path is diverging, and the type of the body would be `!` and it would compile successfully.
>
> ```rust,compile_fail,E0308
> fn borrow_operand_not_read(x: !) -> ! {
>     &x;
>     // ERROR: expected `!`, found `()`
> }
> ```
>
> ```rust,compile_fail,E0308
> fn match_scrutinee_not_read_with_non_reading_patterns(x: !) -> ! {
>     match x {
>         _ => {}
>     };
>     // ERROR: expected `!`, found `()`
> }
> ```
>
> In contrast, when an arm uses a binding pattern (which constitutes a read), the scrutinee `x: !` is read, and reading an uninhabited place diverges. Note that the match is a statement here, just like the non-reading example above. The difference is that this statement is now considered diverging, so the implicit `()` tail that follows it is unreachable and does not need to match the `!` return type:
>
> ```rust
> fn match_scrutinee_read_with_reading_patterns(x: !) -> ! {
>     match x {
>         a => {}
>     }; // OK: this statement diverges
> }
> ```

r[divergence.place-read.patterns]
A pattern *constitutes a read* of the value it is matched against in all cases except as follows:

- The [wildcard pattern] `_` does not constitute a read.
- An [or-pattern] `p1 | p2 | ...` constitutes a read only if *all* of its alternatives constitute a read.

```rust
fn pattern_is_read(x: !) -> ! {
    // OK: Pattern is read, so the match expression diverges.
    match x {
        a => {}
    };
}
```

```rust,compile_fail,E0308
fn wildcard_pattern_not_read(x: !) -> ! {
    match x {
        _ => {}
    };
    // ERROR: expected `!`, found `()`
}
```

```rust
fn or_pattern_read_in_all_patterns(x: !) -> ! {
    // OK: This statement is diverging.
    let (a | a) = x;
}
```

```rust,compile_fail,E0308
fn or_pattern_not_read_if_any_alternative_is_non_reading(x: !) -> ! {
    let (_ | _) = x;
    // ERROR: expected `!`, found `()`
}
```

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
[assignment expression]: expr.assign
[borrow expression]: expr.operator.borrow
[`let` condition]: expr.if.let
[`let` statement]: statement.let
[`match` expression]: expr.match
[never type]: type.never
[or-pattern]: patterns.or
[place expression]: expr.place-value.place-memory-location
[wildcard pattern]: patterns.wildcard.intro
