# Textual types

r[type.text]

r[type.text.intro]
The types `char` and `str` hold textual data.

r[type.text.char-value]
A value of type `char` is a [Unicode scalar value] (i.e. a code point that is
not a surrogate), represented as a 32-bit unsigned word in the 0x0000 to 0xD7FF
or 0xE000 to 0x10FFFF range.

> [!NOTE]
> It is immediate [undefined behavior] to create a
> `char` that falls outside this range. A `[char]` is effectively a UCS-4 / UTF-32
> string of length 1.

r[type.text.char-repr]
A value of type `char` is represented as the value of type `u32` with value equal to the code point that it represents.

r[type.text.str-value]
A value of type `str` is represented the same way as `[u8]`, a slice of
8-bit unsigned bytes. However, the Rust standard library makes extra assumptions
about `str`: methods working on `str` assume and ensure that the data in there
is valid UTF-8. Calling a `str` method with a non-UTF-8 buffer can cause
[undefined behavior] now or in the future.

r[type.text.str-unsized]
Since `str` is a [dynamically sized type], it can only be instantiated through a
pointer type, such as `&str`.

[Unicode scalar value]: http://www.unicode.org/glossary/#unicode_scalar_value
[undefined behavior]: ../behavior-considered-undefined.md
[dynamically sized type]: ../dynamically-sized-types.md
