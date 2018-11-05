# Textual types

The types `char` and `str` hold textual data.

A value of type `char` is a [Unicode scalar value] (i.e. a code point that
is not a surrogate), represented as a 32-bit unsigned word in the 0x0000 to
0xD7FF or 0xE000 to 0x10FFFF range. A `[char]` is effectively a UCS-4 / UTF-32
string.

A value of type `str` is a Unicode string, represented as an array of 8-bit
unsigned bytes holding a sequence of UTF-8 code points. Since `str` is a
[dynamically sized type], it is not a _first-class_ type, but can only be
instantiated through a pointer type, such as `&str`.

[Unicode scalar value]: http://www.unicode.org/glossary/#unicode_scalar_value
[dynamically sized type]: dynamically-sized-types.html
