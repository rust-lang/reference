r[type.char]
# Character type

r[type.char.intro]
The `char` type represents a single [Unicode scalar value] (i.e., a code point that is not a surrogate).

> [!EXAMPLE]
> ```rust
> let c: char = 'a';
> let emoji: char = '😀';
> let unicode: char = '\u{1F600}';
> ```

> [!NOTE]
> See [the standard library docs][`char`] for information on the impls of the `char` type.

r[type.char.value]
A value of type `char` is represented as a 32-bit unsigned word in the 0x0000 to 0xD7FF or 0xE000 to 0x10FFFF range. It is immediate [undefined behavior] to create a `char` that falls outside this range.

r[type.char.layout]
`char` is guaranteed to have the same size and alignment as `u32` on all platforms.

r[type.char.validity]
Every byte of a `char` is guaranteed to be initialized. In other words, `transmute::<char, [u8; size_of::<char>()]>(...)` is always sound -- but since some bit patterns are invalid `char`s, the inverse is not always sound.

[Unicode scalar value]: http://www.unicode.org/glossary/#unicode_scalar_value
[undefined behavior]: ../behavior-considered-undefined.md
