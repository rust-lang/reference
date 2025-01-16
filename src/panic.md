r[panic]
# Panic

r[panic.intro]
Rust provides a mechanism to prevent a function from returning normally, and instead "panic," which is a response to an error condition that is typically not expected to be recoverable within the context in which the error is encountered.

r[panic.lang-ops]
Some language constructs, such as out-of-bounds [array indexing], panic automatically.

r[panic.control]
There are also language features that provide a level of control over panic behavior:

* A [_panic runtime_](#panic-runtimes) defined how a panic is handled during runtime.
* [FFI ABIs](items/functions.md#unwinding) may alter how panics behave.

> [!NOTE]
> The standard library provides the capability to explicitly panic via the [`panic!` macro][panic!].

r[panic.runtime]
## Panic runtimes

The actual behavior and implementation of a panic is controlled by the _panic runtime_. The panic runtime is a handler linked into the output which provides the necessary implementation for panicking.

> [!NOTE]
> The Rust standard library provides two panic runtimes: `panic_unwind` (which unwinds the stack and is potentially recoverable) and `panic_abort` (which aborts the process and is non-recoverable). The default runtime depends on the target platform, but is generally `panic_unwind` on platforms with native support for C++ exceptions.

> [!NOTE]
> The panic runtime can be chosen in `rustc` with the [`-C panic`] CLI flag when building any crate type except an rlib.

See also the [`panic_handler` attribute](runtime.md#the-panic_handler-attribute) which can be used to change the behavior of panics.

r[panic.strategy]
## Panic strategy

r[panic.strategy.intro]
The _panic strategy_ defines the kind of panic runtime that a crate is built to support.

> [!NOTE]
> The panic strategy can be chosen in `rustc` with the [`-C panic`] CLI flag.

> [!NOTE]
> When compiling code with a non-recoverable panic strategy, the optimizer may assume that unwinding across Rust frames is impossible, which can result in both code-size and runtime speed improvements.

r[panic.strategy.mixed]
When linking with the `unwind` runtime, all crates must be built with the `unwind` strategy.

r[panic.unwind]
## Unwinding

r[panic.unwind.intro]
Panicking may either be recoverable or non-recoverable, though it can be configured (by choosing the `abort` panic runtime) to always be non-recoverable. (The converse is not true: the `unwind` runtime does not guarantee that all panics are recoverable, only that panicking via the `panic!` macro and similar standard library mechanisms is recoverable.)

r[panic.unwind.destruction]
When panic recovery occurs, the `unwind` runtime "unwinds" Rust frames, just as C++'s `throw` unwinds C++ frames, until the panic reaches the point of recovery (for instance at a thread boundary). This means that as the panic traverses Rust frames, live objects in those frames that [implement `Drop`][destructors] will have their `drop` methods called. Thus, when normal execution resumes, no-longer-accessible objects will have been "cleaned up" just as if they had gone out of scope normally.

> [!NOTE]
> As long as this guarantee of resource-cleanup is preserved, "unwinding" may be implemented without actually using the mechanism used by C++ for the target platform.

> [!NOTE]
> The standard library provides two mechanisms for recovering from a panic, [`std::panic::catch_unwind`] (which enables recovery within the panicking thread) and [`std::thread::spawn`] (which automatically sets up panic recovery for the spawned thread so that other threads may continue running).

r[panic.unwind.ffi]
### Unwinding across FFI boundaries

r[panic.unwind.ffi.intro]
It is possible to unwind across FFI boundaries using an [appropriate ABI declaration][unwind-abi]. While useful in certain cases, this creates unique opportunities for undefined behavior, especially when multiple language runtimes are involved.

r[panic.unwind.ffi.undefined]
Unwinding with the wrong ABI is undefined behavior:

* Causing an unwind into Rust code from a foreign function that was called via a function declaration or pointer declared with a non-unwinding ABI, such as `"C"`, `"system"`, etc. (For example, this case occurs when such a function written in C++ throws an exception that is uncaught and propagates to Rust.)
* Calling a Rust `extern` function that unwinds (with `extern "C-unwind"` or another ABI that permits unwinding) from a runtime that does not support. unwinding, such as code compiled with GCC or Clang using `-fno-exceptions`

r[panic.unwind.ffi.catch-foreign]
Catching a foreign unwinding operation (such as a C++ exception) using [`std::panic::catch_unwind`], [`std::thread::JoinHandle::join`], or by letting it propagate beyond the Rust `main()` function or thread root will have one of two behaviors, and it is unspecified which will occur:

* The process aborts.
* The function returns a [`Result::Err`] containing an opaque type.

> [!NOTE]
>  Rust code compiled or linked with a different instance of the Rust runtime counts as a "foreign exception" for the purpose of this guarantee. Thus, a library that uses `panic!` and is linked against one version of the Rust standard library, invoked from an application that uses a different version of the standard library, may cause the entire application to crash even if the library is only used within a child thread.

r[panic.unwind.ffi.dispose-panic]
There are currently no guarantees about the behavior that occurs when a foreign runtime attempts to dispose of, or rethrow, a Rust `panic` payload. In other words, an unwind originated from a Rust runtime must either lead to termination of the process or be caught by the same runtime.

[array indexing]: expressions/array-expr.md#array-and-slice-indexing-expressions
[destructors]: destructors.md
[runtime]: runtime.md
[unwind-abi]: items/functions.md#unwinding
[`-C panic`]: ../rustc/codegen-options/index.html#panic
