# The Rust runtime

This section documents features that define some aspects of the Rust runtime. A list of such
features is shown below:

- `#[panic_handler]`, the panicking behavior

## `#[panic_handler]`

The `panic_handler` attribute can only be applied to a function with signature
`fn(&PanicInfo) -> !`. The function marked with this attribute defines the behavior of panics. The
[`PanicInfo`] struct contains information about the location of the panic. There must be a single
`panic_handler` function in the dependency graph of a binary, dylib or cdylib crate.

[`PanicInfo`]: https://doc.rust-lang.org/nightly/core/panic/struct.PanicInfo.html

Below is shown a `panic_handler` function that logs the panic message and then halts the
thread.

``` rust
#![no_std]

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut sink = /* .. */;

    // logs "panicked at '$reason', src/main.rs:27:4" to some `sink`
    let _ = writeln!(sink, "{}", info);

    loop {}
}
```

### Standard behavior

The standard library provides an implementation of `panic_handler` than can be
statically customized using the `-C panic` flag. `-C panic=abort` makes panics
abort the process, and `-C panic=unwind` makes panics unwind the panicking
thread. If no panicking behavior is specified using `-C panic` one of these two
behaviors is chosen according to the compilation target.
