# External blocks

External blocks form the basis for Rust's foreign function interface.
Declarations in an external block describe symbols in external, non-Rust
libraries.

Functions within external blocks are declared in the same way as other Rust
functions, with the exception that they may not have a body and are instead
terminated by a semicolon.

Functions within external blocks may be called by Rust code, just like
functions defined in Rust. The Rust compiler automatically translates between
the Rust ABI and the foreign ABI.

Functions within external blocks may be variadic by specifying `...` after one
or more named arguments in the argument list:

```rust,ignore
extern {
    fn foo(x: i32, ...);
}
```

A number of [attributes] control the behavior of external blocks.

[attributes]: attributes.html#ffi-attributes

By default external blocks assume that the library they are calling uses the
standard C ABI on the specific platform. Other ABIs may be specified using an
`abi` string, as shown here:

```rust,ignore
// Interface to the Windows API
extern "stdcall" { }
```

There are three ABI strings which are cross-platform, and which all compilers
are guaranteed to support:

* `extern "Rust"` -- The default ABI when you write a normal `fn foo()` in any
  Rust code.
* `extern "C"` -- This is the same as `extern fn foo()`; whatever the default
  your C compiler supports.
* `extern "system"` -- Usually the same as `extern "C"`, except on Win32, in
  which case it's `"stdcall"`, or what you should use to link to the Windows
  API itself

There are also some platform-specific ABI strings:

* `extern "cdecl"` -- The default for x86\_32 C code.
* `extern "stdcall"` -- The default for the Win32 API on x86\_32.
* `extern "win64"` -- The default for C code on x86\_64 Windows.
* `extern "sysv64"` -- The default for C code on non-Windows x86\_64.
* `extern "aapcs"` -- The default for ARM.
* `extern "fastcall"` -- The `fastcall` ABI -- corresponds to MSVC's
  `__fastcall` and GCC and clang's `__attribute__((fastcall))`
* `extern "vectorcall"` -- The `vectorcall` ABI -- corresponds to MSVC's
  `__vectorcall` and clang's `__attribute__((vectorcall))`

Finally, there are some rustc-specific ABI strings:

* `extern "rust-intrinsic"` -- The ABI of rustc intrinsics.
* `extern "rust-call"` -- The ABI of the Fn::call trait functions.
* `extern "platform-intrinsic"` -- Specific platform intrinsics -- like, for
  example, `sqrt` -- have this ABI. You should never have to deal with it.

The `link` attribute allows the name of the library to be specified. When
specified the compiler will attempt to link against the native library of the
specified name.

```rust,ignore
#[link(name = "crypto")]
extern { }
```

The type of a function declared in an extern block is `extern "abi" fn(A1, ...,
An) -> R`, where `A1...An` are the declared types of its arguments and `R` is
the declared return type.

It is valid to add the `link` attribute on an empty extern block. You can use
this to satisfy the linking requirements of extern blocks elsewhere in your
code (including upstream crates) instead of adding the attribute to each extern
block.
