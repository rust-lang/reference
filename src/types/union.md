r[type.union]
# Union types

r[type.union.intro]
A *union type* is a nominal, heterogeneous C-like union, denoted by the name of a [`union` item][item].

r[type.union.access]
Unions have no notion of an "active field". Instead, every union access transmutes parts of the content of the union to the type of the accessed field.

r[type.union.safety]
Since transmutes can cause unexpected or undefined behaviour, `unsafe` is required to read from a union field.

r[type.union.constraint]
Union field types are also restricted to a subset of types which ensures that they never need dropping. See the [item] documentation for further details.

r[type.union.layout]
The memory layout of a `union` is undefined by default (in particular, fields do *not* have to be at offset 0), but the `#[repr(...)]` attribute can be used to fix a layout.

r[type.union.safe-construction]
Although unions have no validity requirements, the values of union types that can be created exclusively in safe contexts are limited. Specifically, safe contexts cannot create values of union types that have uninitialized bytes at an offset where none of the fields permit uninitialized bytes.

[`Copy`]: ../special-types-and-traits.md#copy
[item]: ../items/unions.md
