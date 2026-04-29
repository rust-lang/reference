r[generics]
# Generics

r[generics.intro]
Generics allow items to be parameterized by types, lifetimes, and constants. This allows definitions to be written in a flexible way that can be reused with different concrete types and values.

r[generics.parameters]
## Generic parameters

r[generics.parameters.intro]
[Functions], [type aliases], [structs], [enumerations], [unions], [traits], and [implementations] may be *parameterized* by types, constants, and lifetimes. These parameters are listed in angle <span class="parenthetical">brackets (`<...>`)</span>, usually immediately after the name of the item and before its definition. For implementations, which don't have a name, they come directly after `impl`.

> [!EXAMPLE]
> ```rust
> fn foo<'a, T>() {}
> trait A<U> {}
> struct Ref<'a, T> where T: 'a { r: &'a T }
> struct InnerArray<T, const N: usize>([T; N]);
> struct EitherOrderWorks<const N: bool, U>(U);
> ```

r[generics.parameters.syntax]
```grammar,items
GenericParams -> `<` ( GenericParam (`,` GenericParam)* `,`? )? `>`

GenericParam -> OuterAttribute* ( LifetimeParam | TypeParam | ConstParam )

LifetimeParam -> Lifetime ( `:` LifetimeBounds )?

TypeParam -> IDENTIFIER ( `:` Bounds? )? ( `=` Type )?

ConstParam ->
    `const` IDENTIFIER `:` Type
    ( `=` ( BlockExpression | IDENTIFIER | `-`?LiteralExpression ) )?
```

r[generics.parameters.decl-order]
The order of generic parameters is restricted to lifetime parameters and then type and const parameters intermixed.

r[generics.parameters.duplicate-params]
The same parameter name may not be declared more than once in a [GenericParams] list.

r[generics.parameters.scope]
Generic parameters are in scope within the item definition where they are declared. They are not in scope for items declared within the body of a function as described in [item declarations]. See [generic parameter scopes] for more details.

r[generics.parameters.builtin-generic-types]
[References], [raw pointers], [arrays], [slices], [tuples], and [function pointers] have lifetime or type parameters as well, but are not referred to with path syntax.

r[generics.parameters.invalid-lifetimes]
`'_` and `'static` are not valid lifetime parameter names.

r[generics.arguments]
## Generic arguments

r[generics.arguments.intro]
Generic arguments are the concrete values provided for generic parameters and associated types when using a parameterized item. They are specified in angle brackets (`<...>`) following the item's path (see [paths in types] and [paths in expressions]). Generic arguments consist of:

1. *Lifetime arguments* (e.g., `'a`)
2. *Type arguments* (e.g., `T`, `Vec<i32>`)
3. *Const arguments* (e.g., `{ N }`, `{ 1 + 2 }`)
4. *Infer arguments* (`_`)
5. *Associated item constraints* (e.g., `Item = T`, `Item: Bound`)

> [!EXAMPLE]
> ```rust
> # struct Foo<'a, T, const N: usize> {
> #     data: &'a [T; N],
> # }
> #
> # fn make_foo<'a, T, const N: usize>(data: &'a [T; N]) -> Foo<'a, T, N> {
> #     Foo { data }
> # }
> #
> // Generic arguments in a type path: lifetime 'static, type i32, const value 3.
> let foo: Foo<'static, i32, 3> = Foo { data: &[1, 2, 3] };
> // Generic arguments in an expression path.
> make_foo::<i32, 3>(&[1, 2, 3]);
> ```

r[generics.arguments.syntax]
```grammar,paths
GenericArgs ->
      `<` GenericArgList? `>`
    | `(` TypeList? `)` (`->` TypeNoBounds)?

GenericArgList ->
    ( GenericArg `,` )* GenericArg `,`?

TypeList ->
    ( Type `,` )* Type `,`?

GenericArg ->
    Lifetime | Type | GenericArgsConst | GenericArgsBinding | GenericArgsBounds

GenericArgsConst ->
      BlockExpression
    | LiteralExpression
    | `-` LiteralExpression
    | SimplePathSegment

GenericArgsBinding ->
    TypePathSegment `=` Type

GenericArgsBounds ->
    TypePathSegment `:` Bounds
```

r[generics.arguments.argument-order]
The order of generic arguments is restricted to lifetime arguments, then type arguments, then const arguments, then equality constraints.

