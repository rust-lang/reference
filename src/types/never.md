# Never type

> **<sup>Syntax</sup>**\
> _NeverType_ : `!`

The never type `!` is a type with no values, representing the result of
computations that never complete. Expressions of type `!` can be coerced into
any other type.

<!-- ignore: unstable -->
```rust,ignore
let x: ! = panic!();
// Can be coerced into any type.
let y: u32 = x;
```

**NB.** The never type was expected to be stabilized in 1.41, but due
to some last minute regressions detected the stabilization was
temporarily reverted. The `!` type can only appear in function return
types presently. See [the tracking
issue](https://github.com/rust-lang/rust/issues/35121) for more
details.
