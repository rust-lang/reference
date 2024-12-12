# Array types

r[type.array]

r[type.array.syntax]
> **<sup>Syntax</sup>**\
> _ArrayType_ :\
> &nbsp;&nbsp; `[` [_Type_] `;` [_Expression_] `]`

r[type.array.intro]
An array is a fixed-size sequence of `N` elements of type `T`. The array type
is written as `[T; N]`.

r[type.array.constraint]
The size is a [constant expression] that evaluates to a [`usize`].

Examples:

```rust
// A stack-allocated array
let array: [i32; 3] = [1, 2, 3];

// A heap-allocated array, coerced to a slice
let boxed_array: Box<[i32]> = Box::new([1, 2, 3]);
```

r[type.array.index]
All elements of arrays are always initialized, and access to an array is
always bounds-checked in safe methods and operators.

> Note: The [`Vec<T>`] standard library type provides a heap-allocated resizable
> array type.

[_Expression_]: ../expressions.md
[_Type_]: ../types.md#type-expressions
[`usize`]: numeric.md#machine-dependent-integer-types
[constant expression]: ../const_eval.md#constant-expressions
