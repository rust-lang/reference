r[destructors]
# Destructors

r[destructors.intro]
When an [initialized]&#32;[variable] or [temporary] goes out of
[scope](#drop-scopes), it is *dropped*, which means its *destructor* is run.
(The two terms are used interchangeably here.)
[Assignment] also runs the destructor of its left-hand operand, if it's
initialized. If a variable has been partially initialized, only its initialized
fields are dropped.

r[destructors.operation]
The destructor of a type `T` consists of:

1. If `T: Drop`, calling [`<T as core::ops::Drop>::drop`](core::ops::Drop::drop)
2. Recursively running the destructor of all of its fields.
    * The fields of a [struct] are dropped in declaration order.
    * The fields of the active [enum variant] are dropped in declaration order.
    * The fields of a [tuple] are dropped in order.
    * The elements of an [array] or owned [slice] are dropped from the
      first element to the last.
    * The variables that a [closure] captures by move are dropped in an
      unspecified order.
    * [Trait objects] run the destructor of the underlying type.
    * Other types don't result in any further drops.

r[destructors.drop_in_place]
If a destructor must be run manually, such as when implementing your own smart
pointer, [`core::ptr::drop_in_place`] can be used.

Some examples:

```rust
struct PrintOnDrop(&'static str);

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

let mut overwritten = PrintOnDrop("drops when overwritten");
overwritten = PrintOnDrop("drops when scope ends");

let tuple = (PrintOnDrop("Tuple first"), PrintOnDrop("Tuple second"));

let moved;
// No destructor run on assignment.
moved = PrintOnDrop("Drops when moved");
// Drops now, but is then uninitialized.
moved;

// Uninitialized does not drop.
let uninitialized: PrintOnDrop;

// After a partial move, only the remaining fields are dropped.
let mut partial_move = (PrintOnDrop("first"), PrintOnDrop("forgotten"));
// Perform a partial move, leaving only `partial_move.0` initialized.
core::mem::forget(partial_move.1);
// When partial_move's scope ends, only the first field is dropped.
```

r[destructors.scope]
## Drop scopes

r[destructors.scope.intro]
Each variable or temporary is associated to a *drop scope*. When control flow
leaves a drop scope all variables associated to that scope are dropped in
reverse order of declaration (for variables) or creation (for temporaries).

r[destructors.scope.desugaring]
Drop scopes can be determined by replacing [`for`], [`if`], and [`while`]
expressions with equivalent expressions using [`match`], [`loop`] and
`break`.

r[destructors.scope.operators]
Overloaded operators are not distinguished from built-in operators and [binding
modes] are not considered.

r[destructors.scope.list]
Given a function, or closure, there are drop scopes for:

r[destructors.scope.function]
* The entire function

r[destructors.scope.statement]
* Each [statement]

r[destructors.scope.expression]
* Each [expression]

r[destructors.scope.block]
* Each block, including the function body
    * In the case of a [block expression], the scope for the block and the
      expression are the same scope.

r[destructors.scope.match-arm]
* Each arm of a `match` expression

r[destructors.scope.nesting]
Drop scopes are nested within one another as follows. When multiple scopes are
left at once, such as when returning from a function, variables are dropped
from the inside outwards.

r[destructors.scope.nesting.function]
* The entire function scope is the outer most scope.

r[destructors.scope.nesting.function-body]
* The function body block is contained within the scope of the entire function.

r[destructors.scope.nesting.expr-statement]
* The parent of the expression in an expression statement is the scope of the
  statement.

r[destructors.scope.nesting.let-initializer]
* The parent of the initializer of a [`let` statement] is the `let` statement's
  scope.

r[destructors.scope.nesting.statement]
* The parent of a statement scope is the scope of the block that contains the
  statement.

r[destructors.scope.nesting.match-guard]
* The parent of the expression for a `match` guard is the scope of the arm that
  the guard is for.

r[destructors.scope.nesting.match-arm]
* The parent of the expression after the `=>` in a `match` expression is the
  scope of the arm that it's in.

r[destructors.scope.nesting.match]
* The parent of the arm scope is the scope of the `match` expression that it
  belongs to.

r[destructors.scope.nesting.other]
* The parent of all other scopes is the scope of the immediately enclosing
  expression.

r[destructors.scope.params]
### Scopes of function parameters

All function parameters are in the scope of the entire function body, so are
dropped last when evaluating the function. Each actual function parameter is
dropped after any bindings introduced in that parameter's pattern.

```rust
# struct PrintOnDrop(&'static str);
# impl Drop for PrintOnDrop {
#     fn drop(&mut self) {
#         println!("drop({})", self.0);
#     }
# }
// Drops `y`, then the second parameter, then `x`, then the first parameter
fn patterns_in_parameters(
    (x, _): (PrintOnDrop, PrintOnDrop),
    (_, y): (PrintOnDrop, PrintOnDrop),
) {}

// drop order is 3 2 0 1
patterns_in_parameters(
    (PrintOnDrop("0"), PrintOnDrop("1")),
    (PrintOnDrop("2"), PrintOnDrop("3")),
);
```

r[destructors.scope.bindings]
### Scopes of local variables

r[destructors.scope.bindings.intro]
Local variables declared in a `let` statement are associated to the scope of
the block that contains the `let` statement. Local variables declared in a
`match` expression are associated to the arm scope of the `match` arm that they
are declared in.

```rust
# struct PrintOnDrop(&'static str);
# impl Drop for PrintOnDrop {
#     fn drop(&mut self) {
#         println!("drop({})", self.0);
#     }
# }
let declared_first = PrintOnDrop("Dropped last in outer scope");
{
    let declared_in_block = PrintOnDrop("Dropped in inner scope");
}
let declared_last = PrintOnDrop("Dropped first in outer scope");
```

r[destructors.scope.bindings.patterns]
Variables in patterns are dropped in reverse order of declaration within the pattern.

```rust
# struct PrintOnDrop(&'static str);
# impl Drop for PrintOnDrop {
#     fn drop(&mut self) {
#         println!("drop({})", self.0);
#     }
# }
let (declared_first, declared_last) = (
    PrintOnDrop("Dropped last"),
    PrintOnDrop("Dropped first"),
);
```

r[destructors.scope.bindings.or-patterns]
For the purpose of drop order, [or-patterns] declare bindings in the order given by the first subpattern.

```rust
# struct PrintOnDrop(&'static str);
# impl Drop for PrintOnDrop {
#     fn drop(&mut self) {
#         println!("drop({})", self.0);
#     }
# }
// Drops `x` before `y`.
fn or_pattern_drop_order<T>(
    (Ok([x, y]) | Err([y, x])): Result<[T; 2], [T; 2]>
//   ^^^^^^^^^^   ^^^^^^^^^^^ This is the second subpattern.
//   |
//   This is the first subpattern.
//
//   In the first subpattern, `x` is declared before `y`. Since it is
//   the first subpattern, that is the order used even if the second
//   subpattern, where the bindings are declared in the opposite
//   order, is matched.
) {}

// Here we match the first subpattern, and the drops happen according
// to the declaration order in the first subpattern.
or_pattern_drop_order(Ok([
    PrintOnDrop("Declared first, dropped last"),
    PrintOnDrop("Declared last, dropped first"),
]));

// Here we match the second subpattern, and the drops still happen
// according to the declaration order in the first subpattern.
or_pattern_drop_order(Err([
    PrintOnDrop("Declared last, dropped first"),
    PrintOnDrop("Declared first, dropped last"),
]));
```

r[destructors.scope.temporary]
### Temporary scopes

r[destructors.scope.temporary.intro]
The *temporary scope* of an expression is the scope that is used for the
temporary variable that holds the result of that expression when used in a
[place context], unless it is [promoted].

r[destructors.scope.temporary.enclosing]
Apart from lifetime extension, the temporary scope of an expression is the
smallest scope that contains the expression and is one of the following:

* The entire function.
* A statement.
* The body of an [`if`], [`while`] or [`loop`] expression.
* The `else` block of an `if` expression.
* The non-pattern matching condition expression of an `if` or `while` expression,
  or a `match` guard.
* The body expression for a match arm.
* Each operand of a [lazy boolean expression].
* The pattern-matching condition(s) and consequent body of [`if`] ([destructors.scope.temporary.edition2024]).
* The pattern-matching condition and loop body of [`while`].
* The entirety of the tail expression of a block ([destructors.scope.temporary.edition2024]).

> [!NOTE]
> The [scrutinee] of a `match` expression is not a temporary scope, so temporaries in the scrutinee can be dropped after the `match` expression. For example, the temporary for `1` in `match 1 { ref mut z => z };` lives until the end of the statement.

> [!NOTE]
> The desugaring of a [destructuring assignment] restricts the temporary scope of its assigned value operand (the RHS). For details, see [expr.assign.destructure.tmp-scopes].

r[destructors.scope.temporary.edition2024]
> [!EDITION-2024]
> The 2024 edition added two new temporary scope narrowing rules: `if let` temporaries are dropped before the `else` block, and temporaries of tail expressions of blocks are dropped immediately after the tail expression is evaluated.

Some examples:

```rust
# #![allow(irrefutable_let_patterns)]
# struct PrintOnDrop(&'static str);
# impl Drop for PrintOnDrop {
#     fn drop(&mut self) {
#         println!("drop({})", self.0);
#     }
# }
let local_var = PrintOnDrop("local var");

// Dropped once the condition has been evaluated
if PrintOnDrop("If condition").0 == "If condition" {
    // Dropped at the end of the block
    PrintOnDrop("If body").0
} else {
    unreachable!()
};

if let "if let scrutinee" = PrintOnDrop("if let scrutinee").0 {
    PrintOnDrop("if let consequent").0
    // `if let consequent` dropped here
}
// `if let scrutinee` is dropped here
else {
    PrintOnDrop("if let else").0
    // `if let else` dropped here
};

while let x = PrintOnDrop("while let scrutinee").0 {
    PrintOnDrop("while let loop body").0;
    break;
    // `while let loop body` dropped here.
    // `while let scrutinee` dropped here.
}

// Dropped before the first ||
(PrintOnDrop("first operand").0 == ""
// Dropped before the )
|| PrintOnDrop("second operand").0 == "")
// Dropped before the ;
|| PrintOnDrop("third operand").0 == "";

// Scrutinee is dropped at the end of the function, before local variables
// (because this is the tail expression of the function body block).
match PrintOnDrop("Matched value in final expression") {
    // Dropped once the condition has been evaluated
    _ if PrintOnDrop("guard condition").0 == "" => (),
    _ => (),
}
```

r[destructors.scope.operands]
### Operands

Temporaries are also created to hold the result of operands to an expression
while the other operands are evaluated. The temporaries are associated to the
scope of the expression with that operand. Since the temporaries are moved from
once the expression is evaluated, dropping them has no effect unless one of the
operands to an expression breaks out of the expression, returns, or [panics][panic].

```rust
# struct PrintOnDrop(&'static str);
# impl Drop for PrintOnDrop {
#     fn drop(&mut self) {
#         println!("drop({})", self.0);
#     }
# }
loop {
    // Tuple expression doesn't finish evaluating so operands drop in reverse order
    (
        PrintOnDrop("Outer tuple first"),
        PrintOnDrop("Outer tuple second"),
        (
            PrintOnDrop("Inner tuple first"),
            PrintOnDrop("Inner tuple second"),
            break,
        ),
        PrintOnDrop("Never created"),
    );
}
```

r[destructors.scope.const-promotion]
### Constant promotion

Promotion of a value expression to a `'static` slot occurs when the expression
could be written in a constant and borrowed, and that borrow could be dereferenced
where
the expression was originally written, without changing the runtime behavior.
That is, the promoted expression can be evaluated at compile-time and the
resulting value does not contain [interior mutability] or [destructors] (these
properties are determined based on the value where possible, e.g. `&None`
always has the type `&'static Option<_>`, as it contains nothing disallowed).

r[destructors.scope.lifetime-extension]
### Temporary lifetime extension

> [!NOTE]
> The exact rules for temporary lifetime extension are subject to change. This is describing the current behavior only.

r[destructors.scope.lifetime-extension.let]
The temporary scopes for expressions in `let` statements are sometimes
*extended* to the scope of the block containing the `let` statement. This is
done when the usual temporary scope would be too small, based on certain
syntactic rules. For example:

```rust
let x = &mut 0;
// Usually a temporary would be dropped by now, but the temporary for `0` lives
// to the end of the block.
println!("{}", x);
```

r[destructors.scope.lifetime-extension.static]
Lifetime extension also applies to `static` and `const` items, where it
makes temporaries live until the end of the program. For example:

```rust
const C: &Vec<i32> = &Vec::new();
// Usually this would be a dangling reference as the `Vec` would only
// exist inside the initializer expression of `C`, but instead the
// borrow gets lifetime-extended so it effectively has `'static` lifetime.
println!("{:?}", C);
```

r[destructors.scope.lifetime-extension.sub-expressions]
If a [borrow], [dereference][dereference expression], [field][field expression], or [tuple indexing expression] has an extended temporary scope, then so does its operand. If an [indexing expression] has an extended temporary scope, then the indexed expression also has an extended temporary scope.

r[destructors.scope.lifetime-extension.patterns]
#### Extending based on patterns

r[destructors.scope.lifetime-extension.patterns.extending]
An *extending pattern* is either:

* An [identifier pattern] that binds by reference or mutable reference.

  ```rust
  # fn temp() {}
  let ref x = temp(); // Binds by reference.
  # x;
  let ref mut x = temp(); // Binds by mutable reference.
  # x;
  ```

* A [struct][struct pattern], [tuple][tuple pattern], [tuple struct][tuple struct pattern], [slice][slice pattern], or [or-pattern][or-patterns] where at least one of the direct subpatterns is an extending pattern.

  ```rust
  # use core::sync::atomic::{AtomicU64, Ordering::Relaxed};
  # static X: AtomicU64 = AtomicU64::new(0);
  struct W<T>(T);
  # impl<T> Drop for W<T> { fn drop(&mut self) { X.fetch_add(1, Relaxed); } }
  let W { 0: ref x } = W(()); // Struct pattern.
  # x;
  let W(ref x) = W(()); // Tuple struct pattern.
  # x;
  let (W(ref x),) = (W(()),); // Tuple pattern.
  # x;
  let [W(ref x), ..] = [W(())]; // Slice pattern.
  # x;
  let (Ok(W(ref x)) | Err(&ref x)) = Ok(W(())); // Or pattern.
  # x;
  //
  // All of the temporaries above are still live here.
  # assert_eq!(0, X.load(Relaxed));
  ```

So `ref x`, `V(ref x)` and `[ref x, y]` are all extending patterns, but `x`, `&ref x` and `&(ref x,)` are not.

r[destructors.scope.lifetime-extension.patterns.let]
If the pattern in a `let` statement is an extending pattern then the temporary
scope of the initializer expression is extended.

```rust
# fn temp() {}
// This is an extending pattern, so the temporary scope is extended.
let ref x = *&temp(); // OK
# x;
```

```rust,compile_fail,E0716
# fn temp() {}
// This is neither an extending pattern nor an extending expression,
// so the temporary is dropped at the semicolon.
let &ref x = *&&temp(); // ERROR
# x;
```

```rust
# fn temp() {}
// This is not an extending pattern but it is an extending expression,
// so the temporary lives beyond the `let` statement.
let &ref x = &*&temp(); // OK
# x;
```

r[destructors.scope.lifetime-extension.exprs]
#### Extending based on expressions

r[destructors.scope.lifetime-extension.exprs.extending]
For a let statement with an initializer, an *extending expression* is an
expression which is one of the following:

* The initializer expression.
* The operand of an extending [borrow] expression.
* The [super operands] of an extending [super macro call] expression.
* The operand(s) of an extending [array][array expression], [cast][cast
  expression], [braced struct][struct expression], or [tuple][tuple expression]
  expression.
* The arguments to an extending [tuple struct] or [tuple enum variant] constructor expression.
* The final expression of an extending [block expression] except for an [async block expression].
* The final expression of an extending [`if`] expression's consequent, `else if`, or `else` block.
* An arm expression of an extending [`match`] expression.

> [!NOTE]
> The desugaring of a [destructuring assignment] makes its assigned value operand (the RHS) an extending expression within a newly-introduced block. For details, see [expr.assign.destructure.tmp-ext].

So the borrow expressions in `&mut 0`, `(&1, &mut 2)`, and `Some(&mut 3)`
are all extending expressions. The borrows in `&0 + &1` and `f(&mut 0)` are not.

r[destructors.scope.lifetime-extension.exprs.borrows]
The operand of an extending [borrow] expression has its [temporary scope] [extended].

r[destructors.scope.lifetime-extension.exprs.super-macros]
The [super temporaries] of an extending [super macro call] expression have their [scopes][temporary scopes] [extended].

> [!NOTE]
> `rustc` does not treat [array repeat operands] of extending [array] expressions as extending expressions. Whether it should is an open question.
>
> For details, see [Rust issue #146092](https://github.com/rust-lang/rust/issues/146092).

#### Examples

Here are some examples where expressions have extended temporary scopes:

```rust,edition2024
# use core::pin::pin;
# use core::sync::atomic::{AtomicU64, Ordering::Relaxed};
# static X: AtomicU64 = AtomicU64::new(0);
# #[derive(Debug)] struct S;
# impl Drop for S { fn drop(&mut self) { X.fetch_add(1, Relaxed); } }
# const fn temp() -> S { S }
let x = &temp(); // Operand of borrow.
# x;
let x = &raw const *&temp(); // Operand of raw borrow.
# assert_eq!(X.load(Relaxed), 0);
let x = &temp() as &dyn Send; // Operand of cast.
# x;
let x = (&*&temp(),); // Operand of tuple constructor.
# x;
struct W<T>(T);
let x = W(&temp()); // Argument to tuple struct constructor.
# x;
let x = Some(&temp()); // Argument to tuple enum variant constructor.
# x;
let x = { [Some(&temp())] }; // Final expr of block.
# x;
let x = const { &temp() }; // Final expr of `const` block.
# x;
let x = unsafe { &temp() }; // Final expr of `unsafe` block.
# x;
let x = if true { &temp() } else { &temp() };
//              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//           Final exprs of `if`/`else` blocks.
# x;
let x = match () { _ => &temp() }; // `match` arm expression.
# x;
let x = pin!(temp()); // Super operand of super macro call expression.
# x;
let x = pin!({ &mut temp() }); // As above.
# x;
let x = format_args!("{:?}", temp()); // As above.
# x;
//
// All of the temporaries above are still live here.
# assert_eq!(0, X.load(Relaxed));
```

Here are some examples where expressions don't have extended temporary scopes:

```rust,compile_fail,E0716
# fn temp() {}
// Arguments to function calls are not extending expressions. The
// temporary is dropped at the semicolon.
let x = core::convert::identity(&temp()); // ERROR
# x;
```

```rust,compile_fail,E0716
# fn temp() {}
# trait Use { fn use_temp(&self) -> &Self { self } }
# impl Use for () {}
// Receivers of method calls are not extending expressions.
let x = (&temp()).use_temp(); // ERROR
# x;
```

```rust,compile_fail,E0716
# fn temp() {}
// Scrutinees of match expressions are not extending expressions.
let x = match &temp() { x => x }; // ERROR
# x;
```

```rust,compile_fail,E0515
# fn temp() {}
// Final expressions of `async` blocks are not extending expressions.
let x = async { &temp() }; // ERROR
# x;
```

```rust,compile_fail,E0515
# fn temp() {}
// Final expressions of closures are not extending expressions.
let x = || &temp(); // ERROR
# x;
```

```rust,compile_fail,E0716
# fn temp() {}
// Operands of loop breaks are not extending expressions.
let x = loop { break &temp() }; // ERROR
# x;
```

```rust,compile_fail,E0716
# fn temp() {}
// Operands of breaks to labels are not extending expressions.
let x = 'a: { break 'a &temp() }; // ERROR
# x;
```

```rust,edition2024,compile_fail,E0716
# use core::pin::pin;
# fn temp() {}
// The argument to `pin!` is only an extending expression if the call
// is an extending expression. Since it's not, the inner block is not
// an extending expression, so the temporaries in its trailing
// expression are dropped immediately.
pin!({ &temp() }); // ERROR
```

```rust,edition2024,compile_fail,E0716
# fn temp() {}
// As above.
format_args!("{:?}", { &temp() }); // ERROR
```

r[destructors.forget]
## Not running destructors

r[destructors.manually-suppressing]
### Manually suppressing destructors

[`core::mem::forget`] can be used to prevent the destructor of a variable from being run,
and [`core::mem::ManuallyDrop`] provides a wrapper to prevent a
variable or field from being dropped automatically.

> [!NOTE]
> Preventing a destructor from being run via [`core::mem::forget`] or other means is safe even if it has a type that isn't `'static`. Besides the places where destructors are guaranteed to run as defined by this document, types may *not* safely rely on a destructor being run for soundness.

