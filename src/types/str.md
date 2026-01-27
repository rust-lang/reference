r[type.str]
# String slice type

r[type.text.intro]
The types `char` and `str` hold textual data.

r[type.text.str-value]
A value of type `str` is represented the same way as `[u8]`, a slice of 8-bit unsigned bytes. However, the Rust standard library makes extra assumptions about `str`: methods working on `str` assume and ensure that the data in there is valid UTF-8. Calling a `str` method with a non-UTF-8 buffer can cause [undefined behavior] now or in the future.

r[type.text.str-unsized]
Since `str` is a [dynamically sized type], it can only be instantiated through a pointer type, such as `&str`. The layout of `&str` is the same as the layout of `&[u8]`.

[undefined behavior]: ../behavior-considered-undefined.md
[dynamically sized type]: ../dynamically-sized-types.md
