# Beneath `std`

This section documents features that are provided by the standard library and that `#![no_std]`
developers have to deal with (i.e. provide) to build `#![no_std]` binary crates. A list of such
features is shown below:

- `#[panic_implementation]`

## `#[panic_implementation]`

The `panic_implementation` attribute can only be applied to a function with signature
`fn(&PanicInfo) -> !`. The function marked with this attribute defines the behavior of `panic!` in
`#![no_std]` applications. The [`PanicInfo`] struct contains information about the location of the
panic. There must be a single `panic_implementation` function in the dependency graph of a
binary, dylib or cdylib crate.

[`PanicInfo`]: https://doc.rust-lang.org/nightly/core/panic/struct.PanicInfo.html

Below is shown a `panic_implementation` function that logs the panic message and then aborts the
program.

``` rust
#![feature(core_intrinsics)]
#![feature(panic_implementation)]
#![no_std]

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_implementation]
fn panic(info: &PanicInfo) -> ! {
    let mut sink = /* .. */;

    // logs "panicked at '$reason', src/main.rs:27:4" to some `sink`
    let _ = writeln!(sink, "{}", info);

    unsafe { intrinsics::abort() }
}
```
