r[expr]
# Expressions

r[expr.syntax]
```grammar,expressions
Expression ->
      ExpressionWithoutBlock
    | ExpressionWithBlock

ExpressionWithoutBlock ->
    OuterAttribute* ExpressionWithoutBlockNoAttrs

ExpressionWithoutBlockNoAttrs ->
      LiteralExpression
    | PathExpression
    | OperatorExpression
    | GroupedExpression
    | ArrayExpression
    | AwaitExpression
    | IndexExpression
    | TupleExpression
    | TupleIndexingExpression
    | StructExpression
    | CallExpression
    | MethodCallExpression
    | FieldExpression
    | ClosureExpression
    | AsyncBlockExpression
    | ContinueExpression
    | BreakExpression
    | RangeExpression
    | ReturnExpression
    | UnderscoreExpression
    | MacroInvocation

ExpressionWithBlock ->
    OuterAttribute* ExpressionWithBlockNoAttrs

ExpressionWithBlockNoAttrs ->
      BlockExpression
    | ConstBlockExpression
    | UnsafeBlockExpression
    | LoopExpression
    | IfExpression
    | MatchExpression
```

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
In this way, the structure of expressions dictates the structure of execution. Blocks are just another kind of expression, so blocks, statements, expressions, and blocks again can recursively nest inside each other to an arbitrary depth.

> [!NOTE]
> We give names to the operands of expressions so that we may discuss them, but these names are not stable and may be changed.

r[expr.precedence]
## Expression precedence

The precedence of Rust operators and expressions is ordered as follows, going from strong to weak. Binary Operators at the same precedence level are grouped in the order given by their associativity.

