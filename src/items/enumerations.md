# Enumerations

> **<sup>Syntax</sup>**  
> _Enumeration_ :  
> &nbsp;&nbsp; `enum`
>    [IDENTIFIER]&nbsp;
>    [_Generics_]<sup>?</sup>
>    [_WhereClause_]<sup>?</sup>
>    `{` _EnumItems_<sup>?</sup> `}`  
>  
> _EnumItems_ :  
> &nbsp;&nbsp; _EnumItem_ ( `,` _EnumItem_ )<sup>\*</sup> `,`<sup>?</sup>  
>  
> _EnumItem_ :  
> &nbsp;&nbsp; _OuterAttribute_<sup>\*</sup>  
> &nbsp;&nbsp; [IDENTIFIER]&nbsp;( _EnumItemTuple_ | _EnumItemStruct_ 
>                                | _EnumItemDiscriminant_ )<sup>?</sup>  
>  
> _EnumItemTuple_ :  
> &nbsp;&nbsp; `(` [_TupleFields_]<sup>?</sup> `)`  
>   
> _EnumItemStruct_ :  
> &nbsp;&nbsp; `{` [_StructFields_]<sup>?</sup> `}`  
>   
> _EnumItemDiscriminant_ :  
> &nbsp;&nbsp; `=` [_Expression_]  

An _enumeration_ is a simultaneous definition of a nominal [enumerated type] as
well as a set of *constructors*, that can be used to create or pattern-match
values of the corresponding enumerated type.

[enumerated type]: types.html#enumerated-types

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

Enumeration constructors can have either named or unnamed fields:

```rust
enum Animal {
    Dog (String, f64),
    Cat { name: String, weight: f64 },
}

let mut a: Animal = Animal::Dog("Cocoa".to_string(), 37.2);
a = Animal::Cat { name: "Spotty".to_string(), weight: 2.7 };
```

In this example, `Cat` is a _struct-like enum variant_, whereas `Dog` is simply
called an enum variant. Each enum instance has a _discriminant_ which is an
integer associated to it that is used to determine which variant it holds.

## C-like Enumerations

If there is no data attached to *any* of the variants of an enumeration it is
called a *c-like enumeration*. If a discriminant isn't specified, they start at
zero, and add one for each variant, in order. Each enum value is just its
discriminant which you can specify explicitly:

```rust
enum Foo {
    Bar,            // 0
    Baz = 123,
    Quux,           // 124
}
```

The right hand side of the specification is interpreted as an `isize` value,
but the compiler is allowed to use a smaller type in the actual memory layout.
The [`repr` attribute] can be added in order to change the type of the right
hand side and specify the memory layout.

[`repr` attribute]: attributes.html#ffi-attributes

You can also cast a c-like enum to get its discriminant:

```rust
# enum Foo { Baz = 123 }
let x = Foo::Baz as u32; // x is now 123u32
```

This only works as long as none of the variants have data attached. If it were
`Baz(i32)`, this is disallowed.

[IDENTIFIER]: identifiers.html
[_Generics_]: items.html#type-parameters
[_WhereClause_]: items.html#type-parameters
[_Expression_]: expressions.html
[_TupleFields_]: items/structs.html
[_StructFields_]: items/structs.html
