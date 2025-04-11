r[type.pointer]
# Pointer types

r[type.pointer.intro]
All pointers are explicit first-class values.
They can be moved or copied, stored into data structs, and returned from functions.

r[type.pointer.reference]
## References (`&` and `&mut`)

r[type.pointer.reference.syntax]
```grammar,types
ReferenceType -> `&` Lifetime? `mut`? TypeNoBounds
```

r[type.pointer.reference.shared]
### Shared references (`&`)

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

r[type.pointer.reference.mut]
### Mutable references (`&mut`)

r[type.pointer.reference.mut.intro]
Mutable references point to memory which is owned by some other value.
A mutable reference type is written `&mut type` or `&'a mut type`.

r[type.pointer.reference.mut.copy]
A mutable reference (that hasn't been borrowed) is the only way to access the value it points to, so is not `Copy`.

r[type.pointer.raw]
## Raw pointers (`*const` and `*mut`)

r[type.pointer.raw.syntax]
```grammar,types
RawPointerType -> `*` ( `mut` | `const` ) TypeNoBounds
```

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

r[type.pointer.smart]
## Smart Pointers

The standard library contains additional 'smart pointer' types beyond references and raw pointers.

r[type.pointer.validity]
## Bit validity

r[type.pointer.validity.pointer-fragment]
Despite pointers and references being similar to `usize`s in the machine code emitted on most platforms,
the semantics of transmuting a reference or pointer type to a non-pointer type is currently undecided.
Thus, it may not be valid to transmute a pointer or reference type, `P`, to a `[u8; size_of::<P>()]`.

r[type.pointer.validity.raw]
For thin raw pointers (i.e., for `P = *const T` or `P = *mut T` for `T: Sized`),
the inverse direction (transmuting from an integer or array of integers to `P`) is always valid.
However, the pointer produced via such a transmutation may not be dereferenced (not even if `T` has size zero).

[Interior mutability]: ../interior-mutability.md
[`unsafe` operation]: ../unsafety.md
[dynamically sized types]: ../dynamically-sized-types.md
[temporary value]: ../expressions.md#temporaries
