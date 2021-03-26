# Dynamically Sized Types

Most types have a fixed size that is known at compile time and implement the
trait [`Sized`][sized]. A type with a size that is known only at run-time is
called a _dynamically sized type_ (_DST_) or, informally, an unsized type.
[Slices] and [trait objects] are two examples of <abbr title="dynamically sized
types">DSTs</abbr>. Such types can only be used in certain cases:

* [Pointer types] to <abbr title="dynamically sized types">DSTs</abbr> are
  sized but have twice the size of pointers to sized types
    * Pointers to slices also store the number of elements of the slice.
    * Pointers to trait objects also store a pointer to a vtable.
* <abbr title="dynamically sized types">DSTs</abbr> can be provided as
  type arguments when the type parameter has the special lifetime bound [`?Sized`](trait-bounds.html#sized) specified. (In other words by default any type parameter on a non-trait item
  has a `Sized` bound, and specifying the `?Sized` bound removes it.)
* Traits may be implemented for <abbr title="dynamically sized
  types">DSTs</abbr>. Unlike type parameters on other kinds of items, `Self: ?Sized` is the default in trait
  definitions.
* Structs may contain a <abbr title="dynamically sized type">DST</abbr> as the
  last field, this makes the struct itself a
  <abbr title="dynamically sized type">DST</abbr>.

> **Note**: [variables], function parameters, [const] items, and [static] items must be
`Sized`.

[sized]: special-types-and-traits.md#sized
[Slices]: types/slice.md
[trait objects]: types/trait-object.md
[Pointer types]: types/pointer.md
[variables]: variables.md
[const]: items/constant-items.md
[static]: items/static-items.md
