# The Rust runtime

This section documents features that define some aspects of the Rust runtime.

## The `panic_handler` attribute

The *`panic_handler` attribute* can only be applied to a function with signature
`fn(&PanicInfo) -> !`. The function marked with this [attribute] defines the behavior of panics. The
[`PanicInfo`] struct contains information about the location of the panic. There must be a single
`panic_handler` function in the dependency graph of a binary, dylib or cdylib crate.

Below is shown a `panic_handler` function that logs the panic message and then halts the
thread.

<!-- NOTE(ignore) `mdbook test` doesn't support `no_std` code -->

``` rust, ignore
#![no_std]

use core::fmt::{self, Write};
use core::panic::PanicInfo;

struct Sink {
    // ..
#    _0: (),
}
#
# impl Sink {
#     fn new() -> Sink { Sink { _0: () }}
# }
#
# impl fmt::Write for Sink {
#     fn write_str(&mut self, _: &str) -> fmt::Result { Ok(()) }
# }

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut sink = Sink::new();

    // logs "panicked at '$reason', src/main.rs:27:4" to some `sink`
    let _ = writeln!(sink, "{}", info);

    loop {}
}
```

### Standard behavior

The standard library provides an implementation of `panic_handler` that
defaults to unwinding the stack but that can be [changed to abort the
process][abort]. The standard library's panic behavior can be modified at
runtime with the [set_hook] function.

[`PanicInfo`]: ../core/panic/struct.PanicInfo.html
[abort]: ../book/ch09-01-unrecoverable-errors-with-panic.html
[attribute]: attributes.html
[set_hook]: ../std/panic/fn.set_hook.html
