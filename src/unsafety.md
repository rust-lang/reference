# Unsafety

Unsafe operations are those that potentially violate the memory-safety
guarantees of Rust's static semantics.

The following language level features cannot be used in the safe subset of
Rust:

- Dereferencing a [raw pointer](types.html#pointer-types).
- Reading or writing a [mutable static variable](items.html#mutable-statics).
- Reading a field of a [`union`](items.html#unions), or writing to a field of a
  union that isn't [`Copy`](the-copy-trait.html).
- Calling an unsafe function (including an intrinsic or foreign function).
- Implementing an unsafe trait.
