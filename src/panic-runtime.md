# Panic runtimes
[top]: #panic-runtimes

By default, the `panic!` macro unwinds Rust frames, just as C++'s `throw`
unwinds C++ frames. The actual behavior of `panic!` is controlled by the _panic
runtime_. The Rust standard library provides two panic runtimes: `panic_unwind`
(the default) and `panic_abort`, which immediately aborts the process.
When compiling code that is guaranteed (via a [compiler
option][rustc-codegen]), the optimizer may assume that unwinding across Rust
frames is impossible, which can result in both code-size and runtime speed
improvements.

## `rustc` codegen and linking
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

## Custom panic runtimes

<!-- XXX - is this attribute now stable? -->

In addition to the runtimes provided by the standard library, users may
implement their own panic runtime. The `#![panic_runtime]` attribute indicates
that a crate is a runtime implementation of panics.

The actual API of panic runtimes is not currently specified.

With `panic=abort`:
* Cleanup code (`Drop`) can't be called
* Panics can't be caught with `catch_unwind`
