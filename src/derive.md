# Derive

<!-- TODO: Maybe link to https://doc.rust-lang.org/book/appendix-03-derivable-traits.html ?
Should the reference say more about which traits are derivable?
-->

The *`derive` attribute* allows certain traits to be automatically implemented
for data structures. It uses the [_MetaListPaths_] syntax to specify a list of
traits to implement.

For example, the following will create an `impl` for the
`PartialEq` and `Clone` traits for `Foo`, and the type parameter `T` will be
given the `PartialEq` or `Clone` constraints for the appropriate `impl`:

```rust
#[derive(PartialEq, Clone)]
struct Foo<T> {
    a: i32,
    b: T,
}
```

The generated `impl` for `PartialEq` is equivalent to

```rust
# struct Foo<T> { a: i32, b: T }
impl<T: PartialEq> PartialEq for Foo<T> {
    fn eq(&self, other: &Foo<T>) -> bool {
        self.a == other.a && self.b == other.b
    }

    fn ne(&self, other: &Foo<T>) -> bool {
        self.a != other.a || self.b != other.b
    }
}
```

You can implement `derive` for your own traits through [procedural macros].

<!-- TODO: automatically_derived -->

[_MetaListPaths_]: attributes.html#meta-item-attribute-syntax
[procedural macros]: procedural-macros.html
