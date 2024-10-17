# Unsafety

r[safety]

r[safety.intro]
Unsafe operations are those that can potentially violate the memory-safety
guarantees of Rust's static semantics.

r[safety.unsafe-ops]
The following language level features cannot be used in the safe subset of
Rust:

r[safety.unsafe-deref]
- Dereferencing a [raw pointer].

r[safety.unsafe-static]
- Reading or writing a [mutable] or [external] static variable.

r[safety.unsafe-union-access]
- Accessing a field of a [`union`], other than to assign to it.

r[safety.unsafe-call]
- Calling an unsafe function (including an intrinsic or foreign function).

r[safety.unsafe-impl]
- Implementing an [unsafe trait].

r[safety.unsafe-extern]
- Declaring an [`extern`] block.

r[safety.unsafe-attribute]
- Applying an [unsafe attribute] to an item.

[`extern`]: items/external-blocks.md
[`union`]: items/unions.md
[mutable]: items/static-items.md#mutable-statics
[external]: items/external-blocks.md
[raw pointer]: types/pointer.md
[unsafe trait]: items/traits.md#unsafe-traits
[unsafe attribute]: attributes.md
