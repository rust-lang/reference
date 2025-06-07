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

r[const-generics.kinds]
There are three kinds of arguments to a const parameter:
1. Standalone const parameters
2. Inferred consts
3. Arbitrary concrete expressions 

r[const-generics.standalone]
## Standalone const parameters

A const parameter can only be used in a const argument if it is a standalone usage.
The argument must be *only* a usage of a const parameter and can be wrapped
in at most one level of braces. That is, they cannot be combined with other
expressions.
```rust
// Examples of standalone uses of const parameters

fn foo<const N: usize>() {
    let a: [u8; N] = [10; N];
    let b: [u8; { N }] = a;
    foo::<N>();
    foo::<{ N }>();
}
```

Here `a` has type `[u8; N]`, an array with a length of `N`, referring to `foo`'s const parameter.

```rust,compile_fail
// Examples of non-standalone uses of const parameters

fn foo<const N: usize>() {
    let a: [u8; {{ N }}] = [10; (N)];
    foo::<{{ N }}>();
    foo::<(N)>();
}
```

r[const-generics.inferred]
## Inferred consts

r[const-generics.inferred.syntax]
```grammar,types
@root InferredConst -> `_`
```

The inferred const asks the compiler to infer the const if possible based on
the surrounding information available.

It cannot be used in item signatures.

It is often used in repeat expressions:
```rust
fn make_array() -> [u32; 2] {
    [Default::default(); _]
}
```

r[const-generics.concrete-expr]
## Concrete expressions

Most const expressions are allowed as const arguments:
```rust
// Example of a concrete expressions as an argument

fn make_array() -> [u8; 1 + 10 / 2] {
    [1; 6]
}
```

r[const-generics.concrete-expr.limitations]
There are a few limitations about what expressions are allowed:
1. Generic parameters may not be used
2. In-scope where clauses may not be used
3. Must be wrapped in braces in some cases

```rust,compile_fail
// Examples where expressions may not be used as they use generic parameters.

// Not allowed in the const argument for an arrays length
fn bad_function<const N: usize>() -> [u8; N + 1] {
    // Similarly not allowed for array repeat expressions' count argument.
    [1; N + 1]
}

// Using type parameters is also disallowed
fn type_parameters_disallowed<T>(_: [u8; size_of::<T>()]) {}
```

```rust,compile_fail
// Example where an expression may not be used as it depends on an in-scope
// where clause

fn bad_function(_: [u8; { let a: [u8]; 1 }])
where
    for<'a> [u8]: Sized, {}
```

The const expression must be a [block expression][block](surrounded with braces) unless it's:
- a single path segment (an [IDENTIFIER])
- a [literal] (with a possibly leading `-` token)
- an array length or repeat expression count

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