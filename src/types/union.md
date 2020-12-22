# Union types

A *union type* is a nominal, heterogeneous C-like union, denoted by the name of
a [`union` item][item].

Unions have no notion of an "active field". Instead, every union access
transmutes parts of the content of the union to the type of the accessed
field. Since transmutes can cause unexpected or undefined behaviour, `unsafe`
is required to read from a union field, or to write to a field that doesn't
implement [`Copy`] or has a [`ManuallyDrop`] type. See the [item] documentation
for further details.

The memory layout of a `union` is undefined by default, but the `#[repr(...)]`
attribute can be used to fix a layout.

[`Copy`]: ../special-types-and-traits.md#copy
[`ManuallyDrop`]: ../../std/mem/struct.ManuallyDrop.html
[item]: ../items/unions.md
