# Range expressions

> **<sup>Syntax</sup>**  
> _RangeExpression_ :  
> &nbsp;&nbsp; &nbsp;&nbsp; _RangeExpr_  
> &nbsp;&nbsp; | _RangeFromExpr_  
> &nbsp;&nbsp; | _RangeToExpr_  
> &nbsp;&nbsp; | _RangeFullExpr_  
>  
> _RangeExpr_ :  
> &nbsp;&nbsp; [_Expression_] `..` [_Expression_]  
>  
> _RangeFromExpr_ :  
> &nbsp;&nbsp; [_Expression_] `..`  
>  
> _RangeToExpr_ :  
> &nbsp;&nbsp; `..` [_Expression_]  
>  
> _RangeFullExpr_ :  
> &nbsp;&nbsp; `..`  

The `..` operator will construct an object of one of the `std::ops::Range` (or
`core::ops::Range`) variants, according to the following table:

| Production             | Syntax        | Type                         | Range                 |
|------------------------|---------------|------------------------------|-----------------------|
| _RangeExpr_            | start`..`end  | [std::ops::Range]            | start &le; x &lt; end |
| _RangeFromExpr_        | start`..`     | [std::ops::RangeFrom]        | start &le; x          |
| _RangeToExpr_          | `..`end       | [std::ops::RangeTo]          |            x &lt; end |
| _RangeFullExpr_        | `..`          | [std::ops::RangeFull]        |            -          |

Examples:

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

Ranges can be used in `for` loops:

```rust
for i in 1..11 {
    println!("{}", i);
}
```

[_Expression_]: expressions.html

[std::ops::Range]:     https://doc.rust-lang.org/std/ops/struct.Range.html
[std::ops::RangeFrom]: https://doc.rust-lang.org/std/ops/struct.RangeFrom.html
[std::ops::RangeTo]:   https://doc.rust-lang.org/std/ops/struct.RangeTo.html
[std::ops::RangeFull]: https://doc.rust-lang.org/std/ops/struct.RangeFull.html
