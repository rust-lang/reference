# Pointer types

r[type.pointer]

r[type.pointer.intro]
All pointers are explicit first-class values.
They can be moved or copied, stored into data structs, and returned from functions.

## References (`&` and `&mut`)

r[type.pointer.reference]

r[type.pointer.reference.syntax]
> **<sup>Syntax</sup>**\
> _ReferenceType_ :\
> &nbsp;&nbsp; `&` [_Lifetime_]<sup>?</sup> `mut`<sup>?</sup> [_TypeNoBounds_]

### Shared references (`&`)

r[type.pointer.reference.shared]

r[type.pointer.reference.shared.intro]
Shared references point to memory which is owned by some other value.

r[type.pointer.reference.shared.constraint-mutation]
When a shared reference to a value is created, it prevents direct mutation of the value.
[Interior mutability] provides an exception for this in certain circumstances.
As the name suggests, any number of shared references to a value may exist.
A shared reference type is written `&type`, or `&'a type` when you need to specify an explicit lifetime.

r[type.pointer.reference.shared.copy]
Copying a reference is a "shallow" operation:
it involves only copying the pointer itself, that is, pointers are `Copy`.
Releasing a reference has no effect on the value it points to, but referencing of a [temporary value] will keep it alive during the scope of the reference itself.

### Mutable references (`&mut`)

r[type.pointer.reference.mut]

r[type.pointer.reference.mut.intro]
Mutable references point to memory which is owned by some other value.
A mutable reference type is written `&mut type` or `&'a mut type`.

r[type.pointer.reference.mut.copy]
A mutable reference (that hasn't been borrowed) is the only way to access the value it points to, so is not `Copy`.

## Raw pointers (`*const` and `*mut`)

r[type.pointer.raw]

r[type.pointer.raw.syntax]
> **<sup>Syntax</sup>**\
> _RawPointerType_ :\
> &nbsp;&nbsp; `*` ( `mut` | `const` ) [_TypeNoBounds_]

r[type.pointer.raw.intro]
Raw pointers are pointers without safety or liveness guarantees.
Raw pointers are written as `*const T` or `*mut T`.
For example `*const i32` means a raw pointer to a 32-bit integer.

r[type.pointer.raw.copy]
Copying or dropping a raw pointer has no effect on the lifecycle of any other value.

r[type.pointer.raw.safety]
Dereferencing a raw pointer is an [`unsafe` operation].

This can also be used to convert a raw pointer to a reference by reborrowing it (`&*` or `&mut *`).
Raw pointers are generally discouraged;
they exist to support interoperability with foreign code, and writing performance-critical or low-level functions.

r[type.pointer.raw.cmp]
When comparing raw pointers they are compared by their address, rather than by what they point to.
When comparing raw pointers to [dynamically sized types] they also have their additional data compared.

r[type.pointer.raw.constructor]
Raw pointers can be created directly using `&raw const` for `*const` pointers and `&raw mut` for `*mut` pointers.

## Smart Pointers

r[type.pointer.smart]

The standard library contains additional 'smart pointer' types beyond references and raw pointers.

## Pointer values and representation

r[type.pointer.value]

r[type.pointer.value.thin]
Each thin pointer consists of an address and an optional [provenance][type.pointer.provenance]. The address refers to which byte the pointer points to. The provenance refers to which bytes the pointer is allowed to access, and the allocation those bytes are within.

> [!NOTE]
> A pointer that does not have a provenance may be called an invalid or dangling pointer.

r[type.pointer.value.thin-repr]
The representation of a value of a thin pointer is a sequence of initialized bytes with `u8` values given by the representation of its address as a value of type `usize`, and pointer fragments corresponding to its provenance, if present.

r[type.pointer.value.thin-ref]
A thin reference to `T` consists of a non-null, well aligned address, and provenance for `size_of::<T>()` bytes starting from that address. The representation of a thin reference to `T` is the same as the pointer with the same address and provenance.

> [!NOTE]
> This is true for both shared and mutable references. There are additional constraints enforced by the aliasing model that are not yet fully decided.

r[type.pointer.value.wide]
A wide pointer or reference consists of a data pointer or reference, and a pointee-specific metadata value.

r[type.pointer.value.wide-reference]
The data pointer of a wide reference has a non-null address, well aligned for `align_of_val(self)`, and with provenance for `size_of_val(self)` bytes.

r[type.pointer.value.wide-representation]
A wide pointer or reference is represented the same as `struct WidePointer<M>{data: *mut (), metadata: M}` where `M` is the pointee metadata type, and the `data` and `metadata` fields are the corresponding parts of the pointer.

> [!NOTE]
> The `WidePointer` struct has no guarantees about layout, and has the default representation.

## Pointer Provenance

r[type.pointer.provenance]

r[type.pointer.provenance.intro]
Pointer Provenance is a term that refers to additional data carried by pointer values in Rust, beyond its address. When stored in memory, Provenance is encoded in the Pointer Fragment part of each byte of the pointer.

r[type.pointer.provenance.allocation]
Whenever a pointer to a particular allocation is produced by using the reference or raw reference operators, or when a pointer is returned from an allocation function, the resulting pointer has provenance that refers to that allocation.

> [!NOTE]
> There is additional information encoded by provenance, but the exact scope of this information is not yet decided.

r[type.pointer.provenance.dangling]
A pointer is dangling if it has no provenance, or if it has provenance to an allocation that has since been deallocated. An access, except for an access of size zero, using a dangling pointer, is undefined behavior.

> [!NOTE]
> Allocations include local and static variables, as well as temporaries. Local Variables and Temporaries are deallocated when they go out of scope.

> [!WARN]
> The above is necessary, but not sufficient, to avoid undefined behavior. The full requirements for pointer access is not yet decided.
> A reference obtained in safe code is guaranteed to be valid for its usable lifetime, unless interfered with by unsafe code.

[Interior mutability]: ../interior-mutability.md
[_Lifetime_]: ../trait-bounds.md
[_TypeNoBounds_]: ../types.md#type-expressions
[`unsafe` operation]: ../unsafety.md
[dynamically sized types]: ../dynamically-sized-types.md
[temporary value]: ../expressions.md#temporaries
