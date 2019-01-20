# Destructors

When an [initialized]&#32;[variable] or [temporary] in Rust goes out of
[scope](#drop-scopes) its *destructor* is run, or it is *dropped*. [Assignment]
also runs the destructor of its left-hand operand, if it's initialized. If a
variable has been partially initialized, only its initialized fields are
dropped.

The destructor of a type `T` consists of

1. If `T: Drop`, calling [`<T as std::ops::Drop>::drop`]
2. Recursively running the destructor of all of its fields.
    * The fields of a [struct] or [tuple] are dropped in declaration order.
    * The fields of the active [enum variant] are dropped in declaration order.
    * The elements of an [array] or owned [slice] are dropped from the
      first element to the last.
    * The variables that a [closure] captures by move are dropped in an
      unspecified order.
    * [Trait objects] run the destructor of the underlying type.
    * Other types don't result in any further drops.

If a destructor must be run manually, such as when implementing your own smart
pointer, [`std::ptr::drop_in_place`] can be used.

Some examples:

```rust
struct ShowOnDrop(&'static str);

impl Drop for ShowOnDrop {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

let mut overwritten = ShowOnDrop("drops when overwritten");
overwritten = ShowOnDrop("drops when scope ends");

let tuple = (ShowOnDrop("Tuple first"), ShowOnDrop("Tuple second"));

let moved;
// No destructor run on assignment.
moved = ShowOnDrop("Drops when moved");
// drops now, but is then uninitialized
moved;

// Uninitialized does not drop.
let uninitialized: ShowOnDrop;

// After a partial move, only the remaining fields are dropped.
let mut partial_move = (ShowOnDrop("first"), ShowOnDrop("forgotten"));
// Perform a partial move, leaving only `partial_move.0` initialized.
core::mem::forget(partial_move.1);
// When partial_move's scope ends, only the first field is dropped.
```

## Drop scopes

Each variable or temporary is associated to a *drop scope*. When control flow
leaves a drop scope all variables associated to that scope are dropped in
reverse order of declaration (for variables) or creation (for temporaries).

Drop scopes are determined after replacing [`for`], [`if let`] and
[`while let`] expressions with the equivalent expressions using [`match`].
Overloaded operators are not distinguished from built-in operators and [binding
modes] are not considered.

Given a function, or closure, there are drop scopes for:

* The entire function
* Each [statement]
* Each [expression]
* Each block, including the function body
    * [Block expressions] the expression and block scopes are the same scope.
* Each arm of a `match` expression

Drop scopes are nested within one another as follows. When multiple scopes are
left at once, such as when returning from a function, variables are dropped
from the inside outwards.

* The entire function scope is the outer most scope.
* The function body block is contained within the scope of the entire function.
* The parent of the expression in an expression statement is the scope of the
  statement.
* The parent of the initializer of a `let` statement is the `let`
  statement's scope.
* The parent of a statement scope is the scope of the block that contains the
  statement.
* The parent of the expression for a `match` guard is the scope of the arm that
  it's for.
* The parent of the expression for a given `match` arm is that arm's scope.
* The parent of the arm scope is the scope of the match expression that it
  belongs to.
* The parent of all other scopes is the scope of the immediately enclosing
  expression.

### Scopes of function parameters

All function parameters are in the scope of the entire function body, so are
dropped last when evaluating the function. The actual function parameter is
dropped after any named parameters that are bound to parts of it.

```rust
# struct ShowOnDrop(&'static str);
# impl Drop for ShowOnDrop {
#     fn drop(&mut self) {
#         println!("drop({})", self.0);
#     }
# }
// Drops the second parameter, then `y`, then the first parameter, then `x`
fn patterns_in_parameters(
    (x, _): (ShowOnDrop, ShowOnDrop),
    (_, y): (ShowOnDrop, ShowOnDrop),
) {}

// drop order is 3 2 0 1
patterns_in_parameters(
    (ShowOnDrop("0"), ShowOnDrop("1")),
    (ShowOnDrop("2"), ShowOnDrop("3")),
);
```

### Scopes of local variables

Local variables declared in a `let` statement are associated to the scope of
the block that contains the `let` statement. Local variables declared in a
match are associated to the arm scope of the `match` arm that they are declared
in.

```rust
# struct ShowOnDrop(&'static str);
# impl Drop for ShowOnDrop {
#     fn drop(&mut self) {
#         println!("drop({})", self.0);
#     }
# }
let declared_first = ShowOnDrop("Dropped last in outer scope");
{
    let declared_in_block = ShowOnDrop("Dropped in inner scope");
}
let declared_last = ShowOnDrop("Dropped first in outer scope");
```

If multiple patterns are used in the same arm for a match, then an unspecified
pattern will be used to determine the drop order.

### Temporary scopes

The *temporary scope* of an expression is the scope that is used for the
temporary variable that holds the result of that expression when used in a
[place context], unless it is promoted to a `static`.

Apart from lifetime extension, the temporary scope of an expression is the
smallest scope that contains the expression and is for one of the following:

* The entire function body.
* A statement.
* The body of a [`if`], [`while`] or [`loop`] expression.
* The `else` block of an `if` expression.
* The condition expression of an `if` or `while` expression, or a `match` guard.
* The expression for a match arm.
* The second operand of a [lazy boolean expression].

> Note: Temporaries that are created in the final expression of a function body
> are dropped *after* any named variables bound in the function body, as there
> is no smaller enclosing temporary scope.

Some examples:

```rust
# struct ShowOnDrop(&'static str);
# impl Drop for ShowOnDrop {
#     fn drop(&mut self) {
#         println!("drop({})", self.0);
#     }
# }
let local_var = ShowOnDrop("local var");

// Dropped once the condition has been evaluated
if ShowOnDrop("If condition").0 == "If condition" {
    // Dropped at the end of the block
    ShowOnDrop("If body").0
} else {
    unreachable!()
};

// Dropped at the end of the statement
(ShowOnDrop("first operand").0 == ""
// Dropped at the )
|| ShowOnDrop("second operand").0 == "")
// Dropped at the end of the expression
|| ShowOnDrop("third operand").0 == "";

// Dropped at the end of the function, after local variables.
// Changing this to a statement containing a return expression would make the
// temporary be dropped before the local variables. Binding to a variable
// which is then returned would also make the temporary be dropped first.
match ShowOnDrop("Matched value in final expression") {
    // Dropped once the condition has been evaluated
    _ if ShowOnDrop("guard condition").0 == "" => (),
    _ => (),
}
```

### Operands

Temporaries are also created to hold the result of operands to an expression
while the other operands are evaluated. The temporaries are associated to the
scope of the expression with that operand. Since the temporaries are moved from
once the expression is evaluated, dropping them has no effect unless one of the
operands to an expression breaks out of the expression, returns, or panics.

```rust
# struct ShowOnDrop(&'static str);
# impl Drop for ShowOnDrop {
#     fn drop(&mut self) {
#         println!("drop({})", self.0);
#     }
# }
loop {
    // Tuple expression doesn't finish evaluating so operands drop in reverse order
    (
        ShowOnDrop("Outer tuple first"),
        ShowOnDrop("Outer tuple second"),
        (
            ShowOnDrop("Inner tuple first"),
            ShowOnDrop("Inner tuple second"),
            break,
        ),
        ShowOnDrop("Never created"),
    );
}
```

### Constant promotion

Promotion of a value expression to a `'static` slot occurs when the expression
could be written in a constant, borrowed, and dereferencing that borrow where
the expression was originally written, without changing the runtime behavior.
That is, the promoted expression can be evaluated at compile-time and the
resulting value does not contain [interior mutability] or [destructors] (these
properties are determined based on the value where possible, e.g. `&None`
always has the type `&'static Option<_>`, as it contains nothing disallowed).

### Temporary lifetime extension

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

If a borrow, dereference, field or tuple indexing expression has an extended
temporary scope then so does its operand. If an indexing expression has an
extended temporary scope then the indexed expression also has an extended
temporary scope.

#### Extending based on patterns

An *extending pattern* is either

* An [identifier pattern] that binds by reference or mutable reference.
* A [struct][struct pattern], [tuple][tuple pattern], [tuple struct][tuple
  struct pattern] or [slice][slice pattern] pattern where at least one of the
  direct subpatterns is a extending pattern.

So `ref x`, `V(ref x)` and `[ref x, y]` are all extending patterns, but `x`,
`&ref x` and `&(ref x,)` are not.

If the pattern in a `let` statement is an extending pattern then the temporary
scope of the initializer expression is extended.

#### Extending based on expressions

For a let statement with an initializer, an *extending expression* is an
expression which is one of the following:

* The initializer expression.
* The operand of a extending [borrow expression].
* The operand(s) of an extending [array][array expression], [cast][cast
  expression], [braced struct][struct expression] or [tuple][tuple expression]
  expression.
* The final expression of any extending [block expression][block expressions].

So the borrow expressions in `&mut 0`, `(&1, &mut 2)` and `Some { 0: &mut 3 }`
are all extending expressions, while the borrows in `&0 + &1` and
`Some(&mut 0)` are not.

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
let x = { [Some { 0: &temp(), }] };
let ref x = temp();
let ref x = *&temp();
# x;
```

Here are some examples where expressions don't have extended temporary scopes:

```rust,compile_fail
# fn temp() {}
# trait Use { fn use_temp(&self) -> &Self { self } }
# impl Use for () {}
// The temporary that stores the result of `temp()` lives only lives until the
// end of the let statement in these cases.

let x = Some(&temp());         // ERROR
let x = (&temp()).use_temp();  // ERROR
# x;
```

## Not running destructors

Not running destructors in Rust is safe even if it has a type that isn't
`'static`. [`std::mem::ManuallyDrop`] provides a wrapper to prevent a
variable or field from being dropped automatically.

[Assignment]: expressions/operator-expr.md#assignment-expressions
[binding modes]: patterns.md#binding-modes
[closure]: types/closure.md
[destructors]: destructors.md
[expression]: expressions.md
[identifier pattern]: patterns.md#identifier-patterns
[initialized]: glossary.md#initialized
[interior mutability]: interior-mutability.md
[lazy boolean expression]: expressions/operator-expr.md#lazy-boolean-operators
[place context]: expressions.md#place-expressions-and-value-expressions
[statement]: statements.md
[temporary]: expressions.md#temporaries
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

[array expression]: expressions/array-expr.md#array-expressions
[block expressions]: expressions/block-expr.md
[borrow expression]: expressions/operator-expr.md#borrow-operators
[cast expression]: expressions/operator-expr.md#type-cast-expressions
[struct expression]: expressions/struct-expr.md
[tuple expression]: expressions/tuple-expr.md#tuple-expressions

[`for`]: expressions/loop-expr.md#iterator-loops
[`if let`]: expressions/if-expr.md#if-let-expressions
[`if`]: expressions/if-expr.md#if-expressions
[`let` statement]: statements.md#let-statements
[`loop`]: expressions/loop-expr.md#infinite-loops
[`match`]: expressions/match-expr.md
[`while let`]: expressions/loop-expr.md#predicate-pattern-loops
[`while`]: expressions/loop-expr.md#predicate-loops

[`<T as std::ops::Drop>::drop`]: ../std/ops/trait.Drop.html#tymethod.drop
[`std::ptr::drop_in_place`]: ../std/ptr/fn.drop_in_place.html
[`std::mem::ManuallyDrop`]: ../std/mem/struct.ManuallyDrop.html
