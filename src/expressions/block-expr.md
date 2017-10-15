# Block expressions

> **<sup>Syntax</sup>**  
> _BlockExpression_ :  
> &nbsp;&nbsp; `{`  
> &nbsp;&nbsp; &nbsp;&nbsp; [_InnerAttribute_]<sup>\*</sup>  
> &nbsp;&nbsp; &nbsp;&nbsp; [_Statement_]<sup>\*</sup>  
> &nbsp;&nbsp; &nbsp;&nbsp; [_Expression_]<sup>?</sup>  
> &nbsp;&nbsp; `}`  

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

> **<sup>Syntax</sup>**  
> _UnsafeBlockExpression_ :  
> &nbsp;&nbsp; `unsafe` _BlockExpression_

_See [`unsafe` block](unsafe-blocks.html) for more information on when to use `unsafe`_

A block of code can be prefixed with the `unsafe` keyword, to permit calling
`unsafe` functions or dereferencing raw pointers within a safe function. Examples:

```rust
unsafe {
    let b = [13u8, 17u8];
    let a = &b[0] as *const u8;
    assert_eq!(*a, 13);
    assert_eq!(*a.offset(1), 17);
}

# unsafe fn f() -> i32 { 10 }
let a = unsafe { f() };
```

[_InnerAttribute_]: attributes.html
[_Statement_]: statements.html
[_Expression_]: expressions.html
