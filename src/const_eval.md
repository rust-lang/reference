# Constant evaluation

Constant evaluation is the process of computing the result of
[expressions] during compilation.

## Const context

A _const context_ is one of the following:

* [array type length expressions]
* repeat expression length expessions
* the initializer of
  * [constants]
  * [statics]
  * [enum discriminants]

[array type length expressions]: types.html#array-and-slice-types
[enum discriminants]: items/enumerations.html#custom-discriminant-values-for-field-less-enumerations
[constants]: items/constant-items.html
[statics]: items/static-items.html
[expressions]: expressions.html#constant-expressions