| Operator/Expression         | Associativity       |
|-----------------------------|---------------------|
| [Paths][expr.path]          |                     |
| [Method calls][expr.method] |                     |
| [Field expressions][expr.field] | left to right   |
| [Function calls][expr.call], [array indexing][expr.array.index] | |
| [`?`][expr.try]             |                     |
| Unary [`-`][expr.negate] [`!`][expr.negate] [`*`][expr.deref] [borrow][expr.operator.borrow] | |
| [`as`][expr.as]             | left to right       |
| [`*`][expr.arith-logic] [`/`][expr.arith-logic] [`%`][expr.arith-logic] | left to right       |
| [`+`][expr.arith-logic] [`-`][expr.arith-logic] | left to right       |
| [`<<`][expr.arith-logic] [`>>`][expr.arith-logic] | left to right     |
| [`&`][expr.arith-logic]     | left to right       |
| [`^`][expr.arith-logic]     | left to right       |
| [<code>&#124;</code>][expr.arith-logic] | left to right       |
| [`==`][expr.cmp] [`!=`][expr.cmp] [`<`][expr.cmp] [`>`][expr.cmp] [`<=`][expr.cmp] [`>=`][expr.cmp] | Require parentheses |
| [`&&`][expr.bool-logic]     | left to right       |
| [<code>&#124;&#124;</code>][expr.bool-logic] | left to right       |
| [`..`][expr.range] [`..=`][expr.range] | Require parentheses |
| [`=`][expr.assign] [`+=`][expr.compound-assign] [`-=`][expr.compound-assign] [`*=`][expr.compound-assign] [`/=`][expr.compound-assign] [`%=`][expr.compound-assign] <br> [`&=`][expr.compound-assign] [<code>&#124;=</code>][expr.compound-assign] [`^=`][expr.compound-assign] [`<<=`][expr.compound-assign] [`>>=`][expr.compound-assign] | right to left |
| [`return`][expr.return] [`break`][expr.loop.break] [closures][expr.closure]  | |

r[expr.operand-order]
## Evaluation order of operands

r[expr.operand-order.default]
The following list of expressions all evaluate their operands the same way, as described after the list. Other expressions either don't take operands or evaluate them conditionally as described on their respective pages.

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
The operands of these expressions are evaluated prior to applying the effects of the expression. Expressions taking multiple operands are evaluated left to right as written in the source code.

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
## Place expressions and value expressions

r[expr.place-value.intro]
Expressions are divided into two main categories: place expressions and value expressions; there is also a third, minor category of expressions called assignee expressions. Within each expression, operands may likewise occur in either place context or value context. The evaluation of an expression depends both on its own category and the context it occurs within.

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
* The operand of a [field expression].
* The indexed operand of an [array indexing expression].
* The tuple operand of a [tuple indexing expression].
* The operand of any [implicit borrow].
* The initializer of a [let statement].
* The [scrutinee] of an [`if let`], [`match`][match], or [`while let`] expression.
* The base of a [functional update] struct expression.

> [!NOTE]
> Historically, place expressions were called *lvalues* and value expressions were called *rvalues*.

r[expr.place-value.assignee]
An *assignee expression* is an expression that appears in the left operand of an [assignment][assign] expression. Explicitly, the assignee expressions are:

- Place expressions.
- [Underscores].
- [Tuples] of assignee expressions.
- [Slices][expr.array.index] of assignee expressions.
- [Tuple structs] of assignee expressions.
- [Structs] of assignee expressions (with optionally named fields).
- [Unit structs]

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

* [Variables].
* [Temporary values](#temporaries).
* [Fields][field] of a place expression that can be moved out of and whose type does not implement [`Drop`].

  ```rust
  struct D;
  impl Drop for D { fn drop(&mut self) {} }
  struct S(D);
  let x = S(D);
  let y = x.0; // OK: `S` does not implement `Drop`, though `D` does.
  ```

  ```rust,compile_fail,E0509
  struct S(String);
  impl Drop for S { fn drop(&mut self) {} }
  let x = S(String::new());
  let y = x.0; // ERROR: Cannot move a field out of a type that implements `Drop`.
  ```

* [Parenthesized expressions] whose enclosed operand can be moved out of.
* The result of [dereferencing] an expression of type [`Box<T>`] that can itself be moved out of.

> [!NOTE]
> A place expression cannot be moved out of while it is borrowed. See [expr.operator.borrow.lifetime].

r[expr.move.deinitialization]
After moving out of a place expression that evaluates to a local variable, the location is deinitialized and cannot be read from again until it is reinitialized.

r[expr.move.place-invalid]
In all other cases, trying to use a place expression in a value expression context is an error.

r[expr.mut]
### Mutability

r[expr.mut.intro]
The following contexts are *mutable place expression contexts*:

* The left operand of an [assignment] or [compound assignment] expression and, in a destructuring assignment, each place expression it assigns to (see [expr.assign.destructure]).
* The operand of a mutable [borrow] expression (`&mut`) or a mutable [raw borrow] expression (`&raw mut`).
* The operand of an [implicit mutable borrow].
* The output operands of an [inline assembly] expression.
* The place matched by a [`ref mut` binding] whose `ref mut` is written in the pattern.

The *mutable place expressions*, the place expressions that may be used in these contexts, are listed in [expr.mut.valid-places]. All other place expressions are *immutable place expressions*.

> [!NOTE]
> A `ref mut` [binding mode] that arises from matching a reference with a non-reference pattern does not put the scrutinee in a mutable place expression context, so it does not cause an index or dereference within the scrutinee to use `IndexMut` or `DerefMut`. This behavior is settled: [rust-lang/rust#44848], which tracked it, was closed with the current handling judged adequate and stabilized. The place that the binding borrows must nonetheless be a mutable place expression (see [expr.mut.other-borrows]).
>
> ```rust,compile_fail,E0596
> # let mut a = (0u8,);
> let mut v: Vec<Option<&mut (u8,)>> = vec![Some(&mut a)];
> if let Some((x,)) = v[0] { *x = 1; } // ERROR: `v[0]` uses `Index`, so the field that `x` borrows is not mutable.
> ```
>
> ```rust
> # let mut a = (0u8,);
> let mut v: Vec<Option<&mut (u8,)>> = vec![Some(&mut a)];
> if let Some(&mut (ref mut x,)) = v[0] { *x = 1; } // OK: The written `ref mut` makes `v[0]` use `IndexMut`.
> ```

r[expr.mut.operands]
When a place expression is in a mutable place expression context, so is each place expression it is built on, after automatic dereferencing: the enclosed operand of a [parenthesized expression], the container operand of a [field] or [tuple indexing] expression, the indexed operand of an [index expression], the operand of a [dereference], whether written or automatic, and the scrutinee that a pattern destructures. In such a context, an index expression that is not built in (see [expr.array.index.array]) uses [`IndexMut`] when its operand's type implements `IndexMut` for the index type of the [`Index`] implementation it uses, and `Index` otherwise; a dereference, written or automatic, of a type other than a reference, raw pointer, or `Box<T>` (see [expr.deref.traits]) uses [`DerefMut`] when that type implements it, and [`Deref`] otherwise. The implicit mutable borrow that `IndexMut` and `DerefMut` take is a mutable place expression context for their operand.

```rust
# use std::rc::Rc;
# let mut a = 0u8;
struct P { p: *mut u8 }
let rc = Rc::new(P { p: &mut a });
unsafe { *rc.p = 1; } // OK: `Rc` does not implement `DerefMut`, so `*rc` uses `Deref`, and `*rc.p` is mutable because `rc.p` is a `*mut u8`.
```

```rust,compile_fail,E0502
# let mut a = 0u8;
let mut v: Vec<&mut u8> = vec![&mut a];
let r = &v;
*v[0] = 1; // ERROR: `v[0]` uses `IndexMut`, which mutably borrows `v` while `r` borrows it.
let _ = r;
```

r[expr.mut.valid-places]
Only the following place expressions are mutable place expressions.

* Mutable [variables].
* [Variables] not yet initialized on any control flow path reaching the context that initializes them (see [variable.init]).
* [Mutable `static` items].
* [Temporary values].
* [Fields][field]: mutable when the container operand, after automatic dereferencing (see [expr.field.autoref-deref]), is.
* [Parenthesized expressions]: mutable when the enclosed operand is.
* [Dereferences][deref] of a `*mut T` pointer.
* Dereference of a variable, or field of a variable, with type `&mut T`.
* Dereferences of a `Box<T>`: mutable when the operand is.
* Dereferences that use [`DerefMut`] (see [expr.mut.operands]): mutable when the operand is.
* [Indexing] of an array or slice by a value of type `usize`, or indexing that uses [`IndexMut`] (see [expr.mut.operands]): mutable when the indexed operand, after automatic dereferencing (see [expr.array.index.autoderef]), is.

r[expr.mut.invalid-places]
Using any other place expression in one of the contexts listed in [expr.mut.intro] is an error.

r[expr.mut.other-borrows]
The place that a `ref mut` binding mode borrows (see [patterns.ident.binding]) must also be a mutable place expression. That borrow is not a mutable place expression context, so it does not cause an index or dereference within the place to use `IndexMut` or `DerefMut` (see [expr.mut.operands]).

```rust
fn f(r: &mut &mut (u8,)) {
    let (x,) = *r; // OK: The `ref mut` binding mode borrows `(**r).0`, which is mutable.
    *x = 1;
}
```

```rust,compile_fail,E0596
fn f(r: &&mut (u8,)) {
    let (x,) = *r; // ERROR: The `ref mut` binding mode borrows `(**r).0`, which is reached through a shared reference.
    *x = 1;
}
```

> [!NOTE]
> A mutable place expression also cannot be mutated while it is borrowed. See [expr.operator.borrow.lifetime].

r[expr.temporary]
### Temporaries

When using a value expression in most place expression contexts, a temporary unnamed memory location is created and initialized to that value. The expression evaluates to that location instead, except if [promoted] to a `static`. The [drop scope] of the temporary is usually the end of the enclosing statement.

r[expr.super-macros]
### Super macros

r[expr.super-macros.intro]
Certain built-in macros may create [temporaries] whose [scopes][temporary scopes] may be [extended]. These temporaries are *super temporaries* and these macros are *super macros*. [Invocations][macro invocations] of these macros are *super macro call expressions*. Arguments to these macros may be *super operands*.

> [!NOTE]
> When a super macro call expression is an [extending expression], its super operands are [extending expressions] and the [scopes][temporary scopes] of the super temporaries are [extended]. See [destructors.scope.lifetime-extension.exprs].

r[expr.super-macros.format_args]
#### `format_args!`

r[expr.super-macros.format_args.super-operands]
Except for the format string argument, all arguments passed to [`format_args!`] are *super operands*.

```rust,edition2024
# fn temp() -> String { String::from("") }
// Due to the call being an extending expression and the argument
// being a super operand, the inner block is an extending expression,
// so the scope of the temporary created in its trailing expression
// is extended.
let _ = format_args!("{}", { &temp() }); // OK
```

r[expr.super-macros.format_args.super-temporaries]
The super operands of [`format_args!`] are [implicitly borrowed] and are therefore [place expression contexts]. When a [value expression] is passed as an argument, it creates a *super temporary*.

```rust
# fn temp() -> String { String::from("") }
let x = format_args!("{}", temp());
x; // <-- The temporary is extended, allowing use here.
```

The expansion of a call to [`format_args!`] sometimes creates other internal *super temporaries*.

```rust,compile_fail,E0716
let x = {
    // This call creates an internal temporary.
    let x = format_args!("{:?}", 0);
    x // <-- The temporary is extended, allowing its use here.
}; // <-- The temporary is dropped here.
x; // ERROR
```

```rust
// This call doesn't create an internal temporary.
let x = { let x = format_args!("{}", 0); x };
x; // OK
```

> [!NOTE]
> The details of when [`format_args!`] does or does not create internal temporaries are currently unspecified.

r[expr.super-macros.pin]
#### `pin!`

r[expr.super-macros.pin.super-operands]
The argument to [`pin!`] is a *super operand*.

```rust,edition2024
# use core::pin::pin;
# fn temp() {}
// As above for `format_args!`.
let _ = pin!({ &temp() }); // OK
```

r[expr.super-macros.pin.super-temporaries]
The argument to [`pin!`] is a [value expression context] and creates a *super temporary*.

```rust
# use core::pin::pin;
# fn temp() {}
// The argument is evaluated into a super temporary.
let x = pin!(temp());
// The temporary is extended, allowing its use here.
x; // OK
```

r[expr.implicit-borrow]
### Implicit borrows

r[expr.implicit-borrow-intro]
Certain expressions will treat an expression as a place expression by implicitly borrowing it. For example, it is possible to compare two unsized [slices][slice] for equality directly, because the `==` operator implicitly borrows its operands:

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
* Arguments to [`format_args!`] except the format string.

r[expr.overload]
## Overloading traits

Many of the following operators and expressions can also be overloaded for other types using traits in `std::ops` or `std::cmp`. These traits also exist in `core::ops` and `core::cmp` with the same names.

r[expr.attr]
## Expression attributes

r[expr.attr.restriction]
[Outer attributes] before an expression are allowed only in a few specific cases:

* Before an expression used as a [statement].
* Elements of [array expressions], [tuple expressions], [call expressions], and tuple-style [struct] expressions.
* The tail expression of [block expressions].
<!-- Keep list in sync with block-expr.md -->

r[expr.attr.never-before]
They are never allowed before:
* [Range] expressions.
* Binary operator expressions ([ArithmeticOrLogicalExpression], [ComparisonExpression], [LazyBooleanExpression], [TypeCastExpression], [AssignmentExpression], [CompoundAssignmentExpression]).

[`Box<T>`]:             special-types-and-traits.md#boxt
[binding mode]:         patterns.ident.binding
[`Copy`]:               special-types-and-traits.md#copy
[`Deref`]:              lang-types.deref
[`DerefMut`]:           lang-types.deref
[`Drop`]:               special-types-and-traits.md#drop
[`if let`]:             expressions/if-expr.md#if-let-patterns
[`Index`]:              std::ops::Index
[`IndexMut`]:           std::ops::IndexMut
[`format_args!`]:       core::format_args
[`pin!`]:               core::pin::pin
[`ref mut` binding]:    patterns.ident.ref
[`Sized`]:              special-types-and-traits.md#sized
[`while let`]:          expressions/loop-expr.md#while-let-patterns
[array expressions]:    expressions/array-expr.md
[array indexing]:       expressions/array-expr.md#array-and-slice-indexing-expressions
[array indexing expression]: expr.array.index
[assign]:               expressions/operator-expr.md#assignment-expressions
[assignment]:           expr.assign
[block expressions]:    expressions/block-expr.md
[borrow]:               expressions/operator-expr.md#borrow-operators
[call expressions]:     expressions/call-expr.md
[comparison]:           expressions/operator-expr.md#comparison-operators
[compound assignment]:  expressions/operator-expr.md#compound-assignment-expressions
[deref]:                expressions/operator-expr.md#the-dereference-operator
[dereference]:          expr.deref
[dereferencing]:        expr.deref
[destructors]:          destructors.md
[drop scope]:           destructors.md#drop-scopes
[extended]:             destructors.scope.lifetime-extension
[extending expression]: destructors.scope.lifetime-extension.exprs
[extending expressions]: destructors.scope.lifetime-extension.exprs
[field]:                expressions/field-expr.md
[field expression]:     expr.field
[functional update]:    expressions/struct-expr.md#functional-update-syntax
[implicit borrow]:      #implicit-borrows
[implicit mutable borrow]: expr.implicit-borrow
[implicitly borrowed]:  expr.implicit-borrow
[index expression]:     expr.array.index
[indexing]:             expr.array.index
[inline assembly]:      asm
[interior mutability]:  interior-mutability.md
[let statement]:        statements.md#let-statements
[macro invocations]:    macro.invocation
[match]:                expressions/match-expr.md
[method-call]:          expressions/method-call-expr.md
[Mutable `static` items]: items/static-items.md#mutable-statics
[Outer attributes]:     attributes.md
[parenthesized expression]: expr.paren
[parenthesized expressions]: expr.paren
[paths]:                expressions/path-expr.md
[place expression contexts]: expr.place-value
[promoted]:             destructors.md#constant-promotion
[Range]:                expressions/range-expr.md
[raw borrow]:           expressions/operator-expr.md#raw-borrow-operators
[rust-lang/rust#44848]: https://github.com/rust-lang/rust/issues/44848
[scrutinee]:            glossary.md#scrutinee
[slice]:                types/slice.md
[statement]:            statements.md
[static variables]:     items/static-items.md
[struct]:               expressions/struct-expr.md
[Structs]:              expr.struct
[temporaries]:          expr.temporary
[temporary scopes]:     destructors.scope.temporary
[Temporary values]:     #temporaries
[tuple expressions]:    expressions/tuple-expr.md
[tuple indexing]:       expr.tuple-index
[tuple indexing expression]: expr.tuple-index
[Tuple structs]:        items.struct.tuple
[Tuples]:               expressions/tuple-expr.md
[Underscores]:          expressions/underscore-expr.md
[Unit structs]:         items.struct.unit
[value expression context]: expr.place-value
[value expression]:     expr.place-value
[Variables]:            variables.md
