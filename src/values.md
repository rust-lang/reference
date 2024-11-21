# Values and representation

r[value]

## Aggregate Values

r[value.aggregate]

r[value.aggregate.value-bytes]
A byte `b` in the representation of an aggregate is a value byte if there exists a field of that aggregate such that:
* The field has some type `T`,
* The offset of that field `o` is such that `b` falls at an offset in `o..(o+size_of::<T>())`,
* Either `T` is a primitive type or the offset of `b` within the field is a value byte in the representation of `T`.

> [!NOTE]
> A byte in a union is a value byte if it is a value byte in *any* field.

r[value.aggregate.padding]
Every byte in an aggregate which is not a value byte is a padding byte.

r[value.aggregate.struct]
A value of a struct type consists of the values of each of its fields.
The representation of such a struct contains the representation of the value of each field at its corresponding offset.

r[value.aggregate.union]
A value of a union type consists of a sequence of bytes, corresponding to each value byte. The value bytes of a union are represented exactly.

> [!NOTE]
> When a union value is constructed or a field is read/written to, the value of that field is encoded or decoded appropriately.

r[value.aggregate.padding-uninit]
When a value of an aggregate is encoded, each padding byte is left as uninit

> [!NOTE]
> It is valid for padding bytes to hold a value other than uninit when decoded, and these bytes are ignored when decoding an aggregate.

r[value.aggregate.tuple-array]
The fields of a tuple or an array are the elements of that tuple or array.

## Enum Values

r[value.enum]

r[value.enum.intro]
An enum value corresponds to exactly one variant of the enum, and consists of the fields of that variant

> [!NOTE]
> An enum with no variants therefore has no values.

r[value.enum.variant-padding]
A byte is a padding byte in a variant `V` if the byte is not used for computing the discriminant, and the byte would be a padding byte in a struct consisting of the fields of the variant at the same offsets.

r[value.enum.value-padding]
A byte is a padding byte of an enum if it is a padding byte in each variant of the enum. A byte that is not a padding byte of an enum is a value byte.

r[value.enum.repr]
The representation of a value of an enum type includes the representation of each field of the variant at the appropriate offsets. When encoding a value of an enum type, each byte which is a padding byte in the variant is set to uninit. In the case of a [`repr(C)`][layout.repr.c.adt] or a [primitive-repr][layout.repr.primitive.adt] enum, the discriminant of the variant is represented as though by the appropriate integer type stored at offset 0.

> [!NOTE]
> Most `repr(Rust)` enums will also store a discriminant in the representation of the enum, but the exact placement or type of the discriminant is unspecified, as is the value that represents each variant.
