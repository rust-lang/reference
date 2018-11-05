{{#include types-redirect.html}}
# Types

Every variable, item and value in a Rust program has a type. The _type_ of a
*value* defines the interpretation of the memory holding it and the operations
that may be performed on the value.

Built-in types are tightly integrated into the language, in nontrivial ways
that are not possible to emulate in user-defined types. User-defined types have
limited capabilities.

The list of types is:

* Primitive types:
    * [Boolean] — `true` or `false`
    * [Numeric] — integer and float
    * [Textual] — `char` and `str`
    * [Never] — `!` — a type with no values
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

## Type expressions

> **<sup>Syntax</sup>**\
> _Type_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; _TypeNoBounds_\
> &nbsp;&nbsp; | [_ImplTraitType_]\
> &nbsp;&nbsp; | [_TraitObjectType_]
>
> _TypeNoBounds_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; [_ParenthesizedType_]\
> &nbsp;&nbsp; | [_ImplTraitTypeOneBound_]\
> &nbsp;&nbsp; | [_TraitObjectTypeOneBound_]\
> &nbsp;&nbsp; | [_TypePath_]\
> &nbsp;&nbsp; | [_TupleType_]\
> &nbsp;&nbsp; | [_NeverType_]\
> &nbsp;&nbsp; | [_RawPointerType_]\
> &nbsp;&nbsp; | [_ReferenceType_]\
> &nbsp;&nbsp; | [_ArrayType_]\
> &nbsp;&nbsp; | [_SliceType_]\
> &nbsp;&nbsp; | [_InferredType_]\
> &nbsp;&nbsp; | [_QualifiedPathInType_]\
> &nbsp;&nbsp; | [_BareFunctionType_]\
> &nbsp;&nbsp; | [_MacroInvocation_]

A _type expression_ as defined in the _Type_ grammar rule above is the syntax
for referring to a type. It may refer to:

* Sequence types ([tuple], [array], [slice]).
* [Type paths] which can reference:
    * Primitive types ([boolean], [numeric], [textual]).
    * Paths to an [item] ([struct], [enum], [union], [type alias], [trait]).
    * [`Self` path] where `Self` is the implementing type.
    * Generic [type parameters].
* Pointer types ([reference], [raw pointer], [function pointer]).
* The [inferred type] which asks the compiler to determine the type.
* [Parentheses] which are used for disambiguation.
* Trait types: [Trait objects] and [impl trait].
* The [never] type.
* [Macros] which expand to a type expression.

### Parenthesized types

> _ParenthesizedType_ :\
> &nbsp;&nbsp; `(` [_Type_] `)`

In some situations the combination of types may be ambiguous. Use parentheses
around a type to avoid ambiguity. For example, the `+` operator for [type
boundaries] within a [reference type] is unclear where the
boundary applies, so the use of parentheses is required. Grammar rules that
require this disambiguation use the [_TypeNoBounds_] rule instead of
[_Type_].

```rust
# use std::any::Any;
type T<'a> = &'a (dyn Any + Send);
```

## Recursive types

Nominal types &mdash; [structs], [enumerations] and [unions] &mdash; may be
recursive. That is, each `enum` variant or `struct` or `union` field may
refer, directly or indirectly, to the enclosing `enum` or `struct` type
itself. Such recursion has restrictions:

* Recursive types must include a nominal type in the recursion (not mere [type
  aliases], or other structural types such as [arrays] or [tuples]). So `type
  Rec = &'static [Rec]` is not allowed.
* The size of a recursive type must be finite; in other words the recursive
  fields of the type must be [pointer types].
* Recursive type definitions can cross module boundaries, but not module
  *visibility* boundaries, or crate boundaries (in order to simplify the module
  system and type checker).

An example of a *recursive* type and its use:

```rust
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

let a: List<i32> = List::Cons(7, Box::new(List::Cons(13, Box::new(List::Nil))));
```

[_ArrayType_]: types/array.html
[_BareFunctionType_]: types/function-pointer.html
[_ImplTraitTypeOneBound_]: types/impl-trait.html
[_ImplTraitType_]: types/impl-trait.html
[_InferredType_]: types/inferred.html
[_MacroInvocation_]: macros.html#macro-invocation
[_NeverType_]: types/never.html
[_ParenthesizedType_]: types.html#parenthesized-types
[_QualifiedPathInType_]: paths.html#qualified-paths
[_RawPointerType_]: types/pointer.html#raw-pointers-const-and-mut
[_ReferenceType_]: types/pointer.html#shared-references-
[_SliceType_]: types/slice.html
[_TraitObjectTypeOneBound_]: types/trait-object.html
[_TraitObjectType_]: types/trait-object.html
[_TupleType_]: types/tuple.html#tuple-types
[_TypeNoBounds_]: types.html#type-expressions
[_TypePath_]: paths.html#paths-in-types
[_Type_]: types.html#type-expressions

[Array]: types/array.html
[Boolean]: types/boolean.html
[Closures]: types/closure.html
[Enum]: types/enum.html
[Function pointers]: types/function-pointer.html
[Functions]: types/function-item.html
[Impl trait]: types/impl-trait.html
[Macros]: macros.html
[Numeric]: types/numeric.html
[Parentheses]: #parenthesized-types
[Raw pointers]: types/pointer.html#raw-pointers-const-and-mut
[References]: types/pointer.html#shared-references-
[Slice]: types/slice.html
[Struct]: types/struct.html
[Textual]: types/textual.html
[Trait objects]: types/trait-object.html
[Tuple]: types/tuple.html
[Type paths]: paths.html#paths-in-types
[Union]: types/union.html
[`Self` path]: paths.html#self-1
[arrays]: types/array.html
[enumerations]: types/enum.html
[function pointer]: types/function-pointer.html
[inferred type]: types/inferred.html
[item]: items.html
[never]: types/never.html
[pointer types]: types/pointer.html
[raw pointer]: types/pointer.html#raw-pointers-const-and-mut
[reference type]: types/pointer.html#shared-references-
[reference]: types/pointer.html#shared-references-
[structs]: types/struct.html
[trait]: types/trait-object.html
[tuples]: types/tuple.html
[type alias]: items/type-aliases.html
[type aliases]: items/type-aliases.html
[type boundaries]: trait-bounds.html
[type parameters]: types/parameters.html
[unions]: types/union.html
