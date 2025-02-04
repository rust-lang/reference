r[type.numeric]
# Numeric types

r[type.numeric.int]
## Integer types

r[type.numeric.int.unsigned]
The unsigned integer types consist of:

Type   | Minimum | Maximum           | Maximum (decimal)
-------|---------|-------------------|------------------------
`u8`   | 0       | 2<sup>8</sup>-1   | 255
`u16`  | 0       | 2<sup>16</sup>-1  | 65535
`u32`  | 0       | 2<sup>32</sup>-1  | 4294967295
`u64`  | 0       | 2<sup>64</sup>-1  | 18446744073709551615
`u128` | 0       | 2<sup>128</sup>-1 | ≈ 3.4 x 10<sup>38</sup>

r[type.numeric.int.signed]
The signed two's complement integer types consist of:

Type   | Minimum            | Maximum           | Decimal
-------|--------------------|-------------------|----------------------------------------------------
`i8`   | -(2<sup>7</sup>)   | 2<sup>7</sup>-1   | \[-128..127]
`i16`  | -(2<sup>15</sup>)  | 2<sup>15</sup>-1  | \[-32768..32767]
`i32`  | -(2<sup>31</sup>)  | 2<sup>31</sup>-1  | \[-2147483648..2147483647]
`i64`  | -(2<sup>63</sup>)  | 2<sup>63</sup>-1  | \[-9223372036854775808..9223372036854775807]
`i128` | -(2<sup>127</sup>) | 2<sup>127</sup>-1 | \[≈ -1.7 x 10<sup>38</sup>..≈1.7 x 10<sup>38</sup>]


r[type.numeric.float]
## Floating-point types

The IEEE 754-2008 "binary32" and "binary64" floating-point types are `f32` and
`f64`, respectively.

r[type.numeric.int.size]
## Machine-dependent integer types

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

r[type.numeric.validity]
## Bit validity

For every numeric type, `T`, the bit validity of `T` is equivalent to the bit
validity of `[u8; size_of::<T>()]`. An uninitialized byte is not a valid `u8`.
