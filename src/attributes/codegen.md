# Code generation attributes

The following [attributes] are used for controlling code generation.

## Optimization hints

The `cold` and `inline` [attributes] give suggestions to the compiler to
compile your code in a way that may be faster than what it would do without
the hint. The attributes are only suggestions, and the compiler may choose to
ignore it.

Both attributes can be used on [functions]. When applied to a function in a
[trait], they apply only to that function when used as a default function for
a trait implementation and not to all trait implementations. The attributes
have no effect on a trait function without a body.

### The `inline` attribute

The *`inline` [attribute]* suggests to the compiler that it should place a
copy of the attributed function in the caller, rather than generating code to
call the function where it is defined.

> ***Note***: The compiler automatically inlines functions based on internal
> heuristics. Incorrectly inlining functions can actually make the program
> slower, so this attribute should be used with care.

There are three ways to use the inline attribute:

* `#[inline]` hints the compiler to perform an inline expansion.
* `#[inline(always)]` asks the compiler to always perform an inline expansion.
* `#[inline(never)]` asks the compiler to never perform an inline expansion.

### The `cold` attribute

The *`cold` [attribute]* suggests to the compiler that the attributed function
is unlikely to be called.

## The `no_builtins` attribute

The *`no_builtins` [attribute]* may be applied at the crate level to disable
optimizing certain code patterns to invocations of library functions that are
assumed to exist.

[attribute]: attributes.html
[attributes]: attributes.html
[functions]: items/functions.html
[trait]: items/traits.html
