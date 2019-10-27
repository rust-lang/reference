## What is coherence and why do we care?

Coherence means that for any given trait and type, there is one specific 
implementation that applies. This is important for Rust to be easy to reason 
about. When you write `<Foo as Bar>::trait_method`, the compiler needs to know
what actual implementation to use.

In languages without coherence, the compiler has to have some way to choose
which implementation to use when multiple implementations could apply. Scala
does this by having complex scope resolution rules for "implicit" parameters.
Haskell (when a discouraged flag is enabled) does this by picking an impl
arbitrarily.

Rust's solution is to enforce that there is only one impl to choose from at all.
While the rules required to enforce this are quite complex, the result is easy
to reason about, and is generally considered to be quite important for Rust.
New features like specialization allow more than one impl to apply, but for any
given type and trait, there will always be exactly one which is most specific,
and deterministically be chosen.

An important piece of enforcing coherence is restricting "orphan impls". An impl
is orphaned if it is implementing a trait you don't own for a type you don't
own. Rust's rules around this balance two separate, but related goals:

- Ensuring that two crates can't write impls that would overlap (e.g. no crate
  other than `std` can write `impl From<usize> for Vec<i32>`. If they could,
  your program might stop compiling just by using two crates with an overlapping
  impl).
- Restricting the impls that can be written so crates can add implementations
  for traits/types they do own without worrying about breaking downstream
  crates.


## Definitions

Local Trait: A trait which was defined in the current crate. Whether a trait is
local or not has nothing to do with type parameters. Given `trait Foo<T, U>`,
`Foo` is always local, regardless of the types used for `T` or `U`.

Local Type: A struct, enum, or union which was defined in the current crate.
This is not affected by type parameters. `struct Foo` is considered local, but
`Vec<Foo>` is not. `LocalType<ForeignType>` is local. Type aliases and trait
aliases do not affect locality.

Covered Type: A type which appears as a parameter to another type. For example,
`T` is uncovered, but the `T` in `Vec<T>` is covered. This is only relevant for
type parameters.

Blanket Impl: Any implementation where a type appears uncovered. `impl<T> Foo
for T`, `impl<T> Bar<T> for T`, `impl<T> Bar<Vec<T>> for T`, and `impl<T> Bar<T>
for Vec<T>` are considered blanket impls. However, `impl<T> Bar<Vec<T>> for
Vec<T>` is not a blanket impl, as all instances of `T` which appear in this impl
are covered by `Vec`.

Fundamental Type: A type for which you cannot add a blanket impl backwards
compatibly. This includes `&`, `&mut`, and `Box`. Any time a type `T` is
considered local, `&T`, `&mut T`, and `Box<T>` are also considered local.
Fundamental types cannot cover other types. Any time the term "covered type" is
used, the `T` in `&T`, `&mut T`, and `Box<T>` is not considered covered.


## Concrete orphan rules

Assumes the same definitions [as above](#definitions).

Given `impl<P1..=Pn> Trait<T1..=Tn> for T0`, an impl is valid only if at
least one of the following is true:

- `Trait` is a local trait
- All of
  - At least one of the types `T0..=Tn` must be a local type. Let `Ti` be the
    first such type.
  - No uncovered type parameters `P1..=Pn` may appear in `T0..Ti` (excluding
    `Ti`)

We only restrict the appearance of *uncovered* type parameters. Once again, it is
important to note that for the purposes of coherence, `#[fundamental]` types are
special. The `T` in `Box<T>` is not considered covered, and `Box<LocalType>` 
is considered local.
