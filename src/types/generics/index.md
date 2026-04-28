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

LifetimeParam -> Lifetime ( `:` LifetimeBounds? )?

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
    TypePathSegment `:` Bounds?
```

### Argument ordering and matching

r[generics.arguments.lifetime-order]
Lifetime arguments must appear before all other argument kinds.

> [!EXAMPLE]
> ```rust,compile_fail
> struct Foo<'a, T> {
>     data: &'a T,
> }
>
> // ERROR: lifetime argument `'static` must come before type argument `i32`
> let _: Foo<i32, 'static>;
> ```

r[generics.arguments.positional-matching]
Generic arguments are matched to generic parameters positionally:

- The i<sub>th</sub> lifetime argument corresponds to the i<sub>th</sub> lifetime parameter.
- The i<sub>th</sub> type or const argument corresponds to the i<sub>th</sub> type or const parameter (counted together in declaration order).

r[generics.arguments.constraint-order]
Associated item constraints must be listed after all other argument kinds, and may be listed in any order.

> [!EXAMPLE]
> ```rust
> use std::fmt::Display;
>
> trait Container {
>     type Item;
>     type Error;
> }
>
> // Associated item constraints may be listed in any order relative to each other.
> fn process<C: Container<Error = String, Item = i32>>(_: C) {}
> ```

> [!EXAMPLE]
> ```rust,compile_fail
> struct Foo<'a, T> {
>     data: &'a T,
> }
>
> trait MyTrait {
>     type Assoc;
> }
>
> // ERROR: associated item constraints must come after all other argument kinds.
> fn bad<T: MyTrait>(_: &dyn MyTrait<Assoc = i32>) where T: 'static {}
> let _: std::collections::HashMap<Item = i32, String, String>;
> ```

r[generics.arguments.lifetime-elision]
Lifetime arguments may be omitted in the following cases:

- When [lifetime elision] rules apply.
- In [turbofish] expressions (`::<...>`) where all lifetimes can be inferred.

> [!EXAMPLE]
> ```rust
> struct Foo<'a, T> {
>     data: &'a T,
> }
>
> fn make_foo<'a, T>(data: &'a T) -> Foo<'a, T> {
>     Foo { data }
> }
>
> // Turbofish: lifetime arguments omitted because they can be inferred.
> let x = 42i32;
> let foo = make_foo::<i32>(&x); // `'_` lifetime argument elided in turbofish
>
> // Lifetime arguments omitted in a type annotation.
> let foo: Foo<i32> = Foo { data: &x };
> ```

r[generics.arguments.all-lifetimes]
If any lifetime argument is provided, then all lifetime parameters must be specified.

> [!EXAMPLE]
> ```rust,compile_fail
> struct Foo<'a, 'b, T> {
>     x: &'a T,
>     y: &'b T,
> }
>
> fn make_foo<'a, 'b, T>(x: &'a T, y: &'b T) -> Foo<'a, 'b, T> {
>     Foo { x, y }
> }
>
> let a = 1i32;
> let b = 2i32;
>
> // OK: no lifetime arguments supplied (elided).
> let _: Foo<i32> = Foo { x: &a, y: &b };
> // OK: all lifetime arguments supplied.
> let _: Foo<'static, 'static, i32> = Foo { x: &1, y: &2 };
> // ERROR: only one of two lifetime arguments provided.
> let _: Foo<'static, i32> = Foo { x: &1, y: &b };
> ```

r[generics.arguments.defaults]
Type and const parameters with default values need not be supplied. A parameter without a default cannot follow one with a default.

When fewer arguments are supplied than parameters exist, the missing trailing arguments use their defaults if available, or are inferred if inference is enabled for that context.

r[generics.arguments.self-param]
The `Self` parameter (when present, e.g., in [trait definitions][items.traits.self-param]) is implicit and cannot be explicitly specified.

r[generics.arguments.impl-trait-params]
Synthetic type parameters corresponding to `impl Trait` types are implicit and cannot be explicitly specified.

r[generics.arguments.late-bound-lifetimes]
It is an error to provide explicit lifetime arguments when late-bound lifetimes are present.

> [!EXAMPLE]
> ```rust,compile_fail
> fn foo<'a>(x: &'a str) -> &'a str { x }
>
> // ERROR: cannot specify late-bound lifetime arguments explicitly
> foo::<'static>("hello");
> ```

<!--
FCW exists for non-value (function) position, see
https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html#late-bound-lifetime-arguments
-->

r[generics.arguments.inference]
### Infer arguments

r[generics.arguments.inference.intro]
The placeholder `_` may be used for type or const arguments when the compiler can infer the value.

> [!EXAMPLE]
> ```rust
> let v: Vec<_> = vec![1, 2, 3]; // _ inferred as i32
>
> type T<const N: usize> = [i32; N];
> let x: T<_> = [1]; // _ inferred as 1
> ```

> [!NOTE]
> The `_` placeholder cannot be used for lifetime arguments; use `'_` for elided lifetimes instead.

