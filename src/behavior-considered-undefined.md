## Behavior considered undefined

The following is a list of behavior which is forbidden in all Rust code,
including within `unsafe` blocks and `unsafe` functions. Type checking provides
the guarantee that these issues are never caused by safe code.

* Data races.
* Dereferencing a null or dangling raw pointer.
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
* Unwinding into Rust from foreign code or unwinding from Rust into foreign
  code. Rust's panic system is not compatible with exception handling in
  other languages. Unwinding must be caught and handled at FFI boundaries.

[noalias]: http://llvm.org/docs/LangRef.html#noalias
[pointer aliasing rules]: http://llvm.org/docs/LangRef.html#pointer-aliasing-rules
[undef]: http://llvm.org/docs/LangRef.html#undefined-values
[`offset`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.offset
[`std::ptr::copy_nonoverlapping_memory`]: https://doc.rust-lang.org/std/ptr/fn.copy_nonoverlapping.html
[`UnsafeCell<U>`]: https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html