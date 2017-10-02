# Range expressions

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
