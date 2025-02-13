r[safety]
# Unsafety

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

r[safety.unsafe-target-feature-call]
- Calling a safe function marked with a [`target_feature`][attributes.codegen.target_feature] from a function that does not have a `target_feature` attribute enabling the same features (see [attributes.codegen.target_feature.safety-restrictions]).

r[safety.unsafe-impl]
- Implementing an [unsafe trait].

r[safety.unsafe-extern]
- Declaring an [`extern`] block[^extern-2024].

r[safety.unsafe-attribute]
- Applying an [unsafe attribute] to an item.

[^extern-2024]: Prior to the 2024 edition, extern blocks were allowed to be declared without `unsafe`.

[`extern`]: items/external-blocks.md
[`union`]: items/unions.md
[mutable]: items/static-items.md#mutable-statics
[external]: items/external-blocks.md
[raw pointer]: types/pointer.md
[unsafe trait]: items/traits.md#unsafe-traits
[unsafe attribute]: attributes.md
