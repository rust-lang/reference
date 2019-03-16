# Code generation attributes

The following [attributes] are used for controlling code generation.

## Optimization hints

The `cold` and `inline` [attributes] give suggestions to generate code in a
way that may be faster than what it would do without the hint. The attributes
are only hints, and may be ignored.

Both attributes can be used on [functions]. When applied to a function in a
[trait], they apply only to that function when used as a default function for
a trait implementation and not to all trait implementations. The attributes
have no effect on a trait function without a body.

### The `inline` attribute

The *`inline` [attribute]* suggests that a copy of the attributed function
should be placed in the caller, rather than generating code to call the
function where it is defined.

> ***Note***: The `rustc` compiler automatically inlines functions based on
> internal heuristics. Incorrectly inlining functions can make the program
> slower, so this attribute should be used with care.

There are three ways to use the inline attribute:

* `#[inline]` suggests performing an inline expansion.
* `#[inline(always)]` suggests that an inline expansion should always be
  performed.
* `#[inline(never)]` suggests that an inline expansion should never be
  performed.

### The `cold` attribute

The *`cold` [attribute]* suggests that the attributed function is unlikely to
be called.

## The `no_builtins` attribute

The *`no_builtins` [attribute]* may be applied at the crate level to disable
optimizing certain code patterns to invocations of library functions that are
assumed to exist.

[attribute]: attributes.html
[attributes]: attributes.html
[functions]: items/functions.html
[trait]: items/traits.html
