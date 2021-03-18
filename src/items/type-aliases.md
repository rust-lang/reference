
# Type aliases

> **<sup>Syntax</sup>**\
> _TypeAlias_ :\
> &nbsp;&nbsp; `type` [IDENTIFIER]&nbsp;[_GenericParams_]<sup>?</sup>
>              [_WhereClause_]<sup>?</sup> ( `=` [_Type_] ) `;`

A _type alias_ defines a new name for an existing [type]. Type aliases are
declared with the keyword `type`. Every value has a single, specific type, but
may implement several different traits, or be compatible with several different
type constraints.

For example, the following defines the type `Point` as a synonym for the type
`(u8, u8)`, the type of pairs of unsigned 8 bit integers:

```rust
type Point = (u8, u8);
let p: Point = (41, 68);
```

A type alias to a tuple-struct or unit-struct cannot be used to qualify that type's constructor:

```rust,edition2018,compile_fail
struct MyStruct(u32);

use MyStruct as UseAlias;
type TypeAlias = MyStruct;

let _ = UseAlias(5); // OK
let _ = TypeAlias(5); // Doesn't work
```

A type alias without the [_Type_] specification may only appear as an
[associated type] in a [trait].

A type alias to an enum cannot refer to the enum's variants within a [use declaration]:

```rust,edition2018,compile_fail
mod my_mod {
    pub enum MyEnum {
        MyVariant
    }

    pub type TypeAlias = MyEnum;
}

use my_mod::MyEnum; // OK
use my_mod::MyEnum::MyVariant; // OK
use my_mod::TypeAlias; // OK
use my_mod::TypeAlias::MyVariant; // Doesn't work

let _ = my_mod::TypeAlias::MyVariant; // OK
```

[IDENTIFIER]: ../identifiers.md
[_GenericParams_]: generics.md
[_WhereClause_]: generics.md#where-clauses
[_Type_]: ../types.md#type-expressions
[associated type]: associated-items.md#associated-types
[trait]: traits.md
[type]: ../types.md
[use declaration]: use-declarations.md
