r[dynamic-sized]
# Dynamically Sized Types

r[dynamic-sized.intro]
Most types have a fixed size that is known at compile time and implement the trait [`Sized`][sized]. A type with a size that is known only at run-time is called a _dynamically sized type_ (_DST_) or, informally, an unsized type.  [Slices], [trait objects], and [str] are examples of <abbr title="dynamically sized types">DSTs</abbr>.

r[dynamic-sized.restriction]
Such types can only be used in certain cases:

r[dynamic-sized.pointer-types]
* [Pointer types] to <abbr title="dynamically sized types">DSTs</abbr> are
  sized but have twice the size of pointers to sized types
    * Pointers to slices and `str` also store the number of elements.
    * Pointers to trait objects also store a pointer to a vtable.

r[dynamic-sized.question-sized]
* <abbr title="dynamically sized types">DSTs</abbr> can be provided as
  type arguments to generic type parameters having the special `?Sized` bound.
  They can also be used for associated type definitions when the corresponding associated type declaration has a `?Sized` bound.
  By default, any type parameter or associated type has a `Sized` bound, unless it is relaxed using `?Sized`.

r[dynamic-sized.trait-impl]
* Traits may be implemented for <abbr title="dynamically sized
  types">DSTs</abbr>.
  Unlike with generic type parameters, `Self: ?Sized` is the default in trait definitions.

r[dynamic-sized.struct-field]
* Structs may contain a <abbr title="dynamically sized type">DST</abbr> as the
  last field; this makes the struct itself a
  <abbr title="dynamically sized type">DST</abbr>.

> [!NOTE]
> [Variables], function parameters, [const] items, and [static] items must be `Sized`.

[sized]: special-types-and-traits.md#sized
[Slices]: types/slice.md
[str]: types/textual.md
[trait objects]: types/trait-object.md
[Pointer types]: types/pointer.md
[Variables]: variables.md
[const]: items/constant-items.md
[static]: items/static-items.md
