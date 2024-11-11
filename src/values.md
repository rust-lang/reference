# Values and representation

r[value]

## Bytes

r[value.byte]

r[value.byte.intro]
The most basic unit of memory in Rust is a byte. All values in Rust are computed from 0 or more bytes read from an allocation.

> [!NOTE]
> While bytes in Rust are typically lowered to hardware bytes, they may contain additional values,
> such as being uninitialized, or storing part of a pointer.

r[value.byte.init]
Each byte may be initialized, and contain a value of type `u8`, as well as an optional pointer fragment.

r[value.byte.uninit]
Each byte may be uninitialized.

> [!NOTE]
> Uninitialized bytes do not have a value and do not have a pointer fragment.

## Value Encoding

r[value.encoding]

r[value.encoding.intro]
Each type in Rust has 0 or more values, which can have operations performed on them

> [!NOTE]
> `0u8`, `1337i16`, and `Foo{bar: "baz"}` are all values

r[value.encoding.op]
Each value of a type can be encoded into a sequence of bytes, and decoded from a sequence of bytes, which has a length equal to the size of the type.
The operation to encode or decode a value is determined by the representation of the type.

> [!NOTE]
> Representation is related to, but is not the same property as, the layout of the type.

r[value.encoding.decode]
If a value of type `T` is decoded from a sequence of bytes that does not correspond to a defined value, the behavior is undefined. If a value of type `T` is decoded from a sequence of bytes that contain pointer fragments, which are not used to represent the value, the pointer fragments are ignored.

## Pointer Provenance

r[value.provenance]

r[value.provenance.intro]
Pointer Provenance is a term that refers to additional data carried by pointer values in Rust, beyond its address. When stored in memory, Provenance is encoded in the Pointer Fragment part of each byte of the pointer.

r[value.provenance.allocation]
Whenever a pointer to a particular allocation is produced by using the reference or raw reference operators, or when a pointer is returned from an allocation function, the resulting pointer has provenance that refers to that allocation.

> [!NOTE]
> There is additional information encoded by provenance, but the exact scope of this information is not yet decided.

r[value.provenance.dangling]
A pointer is dangling if it has no provenance, or if it has provenance to an allocation that has since been deallocated. An access, except for an access of size zero, using a dangling pointer, is undefined behavior.

> [!NOTE]
> Allocations include local and static variables, as well as temporaries. Local Variables and Temporaries are deallocated when they go out of scope.

> [!WARN]
> The above is necessary, but not sufficient, to avoid undefined behavior. The full requirements for pointer access is not yet decided.
> A reference obtained in safe code is guaranteed to be valid for its usable lifetime, unless interfered with by unsafe code.

## Primitive Values

r[value.primitive]

r[value.primitive.integer]
Each value of an integer type is a whole number. For unsigned types, this is a positive integer or `0`. For signed types, this can either be a positive integer, negative integer, or `0`.

r[value.primtive.integer-width]
The range of values an integer type can represent depends on its signedness and its width, in bits. The width of type `uN` or `iN` is `N`. The width of type `usize` or `isize` is the value of the `target_pointer_width` property.

r[value.primitive.integer-range]
The range of an unsigned integer type of width `N` is between `0` and `1<<N - 1` inclusive. The range of a signed integer type of width `N` is between `-(1<<(N-1)` and `1<<(N-1) - 1` inclusive.

> [!NOTE]
> There are exactly `1<<N` unique values of an integer type of width `N`.

r[value.primitive.unsigned-repr]
A value `i` of an unsigned integer type `U` is represented by a sequence of initialized bytes, where the `m`th successive byte according to the byte order of the platform is `(i >> (m*8)) as u8`, where `m` is between `0` and the size of `U`. None of the bytes produced by encoding an unsigned integer has a pointer fragment.

> [!NOTE]
> The two primary byte orders are `little` endian, where the bytes are ordered from lowest memory address to highest, and `big` endian, where the bytes are ordered from highest memory address to lowest.
> The `cfg` predicate `target_endian` indicates the byte order

> [!WARN]
> On `little` endian, the order of bytes used to decode an integer type is the same as the natural order of a `u8` array - that is, the `m` value corresponds with the `m` index into a same-sized `u8` array. On `big` endian, however, the order is the opposite of this order - that is, the `m` value corresponds with the `size_of::<T>() - m` index in that array.

r[value.primitive.signed-repr]
A value `i` of a signed integer type with width `N` is represented the same as the corresponding value of the unsigned counterpart type which is congruent modulo `2^N`.

r[value.primitive.char]
Each value of type `char` is a Unicode Scalar Value, between `U+0000` and `U+10FFFF` (excluding the surrogate range `U+D800` through `U+DFFF`).

r[value.primitive.char-repr]
The representation of type `char` is the same as the representation of the `u32` corresponding to the Code Point Number encoding by the `char`.

r[value.primitive.bool]
The two values of type `bool` are `true` and `false`. The representation of `true` is an initialized byte with value `0x01`, and the representation of `false` is an initialized  byte with value `0x00`. Neither value is represented with a pointer fragment.

r[value.primitive.float]
A floating-point value consists of either a rational number, which is within the range and precision dictated by the type, an infinity, or a NaN value.

r[value.primitive.float-repr]
A floating-point value is represented the same as a value of the unsigned integer type with the same width given by its [IEEE 754-2019] encoding.

r[value.primitive.float-format]
The [IEEE 754-2019] `binary32` format is used for `f32`, and the `binary64` format is used for `f64`.

[IEEE 754-2019]: https://ieeexplore.ieee.org/document/8766229

## Pointer Value

r[value.pointer]

r[value.pointer.thin]
Each thin pointer consists of an address and an optional provenance. The address refers to which byte the pointer points to. The provenance refers to which bytes the pointer is allowed to access, and the allocation those bytes are within.

> [!NOTE]
> A pointer that does not have a provenance may be called an invalid or dangling pointer.

r[value.pointer.thin-repr]
The representation of a value of a thin pointer is a sequence of initialized bytes with `u8` values given by the representation of its address as a value of type `usize`, and pointer fragments corresponding to its provenance, if present.

r[value.pointer.thin-ref]
A thin reference to `T` consists of a non-null, well aligned address, and provenance for `size_of::<T>()` bytes starting from that address. The representation of a thin reference to `T` is the same as the pointer with the same address and provenance.

> [!NOTE]
> This is true for both shared and mutable references. There are additional constraints enforced by the aliasing model.

r[value.pointer.wide]
A wide pointer or reference consists of a data pointer or reference, and a pointee-specific metadata value.

r[value.pointer.wide-reference]
The data pointer of a wide reference has a non-null address, well aligned for `align_of_val(self)`, and with provenance for `size_of_val(self)` bytes.

r[value.pointer.wide-representation]
A wide pointer or reference is represented the same as `struct WidePointer<M>{data: *mut (), metadata: M}` where `M` is the pointee metadata type, and the `data` and `metadata` fields are the corresponding parts of the pointer.

> [!NOTE]
> The `WidePointer` struct has no guarantees about layout, and has the default representation.

r[value.pointer.fn]
A value of a function pointer type consists of an non-null address. A function pointer value is represented the same as an address represented as an unsigned integer type with the same width as the function pointer.

> [!NOTE]
> Whether or not a function pointer value has provenance, and whether or not this provenance is represented as pointer fragments, is not yet decided.

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
