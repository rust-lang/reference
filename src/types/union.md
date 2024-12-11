# Union types

r[type.union]

r[type.union.intro]
A *union type* is a nominal, heterogeneous C-like union, denoted by the name of
a [`union` item][item].

r[type.union.access]
Unions have no notion of an "active field". Instead, every union access
transmutes parts of the content of the union to the type of the accessed field.

r[type.union.safety]
Since transmutes can cause unexpected or undefined behaviour, `unsafe` is
required to read from a union field.

r[type.union.constraint]
Union field types are also restricted to a
subset of types which ensures that they never need dropping. See the [item]
documentation for further details.

r[type.union.layout]
The memory layout of a `union` is undefined by default (in particular, fields do
*not* have to be at offset 0), but the `#[repr(...)]` attribute can be used to
fix a layout.

r[type.union.value]
A value of a union type consists of a sequence of bytes, corresponding to each [value byte][type.struct.value.value-bytes]. The value bytes of a union are represented exactly. Each [padding byte][type.struct.value.padding] is set to uninit when encoded.

> [!NOTE]
> A given value byte is guaranteed allowed to be uninit if it is padding in any field, recursively expanding union fields. Whether a byte of a union is allowed to be uninit in any other case is not yet decided.

r[type.union.constructor]
The constructor of a union type encodes the initialized field value into the corresponding bytes of the union, and sets all bytes that are not used by the field to uninit.

r[type.union.field-access]
When a field is written to by a field access expression, the value written is encoded into the corresponding bytes of the union. When a field is read, the value of that field is decoded from the corresponding bytes.

[`Copy`]: ../special-types-and-traits.md#copy
[item]: ../items/unions.md
