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

Panicking may either be recoverable or non-recoverable, though its behavior
must be homogenous throughout program execution. A recoverable panic "unwinds"
Rust frames, just as C++'s `throw` unwinds C++ frames. This means that as the
panic traverses Rust frames, live objects in those frames that implement `Drop`
will have their `drop` methods called. Thus, if panic recovery does occur (for
instance at a thread boundary), the objects will have been "cleaned up" just as
if the stack frames had returned normally.

> As long as this guarantee of resource-cleanup is preserved, "unwinding" may
> be implemented without actually using the mechanism used by C++ for the
> target platform.

> The Standard Library provides two mechanisms for recovering from a panic,
> [`catch_unwind`][fn-catch-unwind] (which enables recovery within the
> panicking thread) and [`JoinHandle::join`][join] (which enables a process to
> continue execution without recovering the panicked thread).

## Panic runtimes
[top]: #panic-runtimes

The actual behavior and implementation of `panic!` is controlled by the _panic
runtime_.

> The Rust standard library provides two panic runtimes: `panic_unwind` (the
> default) and `panic_abort`, which immediately aborts the process (which is
> non-recoverable).

When compiling code that is guaranteed (via a [compiler option][rustc-codegen])
not to unwind, the optimizer may assume that unwinding across Rust frames is
impossible, which can result in both code-size and runtime speed improvements.
