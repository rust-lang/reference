## #[panic_implementation]

The `#[panic_implementation]` attribute can only be applied to a function with signature
`fn(&PanicInfo) -> !`. The function marked with this attribute defines the behavior of `panic!` in
`#![no_std]` applications. There must be a *single* `#[panic_implementation]` function in the
dependency graph of a binary / dylib / cdylib crate.

The `PanicInfo` struct contains information about the location of the panic. The API of `PanicInfo`
can be found in the [API docs]. As of 1.28-beta (2018-06-21) there's a difference between
`core::panic!` and `std::panic!`: the former doesn't accept non-string payloads -- that is
`core::panic!(42)` is not accepted. In the future `core::panic!` may gain support for non-string
payloads. The implementation of `core::panic!` can be found in `src/libcore/{macros,panicking}.rs`

[API docs]: https://doc.rust-lang.org/nightly/core/panic/struct.PanicInfo.html

Below is shown a `#[panic_implementation]` that logs the panic message and then aborts the program.

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
