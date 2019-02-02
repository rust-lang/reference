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

[`Copy`]: special-types-and-traits.md#copy
[`union`]: items/unions.md
[mutable static variable]: items/static-items.md#mutable-statics
[external static variable]: items/external-blocks.md
[raw pointer]: types/pointer.md
[unsafe trait]: items/traits.md#unsafe-traits
