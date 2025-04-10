r[type.fn-pointer]
# Function pointer types

r[type.fn-pointer.syntax]
```grammar,types
BareFunctionType ->
    ForLifetimes? FunctionTypeQualifiers `fn`
       `(` FunctionParametersMaybeNamedVariadic? `)` BareFunctionReturnType?

FunctionTypeQualifiers -> `unsafe`? (`extern` Abi?)?

BareFunctionReturnType -> `->` TypeNoBounds

FunctionParametersMaybeNamedVariadic ->
    MaybeNamedFunctionParameters | MaybeNamedFunctionParametersVariadic

MaybeNamedFunctionParameters ->
    MaybeNamedParam ( `,` MaybeNamedParam )* `,`?

MaybeNamedParam ->
    OuterAttribute* ( ( IDENTIFIER | `_` ) `:` )? Type

MaybeNamedFunctionParametersVariadic ->
    ( MaybeNamedParam `,` )* MaybeNamedParam `,` OuterAttribute* `...`
```

r[type.fn-pointer.intro]
Function pointer types, written using the `fn` keyword, refer to a function
whose identity is not necessarily known at compile-time.

An example where `Binop` is defined as a function pointer type:

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

let mut x = add(5,7);

type Binop = fn(i32, i32) -> i32;
let bo: Binop = add;
x = bo(5,7);
```

r[type.fn-pointer.coercion]
Function pointers can be created via a coercion from both [function items] and non-capturing, non-async [closures].

r[type.fn-pointer.qualifiers]
The `unsafe` qualifier indicates that the type's value is an [unsafe
function], and the `extern` qualifier indicates it is an [extern function].

r[type.fn-pointer.constraint-variadic]
Variadic parameters can only be specified with [`extern`] function types with
the `"C"` or `"cdecl"` calling convention.

This also includes the corresponding [`-unwind` variants][items.fn.extern.unwind].

r[type.fn-pointer.attributes]
## Attributes on function pointer parameters

Attributes on function pointer parameters follow the same rules and
restrictions as [regular function parameters].

[`extern`]: ../items/external-blocks.md
[closures]: closure.md
[extern function]: ../items/functions.md#extern-function-qualifier
[function items]: function-item.md
[unsafe function]: ../unsafe-keyword.md
[regular function parameters]: ../items/functions.md#attributes-on-function-parameters