r[generics.arguments.inference.parentheses]
Infer arguments may be surrounded by any number of matching parentheses.

r[generics.associated]
## Associated item constraints

r[generics.associated.intro]
An *associated item constraint* constrains an [associated type] of a trait. There are two kinds of associated item constraints: equality constraints and bound constraints.

r[generics.associated.equality]
An *equality constraint* fixes the associated item to a specific type. It is specified with the [GenericArgsBinding] syntax.

> [!EXAMPLE]
> ```rust
> // The `Item` associated item is specified to be `i32`.
> fn sum_iter(iter: impl Iterator<Item = i32>) -> i32 {
>     iter.sum()
> }
> ```

r[generics.associated.bound]
A *bound constraint* requires the associated item to satisfy a [trait bound] without fixing it to a concrete type. It is specified with the [GenericArgsBounds] syntax.

> [!EXAMPLE]
> ```rust
> # use std::fmt::Display;
> #
> // The `Item` associated type is required to implement Display.
> fn print_iter(iter: impl Iterator<Item: Display>) {
>     for item in iter {
>         println!("{item}");
>     }
> }
> ```

r[generics.associated.constraints-position]
Associated item constraints are only permitted when the path refers to a [trait] in a type position. They are permitted in the following positions:

- [Trait bounds], including inline bounds (`T: Trait<Assoc = Type>`) and `where` clauses
- [`impl Trait`][impl trait] argument and return types
- [Trait object] types (`dyn Trait<Assoc = Type>`)

They are not permitted in the following positions:

- On non-trait generic type paths such as structs, enums, or type aliases: `Struct<Assoc = Type>`
- On the trait reference in an `impl` block header: `impl Trait<Assoc = Type> for OtherType`
- On the trait segment of a [qualified path]: `<Type as Trait<Assoc = X>>::AssocItem`
- On an associated item's path segment: `<Type as Trait>::AssocItem<Assoc2 = X>`
- In expression or method call [turbofish]: `Trait::<Assoc = X>::method()` or `value.method::<Assoc = X>()`

> [!EXAMPLE]
> The following are invalid uses of associated item constraints:
>
> ```rust,compile_fail
> // ERROR: constraint on a non-trait generic type (struct).
> struct Container<T>(T);
> fn f1() {
>     let _: Container<T = i32>;
> }
>
> // ERROR: constraint on the trait reference in an `impl` block header.
> trait Produce { type Output; }
> struct Widget;
> impl Produce<Output = i32> for Widget {
>     type Output = i32;
> }
>
> // ERROR: constraint on the trait segment of a qualified type path.
> trait Source { type Data; }
> fn f3<I: Source>(_: &<I as Source<Data = ()>>::Data) {}
>
> // ERROR: constraint in an expression-position turbofish.
> trait Create { type Item; fn new() -> i32 { 0 } }
> fn f4() { Create::<Item = i32>::new(); }
>
> // ERROR: constraint in a method call turbofish.
> fn f5() { 0u32.clone::<T = u32>(); }
> ```

r[generics.const]
## Const generics

r[generics.const.intro]
*Const generic parameters* allow items to be generic over constant values.

> [!EXAMPLE]
> ```rust
> struct Grid<const WIDTH: usize, const HEIGHT: usize> {
>     data: [[f32; WIDTH]; HEIGHT],
> }
>
> impl<const WIDTH: usize, const HEIGHT: usize> Grid<WIDTH, HEIGHT> {
>     fn new() -> Self {
>         Grid { data: [[0.0; WIDTH]; HEIGHT] }
>     }
>
>     fn size(&self) -> usize {
>         WIDTH * HEIGHT
>     }
> }
>
> let grid: Grid<4, 4> = Grid::new();
> assert_eq!(grid.size(), 16);
> ```

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

> [!EXAMPLE]
> ```rust
> // Examples where const generic parameters can be used.
>
> // Used in the signature of the item itself.
> fn foo<const N: usize>(arr: [i32; N]) {
>     // Used as a type within a function body.
>     let x: [i32; N];
>     // Used as an expression.
>     println!("{}", N * 2);
> }
>
> // Used as a field of a struct.
> struct Foo<const N: usize>([i32; N]);
>
> impl<const N: usize> Foo<N> {
>     // Used as an associated constant.
>     const CONST: usize = N * 4;
> }
>
> trait Trait {
>     type Output;
> }
>
> impl<const N: usize> Trait for Foo<N> {
>     // Used as an associated type.
>     type Output = [i32; N];
> }
> ```
>
> ```rust,compile_fail
> // Examples where const generic parameters cannot be used.
> fn foo<const N: usize>() {
>     // Cannot use in item definitions within a function body.
>     const BAD_CONST: [usize; N] = [1; N];
>     static BAD_STATIC: [usize; N] = [1; N];
>     fn inner(bad_arg: [usize; N]) {
>         let bad_value = N * 2;
>     }
>     type BadAlias = [usize; N];
>     struct BadStruct([usize; N]);
> }
> ```

