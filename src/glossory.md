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

### Statement

A statement is the smallest standalone element of a programming language
that commands a computer to perform an action.