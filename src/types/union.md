r[type.union]
# Union types

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

[`Copy`]: ../special-types-and-traits.md#copy
[item]: ../items/unions.md
