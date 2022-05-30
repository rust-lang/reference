# Panic

Rust provides the ability to "panic" upon encountering a runtime error that is
not part of a function's signature; such an error is typically not expected to
be recoverable within the context in which the error is encountered.

> The Standard Library provides this capability via the [`panic!` macro][macro-panic].

Although it is not part of the core language, panics interact with several core
language features; for instance, out-of-bounds array indexing using the
`array[index]` syntax will automatically panic.

[macro-panic]: ../std/macro.panic.html
[fn-catch-unwind]: ../std/panic/fn.catch_unwind.html
[join]: ../std/thread/struct.JoinHandle.html#method.join

## Unwinding

By default, the `panic!` macro unwinds Rust frames, just as C++'s `throw`
unwinds C++ frames. This means that as the panic traverses Rust frames, live
objects in those frames that implement `Drop` will have their `drop` methods
called. It also enables the runtime to recover from the panic rather than
terminating execution.

> The Standard Library provides two mechanisms for recovering from a panic,
> [`catch_unwind`][fn-catch-unwind] (which enables a thread to recover) and
> [`JoinHandle::join`][join] (which enables a process to recover without
> recovering the thread state).

## Panic runtimes
[top]: #panic-runtimes

The actual behavior of `panic!` is controlled by the _panic runtime_. The Rust
standard library provides two panic runtimes: `panic_unwind` (the default) and
`panic_abort`, which immediately aborts the process.  When compiling code that
is guaranteed (via a [compiler option][rustc-codegen]), the optimizer may
assume that unwinding across Rust frames is impossible, which can result in
both code-size and runtime speed improvements.

### `rustc` codegen and linking
[rustc-codegen]: #rustc-codegen

`rustc` provides two supported runtime strategies:

* `-C panic=unwind` - this links against `panic_unwind` and ensures that
  unwinding across the compiled frames will perform all appropriate clean-up.
  In particular, `drop` will be called for live `Drop` objects.
* `-C panic=abort` - this links against `panic_abort`, and optimizes with the
  assumption that frames cannot be unwound. Since unwinding does not occur with
  this runtime, it is impossible to catch a `panic!` using `catch_unwind`.

This codegen option will default to `unwind` if not specified, and the value is
encoded into the crate metadata.

Linking against the actual panic runtime is performed lazily, so that if
different crates specify different runtimes, the `panic_abort` runtime is
preferred. This ensures that `panic!` cannot cause a soundness hole by
unwinding across Rust frames compiled with `panic=abort`.
