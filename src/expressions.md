# Expressions

An expression may have two roles: it always produces a *value*, and it may have
*effects* (otherwise known as "side effects"). An expression *evaluates to* a
value, and has effects during *evaluation*. Many expressions contain
sub-expressions (operands). The meaning of each kind of expression dictates
several things:

* Whether or not to evaluate the sub-expressions when evaluating the expression
* The order in which to evaluate the sub-expressions
* How to combine the sub-expressions' values to obtain the value of the
  expression

In this way, the structure of expressions dictates the structure of execution.
Blocks are just another kind of expression, so blocks, statements, expressions,
and blocks again can recursively nest inside each other to an arbitrary depth.

## Lvalues and rvalues

Expressions are divided into two main categories: _lvalues_ and _rvalues_.
Likewise within each expression, sub-expressions may occur in _lvalue context_
or _rvalue context_. The evaluation of an expression depends both on its own
category and the context it occurs within.

An lvalue is an expression that represents a memory location. These expressions
are [paths](#path-expressions) which refer to local variables, function and
method arguments, or static variables,
[dereferences](#the-dereference-operator) (`*expr`), [indexing
expressions](#index-expressions) (`expr[expr]`), [field
references](#field-expressions) (`expr.f`) and parenthesized lvalue
expressions. All other expressions are rvalues.

The left operand of an [assignment](#assignment-expressions) or
[compound-assignment](#compound-assignment-expressions) expression is an lvalue
context, as is the single operand of a unary [borrow](#borrow-operators), and
the operand of any [implicit borrow](#implicit-borrows). The discriminant or
subject of a [match expression](#match-expressions) and right side of a `let`
binding may be an lvalue context, if ref bindings are made, but is otherwise an
rvalue context. All other expression contexts are rvalue contexts.

### Moved and copied types

When an lvalue is evaluated in an _rvalue context_, it denotes the value held
_in_ that memory location. If value is of a type that implements `Copy`, then
the value will be copied. In the remaining situations if the type of the value
is [`Sized`](the-sized-trait.html) it may be possible to move the value. Only
the following lvalues may be moved out of:

* [Variables](variables.html) which are not currently borrowed.
* [Temporary values](#temporary-lifetimes).
* [Fields](#field-expressions) of an lvalue which can be moved out of and
  doesn't implement [`Drop`](the-drop-trait.html).
* The result of [dereferencing](#the-dereference-operator) an expression with
  type `Box<T>` and that can also be moved out of.

Moving out of an lvalue deinitializes that location (if it comes from a local
variable), so that it can't be read from again. In all other cases, trying to
use an lvalue in an rvalue context is an error.

### Mutability

For an lvalue to be [assigned](#assignment-expressions) to, [mutably
borrowed](#borrow-operators), [implicitly mutably borrowed](#implicit-borrows)
or bound to a pattern containing `ref mut` it must be _mutable_, we call these
contexts _mutable_ lvalue contexts, other lvalue contexts are called
_immutable_.

The following expressions can create mutable lvalues:

* Mutable [variables](variables.html), which are not currently borrowed.
* [Mutable `static` items](items.html#mutable-statics).
* [Temporary values](#temporary-lifetimes).
* [Fields](#field-expressions), this evaluates the subexpression in a mutable
  lvalue context.
* [Dereferenes](#the-dereference-operator) of a `*mut T` pointer.
* Dereference of a variable, or field of a variable, with type `&mut T`. Note:
  this is an exception to the requirement for the next rule.
* Dereferences of a type that implements `DerefMut`, this then requires that
  the value being dereferenced is evaluated is a mutable lvalue context.
* [Indexing](#index-expressions) of a type that implements `DerefMut`, this
  then evalutes the value being indexed (but not the index) in mutable lvalue
  context.

### Temporary lifetimes

When using an rvalue in most lvalue contexts, a temporary unnamed lvalue is
created and used instead. The lifetime of temporary values is typically the
innermost enclosing statement; the tail expression of a block is considered
part of the statement that encloses the block.

When a temporary rvalue is being created that is assigned into a `let`
declaration, however, the temporary is created with the lifetime of the
enclosing block instead, as using the enclosing statement (the `let`
declaration) would be a guaranteed error (since a pointer to the temporary
would be stored into a variable, but the temporary would be freed before the
variable could be used). The compiler uses simple syntactic rules to decide
which values are being assigned into a `let` binding, and therefore deserve a
longer temporary lifetime.

Here are some examples:

- `let x = foo(&temp())`. The expression `temp()` is an rvalue. As it
  is being borrowed, a temporary is created which will be freed after
  the innermost enclosing statement (the `let` declaration, in this case).
- `let x = temp().foo()`. This is the same as the previous example,
  except that the value of `temp()` is being borrowed via autoref on a
  method-call. Here we are assuming that `foo()` is an `&self` method
  defined in some trait, say `Foo`. In other words, the expression
  `temp().foo()` is equivalent to `Foo::foo(&temp())`.
- `let x = &temp()`. Here, the same temporary is being assigned into
  `x`, rather than being passed as a parameter, and hence the
  temporary's lifetime is considered to be the enclosing block.
- `let x = SomeStruct { foo: &temp() }`. As in the previous case, the
  temporary is assigned into a struct which is then assigned into a
  binding, and hence it is given the lifetime of the enclosing block.
- `let x = [ &temp() ]`. As in the previous case, the
  temporary is assigned into an array which is then assigned into a
  binding, and hence it is given the lifetime of the enclosing block.
- `let ref x = temp()`. In this case, the temporary is created using a ref binding,
  but the result is the same: the lifetime is extended to the enclosing block.

### Implicit Borrows

Certain expressions will treat an expression as an lvalue by implicitly
borrowing it. For example, it is possible to compare two unsized
[slices](types.html#array-and-slice-types) for equality directly, because the
`==` operator implicitly borrows it's operands:

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

Implicit borrows may be taken in the following expressions:

* Left operand in [method-call expressions](#method-call-expressions).
* Left operand in [field expressions](#field-expressions).
* Left operand in [call expressions](#call-expressions).
* Left operand in [index expressions](#index-expressions).
* Operand of the [dereference](#the-dereference-operator) (`*`) operator.
* Operands of [comparison operators](#comparison-operators).
* Left operands of the [compound assignment](#compound-assignment-expressions).

## Constant expressions

Certain types of expressions can be evaluated at compile time. These are called
_constant expressions_. Certain places, such as in
[constants](items.html#constant-items) and [statics](items.html#static-items),
require a constant expression, and are always evaluated at compile time. In
other places, such as in [`let` statements](statements.html#let-statements),
constant expressions may be evaluated at compile time. If errors, such as out
of bounds [array access](#index-expressions) or [overflow](#overflow) occurs,
then it is a compiler error if the value must be evaluated at compile time,
otherwise it is just a warning, but the code will most likely panic when run.

The following expressions are constant expressions, so long as any operands are
also constant expressions:

* [Literals](#literal-expressions).
* [Paths](#path-expressions) to [functions](items.html#functions) and constants.
  Recursively defining constants is not allowed.
* Paths to statics, so long as only their address, not their value, is used.
  This includes using their value indirectly through a compilicated expression.
  \*
* [Tuple expressions](#tuple-expressions).
* [Array expressions](#array-expressions).
* [Struct expressions](#struct-expressions), where the type does not implement
  [`Drop`](the-drop-trait.html).
* [Variant expressions](#enumeration-variant-expressions), where the
  enumeration type does not implement `Drop`.
* [Block expressions](#block-expressions) (and `unsafe` blocks) which contain
  only items and possibly a (constant) tail expression.
* [Field expressions](#field-expressions).
* [Index expressions](#index-expressions), indexing a [array or
  slice](types.html#array-and-slice-types) with a `usize`.
* [Range expressions](#range-expressions).
* [Closure expressions](#closure-expressions) which don't capture variables
  from the environment.
* Built in [negation](#negation-operators), [arithmetic,
  logical](#arithmetic-and-logical-binary-operators),
  [comparison](#comparison-operators) or [lazy
  boolean](#lazy-boolean-operators) operators used on integer and floating
  point types, `bool` and `char`.
* Shared [borrow expressions](#borrow-operators).
* The [dereference operator](#the-dereference-operator), but not to circumvent the
  rule on statics.
* [Grouped expressions](#grouped-expressions).
* [Cast expressions](#type-cast-expressions), except pointer to address and
  function pointer to address casts.

\* Only in static items.

## Overloading Traits

Many of the following operators and expressions can also be overloaded for
other types using traits in `std::ops` or `std::cmp`, these traits here also
exist in `core::ops` and `core::cmp` with the same names.

## Literal expressions

A _literal expression_ consists of one of the [literal](tokens.html#literals)
forms described earlier. It directly describes a number, character, string,
boolean value, or the unit value.

```rust
();        // unit type
"hello";   // string type
'5';       // character type
5;         // integer type
```

## Path expressions

A [path](paths.html) used as an expression context denotes either a local
variable or an item. Path expressions that resolve to local or static variables
are [lvalues](expressions.html#lvalues-and-rvalues), other paths
are rvalues. Using a `static mut` variable requires an [`unsafe`
block](#unsafe-blocks).

```rust
# mod globals {
#     pub static STATIC_VAR: i32 = 5;
#     pub static mut STATIC_MUT_VAR: i32 = 7;
# }
# let local_var = 3;
local_var;
globals::STATIC_VAR;
unsafe { globals::STATIC_MUT_VAR };
let some_constructor = Some::<i32>;
let push_integer = Vec::<i32>::push;
let slice_reverse = <[i32]>::reverse;
```

## Tuple expressions

Tuples are written by enclosing zero or more comma-separated expressions in
parentheses. They are used to create [tuple-typed](types.html#tuple-types)
values.

```rust
(0.0, 4.5);
("a", 4usize, true);
```

You can disambiguate a single-element tuple from a value in parentheses with a
comma:

```rust
(0,); // single-element tuple
(0); // zero in parentheses
```

## Struct expressions

There are several forms of struct expressions. A _struct expression_ consists
of the [path](paths.html) of a [struct item](items.html#structs), followed by a
brace-enclosed list of zero or more comma-separated name-value pairs, providing
the field values of a new instance of the struct. A field name can be any
[identifier](identifiers.html), and is separated from its value expression by a
colon. In the case of a tuple struct the field names are numbers corresponding
to the position of the field. The numbers must be written in decimal,
containing no underscores and with no leading zeros or integer suffix.

Struct expressions can't be used directly in the head of a [loop](#loops) or an
[`if`](#if-expressions), [`if let`](#if-let-expressions) or
[`match`](#match-expressions) expression. But struct expressions can still be
in used inside parentheses, for example.

A _tuple struct expression_ consists of the path of a struct item, followed by
a parenthesized list of one or more comma-separated expressions (in other
words, the path of a struct item followed by a tuple expression). The struct
item must be a tuple struct item.

A _unit-like struct expression_ consists only of the path of a struct item.

The following are examples of struct expressions:

```rust
# struct Point { x: f64, y: f64 }
# struct NothingInMe { }
# struct TuplePoint(f64, f64);
# mod game { pub struct User<'a> { pub name: &'a str, pub age: u32, pub score: usize } }
# struct Cookie; fn some_fn<T>(t: T) {}
Point {x: 10.0, y: 20.0};
NothingInMe {};
TuplePoint(10.0, 20.0);
TuplePoint { 0: 10.0, 1: 20.0 }; // Results in the same value as the above line
let u = game::User {name: "Joe", age: 35, score: 100_000};
some_fn::<Cookie>(Cookie);
```

A struct expression forms a new value of the named struct type. Note that for a
given *unit-like* struct type, this will always be the same value.

A struct expression can terminate with the syntax `..` followed by an
expression to denote a functional update. The expression following `..` (the
base) must have the same struct type as the new struct type being formed. The
entire expression denotes the result of constructing a new struct (with the
same type as the base expression) with the given values for the fields that
were explicitly specified and the values in the base expression for all other
fields. Just as with all struct expressions, all of the fields of the struct
must be [visible](visibility-and-privacy.html), even those not explicitly
named.

```rust
# struct Point3d { x: i32, y: i32, z: i32 }
let base = Point3d {x: 1, y: 2, z: 3};
Point3d {y: 0, z: 10, .. base};
```

#### Struct field init shorthand

When initializing a data structure (struct, enum, union) with named (but not
numbered) fields, it is allowed to write `fieldname` as a shorthand for
`fieldname: fieldname`. This allows a compact syntax with less duplication.

Example:

```rust
# struct Point3d { x: i32, y: i32, z: i32 }
# let x = 0;
# let y_value = 0;
# let z = 0;
Point3d { x: x, y: y_value, z: z };
Point3d { x, y: y_value, z };
```

### Enumeration Variant expressions

Enumeration variants can be constructed similarly to structs, using a path to
an enum variant instead of to a struct:

```rust
# enum Message {
#     Quit,
#     WriteString(String),
#     Move { x: i32, y: i32 },
# }
let q = Message::Quit;
let w = Message::WriteString("Some string".to_string());
let m = Message::Move { x: 50, y: 200 };
```

## Block expressions

A _block expression_ is similar to a module in terms of the declarations that
are possible, but can also contain [statements](statements.html) and end with
an expression. Each block conceptually introduces a new namespace scope. Use
items can bring new names into scopes and declared items are in scope for only
the block itself.

A block will execute each statement sequentially, and then execute the
expression (if given). If the block doesn't end in an expression, its value is
`()`:

```rust
let x: () = { println!("Hello."); };
```

If it ends in an expression, its value and type are that of the expression:

```rust
let x: i32 = { println!("Hello."); 5 };

assert_eq!(5, x);
```

Blocks are always [rvalues](expressions.html#lvalues-and-rvalues) and evaluate the last
expression in rvalue context. This can be used to force moving a value
if really needed.

### `unsafe` blocks

_See [`unsafe` block](unsafe-blocks.html) for more information on when to use `unsafe`_

A block of code can be prefixed with the `unsafe` keyword, to permit calling
`unsafe` functions or dereferencing raw pointers within a safe function.

## Method-call expressions

A _method call_ consists of an expression followed by a single dot, an
[identifier](identifiers.html), and a parenthesized expression-list. Method
calls are resolved to methods on specific traits, either statically dispatching
to a method if the exact `self`-type of the left-hand-side is known, or
dynamically dispatching if the left-hand-side expression is an indirect [trait
object](types.html#trait-objects). Method call expressions will automatically
take a shared or mutable borrow of the receiver if needed.

```rust
let pi: Result<f32, _> = "3.14".parse();
let log_pi = pi.unwrap_or(1.0).log(2.72);
# assert!(1.14 < log_pi && log_pi < 1.15)
```

When resolving method calls on an expression of type `A`, Rust will use the
following order:

1. Inherent methods, with receiver of type `A`, `&A`, `&mut A`.
1. Trait methods with receiver of type `A`.
1. Trait methods with receiver of type `&A`.
1. Trait methods with receiver of type `&mut A`.
1. If it's possible, Rust will then repeat steps 1-5 with
  `<A as std::ops::Deref>::Target`, and insert a dereference operator.
1. If `A` is now an [array](types.html#array-and-slice-types) type, then
  repeat steps 1-4 with the corresponding slice type.

Note: that in steps 1-4 the receiver is used, not the type of `Self` nor the
type of `A`. For example

```rust,ignore
// `Self` is `&A`, receiver is `&A`.
impl<'a> Trait for &'a A {
    fn method(self) {}
}
// If `A` is `&B`, then `Self` is `B` and the receiver is `A`.
impl B {
    fn method(&self) {}
}
```

Another note: this process does not use the mutability or lifetime of the
receiver, or whether `unsafe` methods can currently be called to resolve
methods. These constraints instead lead to compiler errors.

If a step is reached where there is more than one possible method (where
generic methods or traits are considered the same), then it is a compiler
error. These cases require a [more specific
syntax.](#disambiguating-function-calls) for method and function invocation.

## Field expressions

A _field expression_ consists of an expression followed by a single dot and an
[identifier](identifiers.html), when not immediately followed by a
parenthesized expression-list (the latter is always a [method call
expression](#method-call-expressions)). A field expression denotes a field of a
[struct](types.html#struct-types). To call a function stored in a struct
parentheses are needed around the field expression

```rust,ignore
mystruct.myfield;
foo().x;
(Struct {a: 10, b: 20}).a;
mystruct.method();          // Method expression
(mystruct.function_field)() // Call expression containing a field expression
```

A field access is an [lvalue](expressions.html#lvalues-and-rvalues) referring to the value of
that field. When the subexpression is [mutable](#mutability), the field
expression is also mutable.

Also, if the type of the expression to the left of the dot is a pointer, it is
automatically dereferenced as many times as necessary to make the field access
possible. In cases of ambiguity, we prefer fewer autoderefs to more.

Finally the fields of a struct, a reference to a struct are treated as separate
entities when borrowing. If the struct does not implement
[`Drop`](the-drop-trait.html) this also applies to moving out of each of its fields
where possible. This also does not apply if automatic dereferencing is done
though user defined types.

```rust
# struct A { f1: String, f2: String, f3: String }
# let mut x = A {
#     f1: "f1".to_string(),
#     f2: "f2".to_string(),
#     f3: "f3".to_string()
# };
let a: &mut String = &mut x.f1; // x.f1 borrowed mutably
let b: &String = &x.f2;         // x.f2 borrowed immutably
let c: &String = &x.f2;         // Can borrow again
let d: String = x.f3;           // Move out of x.f3
```

### Tuple indexing expressions

[Tuples](types.html#tuple-types) and [struct tuples](items.html#structs) can be
indexed using the number corresponding to the possition of the field. The index
must be written as a [decimal literal](tokens.html#integer-literals) with no
underscores or suffix. Tuple indexing expressions also differ from field
expressions in that they can unambiguously be called as a function. In all
other aspects they have the same behavior.

```rust
# struct Point(f32, f32);
let pair = (1, 2);
assert_eq!(pair.1, 2);
let unit_x = Point(1.0, 0.0);
assert_eq!(unit_x.0, 1.0);
```

## Call expressions

A _call expression_ consists of an expression followed by a parenthesized
expression-list. It invokes a function, providing zero or more input variables.
If the function eventually returns, then the expression completes. For
[non-function types](types.html#function-types), the expression f(...) uses the
method on one of the `std::ops::Fn`, `std::ops::FnMut` or `std::ops::FnOnce`
traits, which differ in whether they take the type by reference, mutable
reference, or take ownership respectively. An automatic borrow will be taken if
needed. Rust will also automatically dereference `f` as required. Some examples
of call expressions:

```rust
# fn add(x: i32, y: i32) -> i32 { 0 }
let three: i32 = add(1i32, 2i32);
let name: &'static str = (|| "Rust")();
```

### Disambiguating Function Calls

Rust treats all function calls as sugar for a more explicit, fully-qualified
syntax. Upon compilation, Rust will desugar all function calls into the explicit
form. Rust may sometimes require you to qualify function calls with trait,
depending on the ambiguity of a call in light of in-scope items.

> **Note**: In the past, the Rust community used the terms "Unambiguous
> Function Call Syntax", "Universal Function Call Syntax", or "UFCS", in
> documentation, issues, RFCs, and other community writings. However, the term
> lacks descriptive power and potentially confuses the issue at hand. We mention
> it here for searchability's sake.

Several situations often occur which result in ambiguities about the receiver or
referent of method or associated function calls. These situations may include:

* Multiple in-scope traits define methods with the same name for the same types
* Auto-`deref` is undesirable; for example, distinguishing between methods on a
  smart pointer itself and the pointer's referent
* Methods which take no arguments, like `default()`, and return properties of a
  type, like `size_of()`

To resolve the ambiguity, the programmer may refer to their desired method or
function using more specific paths, types, or traits.

For example,

```rust
trait Pretty {
    fn print(&self);
}

trait Ugly {
  fn print(&self);
}

struct Foo;
impl Pretty for Foo {
    fn print(&self) {}
}

struct Bar;
impl Pretty for Bar {
    fn print(&self) {}
}
impl Ugly for Bar{
    fn print(&self) {}
}

fn main() {
    let f = Foo;
    let b = Bar;

    // we can do this because we only have one item called `print` for `Foo`s
    f.print();
    // more explicit, and, in the case of `Foo`, not necessary
    Foo::print(&f);
    // if you're not into the whole brevity thing
    <Foo as Pretty>::print(&f);

    // b.print(); // Error: multiple 'print' found
    // Bar::print(&b); // Still an error: multiple `print` found

    // necessary because of in-scope items defining `print`
    <Bar as Pretty>::print(&b);
}
```

Refer to [RFC 132] for further details and motivations.

[RFC 132]: https://github.com/rust-lang/rfcs/blob/master/text/0132-ufcs.md

## Closure expressions

A _closure expression_ defines a closure and denotes it as a value, in a single
expression. A closure expression is a pipe-symbol-delimited (`|`) list of
patterns followed by an expression. Type annotations may optionally be added
for the type of the parameters or for the return type. If there is a return
type, the expression used for the body of the closure must be a normal
[block](#block-expressions). A closure expression also may begin with the
`move` keyword before the initial `|`.

A closure expression denotes a function that maps a list of parameters
(`ident_list`) onto the expression that follows the `ident_list`. The patterns
in the `ident_list` are the parameters to the closure. If a parameter's types
is not specified, then the compiler infers it from context. Each closure
expression has a unique anonymous type.

Closure expressions are most useful when passing functions as arguments to other
functions, as an abbreviation for defining and capturing a separate function.

Significantly, closure expressions _capture their environment_, which regular
[function definitions](items.html#functions) do not. Without the `move`
keyword, the closure expression infers how it captures each variable from its
environment, preferring to capture by shared reference, effectively borrowing
all outer variables mentioned inside the closure's body. If needed the compiler
will infer that instead mutable references should be taken, or that the values
should be moved or copied (depending on their type) from the environment. A
closure can be forced to capture its environment by copying or moving values by
prefixing it with the `move` keyword. This is often used to ensure that the
closure's type is `'static`.

The compiler will determine which of the [closure
traits](types.html#closure-types) the closure's type will implement by how it
acts on its captured variables. The closure will also implement
[`Send`](the-send-trait.html) and/or [`Sync`](the-sync-trait.html) if all of
its captured types do. These traits allow functions to accept closures using
generics, even though the exact types can't be named.

In this example, we define a function `ten_times` that takes a higher-order
function argument, and we then call it with a closure expression as an argument,
followed by a closure expression that moves values from its environment.

```rust
fn ten_times<F>(f: F) where F: Fn(i32) {
    for index in 0..10 {
        f(index);
    }
}

ten_times(|j| println!("hello, {}", j));
// With type annotations
ten_times(|j: i32| -> () { println!("hello, {}", j) });

let word = "konnichiwa".to_owned();
ten_times(move |j| println!("{}, {}", word, j));
```

## Array expressions

An _[array](types.html#array-and-slice-types) expression_ can be written by
enclosing zero or more comma-separated expressions of uniform type in square
brackets. This produces and array containing each of these values in the
order they are written.

Alternatively there can be exactly two expressions inside the brackets,
separated by a semi-colon. The expression after the `;` must be a have type
`usize` and be a [constant expression](#constant-expressions), such as a
[literal](tokens.html#literals) or a [constant
item](items.html#constant-items). `[a; b]` creates an array containing `b`
copies of the value of `a`. If the expression after the semi-colon has a value
greater than 1 then this requires that the type of `a` is
[`Copy`](the-copy-trait.html).

```rust
[1, 2, 3, 4];
["a", "b", "c", "d"];
[0; 128];              // array with 128 zeros
[0u8, 0u8, 0u8, 0u8];
```

## Index expressions

[Array and slice](types.html#array-and-slice-types)-typed expressions can be
indexed by writing a square-bracket-enclosed expression (the index) after them.
When the array is mutable, the resulting
[lvalue](expressions.html#lvalues-and-rvalues) can be assigned to.
For other types an index expression `a[b]` is equivalent to
`*std::ops::Index::index(&a, b)`, or `*std::opsIndexMut::index_mut(&mut a, b)`
in a mutable lvalue context. Just as with methods, Rust will also insert
dereference operations on `a` repeatedly to find an implementation.

Indices are zero-based, and are of type `usize` for arrays and slices. Array
access is a [constant expression](#constant-expressions), so bounds can be
checked at compile-time for constant arrays with a constant index value.
Otherwise a check will be performed at run-time that will put the thread in a
_panicked state_ if it fails.

```rust,should_panic
([1, 2, 3, 4])[2];        // Evaluates to 3

let x = (["a", "b"])[10]; // warning: const index-expr is out of bounds

let n = 10;
let y = (["a", "b"])[n];  // panics

let arr = ["a", "b"];
arr[10];                  // panics
```

## Range expressions

The `..` operator will construct an object of one of the `std::ops::Range` (or
`core::ops::Range`) variants.

```rust
1..2;   // std::ops::Range
3..;    // std::ops::RangeFrom
..4;    // std::ops::RangeTo
..;     // std::ops::RangeFull
```

The following expressions are equivalent.

```rust
let x = std::ops::Range {start: 0, end: 10};
let y = 0..10;

assert_eq!(x, y);
```

## Operator expressions

Operators are defined for built in types by the Rust language. Many of the
following operators can also be overloaded using traits in `std::ops` or
`std::cmp`.

### Overflow

Integer operators will panic when they overflow when compiled in debug mode.
The `-C debug-assertions` and `-C overflow-checks` compiler flags can be used
to control this more directly. The following things are considered to be
overflow:

* When `+`, `*` or `-` create a value greater than the maximum value, or less
  than the minimum value that can be stored. This includes unary `-` on the
  smallest value of any signed integer type.
* Using `/` or `%`, where the left-hand argument is the smallest integer of a
  signed integer type and the right-hand argument is `-1`.
* Using `<<` or `>>` where the right-hand argument is greater than or equal to
  the number of bits in the type of the left-hand argument, or is negative.

### Borrow operators

The `&` (shared borrow) and `&mut` (mutable borrow) operators are unary prefix
operators. When applied to an lvalue produce a reference (pointer) to the
location that the value refers to. The lvalue is also placed into a borrowed
state for the duration of the reference. For a shared borrow (`&`), this
implies that the lvalue may not be mutated, but it may be read or shared again.
For a mutable borrow (`&mut`), the lvalue may not be accessed in any way until
the borrow expires. `&mut` evaluates its operand in a mutable lvalue context.
If the `&` or `&mut` operators are applied to an rvalue, a temporary value is
created; the lifetime of this temporary value is defined by [syntactic
rules](#temporary-lifetimes). These operators cannot be overloaded.

```rust
{
    // a temporary with value 7 is created that lasts for this scope.
    let shared_reference = &7;
}
let mut array = [-2, 3, 9];
{
    // Mutably borrows `array` for this scope.
    // `array` may only be used through `mutable_reference`.
    let mutable_reference = &mut array;
}
```

### The dereference operator

The `*` (dereference) operator is also a unary prefix operator. When applied to
a [pointer](types.html#pointer-types) it denotes the pointed-to location. If
the expression is of type `&mut T` and `*mut T`, and is either a local
variable, a (nested) field of a local variance or is a mutable lvalue, then the
resulting [lvalue](expressions.html#lvalues-and-rvalues) can be
assigned to. Dereferencing a raw pointer requires `unsafe`.

On non-pointer types `*x` is equivalent to `*std::ops::Deref::deref(&x)` in an
[immutable lvalue context](#mutability) and `*std::ops::Deref::deref_mut(&mut
x)` in a mutable lvalue context.

```rust
let x = &7;
assert_eq!(*x, 7);
let y = &mut 9;
*y = 11;
assert_eq!(*y, 11);
```

### The `?` operator.

The `?` ("question mark") operator can be applied to values of the `Result<T,
E>` type to propagate errors. If applied to `Err(e)` it will return
`Err(From::from(e))` from the enclosing function or closure. If applied to
`Ok(x)` it will unwrap the value to return `x`. Unlike other unary operators
`?` is written in postfix notation. `?` cannot be overloaded.

```rust
# use std::num::ParseIntError;
fn try_to_parse() -> Result<i32, ParseIntError> {
    let x: i32 = "123".parse()?; // x = 123
    let y: i32 = "24a".parse()?; // returns an Err() immediately
    Ok(x + y)                    // Doesn't run.
}

let res = try_to_parse();
println!("{:?}", res);
# assert!(res.is_err())
```

### Negation operators

These are the last two unary operators. This table summarizes the behavior of
them on primitive types and which traits are used to overload these operators
for other types. Remember that signed integers are always represented using
two's complement. The operands of all of these operators are evaluated in
rvalue context so are moved or copied.

| Symbol | Integer     | `bool`      | Floating Point | Overloading Trait  |
|--------|-------------|-------------|----------------|--------------------|
| `-`    | Negation*   |             | Negation       | `std::ops::Neg`    |
| `!`    | Bitwise NOT | Logical NOT |                | `std::ops::Not`    |

\* Only for signed integer types.

Here are some example of these operators

```rust
let x = 6;
assert_eq!(-x, -6);
assert_eq!(!x, -7);
assert_eq!(true, !false);
```

### Arithmetic and Logical Binary Operators

Binary operators expressions are all written with infix notation. This table
summarizes the behavior of arithmetic and logical binary operators on
primitive types and which traits are used to overload these operators for other
types. Remember that signed integers are always represented using two's
complement. The operands of all of these operators are evaluated in rvalue
context so are moved or copied.

| Symbol | Integer                 | `bool`      | Floating Point | Overloading Trait  |
|--------|-------------------------|-------------|----------------|--------------------|
| `+`    | Addition                |             | Addition       | `std::ops::Add`    |
| `-`    | Subtraction             |             | Subtraction    | `std::ops::Sub`    |
| `*`    | Multiplication          |             | Multiplication | `std::ops::Mul`    |
| `/`    | Division                |             | Division       | `std::ops::Div`    |
| `%`    | Remainder               |             | Remainder      | `std::ops::Rem`    |
| `&`    | Bitwise AND             | Logical AND |                | `std::ops::BitAnd` |
| <code>&#124;</code> | Bitwise OR | Logical OR  |                | `std::ops::BitOr`  |
| `^`    | Bitwise XOR             | Logical XOR |                | `std::ops::BitXor` |
| `<<`   | Left Shift              |             |                | `std::ops::Shl`    |
| `>>`   | Right Shift*            |             |                | `std::ops::Shr`    |

\* Arithmetic right shift on signed integer types, logical right shift on
unsigned integer types.

Here are examples of these operators being used.

```rust
assert_eq!(3 + 6, 9);
assert_eq!(5.5 - 1.25, 4.25);
assert_eq!(-5 * 14, -70);
assert_eq!(14 / 3, 4);
assert_eq!(100 % 7, 2);
assert_eq!(0b1010 & 0b1100, 0b1000);
assert_eq!(0b1010 | 0b1100, 0b1110);
assert_eq!(0b1010 ^ 0b1100, 0b110);
assert_eq!(13 << 3, 104);
assert_eq!(-10 >> 2, -3);
```

### Comparison Operators

Comparison operators are also defined both for primitive types and many type in
the standard library. Parentheses are required when chaining comparison
operators. For example, the expression `a == b == c` is invalid and may be
written as `(a == b) == c`.

Unlike arithmetic and logical operators, the traits for
overloading the operators the traits for these operators are used more
generally to show how a type may be compared and will likely be assumed to
define actual comparisons by functions that use these traits as bounds. Many
functions and macros in the standard library can then use that assumption
(although not to ensure safety). Unlike the arithmetic and logical operators
above, these operators implicitly take shared borrows of their operands,
evaluating them in lvalue context:

```rust,ignore
a == b;
// is equivalent to
::std::cmp::PartialEq::eq(&a, &b);
```

This means that the operands don't have to be moved out of.

| Symbol | Meaning                  | Overloading method         |
|--------|--------------------------|----------------------------|
| `==`   | Equal                    | `std::cmp::PartialEq::eq`  |
| `!=`   | Not equal                | `std::cmp::PartialEq::ne`  |
| `>`    | Greater than             | `std::cmp::PartialOrd::gt` |
| `<`    | Less than                | `std::cmp::PartialOrd::lt` |
| `>=`   | Greater than or equal to | `std::cmp::PartialOrd::ge` |
| `<=`   | Less than or equal to    | `std::cmp::PartialOrd::le` |

Here are examples of the comparison operators being used.

```rust
assert!(123 == 123);
assert!(23 != -12);
assert!(12.5 > 12.2);
assert!([1, 2, 3] < [1, 3, 4]);
assert!('A' <= 'B');
assert!("World" >= "Hello");
```

### Lazy boolean operators

The operators `||` and `&&` may be applied to operands of boolean type. The
`||` operator denotes logical 'or', and the `&&` operator denotes logical
'and'. They differ from `|` and `&` in that the right-hand operand is only
evaluated when the left-hand operand does not already determine the result of
the expression. That is, `||` only evaluates its right-hand operand when the
left-hand operand evaluates to `false`, and `&&` only when it evaluates to
`true`.

```rust
let x = false || true; // true
let y = false && panic!(); // false, doesn't evaluate `panic!()`
```

### Type cast expressions

A type cast expression is denoted with the binary operator `as`.

Executing an `as` expression casts the value on the left-hand side to the type
on the right-hand side.

An example of an `as` expression:

```rust
# fn sum(values: &[f64]) -> f64 { 0.0 }
# fn len(values: &[f64]) -> i32 { 0 }
fn average(values: &[f64]) -> f64 {
    let sum: f64 = sum(values);
    let size: f64 = len(values) as f64;
    sum / size
}
```

`as` can be used to explicitly perform [coercions](type-coercions.html), as
well as the following additional casts. Here `*T` means either `*const T` or
`*mut T`.

| Type of `e`           | `U`                   | Cast performed by `e as U`       |
|-----------------------|-----------------------|----------------------------------|
| Integer or Float type | Integer or Float type | Numeric cast                     |
| C-like enum           | Integer type          | Enum cast                        |
| `bool` or `char`      | Integer type          | Primitive to integer cast        |
| `u8`                  | `char`                | `u8` to `char` cast              |
| `*T`                  | `*V` where `V: Sized` \* | Pointer to pointer cast       |
| `*T` where `T: Sized` | Numeric type          | Pointer to address cast          |
| Integer type          | `*V` where `V: Sized` | Address to pointer cast          |
| `&[T; n]`             | `*const T`            | Array to pointer cast            |
| [Function pointer](types.html#function-types) | `*V` where `V: Sized` | Function pointer to pointer cast |
| Function pointer      | Integer               | Function pointer to address cast |

\* or `T` and `V` are compatible unsized types, e.g., both slices, both the
same trait object.

#### Semantics

* Numeric cast
    * Casting between two integers of the same size (e.g. i32 -> u32) is a no-op
    * Casting from a larger integer to a smaller integer (e.g. u32 -> u8) will
      truncate
    * Casting from a smaller integer to a larger integer (e.g. u8 -> u32) will
        * zero-extend if the source is unsigned
        * sign-extend if the source is signed
    * Casting from a float to an integer will round the float towards zero
        * **[NOTE: currently this will cause Undefined Behavior if the rounded
          value cannot be represented by the target integer type][float-int]**.
          This includes Inf and NaN. This is a bug and will be fixed.
    * Casting from an integer to float will produce the floating point
      representation of the integer, rounded if necessary (rounding strategy
      unspecified)
    * Casting from an f32 to an f64 is perfect and lossless
    * Casting from an f64 to an f32 will produce the closest possible value
      (rounding strategy unspecified)
        * **[NOTE: currently this will cause Undefined Behavior if the value
          is finite but larger or smaller than the largest or smallest finite
          value representable by f32][float-float]**. This is a bug and will
          be fixed.
* Enum cast
    * Casts an enum to its discriminant, then uses a numeric cast if needed.
* Primitive to integer cast
    * `false` casts to `0`, `true` casts to `1`
    * `char` casts to the value of the code point, then uses a numeric cast if needed.
* `u8` to `char` cast
    * Casts to the `char` with the corresponding code point.

[float-int]: https://github.com/rust-lang/rust/issues/10184
[float-float]: https://github.com/rust-lang/rust/issues/15536

### Assignment expressions

An _assignment expression_ consists of an
[lvalue](expressions.html#lvalues-and-rvalues) expression followed
by an equals sign (`=`) and an
[rvalue](expressions.html#lvalues-and-rvalues) expression.

Evaluating an assignment expression [either copies or
moves](#moved-and-copied-types) its right-hand operand to its left-hand
operand. The left-hand operand must be an lvalue: using an rvalue results in a
compiler error, rather than promoting it to a temporary.

```rust
# let mut x = 0;
# let y = 0;
x = y;
```

### Compound assignment expressions

The `+`, `-`, `*`, `/`, `%`, `&`, `|`, `^`, `<<`, and `>>` operators may be
composed with the `=` operator. The expression `lval OP= val` is equivalent to
`lval = lval OP val`. For example, `x = x + 1` may be written as `x += 1`.
Any such expression always has the [`unit`](types.html#tuple-types) type.
These operators can all be overloaded using the trait with the same name as for
the normal operation followed by 'Assign', for example, `std::ops::AddAssign`
is used to overload `+=`. As with `=`, `lval` must be an lvalue.

```rust
let mut x = 10;
x += 4;
assert_eq!(x, 14);
```

### Operator precedence

The precedence of Rust operators is ordered as follows, going from strong to
weak. Binary Operators at the same precedence level are evaluated in the order
given by their associativity.


| Operator                    | Associativity       |
|-----------------------------|---------------------|
| `?`                         |                     |
| Unary `-` `*` `!` `&` `&mut` |                    |
| `as` `:`                    | left to right       |
| `*` `/` `%`                 | left to right       |
| `+` `-`                     | left to right       |
| `<<` `>>`                   | left to right       |
| `&`                         | left to right       |
| `^`                         | left to right       |
| <code>&#124;</code>         | left to right       |
| `==` `!=` `<` `>` `<=` `>=` | Require parentheses |
| `&&`                        | left to right       |
| <code>&#124;&#124;</code>   | left to right       |
| `..` `...`                  | Require parentheses |
| `<-`                        | right to left       |
| `=` `+=` `-=` `*=` `/=` `%=` <br> `&=` <code>&#124;=</code> `^=` `<<=` `>>=` | right to left |

## Grouped expressions

An expression enclosed in parentheses evaluates to the result of the enclosed
expression. Parentheses can be used to explicitly specify evaluation order
within an expression.

An example of a parenthesized expression:

```rust
let x: i32 = 2 + 3 * 4;
let y: i32 = (2 + 3) * 4;
assert_eq!(x, 14);
assert_eq!(y, 20);
```

## Loops

Rust supports three loop expressions:

*   A [`loop` expression](#infinite-loops) denotes an infinite loop.
*   A [`while` expression](#predicate-loops) loops until a predicate is false.
*   A [`for` expression](#iterator-loops) extracts values from an iterator,
    looping until the iterator is empty.

All three types of loop support [`break` expressions](#break-expressions),
[`continue` expressions](#continue-expressions), and [labels](#loop-labels).
Only `loop` supports [evaluation to non-trivial values](#break-and-loop-values).

### Infinite loops

A `loop` expression repeats execution of its body continuously:
`loop { println!("I live."); }`.

A `loop` expression without an associated `break` expression is
[diverging](items.html#diverging-functions), and doesn't
return anything. A `loop` expression containing associated
[`break` expression(s)](#break-expressions)
may terminate, and must have type compatible with the value of the `break`
expression(s).

### Predicate loops

A `while` loop begins by evaluating the boolean loop conditional expression. If
the loop conditional expression evaluates to `true`, the loop body block
executes, then control returns to the loop conditional expression. If the loop
conditional expression evaluates to `false`, the `while` expression completes.

An example:

```rust
let mut i = 0;

while i < 10 {
    println!("hello");
    i = i + 1;
}
```

### Iterator loops

A `for` expression is a syntactic construct for looping over elements provided
by an implementation of `std::iter::IntoIterator`. If the iterator yields a
value, that value is given the specified name and the body of the loop is
executed, then control returns to the head of the `for` loop. If the iterator
is empty, the `for` expression completes.

An example of a `for` loop over the contents of an array:

```rust
let v = &["apples", "cake", "coffee"];

for text in v {
    println!("I like {}.", text);
}
```

An example of a for loop over a series of integers:

```rust
let mut sum = 0;
for n in 1..11 {
    sum += n;
}
assert_eq!(sum, 55);
```

### Loop labels

A loop expression may optionally have a _label_. The label is written as
a lifetime preceding the loop expression, as in `'foo: loop { break 'foo; }`,
`'bar: while false {}`, `'humbug: for _ in 0..0 {}`.
If a label is present, then labeled `break` and `continue` expressions nested
within this loop may exit out of this loop or return control to its head.
See [break expressions](#break-expressions) and [continue
expressions](#continue-expressions).

### `break` expressions

When `break` is encountered, execution of the associated loop body is
immediately terminated, for example:

```rust
let mut last = 0;
for x in 1..100 {
    if x > 12 {
        break;
    }
    last = x;
}
assert_eq!(last, 12);
```

A `break` expression is normally associated with the innermost `loop`, `for` or
`while` loop enclosing the `break` expression, but a [label](#loop-labels) can
be used to specify which enclosing loop is affected. Example:

```rust
'outer: loop {
    while true {
        break 'outer;
    }
}
```

A `break` expression is only permitted in the body of a loop, and has one of
the forms `break`, `break 'label` or ([see below](#break-and-loop-values))
`break EXPR` or `break 'label EXPR`.

### `continue` expressions

When `continue` is encountered, the current iteration of the associated loop
body is immediately terminated, returning control to the loop *head*. In
the case of a `while` loop, the head is the conditional expression controlling
the loop. In the case of a `for` loop, the head is the call-expression
controlling the loop.

Like `break`, `continue` is normally associated with the innermost enclosing
loop, but `continue 'label` may be used to specify the loop affected.
A `continue` expression is only permitted in the body of a loop.

### `break` and loop values

When associated with a `loop`, a break expression may be used to return a value
from that loop, via one of the forms `break EXPR` or `break 'label EXPR`, where
`EXPR` is an expression whose result is returned from the `loop`. For example:

```rust
let (mut a, mut b) = (1, 1);
let result = loop {
    if b > 10 {
        break b;
    }
    let c = a + b;
    a = b;
    b = c;
};
// first number in Fibonacci sequence over 10:
assert_eq!(result, 13);
```

In the case a `loop` has an associated `break`, it is not considered diverging,
and the `loop` must have a type compatible with each `break` expression.
`break` without an expression is considered identical to `break` with
expression `()`.

## `if` expressions

An `if` expression is a conditional branch in program control. The form of an
`if` expression is a condition expression, followed by a consequent block, any
number of `else if` conditions and blocks, and an optional trailing `else`
block. The condition expressions must have type `bool`. If a condition
expression evaluates to `true`, the consequent block is executed and any
subsequent `else if` or `else` block is skipped. If a condition expression
evaluates to `false`, the consequent block is skipped and any subsequent `else
if` condition is evaluated. If all `if` and `else if` conditions evaluate to
`false` then any `else` block is executed. An if expression evaluates to the
same value as the executed block, or `()` if no block is evaluated. An `if`
expression must have the same type in all situations.

```rust
# let x = 3;
if x == 4 {
    println!("x is four");
} else if x == 3 {
    println!("x is three");
} else {
    println!("x is something else");
}

let y = if 12 * 15 > 150 {
    "Bigger"
} else {
    "Smaller"
};
assert_eq!(y, "Bigger");
```

## `match` expressions

A `match` expression branches on a *pattern*. The exact form of matching that
occurs depends on the pattern. Patterns consist of some combination of
literals, destructured arrays or enum constructors, structs and tuples,
variable binding specifications, wildcards (`..`), and placeholders (`_`). A
`match` expression has a *head expression*, which is the value to compare to
the patterns. The type of the patterns must equal the type of the head
expression.

A `match` behaves differently depending on whether or not the head expression
is an [lvalue or an rvalue](expressions.html#lvalues-and-rvalues).
If the head expression is an rvalue, it is first evaluated into a temporary
location, and the resulting value is sequentially compared to the patterns in
the arms until a match is found. The first arm with a matching pattern is
chosen as the branch target of the `match`, any variables bound by the pattern
are assigned to local variables in the arm's block, and control enters the
block.

When the head expression is an lvalue, the match does not allocate a temporary
location (however, a by-value binding may copy or move from the lvalue). When
possible, it is preferable to match on lvalues, as the lifetime of these
matches inherits the lifetime of the lvalue, rather than being restricted to
the inside of the match.

An example of a `match` expression:

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
```

Patterns that bind variables default to binding to a copy or move of the
matched value (depending on the matched value's type). This can be changed to
bind to a reference by using the `ref` keyword, or to a mutable reference using
`ref mut`.

Patterns can be used to *destructure* structs, enums, and tuples. Destructuring
breaks a value up into its component pieces. The syntax used is the same as
when creating such values. When destructing a data structure with named (but
not numbered) fields, it is allowed to write `fieldname` as a shorthand for
`fieldname: fieldname`. In a pattern whose head expression has a `struct`,
`enum` or `tupl` type, a placeholder (`_`) stands for a *single* data field,
whereas a wildcard `..` stands for *all* the fields of a particular variant.

```rust
# enum Message {
#     Quit,
#     WriteString(String),
#     Move { x: i32, y: i32 },
#     ChangeColor(u8, u8, u8),
# }
# let message = Message::Quit;
match message {
    Message::Quit => println!("Quit"),
    Message::WriteString(write) => println!("{}", &write),
    Message::Move{ x, y: 0 } => println!("move {} horizontally", x),
    Message::Move{ .. } => println!("other move"),
    Message::ChangeColor { 0: red, 1: green, 2: _ } => {
        println!("color change, red: {}, green: {}", red, green);
    }
};
```

Patterns can also dereference pointers by using the `&`, `&mut` and `box`
symbols, as appropriate. For example, these two matches on `x: &i32` are
equivalent:

```rust
# let x = &3;
let y = match *x { 0 => "zero", _ => "some" };
let z = match x { &0 => "zero", _ => "some" };

assert_eq!(y, z);
```

Subpatterns can also be bound to variables by the use of the syntax `variable @
subpattern`. For example:

```rust
let x = 1;

match x {
    e @ 1 ... 5 => println!("got a range element {}", e),
    _ => println!("anything"),
}
```

Multiple match patterns may be joined with the `|` operator. A range of values
may be specified with `...`. For example:

```rust
# let x = 2;
let message = match x {
    0 | 1  => "not many",
    2 ... 9 => "a few",
    _      => "lots"
};
```

Range patterns only work on scalar types (like integers and characters; not
like arrays and structs, which have sub-components). A range pattern may not be
a sub-range of another range pattern inside the same `match`.

Finally, match patterns can accept *pattern guards* to further refine the
criteria for matching a case. Pattern guards appear after the pattern and
consist of a bool-typed expression following the `if` keyword. A pattern guard
may refer to the variables bound within the pattern they follow.

```rust
# let maybe_digit = Some(0);
# fn process_digit(i: i32) { }
# fn process_other(i: i32) { }
let message = match maybe_digit {
    Some(x) if x < 10 => process_digit(x),
    Some(x) => process_other(x),
    None => panic!(),
};
```

## `if let` expressions

An `if let` expression is semantically similar to an `if` expression but in
place of a condition expression it expects the keyword `let` followed by a
refutable pattern, an `=` and an expression. If the value of the expression on
the right hand side of the `=` matches the pattern, the corresponding block
will execute, otherwise flow proceeds to the following `else` block if it
exists. Like `if` expressions, `if let` expressions have a value determined by
the block that is evaluated.

```rust
let dish = ("Ham", "Eggs");

// this body will be skipped because the pattern is refuted
if let ("Bacon", b) = dish {
    println!("Bacon is served with {}", b);
} else {
    // This block is evaluated instead.
    println!("No bacon will be served");
}

// this body will execute
if let ("Ham", b) = dish {
    println!("Ham is served with {}", b);
}
```

## `while let` loops

A `while let` loop is semantically similar to a `while` loop but in place of a
condition expression it expects the keyword `let` followed by a refutable
pattern, an `=` and an expression. If the value of the expression on the right
hand side of the `=` matches the pattern, the loop body block executes then
control returns to the pattern matching statement. Otherwise, the while
expression completes.

```rust
let mut x = vec![1, 2, 3];

while let Some(y) = x.pop() {
    println!("y = {}", y);
}
```

## `return` expressions

Return expressions are denoted with the keyword `return`. Evaluating a `return`
expression moves its argument into the designated output location for the
current function call, destroys the current function activation frame, and
transfers control to the caller frame.

An example of a `return` expression:

```rust
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    return b;
}
```
