r[lex.token]
# Tokens

r[lex.token.syntax]
```grammar,lexer
Token ->
      IDENTIFIER_OR_KEYWORD
    | RAW_IDENTIFIER
    | CHAR_LITERAL
    | STRING_LITERAL
    | RAW_STRING_LITERAL
    | BYTE_LITERAL
    | BYTE_STRING_LITERAL
    | RAW_BYTE_STRING_LITERAL
    | C_STRING_LITERAL
    | RAW_C_STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | LIFETIME_TOKEN
    | PUNCTUATION
    | RESERVED_TOKEN
```

r[lex.token.intro]
Tokens are primitive productions in the grammar defined by regular
(non-recursive) languages.  Rust source input can be broken down
into the following kinds of tokens:

* [Keywords]
* [Identifiers][identifier]
* [Literals](#literals)
* [Lifetimes](#lifetimes-and-loop-labels)
* [Punctuation](#punctuation)
* [Delimiters](#delimiters)

Within this documentation's grammar, "simple" tokens are given in [string
table production] form, and appear in `monospace` font.

[string table production]: notation.md#string-table-productions

r[lex.token.literal]
## Literals

Literals are tokens used in [literal expressions].

### Examples

#### Characters and strings

|                                              | Example         | `#`&nbsp;sets[^nsets] | Characters  | Escapes             |
|----------------------------------------------|-----------------|------------|-------------|---------------------|
| [Character](#character-literals)             | `'H'`           | 0          | All Unicode | [Quote](#quote-escapes) & [ASCII](#ascii-escapes) & [Unicode](#unicode-escapes) |
| [String](#string-literals)                   | `"hello"`       | 0          | All Unicode | [Quote](#quote-escapes) & [ASCII](#ascii-escapes) & [Unicode](#unicode-escapes) |
| [Raw string](#raw-string-literals)           | `r#"hello"#`    | <256       | All Unicode | `N/A`                                                      |
| [Byte](#byte-literals)                       | `b'H'`          | 0          | All ASCII   | [Quote](#quote-escapes) & [Byte](#byte-escapes)                               |
| [Byte string](#byte-string-literals)         | `b"hello"`      | 0          | All ASCII   | [Quote](#quote-escapes) & [Byte](#byte-escapes)                               |
| [Raw byte string](#raw-byte-string-literals) | `br#"hello"#`   | <256       | All ASCII   | `N/A`                                                      |
| [C string](#c-string-literals)               | `c"hello"`      | 0          | All Unicode | [Quote](#quote-escapes) & [Byte](#byte-escapes) & [Unicode](#unicode-escapes)   |
| [Raw C string](#raw-c-string-literals)       | `cr#"hello"#`   | <256       | All Unicode | `N/A`                                                                           |

[^nsets]: The number of `#`s on each side of the same literal must be equivalent.


#### ASCII escapes

|   | Name |
|---|------|
| `\x41` | 7-bit character code (exactly 2 digits, up to 0x7F) |
| `\n` | Newline |
| `\r` | Carriage return |
| `\t` | Tab |
| `\\` | Backslash |
| `\0` | Null |

#### Byte escapes

|   | Name |
|---|------|
| `\x7F` | 8-bit character code (exactly 2 digits) |
| `\n` | Newline |
| `\r` | Carriage return |
| `\t` | Tab |
| `\\` | Backslash |
| `\0` | Null |

#### Unicode escapes

|   | Name |
|---|------|
| `\u{7FFF}` | 24-bit Unicode character code (up to 6 digits) |

#### Quote escapes

|   | Name |
|---|------|
| `\'` | Single quote |
| `\"` | Double quote |

#### Numbers

| [Number literals](#number-literals)[^nl] | Example | Exponentiation |
|----------------------------------------|---------|----------------|
| Decimal integer | `98_222` | `N/A` |
| Hex integer | `0xff` | `N/A` |
| Octal integer | `0o77` | `N/A` |
| Binary integer | `0b1111_0000` | `N/A` |
| Floating-point | `123.0E+77` | `Optional` |

[^nl]: All number literals allow `_` as a visual separator: `1_234.0E+18f64`

r[lex.token.literal.suffix]
#### Suffixes

r[lex.token.literal.literal.suffix.intro]
A suffix is a sequence of characters following the primary part of a literal (without intervening whitespace), of the same form as a non-raw identifier or keyword.

r[lex.token.literal.suffix.syntax]
```grammar,lexer
SUFFIX -> IDENTIFIER_OR_KEYWORD

SUFFIX_NO_E -> SUFFIX _not beginning with `e` or `E`_
```

r[lex.token.literal.suffix.validity]
Any kind of literal (string, integer, etc) with any suffix is valid as a token.

A literal token with any suffix can be passed to a macro without producing an error.
The macro itself will decide how to interpret such a token and whether to produce an error or not.
In particular, the `literal` fragment specifier for by-example macros matches literal tokens with arbitrary suffixes.

```rust
macro_rules! blackhole { ($tt:tt) => () }
macro_rules! blackhole_lit { ($l:literal) => () }

blackhole!("string"suffix); // OK
blackhole_lit!(1suffix); // OK
```

r[lex.token.literal.suffix.parse]
However, suffixes on literal tokens which are interpreted as literal expressions or patterns are restricted.
Any suffixes are rejected on non-numeric literal tokens,
and numeric literal tokens are accepted only with suffixes from the list below.

| Integer | Floating-point |
|---------|----------------|
| `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `usize`, `isize` | `f32`, `f64` |

### Character and string literals

r[lex.token.literal.char]
#### Character literals

r[lex.token.literal.char.syntax]
```grammar,lexer
CHAR_LITERAL ->
    `'`
        ( ~[`'` `\` LF CR TAB] | QUOTE_ESCAPE | ASCII_ESCAPE | UNICODE_ESCAPE )
    `'` SUFFIX?

QUOTE_ESCAPE -> `\'` | `\"`

ASCII_ESCAPE ->
      `\x` OCT_DIGIT HEX_DIGIT
    | `\n` | `\r` | `\t` | `\\` | `\0`

UNICODE_ESCAPE ->
    `\u{` ( HEX_DIGIT `_`* ){1..6} `}`
```

r[lex.token.literal.char.intro]
A _character literal_ is a single Unicode character enclosed within two
`U+0027` (single-quote) characters, with the exception of `U+0027` itself,
which must be _escaped_ by a preceding `U+005C` character (`\`).

r[lex.token.literal.str]
#### String literals

r[lex.token.literal.str.syntax]
```grammar,lexer
STRING_LITERAL ->
    `"` (
        ~[`"` `\` CR]
      | QUOTE_ESCAPE
      | ASCII_ESCAPE
      | UNICODE_ESCAPE
      | STRING_CONTINUE
    )* `"` SUFFIX?

STRING_CONTINUE -> `\` LF
```

r[lex.token.literal.str.intro]
A _string literal_ is a sequence of any Unicode characters enclosed within two
`U+0022` (double-quote) characters, with the exception of `U+0022` itself,
which must be _escaped_ by a preceding `U+005C` character (`\`).

r[lex.token.literal.str.linefeed]
Line-breaks, represented by the  character `U+000A` (LF), are allowed in string literals.
The character `U+000D` (CR) may not appear in a string literal.
When an unescaped `U+005C` character (`\`) occurs immediately before a line break, the line break does not appear in the string represented by the token.
See [String continuation escapes] for details.

r[lex.token.literal.char-escape]
#### Character escapes

r[lex.token.literal.char-escape.intro]
Some additional _escapes_ are available in either character or non-raw string
literals. An escape starts with a `U+005C` (`\`) and continues with one of the
following forms:

r[lex.token.literal.char-escape.ascii]
* A _7-bit code point escape_ starts with `U+0078` (`x`) and is
  followed by exactly two _hex digits_ with value up to `0x7F`. It denotes the
  ASCII character with value equal to the provided hex value. Higher values are
  not permitted because it is ambiguous whether they mean Unicode code points or
  byte values.

r[lex.token.literal.char-escape.unicode]
* A _24-bit code point escape_ starts with `U+0075` (`u`) and is followed
  by up to six _hex digits_ surrounded by braces `U+007B` (`{`) and `U+007D`
  (`}`). It denotes the Unicode code point equal to the provided hex value.

r[lex.token.literal.char-escape.whitespace]
* A _whitespace escape_ is one of the characters `U+006E` (`n`), `U+0072`
  (`r`), or `U+0074` (`t`), denoting the Unicode values `U+000A` (LF),
  `U+000D` (CR) or `U+0009` (HT) respectively.

r[lex.token.literal.char-escape.null]
* The _null escape_ is the character `U+0030` (`0`) and denotes the Unicode
  value `U+0000` (NUL).

r[lex.token.literal.char-escape.slash]
* The _backslash escape_ is the character `U+005C` (`\`) which must be
  escaped in order to denote itself.

r[lex.token.literal.str-raw]
#### Raw string literals

r[lex.token.literal.str-raw.syntax]
```grammar,lexer
RAW_STRING_LITERAL -> `r` RAW_STRING_CONTENT SUFFIX?

RAW_STRING_CONTENT ->
      `"` ( ~CR )*? `"`
    | `#` RAW_STRING_CONTENT `#`
```

r[lex.token.literal.str-raw.intro]
Raw string literals do not process any escapes. They start with the character
`U+0072` (`r`), followed by fewer than 256 of the character `U+0023` (`#`) and a
`U+0022` (double-quote) character.

r[lex.token.literal.str-raw.body]
The _raw string body_ can contain any sequence of Unicode characters other than `U+000D` (CR).
It is terminated only by another `U+0022` (double-quote) character, followed by the same number of `U+0023` (`#`) characters that preceded the opening `U+0022` (double-quote) character.

r[lex.token.literal.str-raw.content]
All Unicode characters contained in the raw string body represent themselves,
the characters `U+0022` (double-quote) (except when followed by at least as
many `U+0023` (`#`) characters as were used to start the raw string literal) or
`U+005C` (`\`) do not have any special meaning.

Examples for string literals:

```rust
"foo"; r"foo";                     // foo
"\"foo\""; r#""foo""#;             // "foo"

"foo #\"# bar";
r##"foo #"# bar"##;                // foo #"# bar

"\x52"; "R"; r"R";                 // R
"\\x52"; r"\x52";                  // \x52
```

### Byte and byte string literals

r[lex.token.byte]
#### Byte literals

r[lex.token.byte.syntax]
```grammar,lexer
BYTE_LITERAL ->
    `b'` ( ASCII_FOR_CHAR | BYTE_ESCAPE )  `'` SUFFIX?

ASCII_FOR_CHAR ->
    <any ASCII (i.e. 0x00 to 0x7F) except `'`, `\`, LF, CR, or TAB>

BYTE_ESCAPE ->
      `\x` HEX_DIGIT HEX_DIGIT
    | `\n` | `\r` | `\t` | `\\` | `\0` | `\'` | `\"`
```

r[lex.token.byte.intro]
A _byte literal_ is a single ASCII character (in the `U+0000` to `U+007F`
range) or a single _escape_ preceded by the characters `U+0062` (`b`) and
`U+0027` (single-quote), and followed by the character `U+0027`. If the character
`U+0027` is present within the literal, it must be _escaped_ by a preceding
`U+005C` (`\`) character. It is equivalent to a `u8` unsigned 8-bit integer
_number literal_.

r[lex.token.str-byte]
#### Byte string literals

r[lex.token.str-byte.syntax]
```grammar,lexer
BYTE_STRING_LITERAL ->
    `b"` ( ASCII_FOR_STRING | BYTE_ESCAPE | STRING_CONTINUE )* `"` SUFFIX?

ASCII_FOR_STRING ->
    <any ASCII (i.e 0x00 to 0x7F) except `"`, `\`, or CR>
```

r[lex.token.str-byte.intro]
A non-raw _byte string literal_ is a sequence of ASCII characters and _escapes_,
preceded by the characters `U+0062` (`b`) and `U+0022` (double-quote), and
followed by the character `U+0022`. If the character `U+0022` is present within
the literal, it must be _escaped_ by a preceding `U+005C` (`\`) character.
Alternatively, a byte string literal can be a _raw byte string literal_, defined
below.

r[lex.token.str-byte.linefeed]
Line-breaks, represented by the  character `U+000A` (LF), are allowed in byte string literals.
The character `U+000D` (CR) may not appear in a byte string literal.
When an unescaped `U+005C` character (`\`) occurs immediately before a line break, the line break does not appear in the string represented by the token.
See [String continuation escapes] for details.

r[lex.token.str-byte.escape]
Some additional _escapes_ are available in either byte or non-raw byte string
literals. An escape starts with a `U+005C` (`\`) and continues with one of the
following forms:

r[lex.token.str-byte.escape-byte]
* A _byte escape_ escape starts with `U+0078` (`x`) and is
  followed by exactly two _hex digits_. It denotes the byte
  equal to the provided hex value.

r[lex.token.str-byte.escape-whitespace]
* A _whitespace escape_ is one of the characters `U+006E` (`n`), `U+0072`
  (`r`), or `U+0074` (`t`), denoting the bytes values `0x0A` (ASCII LF),
  `0x0D` (ASCII CR) or `0x09` (ASCII HT) respectively.

r[lex.token.str-byte.escape-null]
* The _null escape_ is the character `U+0030` (`0`) and denotes the byte
  value `0x00` (ASCII NUL).

r[lex.token.str-byte.escape-slash]
* The _backslash escape_ is the character `U+005C` (`\`) which must be
  escaped in order to denote its ASCII encoding `0x5C`.

r[lex.token.str-byte-raw]
#### Raw byte string literals

r[lex.token.str-byte-raw.syntax]
```grammar,lexer
RAW_BYTE_STRING_LITERAL ->
    `br` RAW_BYTE_STRING_CONTENT SUFFIX?

RAW_BYTE_STRING_CONTENT ->
      `"` ASCII_FOR_RAW*? `"`
    | `#` RAW_BYTE_STRING_CONTENT `#`

ASCII_FOR_RAW ->
    <any ASCII (i.e. 0x00 to 0x7F) except CR>
```

r[lex.token.str-byte-raw.intro]
Raw byte string literals do not process any escapes. They start with the
character `U+0062` (`b`), followed by `U+0072` (`r`), followed by fewer than 256
of the character `U+0023` (`#`), and a `U+0022` (double-quote) character.

r[lex.token.str-byte-raw.body]
The _raw string body_ can contain any sequence of ASCII characters other than `U+000D` (CR).
It is terminated only by another `U+0022` (double-quote) character, followed by the same number of `U+0023` (`#`) characters that preceded the opening `U+0022` (double-quote) character.
A raw byte string literal can not contain any non-ASCII byte.

r[lex.token.literal.str-byte-raw.content]
All characters contained in the raw string body represent their ASCII encoding,
the characters `U+0022` (double-quote) (except when followed by at least as
many `U+0023` (`#`) characters as were used to start the raw string literal) or
`U+005C` (`\`) do not have any special meaning.

Examples for byte string literals:

```rust
b"foo"; br"foo";                     // foo
b"\"foo\""; br#""foo""#;             // "foo"

b"foo #\"# bar";
br##"foo #"# bar"##;                 // foo #"# bar

b"\x52"; b"R"; br"R";                // R
b"\\x52"; br"\x52";                  // \x52
```

### C string and raw C string literals

r[lex.token.str-c]
#### C string literals

r[lex.token.str-c.syntax]
```grammar,lexer
C_STRING_LITERAL ->
    `c"` (
        ~[`"` `\` CR NUL]
      | BYTE_ESCAPE _except `\0` or `\x00`_
      | UNICODE_ESCAPE _except `\u{0}`, `\u{00}`, …, `\u{000000}`_
      | STRING_CONTINUE
    )* `"` SUFFIX?

```

r[lex.token.str-c.intro]
A _C string literal_ is a sequence of Unicode characters and _escapes_,
preceded by the characters `U+0063` (`c`) and `U+0022` (double-quote), and
followed by the character `U+0022`. If the character `U+0022` is present within
the literal, it must be _escaped_ by a preceding `U+005C` (`\`) character.
Alternatively, a C string literal can be a _raw C string literal_, defined below.

[CStr]: core::ffi::CStr

r[lex.token.str-c.null]
C strings are implicitly terminated by byte `0x00`, so the C string literal
`c""` is equivalent to manually constructing a `&CStr` from the byte string
literal `b"\x00"`. Other than the implicit terminator, byte `0x00` is not
permitted within a C string.

r[lex.token.str-c.linefeed]
Line-breaks, represented by the  character `U+000A` (LF), are allowed in C string literals.
The character `U+000D` (CR) may not appear in a C string literal.
When an unescaped `U+005C` character (`\`) occurs immediately before a line break, the line break does not appear in the string represented by the token.
See [String continuation escapes] for details.

r[lex.token.str-c.escape]
Some additional _escapes_ are available in non-raw C string literals. An escape
starts with a `U+005C` (`\`) and continues with one of the following forms:

r[lex.token.str-c.escape-byte]
* A _byte escape_ escape starts with `U+0078` (`x`) and is followed by exactly
  two _hex digits_. It denotes the byte equal to the provided hex value.

r[lex.token.str-c.escape-unicode]
* A _24-bit code point escape_ starts with `U+0075` (`u`) and is followed
  by up to six _hex digits_ surrounded by braces `U+007B` (`{`) and `U+007D`
  (`}`). It denotes the Unicode code point equal to the provided hex value,
  encoded as UTF-8.

r[lex.token.str-c.escape-whitespace]
* A _whitespace escape_ is one of the characters `U+006E` (`n`), `U+0072`
  (`r`), or `U+0074` (`t`), denoting the bytes values `0x0A` (ASCII LF),
  `0x0D` (ASCII CR) or `0x09` (ASCII HT) respectively.

r[lex.token.str-c.escape-slash]
* The _backslash escape_ is the character `U+005C` (`\`) which must be
  escaped in order to denote its ASCII encoding `0x5C`.

r[lex.token.str-c.char-unicode]
A C string represents bytes with no defined encoding, but a C string literal
may contain Unicode characters above `U+007F`. Such characters will be replaced
with the bytes of that character's UTF-8 representation.

The following C string literals are equivalent:

```rust
c"æ";        // LATIN SMALL LETTER AE (U+00E6)
c"\u{00E6}";
c"\xC3\xA6";
```

r[lex.token.str-c.edition2021]
> [!EDITION-2021]
> C string literals are accepted in the 2021 edition or later. In earlier additions the token `c""` is lexed as `c ""`.

r[lex.token.str-c-raw]
#### Raw C string literals

r[lex.token.str-c-raw.syntax]
```grammar,lexer
RAW_C_STRING_LITERAL ->
    `cr` RAW_C_STRING_CONTENT SUFFIX?

RAW_C_STRING_CONTENT ->
      `"` ( ~[CR NUL] )*? `"`
    | `#` RAW_C_STRING_CONTENT `#`
```

r[lex.token.str-c-raw.intro]
Raw C string literals do not process any escapes. They start with the
character `U+0063` (`c`), followed by `U+0072` (`r`), followed by fewer than 256
of the character `U+0023` (`#`), and a `U+0022` (double-quote) character.

r[lex.token.str-c-raw.body]
The _raw C string body_ can contain any sequence of Unicode characters other than `U+0000` (NUL) and `U+000D` (CR).
It is terminated only by another `U+0022` (double-quote) character, followed by the same number of `U+0023` (`#`) characters that preceded the opening `U+0022` (double-quote) character.

r[lex.token.str-c-raw.content]
All characters contained in the raw C string body represent themselves in UTF-8
encoding. The characters `U+0022` (double-quote) (except when followed by at
least as many `U+0023` (`#`) characters as were used to start the raw C string
literal) or `U+005C` (`\`) do not have any special meaning.

r[lex.token.str-c-raw.edition2021]
> [!EDITION-2021]
> Raw C string literals are accepted in the 2021 edition or later. In earlier additions the token `cr""` is lexed as `cr ""`, and `cr#""#` is lexed as `cr #""#` (which is non-grammatical).

#### Examples for C string and raw C string literals

```rust
c"foo"; cr"foo";                     // foo
c"\"foo\""; cr#""foo""#;             // "foo"

c"foo #\"# bar";
cr##"foo #"# bar"##;                 // foo #"# bar

c"\x52"; c"R"; cr"R";                // R
c"\\x52"; cr"\x52";                  // \x52
```

r[lex.token.literal.num]
### Number literals

A _number literal_ is either an _integer literal_ or a _floating-point
literal_. The grammar for recognizing the two kinds of literals is mixed.

r[lex.token.literal.int]
#### Integer literals

r[lex.token.literal.int.syntax]
```grammar,lexer
INTEGER_LITERAL ->
    ( DEC_LITERAL | BIN_LITERAL | OCT_LITERAL | HEX_LITERAL ) SUFFIX_NO_E?

DEC_LITERAL -> DEC_DIGIT (DEC_DIGIT|`_`)*

BIN_LITERAL -> `0b` (BIN_DIGIT|`_`)* BIN_DIGIT (BIN_DIGIT|`_`)*

OCT_LITERAL -> `0o` (OCT_DIGIT|`_`)* OCT_DIGIT (OCT_DIGIT|`_`)*

HEX_LITERAL -> `0x` (HEX_DIGIT|`_`)* HEX_DIGIT (HEX_DIGIT|`_`)*

BIN_DIGIT -> [`0`-`1`]

OCT_DIGIT -> [`0`-`7`]

DEC_DIGIT -> [`0`-`9`]

HEX_DIGIT -> [`0`-`9` `a`-`f` `A`-`F`]
```

r[lex.token.literal.int.kind]
An _integer literal_ has one of four forms:

r[lex.token.literal.int.kind-dec]
* A _decimal literal_ starts with a *decimal digit* and continues with any
  mixture of *decimal digits* and _underscores_.

r[lex.token.literal.int.kind-hex]
* A _hex literal_ starts with the character sequence `U+0030` `U+0078`
  (`0x`) and continues as any mixture (with at least one digit) of hex digits
  and underscores.

r[lex.token.literal.int.kind-oct]
* An _octal literal_ starts with the character sequence `U+0030` `U+006F`
  (`0o`) and continues as any mixture (with at least one digit) of octal digits
  and underscores.

r[lex.token.literal.int.kind-bin]
* A _binary literal_ starts with the character sequence `U+0030` `U+0062`
  (`0b`) and continues as any mixture (with at least one digit) of binary digits
  and underscores.

r[lex.token.literal.int.restriction]
Like any literal, an integer literal may be followed (immediately, without any spaces) by a suffix as described above.
The suffix may not begin with `e` or `E`, as that would be interpreted as the exponent of a floating-point literal.
See [Integer literal expressions] for the effect of these suffixes.

Examples of integer literals which are accepted as literal expressions:

```rust
# #![allow(overflowing_literals)]
123;
123i32;
123u32;
123_u32;

0xff;
0xff_u8;
0x01_f32; // integer 7986, not floating-point 1.0
0x01_e3;  // integer 483, not floating-point 1000.0

0o70;
0o70_i16;

0b1111_1111_1001_0000;
0b1111_1111_1001_0000i64;
0b________1;

0usize;

// These are too big for their type, but are accepted as literal expressions.
128_i8;
256_u8;

// This is an integer literal, accepted as a floating-point literal expression.
5f32;
```

Note that `-1i8`, for example, is analyzed as two tokens: `-` followed by `1i8`.

Examples of integer literals which are not accepted as literal expressions:

```rust
# #[cfg(false)] {
0invalidSuffix;
123AFB43;
0b010a;
0xAB_CD_EF_GH;
0b1111_f32;
# }
```

r[lex.token.literal.int.tuple-field]
#### Tuple index

r[lex.token.literal.int.tuple-field.syntax]
```grammar,lexer
TUPLE_INDEX -> INTEGER_LITERAL
```

r[lex.token.literal.int.tuple-field.intro]
A tuple index is used to refer to the fields of [tuples], [tuple structs], and
[tuple variants].

r[lex.token.literal.int.tuple-field.eq]
Tuple indices are compared with the literal token directly. Tuple indices
start with `0` and each successive index increments the value by `1` as a
decimal value. Thus, only decimal values will match, and the value must not
have any extra `0` prefix characters.

```rust,compile_fail
let example = ("dog", "cat", "horse");
let dog = example.0;
let cat = example.1;
// The following examples are invalid.
let cat = example.01;  // ERROR no field named `01`
let horse = example.0b10;  // ERROR no field named `0b10`
```

> [!NOTE]
> Tuple indices may include certain suffixes, but this is not intended to be valid, and may be removed in a future version. See <https://github.com/rust-lang/rust/issues/60210> for more information.

r[lex.token.literal.float]
#### Floating-point literals

r[lex.token.literal.float.syntax]
```grammar,lexer
FLOAT_LITERAL ->
      DEC_LITERAL `.` _not immediately followed by `.`, `_` or an XID_Start character_
    | DEC_LITERAL `.` DEC_LITERAL SUFFIX_NO_E?
    | DEC_LITERAL (`.` DEC_LITERAL)? FLOAT_EXPONENT SUFFIX?

FLOAT_EXPONENT ->
    (`e`|`E`) (`+`|`-`)? (DEC_DIGIT|`_`)* DEC_DIGIT (DEC_DIGIT|`_`)*
```

r[lex.token.literal.float.form]
A _floating-point literal_ has one of two forms:

* A _decimal literal_ followed by a period character `U+002E` (`.`). This is
  optionally followed by another decimal literal, with an optional _exponent_.
* A single _decimal literal_ followed by an _exponent_.

r[lex.token.literal.float.suffix]
Like integer literals, a floating-point literal may be followed by a
suffix, so long as the pre-suffix part does not end with `U+002E` (`.`).
The suffix may not begin with `e` or `E` if the literal does not include an exponent.
See [Floating-point literal expressions] for the effect of these suffixes.

Examples of floating-point literals which are accepted as literal expressions:

```rust
123.0f64;
0.1f64;
0.1f32;
12E+99_f64;
let x: f64 = 2.;
```

This last example is different because it is not possible to use the suffix
syntax with a floating point literal end.token.ing in a period. `2.f64` would attempt
to call a method named `f64` on `2`.

Note that `-1.0`, for example, is analyzed as two tokens: `-` followed by `1.0`.

Examples of floating-point literals which are not accepted as literal expressions:

```rust
# #[cfg(false)] {
2.0f80;
2e5f80;
2e5e6;
2.0e5e6;
1.3e10u64;
# }
```

r[lex.token.literal.reserved]
#### Reserved forms similar to number literals

r[lex.token.literal.reserved.syntax]
```grammar,lexer
RESERVED_NUMBER ->
      BIN_LITERAL [`2`-`9`]
    | OCT_LITERAL [`8`-`9`]
    | ( BIN_LITERAL | OCT_LITERAL | HEX_LITERAL ) `.` _not immediately followed by `.`, `_` or an XID_Start character_
    | ( BIN_LITERAL | OCT_LITERAL ) (`e`|`E`)
    | `0b` `_`* <end of input or not BIN_DIGIT>
    | `0o` `_`* <end of input or not OCT_DIGIT>
    | `0x` `_`* <end of input or not HEX_DIGIT>
    | DEC_LITERAL ( `.` DEC_LITERAL )? (`e` | `E`) (`+` | `-`)? <end of input or not DEC_DIGIT>

```

r[lex.token.literal.reserved.intro]
The following lexical forms similar to number literals are _reserved forms_.
Due to the possible ambiguity these raise, they are rejected by the tokenizer instead of being interpreted as separate tokens.

r[lex.token.literal.reserved.out-of-range]
* An unsuffixed binary or octal literal followed, without intervening whitespace, by a decimal digit out of the range for its radix.

r[lex.token.literal.reserved.period]
* An unsuffixed binary, octal, or hexadecimal literal followed, without intervening whitespace, by a period character (with the same restrictions on what follows the period as for floating-point literals).

r[lex.token.literal.reserved.exp]
* An unsuffixed binary or octal literal followed, without intervening whitespace, by the character `e` or `E`.

r[lex.token.literal.reserved.empty-with-radix]
* Input which begins with one of the radix prefixes but is not a valid binary, octal, or hexadecimal literal (because it contains no digits).

r[lex.token.literal.reserved.empty-exp]
* Input which has the form of a floating-point literal with no digits in the exponent.

Examples of reserved forms:

```rust,compile_fail
0b0102;  // this is not `0b010` followed by `2`
0o1279;  // this is not `0o127` followed by `9`
0x80.0;  // this is not `0x80` followed by `.` and `0`
0b101e;  // this is not a suffixed literal, or `0b101` followed by `e`
0b;      // this is not an integer literal, or `0` followed by `b`
0b_;     // this is not an integer literal, or `0` followed by `b_`
2e;      // this is not a floating-point literal, or `2` followed by `e`
2.0e;    // this is not a floating-point literal, or `2.0` followed by `e`
2em;     // this is not a suffixed literal, or `2` followed by `em`
2.0em;   // this is not a suffixed literal, or `2.0` followed by `em`
```

r[lex.token.life]
## Lifetimes and loop labels

r[lex.token.life.syntax]
```grammar,lexer
LIFETIME_TOKEN ->
      `'` IDENTIFIER_OR_KEYWORD _not immediately followed by `'`_
    | `'_` _not immediately followed by `'`_
    | RAW_LIFETIME

LIFETIME_OR_LABEL ->
      `'` NON_KEYWORD_IDENTIFIER _not immediately followed by `'`_
    | RAW_LIFETIME

RAW_LIFETIME ->
    `'r#` IDENTIFIER_OR_KEYWORD _except `crate`, `self`, `super`, `Self` and not immediately followed by `'`_

RESERVED_RAW_LIFETIME -> `'r#_` _not immediately followed by `'`_
```

r[lex.token.life.intro]
Lifetime parameters and [loop labels] use LIFETIME_OR_LABEL tokens. Any
LIFETIME_TOKEN will be accepted by the lexer, and for example, can be used in
macros.

r[lex.token.life.raw.intro]
A raw lifetime is like a normal lifetime, but its identifier is prefixed by `r#`. (Note that the `r#` prefix is not included as part of the actual lifetime.)

r[lex.token.life.raw.allowed]
Unlike a normal lifetime, a raw lifetime may be any strict or reserved keyword except the ones listed above for `RAW_LIFETIME`.

r[lex.token.life.raw.reserved]
It is an error to use the RESERVED_RAW_LIFETIME token `'r#_` in order to avoid confusion with the [placeholder lifetime].

r[lex.token.life.raw.edition2021]
> [!EDITION-2021]
> Raw lifetimes are accepted in the 2021 edition or later. In earlier additions the token `'r#lt` is lexed as `'r # lt`.

r[lex.token.punct]
## Punctuation

r[lex.token.punct.syntax]
```grammar,lexer
PUNCTUATION ->
      `=`
    | `<`
    | `<=`
    | `==`
    | `!=`
    | `>=`
    | `>`
    | `&&`
    | `||`
    | `!`
    | `~`
    | `+`
    | `-`
    | `*`
    | `/`
    | `%`
    | `^`
    | `&`
    | `|`
    | `<<`
    | `>>`
    | `+=`
    | `-=`
    | `*=`
    | `/=`
    | `%=`
    | `^=`
    | `&=`
    | `|=`
    | `<<=`
    | `>>=`
    | `@`
    | `.`
    | `..`
    | `...`
    | `..=`
    | `,`
    | `;`
    | `:`
    | `::`
    | `->`
    | `<-`
    | `=>`
    | `#`
    | `$`
    | `?`
    | `_`
    | `{`
    | `}`
    | `[`
    | `]`
    | `(`
    | `)`
```

r[lex.token.punct.intro]
Punctuation symbol tokens are listed here for completeness. Their individual
usages and meanings are defined in the linked pages.

| Symbol | Name        | Usage |
|--------|-------------|-------|
| `+`    | Plus        | [Addition][arith], [Trait Bounds], [Macro Kleene Matcher][macros]
| `-`    | Minus       | [Subtraction][arith], [Negation]
| `*`    | Star        | [Multiplication][arith], [Dereference], [Raw Pointers], [Macro Kleene Matcher][macros], [Use wildcards]
| `/`    | Slash       | [Division][arith]
| `%`    | Percent     | [Remainder][arith]
| `^`    | Caret       | [Bitwise and Logical XOR][arith]
| `!`    | Not         | [Bitwise and Logical NOT][negation], [Macro Calls][macros], [Inner Attributes][attributes], [Never Type], [Negative impls]
| `&`    | And         | [Bitwise and Logical AND][arith], [Borrow], [References], [Reference patterns]
| <code>\|</code> | Or | [Bitwise and Logical OR][arith], [Closures], Patterns in [match], [if let], and [while let]
| `&&`   | AndAnd      | [Lazy AND][lazy-bool], [Borrow], [References], [Reference patterns]
| <code>\|\|</code> | OrOr | [Lazy OR][lazy-bool], [Closures]
| `<<`   | Shl         | [Shift Left][arith], [Nested Generics][generics]
| `>>`   | Shr         | [Shift Right][arith], [Nested Generics][generics]
| `+=`   | PlusEq      | [Addition assignment][compound]
| `-=`   | MinusEq     | [Subtraction assignment][compound]
| `*=`   | StarEq      | [Multiplication assignment][compound]
| `/=`   | SlashEq     | [Division assignment][compound]
| `%=`   | PercentEq   | [Remainder assignment][compound]
| `^=`   | CaretEq     | [Bitwise XOR assignment][compound]
| `&=`   | AndEq       | [Bitwise And assignment][compound]
| <code>\|=</code> | OrEq | [Bitwise Or assignment][compound]
| `<<=`  | ShlEq       | [Shift Left assignment][compound]
| `>>=`  | ShrEq       | [Shift Right assignment][compound], [Nested Generics][generics]
| `=`    | Eq          | [Assignment], [Attributes], Various type definitions
| `==`   | EqEq        | [Equal][comparison]
| `!=`   | Ne          | [Not Equal][comparison]
| `>`    | Gt          | [Greater than][comparison], [Generics], [Paths]
| `<`    | Lt          | [Less than][comparison], [Generics], [Paths]
| `>=`   | Ge          | [Greater than or equal to][comparison], [Generics]
| `<=`   | Le          | [Less than or equal to][comparison]
| `@`    | At          | [Subpattern binding]
| `_`    | Underscore  | [Wildcard patterns], [Inferred types], Unnamed items in [constants], [extern crates], [use declarations], and [destructuring assignment]
| `.`    | Dot         | [Field access][field], [Tuple index]
| `..`   | DotDot      | [Range][range], [Struct expressions], [Patterns], [Range Patterns][rangepat]
| `...`  | DotDotDot   | [Variadic functions][extern], [Range patterns]
| `..=`  | DotDotEq    | [Inclusive Range][range], [Range patterns]
| `,`    | Comma       | Various separators
| `;`    | Semi        | Terminator for various items and statements, [Array types]
| `:`    | Colon       | Various separators
| `::`   | PathSep     | [Path separator][paths]
| `->`   | RArrow      | [Function return type][functions], [Closure return type][closures], [Function pointer type]
| `=>`   | FatArrow    | [Match arms][match], [Macros]
| `<-`   | LArrow      | The left arrow symbol has been unused since before Rust 1.0, but it is still treated as a single token
| `#`    | Pound       | [Attributes]
| `$`    | Dollar      | [Macros]
| `?`    | Question    | [Try propagation expressions][question], [Questionably sized][sized], [Macro Kleene Matcher][macros]
| `~`    | Tilde       | The tilde operator has been unused since before Rust 1.0, but its token may still be used

r[lex.token.delim]
## Delimiters

Bracket punctuation is used in various parts of the grammar. An open bracket
must always be paired with a close bracket. Brackets and the tokens within
them are referred to as "token trees" in [macros].  The three types of brackets are:

| Bracket | Type            |
|---------|-----------------|
| `{` `}` | Curly braces    |
| `[` `]` | Square brackets |
| `(` `)` | Parentheses     |

r[lex.token.reserved]
## Reserved tokens

r[lex.token.reserved.intro]
Several token forms are reserved for future use. It is an error for the source input to match one of these forms.

r[lex.token.reserved.syntax]
```grammar,lexer
RESERVED_TOKEN ->
      RESERVED_GUARDED_STRING_LITERAL
    | RESERVED_NUMBER
    | RESERVED_POUNDS
    | RESERVED_RAW_IDENTIFIER
    | RESERVED_RAW_LIFETIME
    | RESERVED_TOKEN_DOUBLE_QUOTE
    | RESERVED_TOKEN_LIFETIME
    | RESERVED_TOKEN_POUND
    | RESERVED_TOKEN_SINGLE_QUOTE
```

r[lex.token.reserved-prefix]
## Reserved prefixes

r[lex.token.reserved-prefix.syntax]
```grammar,lexer
RESERVED_TOKEN_DOUBLE_QUOTE ->
    ( IDENTIFIER_OR_KEYWORD _except `b` or `c` or `r` or `br` or `cr`_ | `_` ) `"`

RESERVED_TOKEN_SINGLE_QUOTE ->
    ( IDENTIFIER_OR_KEYWORD _except `b`_ | `_` ) `'`

RESERVED_TOKEN_POUND ->
    ( IDENTIFIER_OR_KEYWORD _except `r` or `br` or `cr`_ | `_` ) `#`

RESERVED_TOKEN_LIFETIME ->
    `'` ( IDENTIFIER_OR_KEYWORD _except `r`_ | `_` ) `#`
```

r[lex.token.reserved-prefix.intro]
Some lexical forms known as _reserved prefixes_ are reserved for future use.

r[lex.token.reserved-prefix.id]
Source input which would otherwise be lexically interpreted as a non-raw identifier (or a keyword or `_`) which is immediately followed by a `#`, `'`, or `"` character (without intervening whitespace) is identified as a reserved prefix.

r[lex.token.reserved-prefix.raw-token]
Note that raw identifiers, raw string literals, and raw byte string literals may contain a `#` character but are not interpreted as containing a reserved prefix.

r[lex.token.reserved-prefix.strings]
Similarly the `r`, `b`, `br`, `c`, and `cr` prefixes used in raw string literals, byte literals, byte string literals, raw byte string literals, C string literals, and raw C string literals are not interpreted as reserved prefixes.

r[lex.token.reserved-prefix.life]
Source input which would otherwise be lexically interpreted as a non-raw lifetime (or a keyword or `_`) which is immediately followed by a `#` character (without intervening whitespace) is identified as a reserved lifetime prefix.

r[lex.token.reserved-prefix.edition2021]
> [!EDITION-2021]
> Starting with the 2021 edition, reserved prefixes are reported as an error by the lexer (in particular, they cannot be passed to macros).
>
> Before the 2021 edition, reserved prefixes are accepted by the lexer and interpreted as multiple tokens (for example, one token for the identifier or keyword, followed by a `#` token).
>
> Examples accepted in all editions:
> ```rust
> macro_rules! lexes {($($_:tt)*) => {}}
> lexes!{a #foo}
> lexes!{continue 'foo}
> lexes!{match "..." {}}
> lexes!{r#let#foo}         // three tokens: r#let # foo
> lexes!{'prefix #lt}
> ```
>
> Examples accepted before the 2021 edition but rejected later:
> ```rust,edition2018
> macro_rules! lexes {($($_:tt)*) => {}}
> lexes!{a#foo}
> lexes!{continue'foo}
> lexes!{match"..." {}}
> lexes!{'prefix#lt}
> ```

r[lex.token.reserved-guards]
## Reserved guards

r[lex.token.reserved-guards.syntax]
```grammar,lexer
RESERVED_GUARDED_STRING_LITERAL -> `#`+ STRING_LITERAL

RESERVED_POUNDS -> `#`{2..}
```

r[lex.token.reserved-guards.intro]
The reserved guards are syntax reserved for future use, and will generate a compile error if used.

r[lex.token.reserved-guards.string-literal]
The *reserved guarded string literal* is a token of one or more `U+0023` (`#`) immediately followed by a [STRING_LITERAL].

r[lex.token.reserved-guards.pounds]
The *reserved pounds* is a token of two or more `U+0023` (`#`).

r[lex.token.reserved-guards.edition2024]
> [!EDITION-2024]
> Before the 2024 edition, reserved guards are accepted by the lexer and interpreted as multiple tokens. For example, the `#"foo"#` form is interpreted as three tokens. `##` is interpreted as two tokens.

[Inferred types]: types/inferred.md
[Range patterns]: patterns.md#range-patterns
[Reference patterns]: patterns.md#reference-patterns
[Subpattern binding]: patterns.md#identifier-patterns
[Wildcard patterns]: patterns.md#wildcard-pattern
[arith]: expressions/operator-expr.md#arithmetic-and-logical-binary-operators
[array types]: types/array.md
[assignment]: expressions/operator-expr.md#assignment-expressions
[attributes]: attributes.md
[borrow]: expressions/operator-expr.md#borrow-operators
[closures]: expressions/closure-expr.md
[comparison]: expressions/operator-expr.md#comparison-operators
[compound]: expressions/operator-expr.md#compound-assignment-expressions
[constants]: items/constant-items.md
[dereference]: expressions/operator-expr.md#the-dereference-operator
[destructuring assignment]: expressions/underscore-expr.md
[extern crates]: items/extern-crates.md
[extern]: items/external-blocks.md
[field]: expressions/field-expr.md
[Floating-point literal expressions]: expressions/literal-expr.md#floating-point-literal-expressions
[floating-point types]: types/numeric.md#floating-point-types
[function pointer type]: types/function-pointer.md
[functions]: items/functions.md
[generics]: items/generics.md
[identifier]: identifiers.md
[if let]: expressions/if-expr.md#if-let-patterns
[Integer literal expressions]: expressions/literal-expr.md#integer-literal-expressions
[keywords]: keywords.md
[lazy-bool]: expressions/operator-expr.md#lazy-boolean-operators
[literal expressions]: expressions/literal-expr.md
[loop labels]: expressions/loop-expr.md
[macros]: macros-by-example.md
[match]: expressions/match-expr.md
[negation]: expressions/operator-expr.md#negation-operators
[negative impls]: items/implementations.md
[never type]: types/never.md
[numeric types]: types/numeric.md
[paths]: paths.md
[patterns]: patterns.md
[placeholder lifetime]: lifetime-elision.md
[question]: expressions/operator-expr.md#the-try-propagation-expression
[range]: expressions/range-expr.md
[rangepat]: patterns.md#range-patterns
[raw pointers]: types/pointer.md#raw-pointers-const-and-mut
[references]: types/pointer.md
[sized]: trait-bounds.md#sized
[String continuation escapes]: expressions/literal-expr.md#string-continuation-escapes
[struct expressions]: expressions/struct-expr.md
[trait bounds]: trait-bounds.md
[tuple index]: expressions/tuple-expr.md#tuple-indexing-expressions
[tuple structs]: items/structs.md
[tuple variants]: items/enumerations.md
[tuples]: types/tuple.md
[unary minus operator]: expressions/operator-expr.md#negation-operators
[use declarations]: items/use-declarations.md
[use wildcards]: items/use-declarations.md
[while let]: expressions/loop-expr.md#while-let-patterns
