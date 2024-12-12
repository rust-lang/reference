# Enumerated types

r[type.enum]

r[type.enum.intro]
An *enumerated type* is a nominal, heterogeneous disjoint union type, denoted
by the name of an [`enum` item]. [^enumtype]

r[type.enum.declaration]
An [`enum` item] declares both the type and a number of *variants*, each of
which is independently named and has the syntax of a struct, tuple struct or
unit-like struct.

r[type.enum.constructor]
New instances of an `enum` can be constructed with a [struct expression].

r[type.enum.name]
Enum types cannot be denoted *structurally* as types, but must be denoted by
named reference to an [`enum` item].

## Enum values and representation

r[type.enum.value]

r[type.enum.value.intro]
An enum value corresponds to exactly one variant of the enum, and consists of the fields of that variant

> [!NOTE]
> An enum with no variants therefore has no values.

r[type.enum.value.value-padding]
A byte is a [padding][type.union.value.padding] byte of an enum if that byte is not part of the representation of the discriminant of the enum, and in each variant it either:
* Does not overlap with a field of the variant, or
* Overlaps with a padding byte in a field of that variant.

r[type.enum.value.repr]
The representation of a value of an enum type includes the representation of each field of the variant at the appropriate offsets. When encoding a value of an enum type, each byte which is not use d to store a field of the variant or the discriminant is . In the case of a [`repr(C)`][layout.repr.c.adt] or a [primitive-repr][layout.repr.primitive.adt] enum, the discriminant of the variant is represented as though by the appropriate integer type stored at offset 0.

> [!NOTE]
> Most `repr(Rust)` enums will also store a discriminant in the representation of the enum, but the exact placement or type of the discriminant is unspecified, as is the value that represents each variant.

[^enumtype]: The `enum` type is analogous to a `data` constructor declaration in
             Haskell, or a *pick ADT* in Limbo.

[`enum` item]: ../items/enumerations.md
[struct expression]: ../expressions/struct-expr.md
