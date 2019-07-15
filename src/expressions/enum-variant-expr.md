# Enumeration Variant expressions

> **<sup>Syntax</sup>**\
> _EnumerationVariantExpression_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; _EnumExprStruct_\
> &nbsp;&nbsp; | _EnumExprTuple_\
> &nbsp;&nbsp; | _EnumExprFieldless_
>
> _EnumExprStruct_ :\
> &nbsp;&nbsp; [_PathInExpression_] `{` _EnumExprFields_<sup>?</sup> `}`
>
> _EnumExprFields_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; _EnumExprField_ (`,` _EnumExprField_)<sup>\*</sup> `,`<sup>?</sup>
>
> _EnumExprField_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; [IDENTIFIER]\
> &nbsp;&nbsp; | ([IDENTIFIER] | [TUPLE_INDEX]) `:` [_Expression_]
>
> _EnumExprTuple_ :\
> &nbsp;&nbsp; [_PathInExpression_] `(`\
> &nbsp;&nbsp; &nbsp;&nbsp; ( [_Expression_] (`,` [_Expression_])<sup>\*</sup> `,`<sup>?</sup> )<sup>?</sup>\
> &nbsp;&nbsp; `)`
>
> _EnumExprFieldless_ : [_PathInExpression_]

Enumeration variants can be constructed similarly to [structs], using a path to an enum
variant instead of to a struct:

```rust
# enum Message {
#     Quit,
#     WriteString(String),
#     Move { x: i32, y: i32 },
# }
let q = Message::Quit;
let w = Message::WriteString("Some string".to_string());
let m = Message::Move { x: 50, y: 200 };
```

Enum variant expressions have the same syntax, behavior, and restrictions as [struct
expressions][structs], except they do not support base update with the `..` syntax.

[IDENTIFIER]: ../identifiers.md
[TUPLE_INDEX]: ../tokens.md#integer-literals
[_Expression_]: ../expressions.md
[_PathInExpression_]: ../paths.md#paths-in-expressions
[structs]: struct-expr.md
