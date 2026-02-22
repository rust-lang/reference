r[items.struct]
# Structs

r[items.struct.syntax]
```grammar,items
Struct ->
      StructStruct
    | TupleStruct

StructStruct ->
    `struct` IDENTIFIER GenericParams? WhereClause? ( `{` StructFields? `}` | `;` )

TupleStruct ->
    `struct` IDENTIFIER GenericParams? `(` TupleFields? `)` WhereClause? `;`

StructFields -> StructField (`,` StructField)* `,`?

StructField -> OuterAttribute* Visibility? IDENTIFIER `:` Type
    StructFieldDefault?

StructFieldDefault -> `=` Expression

TupleFields -> TupleField (`,` TupleField)* `,`?

TupleField -> OuterAttribute* Visibility? Type
```

r[items.struct.intro]
A _struct_ is a nominal [struct type] defined with the keyword `struct`.

r[items.struct.namespace]
A struct declaration defines the given name in the [type namespace] of the module or block where it is located.

An example of a `struct` item and its use:

```rust
struct Point {x: i32, y: i32}
let p = Point {x: 10, y: 11};
let px: i32 = p.x;
```

r[items.struct.tuple]
A _tuple struct_ is a nominal [tuple type], and is also defined with the keyword `struct`. In addition to defining a type, it also defines a constructor of the same name in the [value namespace]. The constructor is a function which can be called to create a new instance of the struct. For example:

```rust
struct Point(i32, i32);
let p = Point(10, 11);
let px: i32 = match p { Point(x, _) => x };
```

r[items.struct.unit]
A _unit-like struct_ is a struct without any fields, defined by leaving off the list of fields entirely. Such a struct implicitly defines a [constant] of its type with the same name. For example:

```rust
struct Cookie;
let c = [Cookie, Cookie {}, Cookie, Cookie {}];
```

is equivalent to

```rust
struct Cookie {}
const Cookie: Cookie = Cookie {};
let c = [Cookie, Cookie {}, Cookie, Cookie {}];
```

r[items.struct.layout]
The precise memory layout of a struct is not specified. One can specify a particular layout using the [`repr` attribute].

r[items.struct.default]
## Default field values

r[items.struct.default.intro]
A field in a non-tuple struct can be assigned a default value, which can be used in a [struct expression] using the [default field syntax]:

```rust
struct Pet {
    name: Option<String>,
    age: i128 = 42,
}

let pet = Pet { name: None, .. };
assert_eq!(valid.age, 42);
```

r[items.struct.default.const]
A default field value must be a [constant expression]:

```rust,compile_fail
struct Pet {
    name: Option<String>,
    age: i128 = { println!("calculating age"); 42 },
    // ERROR: cannot call non-const function `_print` in constants
}
```

r[item.struct.default.derive]
The [derive macro] for the [`Default`] trait will use default field values in the implementation:

```rust
#[derive(Default)]
struct Pet {
    name: Option<String>, // impl Default for Pet will use Default::default() for name
    age: i128 = 42, // impl Default for Pet will use the literal 42 for age
}

let default = Pet::default();
assert_eq!(default.name, None);
assert_eq!(default.age, 42);
```

Any fields without a default field value must have an implementation of [`Default`], whose `default` method will be used for these fields instead.

[`repr` attribute]: ../type-layout.md#representations
[constant]: constant-items.md
[struct type]: ../types/struct.md
[struct expression]: ../expressions/struct-expr.md
[tuple type]: ../types/tuple.md
[type namespace]: ../names/namespaces.md
[value namespace]: ../names/namespaces.md
[constant expression]: ../const_eval.md
[derive macro]: ../procedural-macros.md#derive-macros
[default field syntax]: ../expressions/struct-expr.md#default-field-syntax
