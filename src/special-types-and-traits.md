r[lang-types]
# Special types and traits

r[lang-types.intro]
Certain types and traits that exist in [the standard library] are known to the
Rust compiler. This chapter documents the special features of these types and
traits.

r[lang-types.box]
## `Box<T>`

r[lang-types.box.intro]
[`Box<T>`] has a few special features that Rust doesn't currently allow for user
defined types.

r[lang-types.box.deref]
* The [dereference operator] for `Box<T>` produces a place which can be moved
  from. This means that the `*` operator and the destructor of `Box<T>` are
  built-in to the language.

r[lang-types.box.receiver]
* [Methods] can take `Box<Self>` as a receiver.

r[lang-types.box.fundamental]
* A trait may be implemented for `Box<T>` in the same crate as `T`, which the
  [orphan rules] prevent for other generic types.

<!-- Editor Note: This is nowhere close to an exhaustive list -->

r[lang-types.rc]
## `Rc<T>`

r[lang-types.rc.receiver]
[Methods] can take [`Rc<Self>`] as a receiver.

r[lang-types.arc]
## `Arc<T>`

r[lang-types.arc.receiver]
[Methods] can take [`Arc<Self>`] as a receiver.

r[lang-types.pin]
## `Pin<P>`

r[lang-types.pin.receiver]
[Methods] can take [`Pin<P>`] as a receiver.

r[lang-types.unsafe-cell]
## `UnsafeCell<T>`

r[lang-types.unsafe-cell.interior-mut]
[`std::cell::UnsafeCell<T>`] is used for [interior mutability]. It ensures that
the compiler doesn't perform optimisations that are incorrect for such types.

r[lang-types.unsafe-cell.read-only-alloc]
It also ensures that [`static` items] which have a type with interior
mutability aren't placed in memory marked as read only.

r[lang-types.phantom-data]
## `PhantomData<T>`

