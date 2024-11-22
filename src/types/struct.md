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

## Struct and aggregate values

r[type.struct.value]

r[type.struct.value.value-bytes]
A byte `b` in the representation of an aggregate is a value byte if there exists a field of that aggregate such that:
* The field has some type `T`,
* The offset of that field `o` is such that `b` falls at an offset in `o..(o+size_of::<T>())`,
* Either `T` is a primitive type or the offset of `b` within the field is a value byte in the representation of `T`.

> [!NOTE]
> A byte in a union is a value byte if it is a value byte in *any* field.

r[type.struct.value.padding]
Every byte in an aggregate which is not a value byte is a padding byte.

r[type.struct.value.struct]
A value of a struct type consists of the values of each of its fields.
The representation of such a struct contains the representation of the value of each field at its corresponding offset.

r[type.struct.value.padding-uninit]
When a value of an aggregate is encoded, each padding byte is left as uninit

> [!NOTE]
> It is valid for padding bytes to hold a value other than uninit when decoded, and these bytes are ignored when decoding an aggregate.

[^structtype]: `struct` types are analogous to `struct` types in C, the
    *record* types of the ML family, or the *struct* types of the Lisp family.

[`repr` attribute]: ../type-layout.md#representations
[struct expression]: ../expressions/struct-expr.md
[visibility modifiers]: ../visibility-and-privacy.md