r[destructors.process-termination]
### Process termination without unwinding

There are some ways to terminate the process without [unwinding], in which case destructors will not be run.

The standard library provides [`std::process::exit`] and [`std::process::abort`] to do this explicitly. Additionally, if the [panic handler][panic.panic_handler.std] is set to `abort`, panicking will always terminate the process without destructors being run.

There is one additional case to be aware of: when a panic reaches a [non-unwinding ABI boundary], either no destructors will run, or all destructors up until the ABI boundary will run.

[Assignment]: expressions/operator-expr.md#assignment-expressions
[binding modes]: patterns.md#binding-modes
[closure]: types/closure.md
[destructors]: destructors.md
[destructuring assignment]: expr.assign.destructure
[expression]: expressions.md
[identifier pattern]: patterns.md#identifier-patterns
[initialized]: glossary.md#initialized
[interior mutability]: interior-mutability.md
[lazy boolean expression]: expressions/operator-expr.md#lazy-boolean-operators
[non-unwinding ABI boundary]: items/functions.md#unwinding
[panic]: panic.md
[place context]: expressions.md#place-expressions-and-value-expressions
[promoted]: destructors.md#constant-promotion
[scrutinee]: glossary.md#scrutinee
[statement]: statements.md
[temporary]: expressions.md#temporaries
[unwinding]: panic.md#unwinding
[variable]: variables.md

