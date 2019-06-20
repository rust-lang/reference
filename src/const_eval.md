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
    * [assignment operator expressions]
    * [expression statements]
* [Field] expressions.
* Index expressions, [array indexing] or [slice] with a `usize`.
* [Range expressions].
* [Closure expressions] which don't capture variables from the environment.
* Built in [negation], [arithmetic, logical], [comparison] or [lazy boolean]
  operators used on integer and floating point types, `bool` and `char`.
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

[arithmetic, logical]:  expressions/operator-expr.html#arithmetic-and-logical-binary-operators
[array expressions]:    expressions/array-expr.html
[array indexing]:       expressions/array-expr.html#array-and-slice-indexing-expressions
[array indexing]:       expressions/array-expr.html#array-and-slice-indexing-expressions
[array type length expressions]: types/array.html
[assignment expressions]: expressions/operator-expr.html#assignment-expressions
[assignment operator expressions]: expressions/operator-expr.html#compound-assignment-expressions
[block expressions]:    expressions/block-expr.html
[borrow]:               expressions/operator-expr.html#borrow-operators
[cast]:                 expressions/operator-expr.html#type-cast-expressions
[closure expressions]:  expressions/closure-expr.html
[comparison]:           expressions/operator-expr.html#comparison-operators
[const functions]:      items/functions.html#const-functions
[constants]:            items/constant-items.html
[dereference operator]: expressions/operator-expr.html#the-dereference-operator
[destructors]:          destructors.html
[enum discriminants]:   items/enumerations.html#custom-discriminant-values-for-field-less-enumerations
[enum variant]:         expressions/enum-variant-expr.html
[expression statements]: statements.html#expression-statements
[expressions]:          expressions.html
[field]:                expressions/field-expr.html
[functions]:            items/functions.html
[grouped]:              expressions/grouped-expr.html
[interior mutability]:  interior-mutability.html
[lazy boolean]:         expressions/operator-expr.html#lazy-boolean-operators
[let statements]:       statements.html#let-statements
[literals]:             expressions/literal-expr.html
[negation]:             expressions/operator-expr.html#negation-operators
[overflow]:             expressions/operator-expr.html#overflow
[paths]:                expressions/path-expr.html
[patterns]:             patterns.html
[range expressions]:    expressions/range-expr.html
[slice]:                types/slice.html
[statics]:              items/static-items.html
[struct]:               expressions/struct-expr.html
[tuple expressions]:    expressions/tuple-expr.html
