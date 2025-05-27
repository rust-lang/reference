r[const-generics]
# Const Generics

r[const-generics.argument]
A const argument in a [path] specifies the const value to use for that item.

r[const-generics.argument.type]
The argument must be a [const expression] of the type ascribed to the const
parameter. 

r[items.generics.const.type-ambiguity]
When there is ambiguity if a generic argument could be resolved as either a
type or const argument, it is always resolved as a type. Placing the argument
in a block expression can force it to be interpreted as a const argument.

<!-- TODO: Rewrite the paragraph above to be in terms of namespaces, once
    namespaces are introduced, and it is clear which namespace each parameter
    lives in. -->

```rust,compile_fail
type N = u32;
struct Foo<const N: usize>;
// The following is an error, because `N` is interpreted as the type alias `N`.
fn foo<const N: usize>() -> Foo<N> { todo!() } // ERROR
// Can be fixed by wrapping in braces to force it to be interpreted as the `N`
// const parameter:
fn bar<const N: usize>() -> Foo<{ N }> { todo!() } // ok
```

r[items.generics.const.exhaustiveness]
When resolving a trait bound obligation, the exhaustiveness of all
implementations of const parameters is not considered when determining if the
bound is satisfied. For example, in the following, even though all possible
const values for the `bool` type are implemented, it is still an error that
the trait bound is not satisfied:

```rust,compile_fail
struct Foo<const B: bool>;
trait Bar {}
impl Bar for Foo<true> {}
impl Bar for Foo<false> {}

fn needs_bar(_: impl Bar) {}
fn generic<const B: bool>() {
    let v = Foo::<B>;
    needs_bar(v); // ERROR: trait bound `Foo<B>: Bar` is not satisfied
}
```

r[items.generics.const.standalone]
As a further restriction, const parameters may only appear as a standalone
argument inside of a [type] or [array repeat expression]. In those contexts,
they may only be used as a single segment [path expression], possibly inside a
[block] (such as `N` or `{N}`). That is, they cannot be combined with other
expressions.

```rust,compile_fail
// Examples where const parameters may not be used.

// Not allowed to combine in other expressions in types, such as the
// arithmetic expression in the return type here.
fn bad_function<const N: usize>() -> [u8; {N + 1}] {
    // Similarly not allowed for array repeat expressions.
    [1; {N + 1}]
}
```

The const expression must be a [block expression][block]
(surrounded with braces) unless it is a single path segment (an [IDENTIFIER])
or a [literal] (with a possibly leading `-` token).

> [!NOTE]
> This syntactic restriction is necessary to avoid requiring infinite lookahead when parsing an expression inside of a type.

```rust
fn double<const N: i32>() {
    println!("doubled: {}", N * 2);
}

const SOME_CONST: i32 = 12;

fn example() {
    // Example usage of a const argument.
    double::<9>();
    double::<-123>();
    double::<{7 + 8}>();
    double::<SOME_CONST>();
    double::<{ SOME_CONST + 5 }>();
}
```

[block]: ../expressions/block-expr.md
[const expression]: ../const_eval.md#constant-expressions
[literal]: ../expressions/literal-expr.md
[path]: ../paths.md
