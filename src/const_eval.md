# Constant evaluation

Constant evaluation is the process of computing the result of
[expressions] during compilation. Only a subset of all expressions
can be evaluated at compile-time.

## Constant expressions

Certain forms of expressions, called constant expressions, can be evaluated at
compile time. In [const contexts](#const-context), these are the only allowed
expressions, and are always evaluated at compile time. In other places, such as
[let statements], constant expressions *may*
be, but are not guaranteed to be, evaluated at compile time. Behaviors such as
out of bounds [array indexing] or [overflow] are compiler errors if the value
must be evaluated at compile time (i.e. in const contexts). Otherwise, these
behaviors are warnings, but will likely panic at run-time.

The following expressions are constant expressions, so long as any operands are
also constant expressions and do not cause any [`Drop::drop`][destructors] calls
to be run.

* [Literals].
* [Paths] to [functions] and constants.
  Recursively defining constants is not allowed.
* [Tuple expressions].
* [Array expressions].
* [Struct] expressions.
* [Enum variant] expressions.
* [Block expressions], including `unsafe` blocks.
    * [let statements] and thus irrefutable [patterns], with the caveat that until `if` and `match`
    are implemented, one cannot use both short circuiting operators (`&&` and `||`) and let
    statements within the same constant.
    * [assignment expressions]
    * [compound assignment expressions]
    * [expression statements]
* [Field] expressions.
* Index expressions, [array indexing] or [slice] with a `usize`.
* [Range expressions].
* [Closure expressions] which don't capture variables from the environment.
* Built-in [negation], [arithmetic], [logical], [comparison] or [lazy boolean]
  operators used on integer and floating point types, `bool`, and `char`.
* Shared [borrow]s, except if applied to a type with [interior mutability].
* The [dereference operator].
* [Grouped] expressions.
* [Cast] expressions, except pointer to address and
  function pointer to address casts.
* Calls of [const functions] and const methods.

## Const context

A _const context_ is one of the following:

* [Array type length expressions]
* Repeat expression length expressions
* The initializer of
  * [constants]
  * [statics]
  * [enum discriminants]

[arithmetic]:           expressions/operator-expr.md#arithmetic-and-logical-binary-operators
[array expressions]:    expressions/array-expr.md
[array indexing]:       expressions/array-expr.md#array-and-slice-indexing-expressions
[array indexing]:       expressions/array-expr.md#array-and-slice-indexing-expressions
[array type length expressions]: types/array.md
[assignment expressions]: expressions/operator-expr.md#assignment-expressions
[compound assignment expressions]: expressions/operator-expr.md#compound-assignment-expressions
[block expressions]:    expressions/block-expr.md
[borrow]:               expressions/operator-expr.md#borrow-operators
[cast]:                 expressions/operator-expr.md#type-cast-expressions
[closure expressions]:  expressions/closure-expr.md
[comparison]:           expressions/operator-expr.md#comparison-operators
[const functions]:      items/functions.md#const-functions
[constants]:            items/constant-items.md
[dereference operator]: expressions/operator-expr.md#the-dereference-operator
[destructors]:          destructors.md
[enum discriminants]:   items/enumerations.md#custom-discriminant-values-for-fieldless-enumerations
[enum variant]:         expressions/enum-variant-expr.md
[expression statements]: statements.md#expression-statements
[expressions]:          expressions.md
[field]:                expressions/field-expr.md
[functions]:            items/functions.md
[grouped]:              expressions/grouped-expr.md
[interior mutability]:  interior-mutability.md
[lazy boolean]:         expressions/operator-expr.md#lazy-boolean-operators
[let statements]:       statements.md#let-statements
[literals]:             expressions/literal-expr.md
[logical]:              expressions/operator-expr.md#arithmetic-and-logical-binary-operators
[negation]:             expressions/operator-expr.md#negation-operators
[overflow]:             expressions/operator-expr.md#overflow
[paths]:                expressions/path-expr.md
[patterns]:             patterns.md
[range expressions]:    expressions/range-expr.md
[slice]:                types/slice.md
[statics]:              items/static-items.md
[struct]:               expressions/struct-expr.md
[tuple expressions]:    expressions/tuple-expr.md
