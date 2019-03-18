# Limits

The following [attributes] affect compile-time limits.

## The `recursion_limit` attribute

The *`recursion_limit` attribute* may be applied at the crate level to set the
maximum depth for potentially infinitely-recursive compile-time operations
like auto-dereference or macro expansion. It uses the [_MetaNameValueStr_]
syntax to specify the recursion depth. The default is 64.

```rust,ignore
#![recursion_limit = "4"]

macro_rules! a {
    () => { a!(1) };
    (1) => { a!(2) };
    (2) => { a!(3) };
    (3) => { a!(4) };
    (4) => { };
}

// This fails to expand because it requires a recursion depth greater than 4.
a!{}
```

## The `type_length_limit` attribute

The *`type_length_limit` attribute* limits the maximum size of a type
constructed during monomorphization. It is applied at the crate level, and
uses the [_MetaNameValueStr_] syntax to set the limit based on the number of
type substitutions within the type. The default value is 1048576.

```rust,ignore
#![type_length_limit = "8"]

type A = (B, B, B);
type B = (C, C, C);
struct C;

// This fails to compile because monomorphizing to
// `drop::<Option<((C, C, C), (C, C, C), (C, C, C))>>` requires more than 8
// type elements.
drop::<Option<A>>(None);
```

[attributes]: attributes.html
[_MetaNameValueStr_]: attributes.html#meta-item-attribute-syntax