[`std::marker::PhantomData<T>`] is a zero-sized, minimum alignment, type that
is considered to own a `T` for the purposes of [variance], [drop check], and
[auto traits](#auto-traits).

r[lang-types.ops]
## Operator Traits

The traits in [`std::ops`] and [`std::cmp`] are used to overload [operators],
[indexing expressions], and [call expressions].

r[lang-types.deref]
## `Deref` and `DerefMut`

As well as overloading the unary `*` operator, [`Deref`] and [`DerefMut`] are
also used in [method resolution] and [deref coercions].

r[lang-types.drop]
## `Drop`

The [`Drop`] trait provides a [destructor], to be run whenever a value of this
type is to be destroyed.

r[lang-types.copy]
## `Copy`

r[lang-types.copy.intro]
The [`Copy`] trait changes the semantics of a type implementing it.

r[lang-types.copy.behavior]
Values whose type implements `Copy` are copied rather than moved upon assignment.

r[lang-types.copy.constraint]
`Copy` can only be implemented for types which do not implement `Drop`, and whose fields are all `Copy`.
For enums, this means all fields of all variants have to be `Copy`.
For unions, this means all variants have to be `Copy`.

r[lang-types.copy.builtin-types]
`Copy` is implemented by the compiler for

r[lang-types.copy.tuple]
* [Tuples] of `Copy` types

r[lang-types.copy.fn-pointer]
* [Function pointers]

r[lang-types.copy.fn-item]
* [Function items]

r[lang-types.copy.closure]
* [Closures] that capture no values or that only capture values of `Copy` types

r[lang-types.clone]
## `Clone`

r[lang-types.clone.intro]
The [`Clone`] trait is a supertrait of `Copy`, so it also needs compiler
generated implementations.

r[lang-types.clone.builtin-types]
It is implemented by the compiler for the following types:

r[lang-types.clone.builtin-copy]
* Types with a built-in `Copy` implementation (see above)

r[lang-types.clone.tuple]
* [Tuples] of `Clone` types

r[lang-types.clone.closure]
* [Closures] that only capture values of `Clone` types or capture no values from the environment

r[lang-types.send]
## `Send`

The [`Send`] trait indicates that a value of this type is safe to send from one
thread to another.

r[lang-types.sync]
## `Sync`

r[lang-types.sync.intro]
The [`Sync`] trait indicates that a value of this type is safe to share between
multiple threads.

r[lang-types.sync.static-constraint]
This trait must be implemented for all types used in immutable [`static` items].

r[lang-types.termination]
## `Termination`

The [`Termination`] trait indicates the acceptable return types for the [main function] and [test functions].

r[lang-types.auto-traits]
## Auto traits

The [`Send`], [`Sync`], [`Unpin`], [`UnwindSafe`], and [`RefUnwindSafe`] traits are _auto
traits_. Auto traits have special properties.

r[lang-types.auto-traits.auto-impl]
If no explicit implementation or negative implementation is written out for an
auto trait for a given type, then the compiler implements it automatically
according to the following rules:

r[lang-types.auto-traits.builtin-composite]
* `&T`, `&mut T`, `*const T`, `*mut T`, `[T; n]`, and `[T]` implement the trait
  if `T` does.

r[lang-types.auto-traits.fn-item-pointer]
* Function item types and function pointers automatically implement the trait.

r[lang-types.auto-traits.aggregate]
* Structs, enums, unions, and tuples implement the trait if all of their fields
  do.

r[lang-types.auto-traits.closure]
* Closures implement the trait if the types of all of their captures do. A
  closure that captures a `T` by shared reference and a `U` by value implements
  any auto traits that both `&T` and `U` do.

r[lang-types.auto-traits.generic-impl]
For generic types (counting the built-in types above as generic over `T`), if a
generic implementation is available, then the compiler does not automatically
implement it for types that could use the implementation except that they do not
meet the requisite trait bounds. For instance, the standard library implements
`Send` for all `&T` where `T` is `Sync`; this means that the compiler will not
implement `Send` for `&T` if `T` is `Send` but not `Sync`.

r[lang-types.auto-traits.negative]
Auto traits can also have negative implementations, shown as `impl !AutoTrait
for T` in the standard library documentation, that override the automatic
implementations. For example `*mut T` has a negative implementation of `Send`,
and so `*mut T` is not `Send`, even if `T` is. There is currently no stable way
to specify additional negative implementations; they exist only in the standard
library.

r[lang-types.auto-traits.trait-object-marker]
Auto traits may be added as an additional bound to any [trait object], even
though normally only one trait is allowed. For instance, `Box<dyn Debug + Send +
UnwindSafe>` is a valid type.

r[lang-types.sized]
## `Sized`

r[lang-types.sized.intro]
The [`Sized`] trait indicates that the size of this type is known at compile-time; that is, it's not a [dynamically sized type].

r[lang-types.sized.implicit-sized]
[Type parameters] (except `Self` in traits) are `Sized` by default, as are [associated types].

r[lang-types.sized.implicit-impl]
`Sized` is always implemented automatically by the compiler, not by [implementation items].

r[lang-types.sized.relaxation]
These implicit `Sized` bounds may be relaxed by using the special `?Sized` bound.

[`Arc<Self>`]: std::sync::Arc
[`Deref`]: std::ops::Deref
[`DerefMut`]: std::ops::DerefMut
[`Pin<P>`]: std::pin::Pin
[`Rc<Self>`]: std::rc::Rc
[`RefUnwindSafe`]: std::panic::RefUnwindSafe
[`Termination`]: std::process::Termination
[`UnwindSafe`]: std::panic::UnwindSafe
[`Unpin`]: std::marker::Unpin

[Arrays]: types/array.md
[associated types]: items/associated-items.md#associated-types
[call expressions]: expressions/call-expr.md
[deref coercions]: type-coercions.md#coercion-types
[dereference operator]: expressions/operator-expr.md#the-dereference-operator
[destructor]: destructors.md
[drop check]: ../nomicon/dropck.html
[dynamically sized type]: dynamically-sized-types.md
[Function pointers]: types/function-pointer.md
[Function items]: types/function-item.md
[implementation items]: items/implementations.md
[indexing expressions]: expressions/array-expr.md#array-and-slice-indexing-expressions
[interior mutability]: interior-mutability.md
[main function]: crates-and-source-files.md#main-functions
[Methods]: items/associated-items.md#associated-functions-and-methods
[method resolution]: expressions/method-call-expr.md
[operators]: expressions/operator-expr.md
[orphan rules]: items/implementations.md#trait-implementation-coherence
[`static` items]: items/static-items.md
[test functions]: attributes/testing.md#the-test-attribute
[the standard library]: std
[trait object]: types/trait-object.md
[Tuples]: types/tuple.md
[Type parameters]: types/parameters.md
[variance]: subtyping.md#variance
[Closures]: types/closure.md
