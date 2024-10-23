# Module core::ffi

r[core.ffi]


## Module core::ffi Synopsis

r[core.ffi.synopsis]

<!--ignore: synopsis of module, not necessarily valid rust-->
```rust,ignore

pub struct FromBytesUntilNulError(/*private fields*/);
pub struct FromBytesWithNulError(/*private fields*/);
pub struct CStr{/*private fields*/}

pub enum c_void{}

pub type c_char = /* see below */;
pub type c_double = /* see below */;
pub type c_float = /* see below */;
pub type c_int = /* see below */;
pub type c_long = /*see below*/;
pub type c_longlong = /*see below*/;
pub type c_schar = /*see below*/;
pub type c_short = /*see below*/;
pub type c_uchar = /*see below*/;
pub type c_ulong = /*see below*/;
pub type c_ulonglong = /*see below*/;
pub type c_ushort = /* see below */;
```

## CStr

r[core.ffi.cstr]

r[core.ffi.cstr.intro]
A `CStr` is a slice of bytes that contains a nul-terminated string of arbitrary non-nul bytes.

r[core.ffi.cstr.literal]
A literal of the form `c"<string literal content>"` has type `&'static core::ffi::CStr`.

r[core.ffi.cstr.sized]
`CStr` is an unsized type. 

### `CStr::from_ptr`

r[core.ffi.cstr.from_ptr]

r[core.ffi.cstr.from_ptr.def]
<!--ignore: incomplete code fragment, showing a synopsis as a function -->
```rust,ignore
impl CStr{
    pub const unsafe fn from_ptr<'a>(ptr: *const c_char) -> &'a CStr;
}
```

r[core.ffi.cstr.from_ptr.intro]
The `CStr::from_ptr` can be used to `unsafe`ly construct a `CStr` from a pointer to a nul-terminated C string. 

r[core.ffi.cstr.from_ptr.precondition]
The behavior of this function is undefined unless:
* There exists a value `i` of type `usize`, 
 such that the range `[ptr, ptr.add(i)]` is a range valid for reads, and `ptr.add(i).read()==0`,
* `i < (isize::MAX as usize)`,

