# Struct types

r[type.struct]

r[type.struct.intro]
A `struct` *type* is a heterogeneous product of other types, called the
*fields* of the type.[^structtype]

r[type.struct.constructor]
New instances of a `struct` can be constructed with a [struct expression].

r[type.struct.layout]
The memory layout of a `struct` is undefined by default to allow for compiler
optimizations like field reordering, but it can be fixed with the
[`repr` attribute]. In either case, fields may be given in any order in a
corresponding struct *expression*; the resulting `struct` value will always
have the same memory layout.

r[type.struct.field-visibility]
The fields of a `struct` may be qualified by [visibility modifiers], to allow
access to data in a struct outside a module.

r[type.struct.tuple]
A _tuple struct_ type is just like a struct type, except that the fields are
anonymous.

r[type.struct.unit]
A _unit-like struct_ type is like a struct type, except that it has no fields.
The one value constructed by the associated [struct expression] is the only
value that inhabits such a type.

## Struct

r[type.struct.value]

r[type.struct.value.intro]
A value of a struct type consists of a list of values for each field.


r[type.struct.value.encode-decode]
When a value of a struct type is encoded, each field of the struct is encoded at its corresponding offset and each byte that is not within a field of the struct is set to uninit.
When a value of a struct type is decoded, each field of the struct is decoded from its corresponding offset.

> [!NOTE]
> It is valid for padding bytes to hold a value other than uninit when decoded, and these bytes are ignored when decoding an struct value.

[^structtype]: `struct` types are analogous to `struct` types in C, the
    *record* types of the ML family, or the *struct* types of the Lisp family.

[`repr` attribute]: ../type-layout.md#representations
[struct expression]: ../expressions/struct-expr.md
[visibility modifiers]: ../visibility-and-privacy.md
