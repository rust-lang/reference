# Numeric types

r[type.numeric]

## Integer types

r[type.numeric.int]

r[type.numeric.int.unsigned]
The unsigned integer types consist of:

Type   | Minimum | Maximum
-------|---------|-------------------
`u8`   | 0       | 2<sup>8</sup>-1
`u16`  | 0       | 2<sup>16</sup>-1
`u32`  | 0       | 2<sup>32</sup>-1
`u64`  | 0       | 2<sup>64</sup>-1
`u128` | 0       | 2<sup>128</sup>-1

r[type.numeric.int.signed]
The signed two's complement integer types consist of:

Type   | Minimum            | Maximum
-------|--------------------|-------------------
`i8`   | -(2<sup>7</sup>)   | 2<sup>7</sup>-1
`i16`  | -(2<sup>15</sup>)  | 2<sup>15</sup>-1
`i32`  | -(2<sup>31</sup>)  | 2<sup>31</sup>-1
`i64`  | -(2<sup>63</sup>)  | 2<sup>63</sup>-1
`i128` | -(2<sup>127</sup>) | 2<sup>127</sup>-1


## Floating-point types

r[type.numeric.float]

The IEEE 754-2008 "binary32" and "binary64" floating-point types are `f32` and
`f64`, respectively.

## Machine-dependent integer types

r[type.numeric.int.size]

r[type.numeric.int.size.usize]
The `usize` type is an unsigned integer type with the same number of bits as the
platform's pointer type. It can represent every memory address in the process.

r[type.numeric.int.size.isize]
The `isize` type is a signed integer type with the same number of bits as the
platform's pointer type. The theoretical upper bound on object and array size
is the maximum `isize` value. This ensures that `isize` can be used to calculate
differences between pointers into an object or array and can address every byte
within an object along with one byte past the end.

r[type.numeric.int.size.minimum]
`usize` and `isize` are at least 16-bits wide.

> **Note**: Many pieces of Rust code may assume that pointers, `usize`, and
> `isize` are either 32-bit or 64-bit. As a consequence, 16-bit
> pointer support is limited and may require explicit care and acknowledgment
> from a library to support.

## Bit validity

r[type.numeric.validity]

For every numeric type, `T`, the bit validity of `T` is equivalent to the bit
validity of `[u8; size_of::<T>()]`. An uninitialized byte is not a valid `u8`.

## Representation

r[type.numeric.repr]

r[type.numeric.repr.integer]
Each value of an integer type is a whole number. For unsigned types, this is a positive integer or `0`. For signed types, this can either be a positive integer, negative integer, or `0`.

r[type.numeric.repr.integer-width]
The range of values an integer type can represent depends on its signedness and its width, in bits. The width of type `uN` or `iN` is `N`. The width of type `usize` or `isize` is the value of the `target_pointer_width` property.

> [!NOTE]
> There are exactly `1<<N` unique values of an integer type of width `N`.

r[type.numeric.repr.unsigned]
A value `i` of an unsigned integer type `U` is represented by a sequence of initialized bytes, where the `m`th successive byte according to the byte order of the platform is `(i >> (m*8)) as u8`, where `m` is between `0` and the size of `U`. None of the bytes produced by encoding an unsigned integer has a pointer fragment.

> [!NOTE]
> The two primary byte orders are `little` endian, where the bytes are ordered from lowest memory address to highest, and `big` endian, where the bytes are ordered from highest memory address to lowest.
> The `cfg` predicate `target_endian` indicates the byte order

> [!WARN]
> On `little` endian, the order of bytes used to decode an integer type is the same as the natural order of a `u8` array - that is, the `m` value corresponds with the `m` index into a same-sized `u8` array. On `big` endian, however, the order is the opposite of this order - that is, the `m` value corresponds with the `size_of::<T>() - m` index in that array.

r[type.numeric.repr.signed]
A value `i` of a signed integer type with width `N` is represented the same as the corresponding value of the unsigned counterpart type which is congruent modulo `2^N`.

r[type.numeric.repr.float]
A floating-point value is represented the same as a value of the unsigned integer type with the same width given by its [IEEE 754-2019] encoding.

r[type.numeric.repr.float-format]
The [IEEE 754-2019] `binary32` format is used for `f32`, and the `binary64` format is used for `f64`.

[IEEE 754-2019]: https://ieeexplore.ieee.org/document/8766229