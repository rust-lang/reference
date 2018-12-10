# Union types

A *union type* is a nominal, heterogeneous C-like union, denoted by the name of
a [`union` item].

A union access transmutes the content of the union to the type of the accessed
field. Since transmutes can cause unexpected or undefined behaviour, `unsafe` is
required to read from a union field or to write to a field that doesn't
implement [`Copy`].

The memory layout of a `union` is undefined by default, but the `#[repr(...)]`
attribute can be used to fix a layout.

[`Copy`]: special-types-and-traits.html#copy
[`union` item]: items/unions.html