r[generics.arguments.impl-trait-params]
The synthetic type parameters corresponding to `impl Trait` types are implicit, and these cannot be explicitly specified.

r[generics.const]
## Const generics

r[generics.const.intro]
*Const generic parameters* allow items to be generic over constant values.

r[generics.const.namespace]
The const identifier introduces a name in the [value namespace] for the constant parameter, and all instances of the item must be instantiated with a value of the given type.

r[generics.const.allowed-types]
The only allowed types of const parameters are `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `char` and `bool`.

r[generics.const.use]
Const parameters can be used anywhere a [const item] can be used, with the exception that when used in a [type] or [array repeat expression], it must be standalone (as described below). That is, they are allowed in the following places:

1. As an applied const to any type which forms a part of the signature of the item in question.
2. As part of a const expression used to define an [associated const], or as a parameter to an [associated type].
3. As a value in any runtime expression in the body of any functions in the item.
4. As a parameter to any type used in the body of any functions in the item.
5. As a part of the type of any fields in the item.

```rust
// Examples where const generic parameters can be used.

// Used in the signature of the item itself.
fn foo<const N: usize>(arr: [i32; N]) {
    // Used as a type within a function body.
    let x: [i32; N];
    // Used as an expression.
    println!("{}", N * 2);
}

// Used as a field of a struct.
struct Foo<const N: usize>([i32; N]);

impl<const N: usize> Foo<N> {
    // Used as an associated constant.
    const CONST: usize = N * 4;
}

trait Trait {
    type Output;
}

impl<const N: usize> Trait for Foo<N> {
    // Used as an associated type.
    type Output = [i32; N];
}
```

```rust,compile_fail
// Examples where const generic parameters cannot be used.
fn foo<const N: usize>() {
    // Cannot use in item definitions within a function body.
    const BAD_CONST: [usize; N] = [1; N];
    static BAD_STATIC: [usize; N] = [1; N];
    fn inner(bad_arg: [usize; N]) {
        let bad_value = N * 2;
    }
    type BadAlias = [usize; N];
    struct BadStruct([usize; N]);
}
```

r[generics.const.standalone]
As a further restriction, const parameters may only appear as a standalone argument inside of a [type] or [array repeat expression]. In those contexts, they may only be used as a single segment [path expression], possibly inside a [block] (such as `N` or `{N}`). That is, they cannot be combined with other expressions.

```rust,compile_fail
// Examples where const parameters may not be used.

// Not allowed to combine in other expressions in types, such as the
// arithmetic expression in the return type here.
fn bad_function<const N: usize>() -> [u8; {N + 1}] {
    // Similarly not allowed for array repeat expressions.
    [1; {N + 1}]
}
```

r[generics.const.inferred]
Where a const argument is expected, an `_` (optionally surrounded by any number of matching parentheses), called the *inferred const* ([generic argument rules][generics.arguments.complex-const-params], [array expression rules][expr.array.length-restriction]), can be used instead. This asks the compiler to infer the const argument if possible based on surrounding information.

```rust
fn make_buf<const N: usize>() -> [u8; N] {
    [0; _]
    //  ^ Infers `N`.
}
let _: [u8; 1024] = make_buf::<_>();
//                             ^ Infers `1024`.
```

> [!NOTE]
> An [inferred const] is not semantically an [expression][Expression] and so is not accepted within braces.
>
> ```rust,compile_fail
> fn f<const N: usize>() -> [u8; N] { [0; _] }
> let _: [_; 1] = f::<{ _ }>();
> //                    ^ ERROR `_` not allowed here
> ```

r[generics.parameters.const.inferred.constraint]
The inferred const cannot be used in item signatures.

```rust,compile_fail
fn f<const N: usize>(x: [u8; N]) -> [u8; _] { x }
//                                       ^ ERROR not allowed
```

r[generics.const.type-ambiguity]
When there is ambiguity if a generic argument could be resolved as either a type or const argument, it is always resolved as a type. Placing the argument in a block expression can force it to be interpreted as a const argument.

<!-- TODO: Rewrite the paragraph above to be in terms of namespaces, once namespaces are introduced, and it is clear which namespace each parameter lives in. -->

```rust,compile_fail
type N = u32;
struct Foo<const N: usize>;
// The following is an error, because `N` is interpreted as the type alias `N`.
fn foo<const N: usize>() -> Foo<N> { todo!() } // ERROR
// Can be fixed by wrapping in braces to force it to be interpreted as the `N`
// const parameter:
fn bar<const N: usize>() -> Foo<{ N }> { todo!() } // ok
```

r[generics.const.variance]
Unlike type and lifetime parameters, const parameters can be declared without being used inside of a parameterized item, with the exception of implementations as described in [generic implementations]:

```rust,compile_fail
// ok
struct Foo<const N: usize>;
enum Bar<const M: usize> { A, B }

