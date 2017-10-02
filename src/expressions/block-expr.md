# Block expressions

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

## `unsafe` blocks

_See [`unsafe` block](unsafe-blocks.html) for more information on when to use `unsafe`_

A block of code can be prefixed with the `unsafe` keyword, to permit calling
`unsafe` functions or dereferencing raw pointers within a safe function.
