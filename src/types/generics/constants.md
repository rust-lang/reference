r[generics.const]
# Generic constants

r[generics.const.intro]
*Generic constant parameters* allow items to be generic over constant values.

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
> // Examples where generic constant parameters can be used.
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
> // Examples where generic constant parameters cannot be used.
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
## Const arguments

r[generics.const.arguments.intro]
A *const argument* specifies the const value to use for a const parameter.

r[generics.const.arguments.const-expr]
A const argument must either be an [inferred const] or be a [const expression] of the type ascribed to the const parameter.

> [!NOTE]
> In a generic argument list, an [inferred const] is parsed as an [inferred type][InferredType] but then semantically treated as a separate kind of [generic const argument].

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

r[generics.const.arguments.inferred]
## Inferred const arguments

r[generics.const.arguments.inferred.intro]
An *inferred const* is a const argument specified with `_`. This asks the compiler to infer the const argument if possible based on surrounding information.

> [!EXAMPLE]
> ```rust
> fn make_buf<const N: usize>() -> [u8; N] {
>     [0; _]
>     //  ^ Infers `N`.
> }
> let _: [u8; 1024] = make_buf::<_>();
> //                             ^ Infers `1024`.
> ```

> [!NOTE]
> An [inferred const] is not semantically an [expression][Expression] and so is not accepted within braces.
>
> ```rust,compile_fail
> fn f<const N: usize>() -> [u8; N] { [0; _] }
> let _: [_; 1] = f::<{ _ }>();
> //                    ^ ERROR `_` not allowed here
> ```

r[generics.const.arguments.inferred.signature]
The inferred const cannot be used in item signatures.

> [!EXAMPLE]
> ```rust,compile_fail
> fn f<const N: usize>(x: [u8; N]) -> [u8; _] { x }
> //                                       ^ ERROR not allowed
> ```

[array repeat expression]: expr.array
[associated const]: items.associated.const
[associated type]: items.associated.type
[block]: expr.block
[const expression]: const-eval.const-expr
[const item]: items.const
[generic const argument]: generics.const.arguments
[generic implementations]: items.impl.generics
[inferred const]: generics.const.arguments.inferred
[literal]: expr.literal
[path expression]: expr.path
[value namespace]: names.namespaces
