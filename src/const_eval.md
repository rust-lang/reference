# Constant evaluation
r[const-eval]

r[const-eval.general]
Constant evaluation is the process of computing the result of
[expressions] during compilation. Only a subset of all expressions
can be evaluated at compile-time.

## Constant expressions

r[const-eval.const-expr]

r[const-eval.const-expr.general]
Certain forms of expressions, called constant expressions, can be evaluated at
compile time.

r[const-eval.const-expr.const-context]
In [const contexts](#const-context), these are the only allowed
expressions, and are always evaluated at compile time.

r[const-eval.const-expr.runtime-context]
In other places, such as [let statements], constant expressions *may* be, but are not guaranteed to be, evaluated at compile time.

r[const-eval.const-expr.error]
Behaviors such as out of bounds [array indexing] or [overflow] are compiler errors if the value
must be evaluated at compile time (i.e. in const contexts). Otherwise, these
behaviors are warnings, but will likely panic at run-time.

r[const-eval.const-expr.list]
The following expressions are constant expressions, so long as any operands are
also constant expressions and do not cause any [`Drop::drop`][destructors] calls
to be run.

r[const-eval.const-expr.literal]
* [Literals].

r[const-eval.const-expr.parameter]
* [Const parameters].

r[const-eval.const-expr.path-item]
* [Paths] to [functions] and [constants].
  Recursively defining constants is not allowed.

r[const-eval.const-expr.path-static]
* Paths to [statics]. These are only allowed within the initializer of a static.

r[const-eval.const-expr.tuple]
* [Tuple expressions].

r[const-eval.const-expr.array]
* [Array expressions].

r[const-eval.const-expr.constructor]
* [Struct] expressions.

r[const-eval.const-expr.block]
* [Block expressions], including `unsafe` and `const` blocks.
    * [let statements] and thus irrefutable [patterns], including mutable bindings
    * [assignment expressions]
    * [compound assignment expressions]
    * [expression statements]

r[const-eval.const-expr.field]
* [Field] expressions.

r[const-eval.const-expr.index]
* Index expressions, [array indexing] or [slice] with a `usize`.

r[const-eval.const-expr.range]
* [Range expressions].

r[const-eval.const-expr.closure]
* [Closure expressions] which don't capture variables from the environment.

r[const-eval.const-expr.builtin-arith-logic]
* Built-in [negation], [arithmetic], [logical], [comparison] or [lazy boolean]
  operators used on integer and floating point types, `bool`, and `char`.

r[const-eval.const-expr.borrows]
* All forms of [borrow]s, including raw borrows, with one limitation:
  mutable borrows and shared borrows to values with interior mutability
  are only allowed to refer to *transient* places. A place is *transient*
  if its lifetime is strictly contained inside the current [const context].

r[const-eval.const-expr.deref]
* The [dereference operator] except for raw pointers.

r[const-eval.const-expr.group]

* [Grouped] expressions.

r[const-eval.const-expr.cast]
* [Cast] expressions, except
  * pointer to address casts and
  * function pointer to address casts.

r[const-eval.const-expr.const-fn]
* Calls of [const functions] and const methods.

r[const-eval.const-expr.loop]
* [loop], [while] and [`while let`] expressions.

r[const-eval.const-expr.if-match]
* [if], [`if let`] and [match] expressions.

## Const context
[const context]: #const-context

r[const-eval.const-context]

r[const-eval.const-context.general]
A _const context_ is one of the following:

r[const-eval.const-context.array-length]
* [Array type length expressions]

r[const-eval.const-context.repeat-length]
* [Array repeat length expressions][array expressions]

r[const-eval.const-context.init]
* The initializer of
  * [constants]
  * [statics]
  * [enum discriminants]

r[const-eval.const-context.generic]
* A [const generic argument]

r[const-eval.const-context.block]
* A [const block]

Const contexts that are used as parts of types (array type and repeat length
expressions as well as const generic arguments) can only make restricted use of
surrounding generic parameters: such an expression must either be a single bare
const generic parameter, or an arbitrary expression not making use of any
generics.

## Const Functions

r[const-eval.const-fn]

r[const-eval.const-fn.general]
A _const fn_ is a function that one is permitted to call from a const context.

r[const-eval.const-fn.usage]
Declaring a function
`const` has no effect on any existing uses, it only restricts the types that arguments and the
return type may use, and restricts the function body to constant expressions.

r[const-eval.const-fn.const-context]
When called from a const context, the function is interpreted by the
compiler at compile time. The interpretation happens in the
environment of the compilation target and not the host. So `usize` is
`32` bits if you are compiling against a `32` bit system, irrelevant
of whether you are building on a `64` bit or a `32` bit system.

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
[const block]:          expressions/block-expr.md#const-blocks
[const functions]:      items/functions.md#const-functions
[const generic argument]: items/generics.md#const-generics
[const generic parameters]: items/generics.md#const-generics
[constants]:            items/constant-items.md
[Const parameters]:     items/generics.md
[dereference operator]: expressions/operator-expr.md#the-dereference-operator
[destructors]:          destructors.md
[enum discriminants]:   items/enumerations.md#discriminants
[expression statements]: statements.md#expression-statements
[expressions]:          expressions.md
[field]:                expressions/field-expr.md
[functions]:            items/functions.md
[grouped]:              expressions/grouped-expr.md
[interior mutability]:  interior-mutability.md
[if]:                   expressions/if-expr.md#if-expressions
[`if let`]:             expressions/if-expr.md#if-let-expressions
[lazy boolean]:         expressions/operator-expr.md#lazy-boolean-operators
[let statements]:       statements.md#let-statements
[literals]:             expressions/literal-expr.md
[logical]:              expressions/operator-expr.md#arithmetic-and-logical-binary-operators
[loop]:                 expressions/loop-expr.md#infinite-loops
[match]:                expressions/match-expr.md
[negation]:             expressions/operator-expr.md#negation-operators
[overflow]:             expressions/operator-expr.md#overflow
[paths]:                expressions/path-expr.md
[patterns]:             patterns.md
[range expressions]:    expressions/range-expr.md
[slice]:                types/slice.md
[statics]:              items/static-items.md
[struct]:               expressions/struct-expr.md
[tuple expressions]:    expressions/tuple-expr.md
[while]:                expressions/loop-expr.md#predicate-loops
[`while let`]:          expressions/loop-expr.md#predicate-pattern-loops
