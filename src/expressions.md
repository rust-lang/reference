r[expr]
# Expressions

r[expr.syntax]
> **<sup>Syntax</sup>**\
> _Expression_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; _ExpressionWithoutBlock_\
> &nbsp;&nbsp; | _ExpressionWithBlock_
>
> _ExpressionWithoutBlock_ :\
> &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup>[†](#expression-attributes)\
> &nbsp;&nbsp; (\
> &nbsp;&nbsp; &nbsp;&nbsp; &nbsp;&nbsp; [_LiteralExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_PathExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_OperatorExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_GroupedExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_ArrayExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_AwaitExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_IndexExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_TupleExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_TupleIndexingExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_StructExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_CallExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_MethodCallExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_FieldExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_ClosureExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_AsyncBlockExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_ContinueExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_BreakExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_RangeExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_ReturnExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_UnderscoreExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_MacroInvocation_]\
> &nbsp;&nbsp; )
>
> _ExpressionWithBlock_ :\
> &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup>[†](#expression-attributes)\
> &nbsp;&nbsp; (\
> &nbsp;&nbsp; &nbsp;&nbsp; &nbsp;&nbsp; [_BlockExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_ConstBlockExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_UnsafeBlockExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_LoopExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_IfExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_IfLetExpression_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_MatchExpression_]\
> &nbsp;&nbsp; )

r[expr.intro]
An expression may have two roles: it always produces a *value*, and it may have *effects* (otherwise known as "side effects").

r[expr.evaluation]
An expression *evaluates to* a value, and has effects during *evaluation*.

r[expr.operands]
Many expressions contain sub-expressions, called the *operands* of the expression.

r[expr.behavior]
The meaning of each kind of expression dictates several things:

* Whether or not to evaluate the operands when evaluating the expression
* The order in which to evaluate the operands
* How to combine the operands' values to obtain the value of the expression

r[expr.structure]
In this way, the structure of expressions dictates the structure of execution.
Blocks are just another kind of expression, so blocks, statements, expressions, and blocks again can recursively nest inside each other to an arbitrary depth.

> [!NOTE]
> We give names to the operands of expressions so that we may discuss them, but these names are not stable and may be changed.

r[expr.precedence]
## Expression precedence

The precedence of Rust operators and expressions is ordered as follows, going from strong to weak.
Binary Operators at the same precedence level are grouped in the order given by their associativity.

| Operator/Expression         | Associativity       |
|-----------------------------|---------------------|
| Paths                       |                     |
| Method calls                |                     |
| Field expressions           | left to right       |
| Function calls, array indexing |                  |
| `?`                         |                     |
| Unary `-` `*` `!` `&` `&mut` |                    |
| `as`                        | left to right       |
| `*` `/` `%`                 | left to right       |
| `+` `-`                     | left to right       |
| `<<` `>>`                   | left to right       |
| `&`                         | left to right       |
| `^`                         | left to right       |
| <code>&#124;</code>         | left to right       |
| `==` `!=` `<` `>` `<=` `>=` | Require parentheses |
| `&&`                        | left to right       |
| <code>&#124;&#124;</code>   | left to right       |
| `..` `..=`                  | Require parentheses |
| `=` `+=` `-=` `*=` `/=` `%=` <br> `&=` <code>&#124;=</code> `^=` `<<=` `>>=` | right to left |
| `return` `break` closures   |                     |

r[expr.operand-order]
## Evaluation order of operands

r[expr.operand-order.default]
The following list of expressions all evaluate their operands the same way, as described after the list.
Other expressions either don't take operands or evaluate them conditionally as described on their respective pages.

* Dereference expression
* Error propagation expression
* Negation expression
* Arithmetic and logical binary operators
* Comparison operators
* Type cast expression
* Grouped expression
* Array expression
* Await expression
* Index expression
* Tuple expression
* Tuple index expression
* Struct expression
* Call expression
* Method call expression
* Field expression
* Break expression
* Range expression
* Return expression

r[expr.operand-order.operands-before-primary]
The operands of these expressions are evaluated prior to applying the effects of the expression.
Expressions taking multiple operands are evaluated left to right as written in the source code.

> [!NOTE]
> Which subexpressions are the operands of an expression is determined by expression precedence as per the previous section.

For example, the two `next` method calls will always be called in the same order:

```rust
# // Using vec instead of array to avoid references
# // since there is no stable owned array iterator
# // at the time this example was written.
let mut one_two = vec![1, 2].into_iter();
assert_eq!(
    (1, 2),
    (one_two.next().unwrap(), one_two.next().unwrap())
);
```

> [!NOTE]
> Since this is applied recursively, these expressions are also evaluated from innermost to outermost, ignoring siblings until there are no inner subexpressions.

r[expr.place-value]
## Place Expressions and Value Expressions

r[expr.place-value.intro]
Expressions are divided into two main categories: place expressions and value expressions;
there is also a third, minor category of expressions called assignee expressions.
Within each expression, operands may likewise occur in either place context or value context.
The evaluation of an expression depends both on its own category and the context it occurs within.

r[expr.place-value.place-memory-location]
A *place expression* is an expression that represents a memory location.

r[expr.place-value.place-expr-kinds]
These expressions are [paths] which refer to local variables, [static variables], [dereferences][deref] (`*expr`), [array indexing] expressions (`expr[expr]`), [field] references (`expr.f`) and parenthesized place expressions.

r[expr.place-value.value-expr-kinds]
All other expressions are value expressions.

r[expr.place-value.value-result]
A *value expression* is an expression that represents an actual value.

r[expr.place-value.place-context]
The following contexts are *place expression* contexts:

* The left operand of a [compound assignment] expression.
* The operand of a unary [borrow], [raw borrow] or [dereference][deref] operator.
* The operand of a field expression.
* The indexed operand of an array indexing expression.
* The operand of any [implicit borrow].
* The initializer of a [let statement].
* The [scrutinee] of an [`if let`], [`match`][match], or [`while let`]
  expression.
* The base of a [functional update] struct expression.

> [!NOTE]
> Historically, place expressions were called *lvalues* and value expressions were called *rvalues*.

r[expr.place-value.assignee]
An *assignee expression* is an expression that appears in the left operand of an [assignment][assign] expression.
Explicitly, the assignee expressions are:

- Place expressions.
- [Underscores][_UnderscoreExpression_].
- [Tuples][_TupleExpression_] of assignee expressions.
- [Slices][_ArrayExpression_] of assignee expressions.
- [Tuple structs][_StructExpression_] of assignee expressions.
- [Structs][_StructExpression_] of assignee expressions (with optionally named
  fields).
- [Unit structs][_StructExpression_].

r[expr.place-value.parenthesis]
Arbitrary parenthesisation is permitted inside assignee expressions.

r[expr.move]
### Moved and copied types

r[expr.move.intro]
When a place expression is evaluated in a value expression context, or is bound by value in a pattern, it denotes the value held _in_ that memory location.

r[expr.move.copy]
If the type of that value implements [`Copy`], then the value will be copied.

r[expr.move.requires-sized]
In the remaining situations, if that type is [`Sized`], then it may be possible to move the value.

r[expr.move.movable-place]
Only the following place expressions may be moved out of:

* [Variables] which are not currently borrowed.
* [Temporary values](#temporaries).
* [Fields][field] of a place expression which can be moved out of and don't implement [`Drop`].
* The result of [dereferencing][deref] an expression with type [`Box<T>`] and that can also be moved out of.

r[expr.move.deinitialization]
After moving out of a place expression that evaluates to a local variable, the location is deinitialized and cannot be read from again until it is reinitialized.

r[expr.move.place-invalid]
In all other cases, trying to use a place expression in a value expression context is an error.

r[expr.mut]
### Mutability

r[expr.mut.intro]
For a place expression to be [assigned][assign] to, mutably [borrowed][borrow], [implicitly mutably borrowed], or bound to a pattern containing `ref mut`, it must be _mutable_.
We call these *mutable place expressions*.
In contrast, other place expressions are called *immutable place expressions*.

r[expr.mut.valid-places]
The following expressions can be mutable place expression contexts:

* Mutable [variables] which are not currently borrowed.
* [Mutable `static` items].
* [Temporary values].
* [Fields][field]: this evaluates the subexpression in a mutable place expression context.
* [Dereferences][deref] of a `*mut T` pointer.
* Dereference of a [movable place][expr.move.movable-place] with type `&mut T`. This includes variables and their fields, as well as temporaries.
  Note: This is an exception to the requirement of the next rule.
* Dereferences of a type that implements `DerefMut`:
  this then requires that the value being dereferenced is evaluated in a mutable place expression context.
* [Array indexing] of a type that implements `IndexMut`:
  this then evaluates the value being indexed, but not the index, in mutable place expression context.

r[expr.temporary]
### Temporaries

When using a value expression in most place expression contexts, a temporary unnamed memory location is created and initialized to that value.
The expression evaluates to that location instead, except if [promoted] to a `static`.
The [drop scope] of the temporary is usually the end of the enclosing statement.

r[expr.implicit-borrow]
### Implicit Borrows

r[expr.implicit-borrow-intro]
Certain expressions will treat an expression as a place expression by implicitly borrowing it.
For example, it is possible to compare two unsized [slices][slice] for equality directly, because the `==` operator implicitly borrows its operands:

```rust
# let c = [1, 2, 3];
# let d = vec![1, 2, 3];
let a: &[i32];
let b: &[i32];
# a = &c;
# b = &d;
// ...
*a == *b;
// Equivalent form:
::std::cmp::PartialEq::eq(&*a, &*b);
```

r[expr.implicit-borrow.application]
Implicit borrows may be taken in the following expressions:

* Left operand in [method-call] expressions.
* Left operand in [field] expressions.
* Left operand in [call expressions].
* Left operand in [array indexing] expressions.
* Operand of the [dereference operator][deref] (`*`).
* Operands of [comparison].
* Left operands of the [compound assignment].

r[expr.overload]
## Overloading Traits

Many of the following operators and expressions can also be overloaded for other types using traits in `std::ops` or `std::cmp`.
These traits also exist in `core::ops` and `core::cmp` with the same names.

r[expr.attr]
## Expression Attributes

r[expr.attr.restriction]
[Outer attributes][_OuterAttribute_] before an expression are allowed only in a few specific cases:

* Before an expression used as a [statement].
* Elements of [array expressions], [tuple expressions], [call expressions], and tuple-style [struct] expressions.
* The tail expression of [block expressions].
<!-- Keep list in sync with block-expr.md -->

r[expr.attr.never-before]
They are never allowed before:
* [Range][_RangeExpression_] expressions.
* Binary operator expressions ([_ArithmeticOrLogicalExpression_], [_ComparisonExpression_], [_LazyBooleanExpression_], [_TypeCastExpression_], [_AssignmentExpression_], [_CompoundAssignmentExpression_]).

[block expressions]:    expressions/block-expr.md
[call expressions]:     expressions/call-expr.md
[field]:                expressions/field-expr.md
[functional update]:    expressions/struct-expr.md#functional-update-syntax
[`if let`]:             expressions/if-expr.md#if-let-expressions
[match]:                expressions/match-expr.md
[method-call]:          expressions/method-call-expr.md
[paths]:                expressions/path-expr.md
[struct]:               expressions/struct-expr.md
[tuple expressions]:    expressions/tuple-expr.md
[`while let`]:          expressions/loop-expr.md#predicate-pattern-loops

[array expressions]:    expressions/array-expr.md
[array indexing]:       expressions/array-expr.md#array-and-slice-indexing-expressions

[assign]:               expressions/operator-expr.md#assignment-expressions
[borrow]:               expressions/operator-expr.md#borrow-operators
[comparison]:           expressions/operator-expr.md#comparison-operators
[compound assignment]:  expressions/operator-expr.md#compound-assignment-expressions
[deref]:                expressions/operator-expr.md#the-dereference-operator

[destructors]:          destructors.md
[drop scope]:           destructors.md#drop-scopes

[`Copy`]:               special-types-and-traits.md#copy
[`Drop`]:               special-types-and-traits.md#drop
[`Sized`]:              special-types-and-traits.md#sized
[implicit borrow]:      #implicit-borrows
[implicitly mutably borrowed]: #implicit-borrows
[interior mutability]:  interior-mutability.md
[let statement]:        statements.md#let-statements
[Mutable `static` items]: items/static-items.md#mutable-statics
[scrutinee]:            glossary.md#scrutinee
[promoted]:             destructors.md#constant-promotion
[raw borrow]:           expressions/operator-expr.md#raw-borrow-operators
[slice]:                types/slice.md
[statement]:            statements.md
[static variables]:     items/static-items.md
[Temporary values]:     #temporaries
[Variables]:            variables.md

[_ArithmeticOrLogicalExpression_]: expressions/operator-expr.md#arithmetic-and-logical-binary-operators
[_ArrayExpression_]:              expressions/array-expr.md
[_AsyncBlockExpression_]:         expressions/block-expr.md#async-blocks
[_AwaitExpression_]:              expressions/await-expr.md
[_AssignmentExpression_]:         expressions/operator-expr.md#assignment-expressions
[_BlockExpression_]:              expressions/block-expr.md
[_BreakExpression_]:              expressions/loop-expr.md#break-expressions
[_CallExpression_]:               expressions/call-expr.md
[_ClosureExpression_]:            expressions/closure-expr.md
[_ComparisonExpression_]:         expressions/operator-expr.md#comparison-operators
[_CompoundAssignmentExpression_]: expressions/operator-expr.md#compound-assignment-expressions
[_ConstBlockExpression_]:         expressions/block-expr.md#const-blocks
[_ContinueExpression_]:           expressions/loop-expr.md#continue-expressions
[_FieldExpression_]:              expressions/field-expr.md
[_GroupedExpression_]:            expressions/grouped-expr.md
[_IfExpression_]:                 expressions/if-expr.md#if-expressions
[_IfLetExpression_]:              expressions/if-expr.md#if-let-expressions
[_IndexExpression_]:              expressions/array-expr.md#array-and-slice-indexing-expressions
[_LazyBooleanExpression_]:        expressions/operator-expr.md#lazy-boolean-operators
[_LiteralExpression_]:            expressions/literal-expr.md
[_LoopExpression_]:               expressions/loop-expr.md
[_MacroInvocation_]:              macros.md#macro-invocation
[_MatchExpression_]:              expressions/match-expr.md
[_MethodCallExpression_]:         expressions/method-call-expr.md
[_OperatorExpression_]:           expressions/operator-expr.md
[_OuterAttribute_]:               attributes.md
[_PathExpression_]:               expressions/path-expr.md
[_RangeExpression_]:              expressions/range-expr.md
[_ReturnExpression_]:             expressions/return-expr.md
[_StructExpression_]:             expressions/struct-expr.md
[_TupleExpression_]:              expressions/tuple-expr.md
[_TupleIndexingExpression_]:      expressions/tuple-expr.md#tuple-indexing-expressions
[_TypeCastExpression_]:           expressions/operator-expr.md#type-cast-expressions
[_UnderscoreExpression_]:         expressions/underscore-expr.md
[_UnsafeBlockExpression_]:        expressions/block-expr.md#unsafe-blocks
