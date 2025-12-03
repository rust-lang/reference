r[divergence]
# Divergence

r[divergence.intro]
Divergence is the state where a particular section of code could never be encountered at runtime. Importantly, while there are certain language constructs that immediately produce a _diverging expression_ of the type [`!`](./types/never.md), divergence can also propogate to the surrounding block.

Any expression of type [`!`](./types/never.md) is a _diverging expression_, but there are also diverging expressions which are not of type `!` (e.g. `Some(loop {})` produces a type of `Option<!>`).

> [!NOTE]
> Though `!` is considered an uninhabited type, a type being uninhabited is not sufficient for it to diverge.
>
> ```rust,compile_fail,E0308
> # #![ feature(never_type) ]
> # fn make<T>() -> T { loop {} }
> enum Empty {}
> fn diverging() -> ! {
>     // This has a type of `!`.
>     // So, the entire function is considered diverging
>     make::<!>();
> }
> fn not_diverging() -> ! {
>     // This type is uninhabited.
>     // However, the entire function is not considered diverging
>     make::<Empty>();
> }
> ```

r[divergence.fallback]
## Fallback
If a type to be inferred is only unified with diverging expressions, then that type will be inferred to be `!`.

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
> Importantly, type unification may happen *structurally*, so the fallback `!` may be part of a larger type. The > following compiles:
> ```rust
> fn foo() -> i32 { 22 }
> // This has the type `Option<!>`, not `!`
> match foo() {
>     4 => Default::default(),
>     _ => Some(return),
> };
> ```

<!-- TODO: This last point should likely should be moved to a more general "type inference" section discussing generalization + unification. -->
