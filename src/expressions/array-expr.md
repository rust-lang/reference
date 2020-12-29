# Array and array index expressions

## Array expressions

> **<sup>Syntax</sup>**\
> _ArrayExpression_ :\
> &nbsp;&nbsp; `[` [_InnerAttribute_]<sup>\*</sup> _ArrayElements_<sup>?</sup> `]`
>
> _ArrayElements_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; [_Expression_] ( `,` [_Expression_] )<sup>\*</sup> `,`<sup>?</sup>\
> &nbsp;&nbsp; | [_Expression_] `;` [_Expression_]

An _[array] expression_ can be written by enclosing zero or more
comma-separated expressions of uniform type in square brackets. This produces
an array containing each of these values in the order they are written.

Alternatively there can be exactly two expressions inside the brackets,
separated by a semicolon. The expression after the `;` must have type `usize`
and be a [constant expression], such as a [literal] or a [constant item]. `[a;
b]` creates an array containing `b` copies of the value of `a`. If the
expression after the semicolon has a value greater than 1 then this requires
that the type of `a` is [`Copy`], or `a` must be a path to a constant item.

When the repeat expression `a` is a constant item, it is evaluated `b` times.
If `b` is 0, the constant item is not evaluated at all. For expressions that
are not a constant item, it is evaluated exactly once, and then the result is
copied `b` times.

<div class="warning">

Warning: In the case where `b` is 0, and `a` is a non-constant item, there is
currently a bug in `rustc` where the value `a` is evaluated but not dropped,
thus causing a leak. See [issue
#74836](https://github.com/rust-lang/rust/issues/74836).

</div>

```rust
[1, 2, 3, 4];
["a", "b", "c", "d"];
[0; 128];              // array with 128 zeros
[0u8, 0u8, 0u8, 0u8,];
[[1, 0, 0], [0, 1, 0], [0, 0, 1]]; // 2D array
const EMPTY: Vec<i32> = Vec::new();
[EMPTY; 2];
```

### Array expression attributes

[Inner attributes] are allowed directly after the opening bracket of an array
expression in the same expression contexts as [attributes on block
expressions].

## Array and slice indexing expressions

> **<sup>Syntax</sup>**\
> _IndexExpression_ :\
> &nbsp;&nbsp; [_Expression_] `[` [_Expression_] `]`

[Array] and [slice]-typed expressions can be indexed by writing a
square-bracket-enclosed expression of type `usize` (the index) after them.
When the array is mutable, the resulting [memory location] can be assigned to.

For other types an index expression `a[b]` is equivalent to
`*std::ops::Index::index(&a, b)`, or
`*std::ops::IndexMut::index_mut(&mut a, b)` in a mutable place expression
context. Just as with methods, Rust will also insert dereference operations on
`a` repeatedly to find an implementation.

Indices are zero-based for arrays and slices. Array access is a [constant
expression], so bounds can be checked at compile-time with a constant index
value. Otherwise a check will be performed at run-time that will put the thread
in a _panicked state_ if it fails.

```rust,should_panic
// lint is deny by default.
#![warn(unconditional_panic)]

([1, 2, 3, 4])[2];        // Evaluates to 3

let b = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
b[1][2];                  // multidimensional array indexing

let x = (["a", "b"])[10]; // warning: index out of bounds

let n = 10;
let y = (["a", "b"])[n];  // panics

let arr = ["a", "b"];
arr[10];                  // warning: index out of bounds
```

The array index expression can be implemented for types other than arrays and slices
by implementing the [Index] and [IndexMut] traits.

[`Copy`]: ../special-types-and-traits.md#copy
[IndexMut]: ../../std/ops/trait.IndexMut.html
[Index]: ../../std/ops/trait.Index.html
[Inner attributes]: ../attributes.md
[_Expression_]: ../expressions.md
[_InnerAttribute_]: ../attributes.md
[array]: ../types/array.md
[attributes on block expressions]: block-expr.md#attributes-on-block-expressions
[constant expression]: ../const_eval.md#constant-expressions
[constant item]: ../items/constant-items.md
[literal]: ../tokens.md#literals
[memory location]: ../expressions.md#place-expressions-and-value-expressions
[slice]: ../types/slice.md
