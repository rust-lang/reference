# Glossary

### Abstract Syntax Tree

‘Abstract syntax tree’, or ‘AST’. This tree is an intermediate representation of
the structure of your program when the compiler is compiling your program.

For example, `2 + 3` can be turned into a tree:

```text
  +
 / \
2   3
```

And `2 + (3 * 4)` would look like this:

```text
  +
 / \
2   *
   / \
  3   4
```

### Arity

Arity refers to the number of arguments a function or operation takes.

```rust
let x = (2, 3);
let y = (4, 6);
let z = (8, 2, 6);
```

In the example above `x` and `y` have arity 2. `z` has arity 3.

### Bound

Bounds are constraints on a type or trait. For example, if a bound
is placed on the argument a function takes, types passed to that function
must abide by that constraint.

### Combinator

Combinators are higher-order functions that apply only functions and
earlier defined combinators to provide a result from its arguments. 
They can be used to manage control flow in a modular fashion.

### Dynamically Sized Type

A dynamically sized type (DST) is a type without a statically known size or alignment. 

### Expression

An expression is a combination of values, constants, variables, operators 
and functions that evaluate to a single value, with or without side-effects.

For example, `2 + (3 * 4)` is an expression that returns the value 14.

### Prelude

Prelude, or The Rust Prelude, is a small collection of items - mostly traits - that are
imported into very module of every crate. The traits in the prelude are pervasive.

### Slice

A slice is dynamically-sized view into a contiguous sequence, written as `[T]`. 

It is often seen in its borrowed forms, either mutable or shared. The shared 
slice type is `&[T]`, while the mutable slice type is `&mut [T]`, where `T` represents
the element type.

### Statement

A statement is the smallest standalone element of a programming language
that commands a computer to perform an action.

### String literal

A string literal is a string stored directly in the final binary, and so will be 
valid for the `'static` duration. 

Its type is `'static` duration borrowed string slice, `&'static str`.

### String slice

A string slice is the most primitive string type in Rust, written as `str`. It is
often seen in its borrowed forms, either mutable or shared. The shared
string slice type is `&str`, while the mutable string slice type is `&mut str`.

Strings slices are always valid UTF-8.