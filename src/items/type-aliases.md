# Type aliases

> **<sup>Syntax</sup>**\
> _TypeAlias_ :\
> &nbsp;&nbsp; `type` [IDENTIFIER]&nbsp;[_Generics_]<sup>?</sup>
>              [_WhereClause_]<sup>?</sup> `=` [_Type_] `;`

A _type alias_ defines a new name for an existing [type]. Type aliases are
declared with the keyword `type`. Every value has a single, specific type, but
may implement several different traits, or be compatible with several different
type constraints.

[type]: ../types.md

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

[IDENTIFIER]: ../identifiers.md
[_Generics_]: generics.md
[_WhereClause_]: generics.md#where-clauses
[_Type_]: ../types.md#type-expressions
