# Infer Outlives Requirements

This feature allows some outlives requirements to be
inferred by the compiler rather than having to state
them explicitly.

## Motivation

In the following piece of code, we have the outlives
relation `T: 'a`; which can be read as T outlives 'a.
However if we look at the definition of the struct we
see that the field `bar` is a reference to T with a lifetime
of 'a. Thus for this code to be valid, the compiler already
enforces the predicate `T: 'a`.

With this feature the compiler will also infer the outlives
requirements so that the where-statement becomes optional.

```rust,ignore (pseudo-Rust)
struct Foo<'a, T>
  where T: 'a // <-- now optional
  {
      bar: &'a T,
  }
```


For more more information on the motivation and design, please
see RFC [#44493].

[#44493]: https://github.com/rust-lang/rust/issues/44493


## Examples

Code speakers louder than words so here are a few examples
of the feature inferring outlives requirements.

Struct, Enum and Union with *reference* to generic parameter:
```rust,ignore (pseudo-Rust)
// infer T: 'a
struct Foo<'a, T> {
    bar: &'a T,
}
```

```rust,ignore (pseudo-Rust)
// infer T: 'a
union Foo<'a, T> {
    bar: &'a T,
}
```

```rust,ignore (pseudo-Rust)
// infer T: 'a
enum Foo<'a, T> {
  Bar<'a, T>
}
struct Bar<'a, T> {
  field: &'a T
}
```

Struct, Enum and Union with an *explicit outlives clause* on
the nested type:
```rust,ignore (pseudo-Rust)
// infer `T: 'b`
struct Foo<'a, T> {
    bar: Bar<'a, T>
}
// contains explicit where clause
struct Bar<'b, K> where K: 'b {
    x: &'b (),
    y: K,
}
```

Struct, Enum and Union with *lifetime outlives requirements*:
```rust,ignore (pseudo-Rust)
// infer `b': 'a`
struct Foo<'a, 'b, T> {
    x: &'a &'b T
}
```

*Associated types*:
```rust,ignore (pseudo-Rust)
// infer `<T as std::iter::Iterator>::Item : 'a`
struct Foo<'a, T: Iterator> {
    bar: &'a T::Item
```

```rust,ignore (pseudo-Rust)
// infer B: 'a
struct Foo<'a, A, B> where A: Trait<'b, K> {
  field: <A as Trait<'a, B>>::Type
}
trait Bar<'b, K> where K: 'b {
  type Type
}
```

*Trait objects*:
```rust,ignore (pseudo-Rust)
// infer `T': 'a`
struct Foo<'a, T> {
    field: Box<dyn Bar<'a, T>>
}
// contains explicit where clause
trait Bar<'b, K> where K: 'b {
}
```