[array]: types/array.md
[enum variant]: types/enum.md
[slice]: types/slice.md
[struct]: types/struct.md
[Trait objects]: types/trait-object.md
[tuple]: types/tuple.md

[or-patterns]: patterns.md#or-patterns
[slice pattern]: patterns.md#slice-patterns
[struct pattern]: patterns.md#struct-patterns
[tuple pattern]: patterns.md#tuple-patterns
[tuple struct pattern]: patterns.md#tuple-struct-patterns
[tuple struct]: type.struct.tuple
[tuple enum variant]: type.enum.declaration

[array expression]: expressions/array-expr.md#array-expressions
[array repeat operands]: expr.array.repeat-operand
[async block expression]: expr.block.async
[block expression]: expressions/block-expr.md
[borrow]: expr.operator.borrow
[cast expression]: expressions/operator-expr.md#type-cast-expressions
[dereference expression]: expressions/operator-expr.md#the-dereference-operator
[extended]: destructors.scope.lifetime-extension
[field expression]: expressions/field-expr.md
[indexing expression]: expressions/array-expr.md#array-and-slice-indexing-expressions
[struct expression]: expressions/struct-expr.md
[super macro call]: expr.super-macros
[super operands]: expr.super-macros
[super temporaries]: expr.super-macros
[temporary scope]: destructors.scope.temporary
[temporary scopes]: destructors.scope.temporary
[tuple expression]: expressions/tuple-expr.md#tuple-expressions
[tuple indexing expression]: expressions/tuple-expr.md#tuple-indexing-expressions

[`for`]: expressions/loop-expr.md#iterator-loops
[`if let`]: expressions/if-expr.md#if-let-patterns
[`if`]: expressions/if-expr.md#if-expressions
[`let` statement]: statements.md#let-statements
[`loop`]: expressions/loop-expr.md#infinite-loops
[`match`]: expressions/match-expr.md
[`while let`]: expressions/loop-expr.md#while-let-patterns
[`while`]: expressions/loop-expr.md#predicate-loops