r[generics.const.standalone]
As a further restriction, const parameters may only appear as a standalone argument inside of a [type] or [array repeat expression]. In those contexts, they may only be used as a single segment [path expression], possibly inside a [block] (such as `N` or `{N}`). That is, they cannot be combined with other expressions.

> [!EXAMPLE]
> ```rust,compile_fail
> // Examples where const parameters may not be used.
>
> // Not allowed to combine in other expressions in types, such as the
> // arithmetic expression in the return type here.
> fn bad_function<const N: usize>() -> [u8; {N + 1}] {
>     // Similarly not allowed for array repeat expressions.
>     [1; {N + 1}]
> }
> ```

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

r[generics.const.variance]
Unlike type and lifetime parameters, const parameters can be declared without being used inside of a parameterized item, with the exception of implementations as described in [generic implementations]:

> [!EXAMPLE]
> ```rust,compile_fail
> // ok
> struct Foo<const N: usize>;
> enum Bar<const M: usize> { A, B }
>
> // ERROR: unused parameter
> struct Baz<T>;
> struct Biz<'a>;
> struct Unconstrained;
> impl<const N: usize> Unconstrained {}
> ```

r[generics.const.exhaustiveness]
When resolving a trait bound obligation, the exhaustiveness of all implementations of const parameters is not considered when determining if the bound is satisfied. For example, in the following, even though all possible const values for the `bool` type are implemented, it is still an error that the trait bound is not satisfied:

> [!EXAMPLE]
> ```rust,compile_fail
> struct Foo<const B: bool>;
> trait Bar {}
> impl Bar for Foo<true> {}
> impl Bar for Foo<false> {}
>
> fn needs_bar(_: impl Bar) {}
> fn generic<const B: bool>() {
>     let v = Foo::<B>;
>     needs_bar(v); // ERROR: trait bound `Foo<B>: Bar` is not satisfied
> }
> ```

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

r[generics.const.arguments.type-ambiguity]
When there is ambiguity if a generic argument could be resolved as either a type or const argument, it is always resolved as a type. Placing the argument in a block expression can force it to be interpreted as a const argument.

<!-- TODO: Rewrite the paragraph above to be in terms of namespaces, once namespaces are introduced, and it is clear which namespace each parameter lives in. -->

> [!EXAMPLE]
> ```rust,compile_fail
> type N = u32;
> struct Foo<const N: usize>;
> // The following is an error, because `N` is interpreted as the type alias `N`.
> fn foo<const N: usize>() -> Foo<N> { todo!() } // ERROR
> // Can be fixed by wrapping in braces to force it to be interpreted as the `N`
> // const parameter:
> fn bar<const N: usize>() -> Foo<{ N }> { todo!() } // ok
> ```

r[generics.parameters.attributes]
## Attributes on generic parameters

The [built-in attributes] that have meaning on a generic parameter are [`cfg`] and [the lint check attributes].

> [!EXAMPLE]
> ```rust
> use std::fmt::Debug;
>
> struct Wrapper<
>     T,
>     #[cfg(feature = "debug")] U: Debug,
>     #[cfg(not(feature = "debug"))] U,
> > (T, U);
> ```

[`cfg`]: cfg.attr
[array repeat expression]: expr.array
[arrays]: type.array
[associated const]: items.associated.const
[associated type]: items.associated.type
[block]: expr.block
[built-in attributes]: attributes.builtin
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
[impl trait]: type.impl-trait
[implementations]: items.impl
[inferred const]: generics.const.inferred
[item declarations]: statement.item
[item]: items
[lifetime elision]: lifetime-elision
[literal]: expr.literal
[path expression]: expr.path
[path]: paths
[paths in expressions]: paths.expr
[paths in types]: paths.type
[qualified path]: paths.qualified
[raw pointers]: type.pointer.raw
[references]: type.pointer.reference
[slices]: type.slice
[structs]: items.struct
[the lint check attributes]: attributes.diagnostics.lint
[trait bound]: bound
[Trait bounds]: bound
[trait object]: type.trait-object
[traits]: items.traits
[tuples]: type.tuple
[turbofish]: paths.expr.turbofish
[type aliases]: items.type
[unions]: items.union
[value namespace]: names.namespaces
