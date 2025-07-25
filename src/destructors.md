r[destructors]
# Destructors

r[destructors.intro]
When an [initialized]&#32;[variable] or [temporary] goes out of
[scope](#drop-scopes), its *destructor* is run, or it is *dropped*. [Assignment]
also runs the destructor of its left-hand operand, if it's initialized. If a
variable has been partially initialized, only its initialized fields are
dropped.

r[destructors.operation]
The destructor of a type `T` consists of:

1. If `T: Drop`, calling [`<T as std::ops::Drop>::drop`](std::ops::Drop::drop)
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
pointer, [`std::ptr::drop_in_place`] can be used.

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

r[destructors.scope.bindings.match-pattern-order]
If multiple patterns are used in the same arm for a `match` expression, then an
unspecified pattern will be used to determine the drop order.

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
If a [borrow][borrow expression], [dereference][dereference expression],
[field][field expression], or [tuple indexing expression] has an extended
temporary scope then so does its operand. If an [indexing expression] has an
extended temporary scope then the indexed expression also has an extended
temporary scope.

r[destructors.scope.lifetime-extension.patterns]
#### Extending based on patterns

r[destructors.scope.lifetime-extension.patterns.extending]
An *extending pattern* is either

* An [identifier pattern] that binds by reference or mutable reference.
* A [struct][struct pattern], [tuple][tuple pattern], [tuple struct][tuple
  struct pattern], or [slice][slice pattern] pattern where at least one of the
  direct subpatterns is an extending pattern.

So `ref x`, `V(ref x)` and `[ref x, y]` are all extending patterns, but `x`,
`&ref x` and `&(ref x,)` are not.

r[destructors.scope.lifetime-extension.patterns.let]
If the pattern in a `let` statement is an extending pattern then the temporary
scope of the initializer expression is extended.

r[destructors.scope.lifetime-extension.exprs]
#### Extending based on expressions

For a let statement with an initializer, an *extending expression* is an
expression which is one of the following:

* The initializer expression.
* The operand of an extending [borrow expression].
* The operand(s) of an extending [array][array expression], [cast][cast
  expression], [braced struct][struct expression], or [tuple][tuple expression]
  expression.
* The arguments to an extending [tuple struct] or [tuple variant] constructor expression.
* The final expression of any extending [block expression].

So the borrow expressions in `&mut 0`, `(&1, &mut 2)`, and `Some(&mut 3)`
are all extending expressions. The borrows in `&0 + &1` and `f(&mut 0)` are not.

The operand of any extending borrow expression has its temporary scope
extended.

#### Examples

Here are some examples where expressions have extended temporary scopes:

```rust
# fn temp() {}
# trait Use { fn use_temp(&self) -> &Self { self } }
# impl Use for () {}
// The temporary that stores the result of `temp()` lives in the same scope
// as x in these cases.
let x = &temp();
let x = &temp() as &dyn Send;
let x = (&*&temp(),);
let x = { [Some(&temp()) ] };
let ref x = temp();
let ref x = *&temp();
# x;
```

Here are some examples where expressions don't have extended temporary scopes:

```rust,compile_fail
# fn temp() {}
# trait Use { fn use_temp(&self) -> &Self { self } }
# impl Use for () {}
// The temporary that stores the result of `temp()` only lives until the
// end of the let statement in these cases.

let x = std::convert::identity(&temp()); // ERROR
let x = (&temp()).use_temp();  // ERROR
# x;
```

r[destructors.forget]
## Not running destructors

r[destructors.manually-suppressing]
### Manually suppressing destructors

[`std::mem::forget`] can be used to prevent the destructor of a variable from being run,
and [`std::mem::ManuallyDrop`] provides a wrapper to prevent a
variable or field from being dropped automatically.

> [!NOTE]
> Preventing a destructor from being run via [`std::mem::forget`] or other means is safe even if it has a type that isn't `'static`. Besides the places where destructors are guaranteed to run as defined by this document, types may *not* safely rely on a destructor being run for soundness.

r[destructors.process-termination]
### Process termination without unwinding

There are some ways to terminate the process without [unwinding], in which case destructors will not be run.

The standard library provides [`std::process::exit`] and [`std::process::abort`] to do this explicitly. Additionally, if the [panic handler][panic.panic_handler.std] is set to `abort`, panicking will always terminate the process without destructors being run.

There is one additional case to be aware of: when a panic reaches a [non-unwinding ABI boundary], either no destructors will run, or all destructors up until the ABI boundary will run.

[Assignment]: expressions/operator-expr.md#assignment-expressions
[binding modes]: patterns.md#binding-modes
[closure]: types/closure.md
[destructors]: destructors.md
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

[slice pattern]: patterns.md#slice-patterns
[struct pattern]: patterns.md#struct-patterns
[tuple pattern]: patterns.md#tuple-patterns
[tuple struct pattern]: patterns.md#tuple-struct-patterns
[tuple struct]: type.struct.tuple
[tuple variant]: type.enum.declaration

[array expression]: expressions/array-expr.md#array-expressions
[block expression]: expressions/block-expr.md
[borrow expression]: expressions/operator-expr.md#borrow-operators
[cast expression]: expressions/operator-expr.md#type-cast-expressions
[dereference expression]: expressions/operator-expr.md#the-dereference-operator
[field expression]: expressions/field-expr.md
[indexing expression]: expressions/array-expr.md#array-and-slice-indexing-expressions
[struct expression]: expressions/struct-expr.md
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
