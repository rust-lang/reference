# Panic

Rust provides the ability to "panic" upon encountering an error that is not
part of a function's signature; such an error is typically not expected to be
recoverable within the context in which the error is encountered.

Some language constructs, such as out-of-bounds [array indexing], panic
automatically. There are also language features that provide a level of control
over panic behavior:
* A [_panic runtime_](#panic-runtimes) defined how a panic is handled during
  runtime.
* [FFI ABIs](items/functions.md#unwinding) may alter how panics behave.

> **Note**: The Standard Library provides the capability to explicitly panic
> via the [`panic!` macro][macro-panic].

## Unwinding

Panicking may either be recoverable or non-recoverable, though its behavior
must be uniform throughout program execution. A recoverable panic "unwinds"
Rust frames, just as C++'s `throw` unwinds C++ frames. This means that as the
panic traverses Rust frames, live objects in those frames that [implement
`Drop`][destructors] will have their `drop` methods called. Thus, if panic
recovery does occur (for instance at a thread boundary), the objects will have
been "cleaned up" just as if they had gone out of scope normally.

> **Note**: As long as this guarantee of resource-cleanup is preserved,
> "unwinding" may be implemented without actually using the mechanism used by
> C++ for the target platform.

> **Note**: The Standard Library provides two mechanisms for recovering from a panic,
> [`catch_unwind`][fn-catch-unwind] (which enables recovery within the
> panicking thread) and [`JoinHandle::join`][thread-join] (which enables a process to
> continue execution without recovering the panicked thread).

## Panic runtimes

The actual behavior and implementation of `panic!` is controlled by the _panic
runtime_.

> **Note**: The Rust standard library provides two panic runtimes:
> `panic_unwind` (which unwinds the stack and is potentially recoverable) and
> `panic_abort` (which aborts the process and is non-recoverable). The default
> runtime depends on the target platform, but is generally `panic_unwind` on
> platforms with native support for C++ exceptions.

When compiling code that is guaranteed to be linked to a non-recoverable panic
runtime, the optimizer may assume that unwinding across Rust frames is
impossible, which can result in both code-size and runtime speed improvements.

See also the [`panic_handler` attribute](runtime.md#the-panic_handler-attribute) which can be used to change the behavior of panics.

[array indexing]: expressions/array-expr.md#array-and-slice-indexing-expressions
[destructors]: destructors.md
[fn-catch-unwind]: ../std/panic/fn.catch_unwind.html
[macro-panic]: ../std/macro.panic.html
[thread-join]: ../std/thread/struct.JoinHandle.html#method.join
[runtime]: runtime.md
