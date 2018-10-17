# Numeric types

## Machine types

The machine types are the following:

* The unsigned word types `u8`, `u16`, `u32`, `u64`, and `u128` with values
  drawn from the integer intervals [0, 2^8 - 1], [0, 2^16 - 1], [0, 2^32 - 1],
  [0, 2^64 - 1], and [0, 2^128 - 1] respectively.

* The signed two's complement word types `i8`, `i16`, `i32`, `i64`, and `i128`,
  with values drawn from the integer intervals [-(2^7), 2^7 - 1],
  [-(2^15), 2^15 - 1], [-(2^31), 2^31 - 1], [-(2^63), 2^63 - 1], and
  [-(2^127), 2^127 - 1] respectively.

* The IEEE 754-2008 "binary32" and "binary64" floating-point types: `f32` and
  `f64`, respectively.

## Machine-dependent integer types

The `usize` type is an unsigned integer type with the same number of bits as the
platform's pointer type. It can represent every memory address in the process.

The `isize` type is a signed integer type with the same number of bits as the
platform's pointer type. The theoretical upper bound on object and array size
is the maximum `isize` value. This ensures that `isize` can be used to calculate
differences between pointers into an object or array and can address every byte
within an object along with one byte past the end.
