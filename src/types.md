{{#include types-redirect.html}}
r[type]
# Types

r[type.intro]
Every variable, item, and value in a Rust program has a type. The _type_ of a
*value* defines the interpretation of the memory holding it and the operations
that may be performed on the value.

r[type.builtin]
Built-in types are tightly integrated into the language, in nontrivial ways
that are not possible to emulate in user-defined types.

r[type.user-defined]
User-defined types have limited capabilities.

r[type.kinds]
The list of types is:

* Primitive types:
    * [Boolean] --- `bool`
    * [Numeric] --- integer and float
    * [Textual] --- `char` and `str`
    * [Never] --- `!` --- a type with no values
* Sequence types:
    * [Tuple]
    * [Array]
    * [Slice]
* User-defined types:
    * [Struct]
    * [Enum]
    * [Union]
* Function types:
    * [Functions]
    * [Closures]
* Pointer types:
    * [References]
    * [Raw pointers]
    * [Function pointers]
* Trait types:
    * [Trait objects]
    * [Impl trait]

r[type.name]
## Type expressions

r[type.name.syntax]
```grammar,types
Type ->
      TypeNoBounds
    | ImplTraitType
    | TraitObjectType

TypeNoBounds ->
      ParenthesizedType
    | ImplTraitTypeOneBound
    | TraitObjectTypeOneBound
    | TypePath
    | TupleType
    | NeverType
    | RawPointerType
    | ReferenceType
    | ArrayType
    | SliceType
    | InferredType
    | QualifiedPathInType
    | BareFunctionType
    | MacroInvocation
```

r[type.name.intro]
A _type expression_ as defined in the [Type] grammar rule above is the syntax
for referring to a type. It may refer to:

r[type.name.sequence]
* Sequence types ([tuple], [array], [slice]).

r[type.name.path]
* [Type paths] which can reference:
    * Primitive types ([boolean], [numeric], [textual]).
    * Paths to an [item] ([struct], [enum], [union], [type alias], [trait]).
    * [`Self` path] where `Self` is the implementing type.
    * Generic [type parameters].

r[type.name.pointer]
* Pointer types ([reference], [raw pointer], [function pointer]).

r[type.name.inference]
* The [inferred type] which asks the compiler to determine the type.

r[type.name.grouped]
* [Parentheses] which are used for disambiguation.

r[type.name.trait]
* Trait types: [Trait objects] and [impl trait].

r[type.name.never]
* The [never] type.

r[type.name.macro-expansion]
* [Macros] which expand to a type expression.

r[type.name.parenthesized]
### Parenthesized types

r[type.name.parenthesized.syntax]
```grammar,types
ParenthesizedType -> `(` Type `)`
```

r[type.name.parenthesized.intro]
In some situations the combination of types may be ambiguous. Use parentheses
around a type to avoid ambiguity. For example, the `+` operator for [type
boundaries] within a [reference type] is unclear where the
boundary applies, so the use of parentheses is required. Grammar rules that
require this disambiguation use the [TypeNoBounds] rule instead of
[Type][grammar-Type].

```rust
# use std::any::Any;
type T<'a> = &'a (dyn Any + Send);
```

r[type.recursive]
## Recursive types

r[type.recursive.intro]
Nominal types &mdash; [structs], [enumerations], and [unions] &mdash; may be
recursive. That is, each `enum` variant or `struct` or `union` field may
refer, directly or indirectly, to the enclosing `enum` or `struct` type
itself.

r[type.recursive.constraint]
Such recursion has restrictions:

* Recursive types must include a nominal type in the recursion (not mere [type
  aliases], or other structural types such as [arrays] or [tuples]). So `type
  Rec = &'static [Rec]` is not allowed.
* The size of a recursive type must be finite; in other words the recursive
  fields of the type must be [pointer types].

An example of a *recursive* type and its use:

```rust
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

let a: List<i32> = List::Cons(7, Box::new(List::Cons(13, Box::new(List::Nil))));
```

[Array]: types/array.md
[Boolean]: types/boolean.md
[Closures]: types/closure.md
[Enum]: types/enum.md
[Function pointers]: types/function-pointer.md
[Functions]: types/function-item.md
[Impl trait]: types/impl-trait.md
[Macros]: macros.md
[Numeric]: types/numeric.md
[Parentheses]: #parenthesized-types
[Raw pointers]: types/pointer.md#raw-pointers-const-and-mut
[References]: types/pointer.md#shared-references-
[Slice]: types/slice.md
[Struct]: types/struct.md
[Textual]: types/textual.md
[Trait objects]: types/trait-object.md
[Tuple]: types/tuple.md
[Type paths]: paths.md#paths-in-types
[Union]: types/union.md
[`Self` path]: paths.md#self-1
[arrays]: types/array.md
[enumerations]: types/enum.md
[function pointer]: types/function-pointer.md
[inferred type]: types/inferred.md
[item]: items.md
[never]: types/never.md
[pointer types]: types/pointer.md
[raw pointer]: types/pointer.md#raw-pointers-const-and-mut
[reference type]: types/pointer.md#shared-references-
[reference]: types/pointer.md#shared-references-
[structs]: types/struct.md
[trait]: types/trait-object.md
[tuples]: types/tuple.md
[type alias]: items/type-aliases.md
[type aliases]: items/type-aliases.md
[type boundaries]: trait-bounds.md
[type parameters]: types/parameters.md
[unions]: types/union.md
