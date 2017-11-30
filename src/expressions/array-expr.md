# Array and array index expressions

## Array expressions

> **<sup>Syntax</sup>**  
> _ArrayExpression_ :  
> &nbsp;&nbsp; &nbsp;&nbsp; `[`  `]`  
> &nbsp;&nbsp; | `[` [_Expression_] ( `,` [_Expression_] )<sup>\*</sup> `,`<sup>?</sup> `]`  
> &nbsp;&nbsp; | `[` [_Expression_] `;` [_Expression_] `]`  

An _[array](types.html#array-and-slice-types) expression_ can be written by
enclosing zero or more comma-separated expressions of uniform type in square
brackets. This produces and array containing each of these values in the
order they are written.

Alternatively there can be exactly two expressions inside the brackets,
separated by a semi-colon. The expression after the `;` must be a have type
`usize` and be a [constant expression](expressions.html#constant-expressions),
such as a [literal](tokens.html#literals) or a [constant
item](items/constant-items.html). `[a; b]` creates an array containing `b`
copies of the value of `a`. If the expression after the semi-colon has a value
greater than 1 then this requires that the type of `a` is
[`Copy`](the-copy-trait.html).

```rust
[1, 2, 3, 4];
["a", "b", "c", "d"];
[0; 128];              // array with 128 zeros
[0u8, 0u8, 0u8, 0u8,];
[[1, 0, 0], [0, 1, 0], [0, 0, 1]]; // 2D array
```

## Array and slice indexing expressions

> **<sup>Syntax</sup>**  
> _IndexExpression_ :  
> &nbsp;&nbsp; [_Expression_] `[` [_Expression_] `]`

[Array and slice](types.html#array-and-slice-types)-typed expressions can be
indexed by writing a square-bracket-enclosed expression (the index) after them.
When the array is mutable, the resulting
[lvalue](expressions.html#lvalues-and-rvalues) can be assigned to.
For other types an index expression `a[b]` is equivalent to
`*std::ops::Index::index(&a, b)`, or `*std::opsIndexMut::index_mut(&mut a, b)`
in a mutable lvalue context. Just as with methods, Rust will also insert
dereference operations on `a` repeatedly to find an implementation.

Indices are zero-based, and are of type `usize` for arrays and slices. Array
access is a [constant expression](expressions.html#constant-expressions), so bounds can be
checked at compile-time for constant arrays with a constant index value.
Otherwise a check will be performed at run-time that will put the thread in a
_panicked state_ if it fails.

```rust,should_panic
([1, 2, 3, 4])[2];        // Evaluates to 3

let b = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
b[1][2];                  // multidimensional array indexing

let x = (["a", "b"])[10]; // warning: const index-expr is out of bounds

let n = 10;
let y = (["a", "b"])[n];  // panics

let arr = ["a", "b"];
arr[10];                  // panics
```

The array index expression can be implemented for types other than arrays and slices
by implementing the [Index] and [IndexMut] traits.

[Index]: https://doc.rust-lang.org/std/ops/trait.Index.html
[IndexMut]: https://doc.rust-lang.org/std/ops/trait.IndexMut.html
[_Expression_]: expressions.html
