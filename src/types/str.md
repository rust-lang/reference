r[type.str]
# String slice type

r[type.str.intro]
The string slice (`str`) type represents a sequence of characters.

```rust
let greeting1: &str = "Hello, world!";
let greeting2: &str = "你好，世界";
```

> [!NOTE]
> See [the standard library docs][`str`] for information on the impls of the `str` type.

r[type.str.value]
A value of type `str` is represented in the same way as `[u8]`, a slice of 8-bit unsigned bytes.

> [!NOTE]
> The standard library makes extra assumptions about `str`: methods working on `str` assume and ensure that the data it contains is valid UTF-8. Calling a `str` method with a non-UTF-8 buffer can cause [undefined behavior] now or in the future.

r[type.str.unsized]
A `str` is a [dynamically sized type]. It can only be instantiated through a pointer type, such as `&str`. The layout of `&str` is the same as the layout of `&[u8]`.

[undefined behavior]: ../behavior-considered-undefined.md
[dynamically sized type]: ../dynamically-sized-types.md
