## Behavior considered undefined

Rust code, including within `unsafe` blocks and `unsafe` functions is incorrect
if it exhibits any of the behaviors in the following list. It is the
programmer's responsibility when writing `unsafe` code that it is not possible
to let `safe` code exhibit these behaviors.

<div class="warning">

***Warning:*** The following list is not exhaustive. There is no formal model of
Rust's semantics for what is and is not allowed in unsafe code, so there may be
more behavior considered unsafe. The following list is just what we know for
sure is undefined behavior. Please read the [Rustonomicon] before writing unsafe
code.

</div>

* Data races.
* Dereferencing a null or dangling raw pointer.
* Unaligned pointer reading and writing outside of [`read_unaligned`]
  and [`write_unaligned`].
* Reads of [undef] \(uninitialized) memory.
* Breaking the [pointer aliasing rules] on accesses through raw pointers;
  a subset of the rules used by C.
* `&mut T` and `&T` follow LLVM’s scoped [noalias] model, except if the `&T`
  contains an [`UnsafeCell<U>`].
* Mutating non-mutable data &mdash; that is, data reached through a shared
  reference or data owned by a `let` binding), unless that data is contained
  within an [`UnsafeCell<U>`].
* Invoking undefined behavior via compiler intrinsics:
  * Indexing outside of the bounds of an object with [`offset`] with
    the exception of one byte past the end of the object.
  * Using [`std::ptr::copy_nonoverlapping_memory`], a.k.a. the `memcpy32`and
    `memcpy64` intrinsics, on overlapping buffers.
* Invalid values in primitive types, even in private fields and locals:
  * Dangling or null references and boxes.
  * A value other than `false` (`0`) or `true` (`1`) in a `bool`.
  * A discriminant in an `enum` not included in the type definition.
  * A value in a `char` which is a surrogate or above `char::MAX`.
  * Non-UTF-8 byte sequences in a `str`.
* Executing code compiled with platform features that the current platform
  does not support (see [`target_feature`]).

> **Note**: Undefined behavior affects the entire program. For example, calling
> a function in C that exhibits undefined behavior of C means your entire
> program contains undefined behaviour that can also affect the Rust code. And
> vice versa, undefined behavior in Rust can cause adverse affects on code
> executed by any FFI calls to other languages.

[noalias]: http://llvm.org/docs/LangRef.html#noalias
[pointer aliasing rules]: http://llvm.org/docs/LangRef.html#pointer-aliasing-rules
[undef]: http://llvm.org/docs/LangRef.html#undefined-values
[`offset`]: ../std/primitive.pointer.html#method.offset
[`std::ptr::copy_nonoverlapping_memory`]: ../std/ptr/fn.copy_nonoverlapping.html
[`target_feature`]: attributes/codegen.html#the-target_feature-attribute
[`UnsafeCell<U>`]: ../std/cell/struct.UnsafeCell.html
[`read_unaligned`]: ../std/ptr/fn.read_unaligned.html
[`write_unaligned`]: ../std/ptr/fn.write_unaligned.html
[Rustonomicon]: ../nomicon/index.html
