# Enumerations

> **<sup>Syntax</sup>**\
> _Enumeration_ :\
> &nbsp;&nbsp; `enum`
>    [IDENTIFIER]&nbsp;
>    [_Generics_]<sup>?</sup>
>    [_WhereClause_]<sup>?</sup>
>    `{` _EnumItems_<sup>?</sup> `}`
>
> _EnumItems_ :\
> &nbsp;&nbsp; _EnumItem_ ( `,` _EnumItem_ )<sup>\*</sup> `,`<sup>?</sup>
>
> _EnumItem_ :\
> &nbsp;&nbsp; _OuterAttribute_<sup>\*</sup>\
> &nbsp;&nbsp; [IDENTIFIER]&nbsp;( _EnumItemTuple_ | _EnumItemStruct_
>                                | _EnumItemDiscriminant_ )<sup>?</sup>
>
> _EnumItemTuple_ :\
> &nbsp;&nbsp; `(` [_TupleFields_]<sup>?</sup> `)`
>
> _EnumItemStruct_ :\
> &nbsp;&nbsp; `{` [_StructFields_]<sup>?</sup> `}`
>
> _EnumItemDiscriminant_ :\
> &nbsp;&nbsp; `=` [_Expression_]

An *enumeration*, also referred to as *enum* is a simultaneous definition of a
nominal [enumerated type] as well as a set of *constructors*, that can be used
to create or pattern-match values of the corresponding enumerated type.

Enumerations are declared with the keyword `enum`.

An example of an `enum` item and its use:

```rust
enum Animal {
    Dog,
    Cat,
}

let mut a: Animal = Animal::Dog;
a = Animal::Cat;
```

Enum constructors can have either named or unnamed fields:

```rust
enum Animal {
    Dog(String, f64),
    Cat { name: String, weight: f64 },
}

let mut a: Animal = Animal::Dog("Cocoa".to_string(), 37.2);
a = Animal::Cat { name: "Spotty".to_string(), weight: 2.7 };
```

In this example, `Cat` is a _struct-like enum variant_, whereas `Dog` is simply
called an enum variant. Each enum instance has a _discriminant_ which is an
integer associated to it that is used to determine which variant it holds. An
opaque reference to this discriminant can be obtained with the
[`mem::discriminant`] function.

## Custom Discriminant Values for Field-Less Enumerations

If there is no data attached to *any* of the variants of an enumeration,
then the discriminant can be directly chosen and accessed.

These enumerations can be cast to integer types with the `as` operator by a
[numeric cast]. The enumeration can optionally specify which integer each
discriminant gets by following the variant name with `=` followed by a [constant
expression]. If the first variant in the declaration is unspecified, then it is
set to zero. For every other unspecified discriminant, it is set to one higher
than the previous variant in the declaration.

```rust
enum Foo {
    Bar,            // 0
    Baz = 123,      // 123
    Quux,           // 124
}

let baz_discriminant = Foo::Baz as u32;
assert_eq!(baz_discriminant, 123);
```

Under the [default representation], the specified discriminant is interpreted as
an `isize` value although the compiler is allowed to use a smaller type in the
actual memory layout. The size and thus acceptable values can be changed by
using a [primitive representation] or the [`C` representation].

It is an error when two variants share the same discriminant.

```rust,ignore
enum SharedDiscriminantError {
    SharedA = 1,
    SharedB = 1
}

enum SharedDiscriminantError2 {
    Zero,       // 0
    One,        // 1
    OneToo = 1  // 1 (collision with previous!)
}
```

It is also an error to have an unspecified discriminant where the previous
discriminant is the maximum value for the size of the discriminant.

```rust,ignore
#[repr(u8)]
enum OverflowingDiscriminantError {
    Max = 255,
    MaxPlusOne // Would be 256, but that overflows the enum.
}

#[repr(u8)]
enum OverflowingDiscriminantError2 {
    MaxMinusOne = 254, // 254
    Max,               // 255
    MaxPlusOne         // Would be 256, but that overflows the enum.
}
```

## Zero-variant Enums

Enums with zero variants are known as *zero-variant enums*. As they have
no valid values, they cannot be instantiated.

```rust
enum ZeroVariants {}
```

[IDENTIFIER]: identifiers.html
[_Generics_]: items/generics.html
[_WhereClause_]: items/generics.html#where-clauses
[_Expression_]: expressions.html
[_TupleFields_]: items/structs.html
[_StructFields_]: items/structs.html
[enumerated type]: types/enum.html
[`mem::discriminant`]: ../std/mem/fn.discriminant.html
[numeric cast]: expressions/operator-expr.html#semantics
[`repr` attribute]: attributes.html#ffi-attributes
[constant expression]: const_eval.html#constant-expressions
[default representation]: type-layout.html#the-default-representation
[primitive representation]: type-layout.html#primitive-representations
[`C` representation]: type-layout.html#the-c-representation
