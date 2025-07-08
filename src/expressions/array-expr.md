r[expr.array]
# Array and array index expressions

## Array expressions

r[expr.array.syntax]
```grammar,expressions
ArrayExpression -> `[` ArrayElements? `]`

ArrayElements ->
      Expression ( `,` Expression )* `,`?
    | Expression `;` Expression
```

r[expr.array.constructor]
*Array expressions* construct [arrays][array].
Array expressions come in two forms.

r[expr.array.array]
The first form lists out every value in the array.

r[expr.array.array-syntax]
The syntax for this form is a comma-separated list of expressions of uniform type enclosed in square brackets.

r[expr.array.array-behavior]
This produces an array containing each of these values in the order they are written.

r[expr.array.repeat]
The syntax for the second form is two expressions separated by a semicolon (`;`) enclosed in square brackets.

r[expr.array.repeat-operand]
The expression before the `;` is called the *repeat operand*.

r[expr.array.length-operand]
The expression after the `;` is called the *length operand*.

r[expr.array.length-restriction]
The length operand must either be an [inferred const] or be a [constant expression] of type `usize` (e.g. a [literal] or a [constant item]).

```rust
const C: usize = 1;
let _: [u8; C] = [0; 1]; // Literal.
let _: [u8; C] = [0; C]; // Constant item.
let _: [u8; C] = [0; _]; // Inferred const.
let _: [u8; C] = [0; (((_)))]; // Inferred const.
```

> [!NOTE]
> In an array expression, an [inferred const] is parsed as an [expression][Expression] but then semantically treated as a separate kind of [const generic argument].

r[expr.array.repeat-behavior]
An array expression of this form creates an array with the length of the value of the length operand with each element being a copy of the repeat operand.
That is, `[a; b]` creates an array containing `b` copies of the value of `a`.

r[expr.array.repeat-copy]
If the length operand has a value greater than 1 then this requires the repeat operand to have a type that implements [`Copy`], to be a [const block expression], or to be a [path] to a constant item.

r[expr.array.repeat-const-item]
When the repeat operand is a const block or a path to a constant item, it is evaluated the number of times specified in the length operand.

r[expr.array.repeat-evaluation-zero]
If that value is `0`, then the const block or constant item is not evaluated at all.

r[expr.array.repeat-non-const]
For expressions that are neither a const block nor a path to a constant item, it is evaluated exactly once, and then the result is copied the length operand's value times.

```rust
[1, 2, 3, 4];
["a", "b", "c", "d"];
[0; 128];              // array with 128 zeros
[0u8, 0u8, 0u8, 0u8,];
[[1, 0, 0], [0, 1, 0], [0, 0, 1]]; // 2D array
const EMPTY: Vec<i32> = Vec::new();
[EMPTY; 2];
```

r[expr.array.index]
## Array and slice indexing expressions

r[expr.array.index.syntax]
```grammar,expressions
IndexExpression -> Expression `[` Expression `]`
```

r[expr.array.index.array]
[Array] and [slice]-typed values can be indexed by writing a square-bracket-enclosed expression of type `usize` (the index) after them.
When the array is mutable, the resulting [memory location] can be assigned to.

r[expr.array.index.trait]
For other types an index expression `a[b]` is equivalent to `*std::ops::Index::index(&a, b)`, or `*std::ops::IndexMut::index_mut(&mut a, b)` in a mutable place expression context.
Just as with methods, Rust will also insert dereference operations on `a` repeatedly to find an implementation.

r[expr.array.index.zero-index]
Indices are zero-based for arrays and slices.

r[expr.array.index.const]
Array access is a [constant expression], so bounds can be checked at compile-time with a constant index value.
Otherwise a check will be performed at run-time that will put the thread in a [_panicked state_][panic] if it fails.

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

r[expr.array.index.trait-impl]
The array index expression can be implemented for types other than arrays and slices by implementing the [Index] and [IndexMut] traits.

[`Copy`]: ../special-types-and-traits.md#copy
[IndexMut]: std::ops::IndexMut
[Index]: std::ops::Index
[array]: ../types/array.md
[const generic argument]: items.generics.const.argument
[const block expression]: expr.block.const
[constant expression]: ../const_eval.md#constant-expressions
[constant item]: ../items/constant-items.md
[inferred const]: items.generics.const.inferred
[literal]: ../tokens.md#literals
[memory location]: ../expressions.md#place-expressions-and-value-expressions
[panic]: ../panic.md
[path]: path-expr.md
[slice]: ../types/slice.md
