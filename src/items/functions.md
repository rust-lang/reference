# Functions

> **<sup>Syntax</sup>**\
> _Function_ :\
> &nbsp;&nbsp; _FunctionFront_ `fn` [IDENTIFIER]&nbsp;[_Generics_]<sup>?</sup>\
> &nbsp;&nbsp; &nbsp;&nbsp; `(` _FunctionParameters_<sup>?</sup> `)`\
> &nbsp;&nbsp; &nbsp;&nbsp; _FunctionReturnType_<sup>?</sup> [_WhereClause_]<sup>?</sup>\
> &nbsp;&nbsp; &nbsp;&nbsp; [_BlockExpression_]
>
> _FunctionFront_ :\
> &nbsp;&nbsp; `const`<sup>?</sup> `unsafe`<sup>?</sup> (`extern` _Abi_<sup>?</sup>)<sup>?</sup>
>
> _Abi_ :\
> &nbsp;&nbsp; [STRING_LITERAL] | [RAW_STRING_LITERAL]
>
> _FunctionParameters_ :\
> &nbsp;&nbsp; _FunctionParam_ (`,` _FunctionParam_)<sup>\*</sup> `,`<sup>?</sup>
>
> _FunctionParam_ :\
> &nbsp;&nbsp; [_Pattern_] `:` [_Type_]
>
> _FunctionReturnType_ :\
> &nbsp;&nbsp; `->` [_Type_]

A _function_ consists of a [block], along with a name and a set of parameters.
Other than a name, all these are optional. Functions are declared with the
keyword `fn`. Functions may declare a set of *input* [*variables*][variables]
as parameters, through which the caller passes arguments into the function, and
the *output* [*type*][type] of the value the function will return to its caller
on completion.

When referred to, a _function_ yields a first-class *value* of the
corresponding zero-sized [*function item type*], which
when called evaluates to a direct call to the function.

For example, this is a simple function:
```rust
fn answer_to_life_the_universe_and_everything() -> i32 {
    return 42;
}
```

As with `let` bindings, function arguments are irrefutable [patterns], so any
pattern that is valid in a let binding is also valid as an argument:

```rust
fn first((value, _): (i32, i32)) -> i32 { value }
```

The block of a function is conceptually wrapped in a block that binds the
argument patterns and then `return`s the value of the function's block. This
means that the tail expression of the block, if evaluated, ends up being
returned to the caller. As usual, an explicit return expression within
the body of the function will short-cut that implicit return, if reached.

For example, the function above behaves as if it was written as:

```rust,ignore
// argument_0 is the actual first argument passed from the caller
let (value, _) = argument_0;
return {
    value
};
```

## Generic functions

A _generic function_ allows one or more _parameterized types_ to appear in its
signature. Each type parameter must be explicitly declared in an
angle-bracket-enclosed and comma-separated list, following the function name.

```rust
// foo is generic over A and B

fn foo<A, B>(x: A, y: B) {
# }
```

Inside the function signature and body, the name of the type parameter can be
used as a type name. [Trait] bounds can be specified for type
parameters to allow methods with that trait to be called on values of that
type. This is specified using the `where` syntax:

```rust
# use std::fmt::Debug;
fn foo<T>(x: T) where T: Debug {
# }
```

When a generic function is referenced, its type is instantiated based on the
context of the reference. For example, calling the `foo` function here:

```rust
use std::fmt::Debug;

fn foo<T>(x: &[T]) where T: Debug {
    // details elided
}

foo(&[1, 2]);
```

will instantiate type parameter `T` with `i32`.

The type parameters can also be explicitly supplied in a trailing [path]
component after the function name. This might be necessary if there is not
sufficient context to determine the type parameters. For example,
`mem::size_of::<u32>() == 4`.

## Extern functions

Extern functions are part of Rust's foreign function interface, providing the
opposite functionality to [external blocks]. Whereas external
blocks allow Rust code to call foreign code, extern functions with bodies
defined in Rust code _can be called by foreign code_. They are defined in the
same way as any other Rust function, except that they have the `extern`
modifier.

```rust
// Declares an extern fn, the ABI defaults to "C"
extern fn new_i32() -> i32 { 0 }

// Declares an extern fn with "stdcall" ABI
# #[cfg(target_arch = "x86_64")]
extern "stdcall" fn new_i32_stdcall() -> i32 { 0 }
```

