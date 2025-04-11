r[items.type]
# Type aliases

r[items.type.syntax]
```grammar,items
TypeAlias ->
    `type` IDENTIFIER GenericParams? ( `:` TypeParamBounds )?
        WhereClause?
        ( `=` Type WhereClause?)? `;`
```

r[items.type.intro]
A _type alias_ defines a new name for an existing [type] in the [type namespace] of the module or block where it is located.
Type aliases are declared with the keyword `type`.
Every value has a single, specific type, but may implement several different traits, and may be compatible with several different type constraints.

For example, the following defines the type `Point` as a synonym for the type
`(u8, u8)`, the type of pairs of unsigned 8 bit integers:

```rust
type Point = (u8, u8);
let p: Point = (41, 68);
```

r[items.type.constructor-alias]
A type alias to a tuple-struct or unit-struct cannot be used to qualify that type's constructor:

```rust,compile_fail
struct MyStruct(u32);

use MyStruct as UseAlias;
type TypeAlias = MyStruct;

let _ = UseAlias(5); // OK
let _ = TypeAlias(5); // Doesn't work
```

r[items.type.associated-type]
A type alias, when not used as an [associated type], must include a [Type][grammar-Type] and
may not include [TypeParamBounds].

r[items.type.associated-trait]
A type alias, when used as an [associated type] in a [trait], must not include a
[Type][grammar-Type] specification but may include [TypeParamBounds].

r[items.type.associated-impl]
A type alias, when used as an [associated type] in a [trait impl], must include
a [Type][grammar-Type] specification and may not include [TypeParamBounds].

r[items.type.deprecated]
Where clauses before the equals sign on a type alias in a [trait impl] (like
`type TypeAlias<T> where T: Foo = Bar<T>`) are deprecated. Where clauses after
the equals sign (like `type TypeAlias<T> = Bar<T> where T: Foo`) are preferred.

[associated type]: associated-items.md#associated-types
[trait impl]: implementations.md#trait-implementations
[trait]: traits.md
[type namespace]: ../names/namespaces.md
[type]: ../types.md
