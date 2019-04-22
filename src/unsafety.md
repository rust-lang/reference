# Unsafety

Unsafe operations are those that can potentially violate the memory-safety
guarantees of Rust's static semantics.

The following language level features cannot be used in the safe subset of
Rust:

- Dereferencing a [raw pointer].
- Reading or writing a [mutable] or [external] static variable.
- Accessing a field of a [`union`], other than to assign to it.
- Calling an unsafe function (including an intrinsic or foreign function).
- Implementing an [unsafe trait].

[`union`]: items/unions.html
[mutable]: items/static-items.html#mutable-statics
[external]: items/external-blocks.html
[raw pointer]: types/pointer.html
[unsafe trait]: items/traits.html#unsafe-traits