> [!WARNING]
> In addition, to safely use the result, the callee must ensure that the validity of `ptr` remains for the duration of `'a`,
> and that the bytes referred to be the `CStr` aren't modified for the duration of `'a`.

r[core.ffi.cstr.from_ptr.postcondition]
The returned reference is a shared borrow derived from `ptr` which borrows `[ptr, ptr.add(i+1))`

r[core.ffi.cstr.from_ptr.return]
The return value is a `&CStr` that starts at `ptr`, and for which `CStr::count_bytes` returns `i`.

r[core.ffi.cstr.from_ptr.safety]
`CStr::from_ptr` is an `unsafe` function.

r[core.ffi.cstr.from_ptr.const]
`CStr::from_ptr` is a `const` function.

### `CStr::from_bytes_until_nul`

r[core.ffi.cstr.from_bytes_until_nul]

r[core.ffi.cstr.from_bytes_until_nul.def]
<!--ignore: incomplete code fragment, showing a synopsis as a function def -->
```rust,ignore
impl CStr{
    pub const fn from_bytes_until_nul(bytes: &[u8]) -> Result<&CStr, FromBytesUntilNulError>;
}
```

r[core.ffi.cstr.from_bytes_until_nul.intro]
Constructs a `CStr` slice over `bytes`, up to the first instance of a `0` byte, terminating the string. If no such terminator exists, an error is returned instead.

r[core.ffi.cstr.from_bytes_until_nul.return]
If there exists some index `i` of type `usize`, such that `i < bytes.len()` and `bytes[i] == 0`, returns a `CStr` borrowed from `bytes` starting from the first index, 
and such that `CStr::count_bytes` returns `i` . If no such `i` exists, returns a `FromBytesUntilNulError` that indicates the failure.

> [!NOTE]
> If there is exactly one 0 byte in `bytes`, located at the last index of the slice, the returned slice is identical to the one returned by [`CStr::from_bytes_with_nul`][core.ffi.cstr.from_bytes_with_nul]

### `CStr::from_bytes_with_nul`

r[core.ffi.cstr.from_bytes_with_nul]

r[core.ffi.cstr.from_bytes_with_nul.def]
<!--ignore: incomplete code fragment, showing a synopsis as a function def -->
```rust,ignore
impl CStr{
    pub const fn from_bytes_with_nul(bytes: &[u8]) -> Result<&CStr, FromBytesWithNulError>;
}
```

r[core.ffi.cstr.from_bytes_with_nul.intro]
Constructs a `CStr` slice over `bytes`, provided that exactly one `0` byte occurs at the very end of `bytes`. If the last byte of the string is not `0`, or a `0` byte occurs anywhere else in `bytes`, an error is returned instead.

r[core.ffi.cstr.from_bytes_with_nul.return]
If there exists some index `i` of type `usize`, such that `i < bytes.len()` and `bytes[i] == 0`, then:
* If `i + 1 == bytes.len()`, returns a `CStr` borrowed from `bytes` starting from the first index,
 and such that `CStr::count_bytes` returns `i`,
* Otherwise, or if no such index exists, returns a `FromBytesWithNullError` that indicates the failure.

## C-compatible primitive types

r[core.ffi.c-primitives]

r[core.ffi.c-primitives.def]
<!--ignore: incomplete code fragment, showing a synopsis as a function def -->
```rust
pub type c_char = /* see below */;
pub type c_double = /* see below */;
pub type c_float = /* see below */;
pub type c_int = /* see below */;
pub type c_long = /*see below*/;
pub type c_longlong = /*see below*/;
pub type c_schar = i8;
pub type c_short = /*see below*/;
pub type c_uchar = /*see below*/;
pub type c_ulong = /*see below*/;
pub type c_ulonglong = /*see below*/;
pub type c_ushort = u8;
```

r[core.ffi.c-primitives.intro]
The C-compatible primitive types are type aliases of primitive types, which are ABI compatible with the corresponding type in C on the current target.

r[core.ffi.c-primitives.c_char]
The type alias `c_char` is a target dependent integer type with the same width and signedness as the `char` type in C.

> [!NOTE]
> On every platform that can support Rust, this is either `u8` or `i8`.

r[core.ffi.c-primitives.c_double]
The type alias `c_double` is a target dependent floating-point type with the same range and precision as the `double` type in C.

> [!NOTE]
> On most platforms, this is `f64`.

r[core.ffi.c-primitives.c_float]
The type alias `c_float` is a target dependent floating-point type with the same range and precision as the `float` type in C.

> [!NOTE]
> On most platforms, this is `f32`.

r[core.ffi.c-primitives.c_int]
The type alias `c_int` is a target dependent signed integer type with the same width as the `int` type in C. 
The minimum width of this type is 16-bit, and it is at least as wide as the `c_short` alias.

> [!NOTE]
> On most 32 and 64-bit platforms, this is `i32`, 
> but it may be `i16` also on a 16-bit platform.

r[core.ffi.c-primitives.c_long]
The type alias `c_long` is a target dependent signed integer type with the same width as the `long` type in C.
The minimum width of this type is 32-bit, and it is at least as wide as the `c_int` alias.

> [!NOTE]
> The minimum width for this type is 32-bit.
> On most 64-bit platforms, this is `i64`. 

r[core.ffi.c-primitives.c_longlong]
The type alias `c_longlong` is a target dependent signed integer type with the same width as the `long long` type in C.
The minimum width of this type is 64-bit, and it is at least as wide as the `c_long` alias.

> [!NOTE]
> This is almost always `i64`, but may be wider.

r[core.ffi.c-primitives.c_schar]
The type alias `c_schar` is an alias of the type `i8`.

> [!NOTE]
> On every platform that can support Rust, this is compatible with the C type `signed char`

r[core.ffi.c-primitives.c_short]
The type alias `c_short` is a target depedent signed integer type with the same width as the `short` type in C.
The minimum width of this type is 16-bit.

> [!NOTE]
> This is almost always `i16`.

r[core.ffi.c-primitives.c_uchar]
The type alias `c_uchar` is an alias of the type `u8`.

> [!NOTE]
> On every platform that can support Rust, this is compatible with the C type `unsigned char`

r[core.ffi.c-primitives.unsigned]
The aliases `c_uint`, `c_ulong`, `c_ulonglong`, and `c_ushort` are all aliases of the unsigned counterpart of the same integer type as the corresponding signed alias.

> [!NOTE]
> In particular, an alias `c_u`*`ty`* is `usize` if and only if `c_`*`ty`* is `isize`. 
> In all other cases, they will be `uN` and `iN` respectively, where `N` is the appropriate width. 

r[core.ffi.c-primitives.traits]
Each type in this section implements the traits `Copy`, `Clone`, `Send`, `Sync`, `Debug`, and `Display`. The integer type aliases implement `LowerHex`, `UpperHex`, `Octal`, and `Binary`. `c_float` and `c_double` implement `LowerExp` and `UpperExp`.

## c_void

r[core.ffi.c_void]

r[core.ffi.c_void.def]
<!--ignore: incomplete code fragment, showing a synopsis as a function def -->
```rust,ignore
#[non_exhaustive]
pub enum c_void{}
```

r[core.ffi.c_void.intro]
The `c_void` type is an enum type such that `*const c_void` is compatible with the C type `const void*`. 

> [!NOTE]
> This is only true behind an indirection - `c_void` itself is not compatible with a function returning `void`. 

r[core.ffi.c_void.variants]
The `c_void` type cannot be constructed, and it cannot be exhaustively matched.

> [!NOTE]
> The `c_void` type acts as though it has no variants. but cannot be matched like an empty enum. 

r[core.ffi.c_void.traits]
The `c_void` type implements the `Send`, `Sync`, and `Debug` traits. 