// ERROR: unused parameter
struct Baz<T>;
struct Biz<'a>;
struct Unconstrained;
impl<const N: usize> Unconstrained {}
```

r[generics.const.exhaustiveness]
When resolving a trait bound obligation, the exhaustiveness of all implementations of const parameters is not considered when determining if the bound is satisfied. For example, in the following, even though all possible const values for the `bool` type are implemented, it is still an error that the trait bound is not satisfied:

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

r[generics.const.arguments]
### Const arguments

r[generics.const.arguments.intro]
A *const argument* specifies the const value to use for a const parameter.

r[generics.const.arguments.const-expr]
A const argument must either be an [inferred const] or be a [const expression] of the type ascribed to the const parameter.

> [!NOTE]
> In a generic argument list, an [inferred const] is parsed as an [inferred type][InferredType] but then semantically treated as a separate kind of [const generic argument].

r[generics.const.arguments.complex-const-params]
Const argument expressions must be surrounded by braces unless they are a [literal] (with a possibly leading `-` token) or a single segment path.

> [!NOTE]
> This syntactic restriction is necessary to avoid requiring infinite lookahead when parsing an expression inside of a type.

> [!EXAMPLE]
> ```rust
> struct S<const N: i64>;
> const C: i64 = 1;
> fn f<const N: i64>() -> S<N> { S }
>
> let _ = f::<1>(); // Literal.
> let _ = f::<-1>(); // Negative literal.
> let _ = f::<{ 1 + 2 }>(); // Constant expression.
> let _ = f::<C>(); // Single segment path.
> let _ = f::<{ C + 1 }>(); // Constant expression.
> let _: S<1> = f::<_>(); // Inferred const.
> let _: S<1> = f::<(((_)))>(); // Inferred const.
> ```
>
> ```rust,compile_fail
> fn f<const N: usize>() -> [u8; N] { [0; _] }
> let _: [_; 1] = f::<{ _ }>();
> //                    ^ ERROR `_` not allowed here
> ```

r[generics.parameters.attributes]
## Attributes

Generic lifetime and type parameters allow [attributes] on them. There are no built-in attributes that do anything in this position, although custom derive attributes may give meaning to it.

This example shows using a custom derive attribute to modify the meaning of a generic parameter.

<!-- ignore: requires proc macro derive -->
```rust,ignore
// Assume that the derive for MyFlexibleClone declared `my_flexible_clone` as
// an attribute it understands.
#[derive(MyFlexibleClone)]
struct Foo<#[my_flexible_clone(unbounded)] H> {
    a: *const H
}
```

[array repeat expression]: expr.array
[arrays]: type.array
[associated const]: items.associated.const
[associated type]: items.associated.type
[block]: expr.block
[const contexts]: const-eval.const-context
[const expression]: const-eval.const-expr
[const generic argument]: generics.const.argument
[const item]: items.const
[enumerations]: items.enum
[function pointers]: type.fn-pointer
[functions]: items.fn
[generic implementations]: items.impl.generics
[generic parameter scopes]: names.scopes.generic-parameters
[higher-ranked lifetimes]: bound.higher-ranked
[implementations]: items.impl
[inferred const]: generics.const.inferred
[item declarations]: statement.item
[item]: items
[literal]: expr.literal
[path expression]: expr.path
[path]: paths
[paths in expressions]: paths.expr
[paths in types]: paths.type
[raw pointers]: type.pointer.raw
[references]: type.pointer.reference
[slices]: type.slice
[structs]: items.struct
[trait object]: type.trait-object
[traits]: items.traits
[tuples]: type.tuple
[type aliases]: items.type
[unions]: items.union
[value namespace]: names.namespaces