Unlike normal functions, extern fns have type `extern "ABI" fn()`. This is the
same type as the functions declared in an extern block.

```rust
# extern fn new_i32() -> i32 { 0 }
let fptr: extern "C" fn() -> i32 = new_i32;
```

As non-Rust calling conventions do not support unwinding, unwinding past the end
of an extern function will cause the process to abort. In LLVM, this is
implemented by executing an illegal instruction.

## Attributes on functions

[Outer attributes][attributes] are allowed on functions. [Inner
attributes][attributes] are allowed directly after the `{` inside its [block].

This example shows an inner attribute on a function. The function will only be
available while running tests.

```
fn test_only() {
    #![test]
}
```

> Note: Except for lints, it is idiomatic to only use outer attributes on
> function items.

The attributes that have meaning on a function are [`cfg`], [`deprecated`],
[`doc`], `export_name`, `link_section`, `no_mangle`, [the lint check
attributes], [`must_use`], [the procedural macro attributes], [the testing
attributes], and [the optimization hint
attributes].

## Const functions

Functions can be `const`, meaning they can be called from within array length expressions and the initializer of constants, statics and enum discriminants. When called from such a so-called "const context", the function is interpreted by the compiler at compile time. The interpretation happens in the environment of the compilation target and not the host. So `usize` is `32` bits if you are compiling against a `32` bit system, irrelevant of whether you are building on a `64` bit or a `32` bit system.

If a `const fn` is called outside a "const context", it is indistinguishable from any other function. You can freely do anything with a `const fn` that you can do with a regular fn.

`const fn`s have various restrictions to makes sure that you cannot define a `const fn` that can't be evaluated at compile-time. You will, for example, never be able to write a random number generator as a const fn. Calling a const fn at compile-time will always yield the same result as calling it at runtime, even if you call it multiple times. There's one exception to this rule: if you are doing complex floating point operations in extreme situations, then you might get (very slightly) different results. It is adviseable to not make array lengths and enum discriminants depend on floating point computations.

Exhaustive list of permitted structures in `const fn`:

1. type parameters where the parameters have any of the following as part of their bounds (either on `where` or directly on the parameters):
    1. lifetimes
    2. `Sized`

    This means that `<T: 'a + ?Sized>` and `<T: 'b + Sized>` + `<T>` are all permitted.
    Note that `?Sized` is the absence of a constraint when bounds have been fully elaborated
    which includes adding implicit `Sized` bounds.
    This entails that permitting `Sized` + lifetimes allows the above examples.

    This rule also applies to type parameters of items that contain `const fn`s.

2. arithmetic operators on integers
3. boolean operators (except for `&&` and `||` which are banned since they are short-circuiting).
4. any kind of aggregate constructor (array, `struct`, `enum`, tuple, ...)
5. calls to other *safe* `const fn`s (methods and functions)
6. index operations on arrays and slices
7. field accesses on structs and tuples
8. reading from constants (but not statics, not even taking a reference to a static)
9. `&` and `*` (only dereferencing of references, not raw pointers)
10. casts except for raw pointer to `usize` casts
11. `const unsafe fn` is allowed, but the body must consist of safe operations only and you won't be able to call the `const unsafe fn` from within another `const fn` even if you use `unsafe`

[IDENTIFIER]: identifiers.html
[RAW_STRING_LITERAL]: tokens.html#raw-string-literals
[STRING_LITERAL]: tokens.html#string-literals
[_BlockExpression_]: expressions/block-expr.html
[_Generics_]: items/generics.html
[_InnerAttribute_]: attributes.html
[_Pattern_]: patterns.html
[_Statement_]: statements.html
[_Type_]: types.html
[_WhereClause_]: items/generics.html#where-clauses
[external blocks]: items/external-blocks.html
[path]: paths.html
[block]: expressions/block-expr.html
[variables]: variables.html
[type]: types.html
[*function item type*]: types.html#function-item-types
[Trait]: items/traits.html
[attributes]: attributes.html
[`cfg`]: conditional-compilation.html
[the lint check attributes]: attributes.html#lint-check-attributes
[the procedural macro attributes]: procedural-macros.html
[the testing attributes]: attributes.html#testing
[the optimization hint attributes]: attributes.html#optimization-hints
[`deprecated`]: attributes.html#deprecation
[`doc`]: attributes.html#documentation
[`must_use`]: attributes.html#must_use
[patterns]: patterns.html
