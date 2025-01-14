r[interior-mut]
# Interior Mutability

r[interior-mut.intro]
Sometimes a type needs to be mutated while having multiple aliases. In Rust this
is achieved using a pattern called _interior mutability_.

r[interior-mut.shared-ref]
A type has interior mutability if its internal state can be changed through a [shared reference] to
it.

r[interior-mut.no-constraint]
This goes against the usual [requirement][ub] that the value pointed to by a
shared reference is not mutated.

r[interior-mut.unsafe-cell]
[`std::cell::UnsafeCell<T>`] type is the only allowed way to disable
this requirement. When `UnsafeCell<T>` is immutably aliased, it is still safe to
mutate, or obtain a mutable reference to, the `T` it contains.

r[interior-mut.mut-unsafe-cell]
As with all other types, it is undefined behavior to have multiple `&mut UnsafeCell<T>`
aliases.

r[interior-mut.abstraction]
Other types with interior mutability can be created by using `UnsafeCell<T>` as
a field. The standard library provides a variety of types that provide safe
interior mutability APIs.

r[interior-mut.ref-cell]
For example, [`std::cell::RefCell<T>`] uses run-time borrow checks to ensure the usual rules around multiple references.

r[interior-mut.atomic]
The [`std::sync::atomic`] module contains types that wrap a value that is only
accessed with atomic operations, allowing the value to be shared and mutated
across threads.

[shared reference]: types/pointer.md#shared-references-
[ub]: behavior-considered-undefined.md
