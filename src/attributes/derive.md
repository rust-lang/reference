# Derive

The *`derive` attribute* allows new [items] to be automatically generated for
data structures. It uses the [_MetaListPaths_] syntax to specify a list of
traits to implement or paths to [derive macros] to process.

For example, the following will create an [`impl` item] for the
[`PartialEq`] and [`Clone`] traits for `Foo`, and the type parameter `T` will be
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

[_MetaListPaths_]: attributes.html#meta-item-attribute-syntax
[`Clone`]: ../std/clone/trait.Clone.html
[`PartialEq`]: ../std/cmp/trait.PartialEq.html
[`impl` item]: items/implementations.html
[items]: items.html
[derive macros]: procedural-macros.html#derive-macros
[procedural macros]: procedural-macros.html#derive-macros
