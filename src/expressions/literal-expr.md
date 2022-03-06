# Literal expressions

> **<sup>Syntax</sup>**\
> _LiteralExpression_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; [CHAR_LITERAL]\
> &nbsp;&nbsp; | [STRING_LITERAL]\
> &nbsp;&nbsp; | [RAW_STRING_LITERAL]\
> &nbsp;&nbsp; | [BYTE_LITERAL]\
> &nbsp;&nbsp; | [BYTE_STRING_LITERAL]\
> &nbsp;&nbsp; | [RAW_BYTE_STRING_LITERAL]\
> &nbsp;&nbsp; | [INTEGER_LITERAL]\
> &nbsp;&nbsp; | [FLOAT_LITERAL]\
> &nbsp;&nbsp; | [BOOLEAN_LITERAL]

A _literal expression_ is an expression consisting of a single token, rather than a sequence of tokens, that immediately and directly denotes the value it evaluates to, rather than referring to it by name or some other evaluation rule.

A literal is a form of [constant expression], so is evaluated (primarily) at compile time.

Each of the lexical [literal][literal tokens] forms described earlier can make up a literal expression.

```rust
"hello";   // string type
'5';       // character type
5;         // integer type
```

## Integer literal expressions

An integer literal expression consists of a single [INTEGER_LITERAL] token.

If the token has a [suffix], the suffix will be the name of one of the [primitive integer types][numeric types]: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `usize`, or `isize`, and the expression has that type.

If the token has no suffix, the expression's type is determined by type inference:

* If an integer type can be _uniquely_ determined from the surrounding program context, the expression has that type.

* If the program context under-constrains the type, it defaults to the signed 32-bit integer `i32`.

* If the program context over-constrains the type, it is considered a static type error.

Examples of integer literal expressions:

```rust
123;                               // type i32
123i32;                            // type i32
123u32;                            // type u32
123_u32;                           // type u32
let a: u64 = 123;                  // type u64

0xff;                              // type i32
0xff_u8;                           // type u8

0o70;                              // type i32
0o70_i16;                          // type i16

0b1111_1111_1001_0000;             // type i32
0b1111_1111_1001_0000i64;          // type i64

0usize;                            // type usize
```

## Floating-point literal expressions

A floating-point literal expression consists of a single [FLOAT_LITERAL] token.

If the token has a [suffix], the suffix will be the name of one of the [primitive floating-point types][floating-point types]: `f32` or `f64`, and the expression has that type.

If the token has no suffix, the expression's type is determined by type inference:

* If a floating-point type can be _uniquely_ determined from the surrounding program context, the expression has that type.

* If the program context under-constrains the type, it defaults to `f64`.

* If the program context over-constrains the type, it is considered a static type error.

Examples of floating-point literal expressions:

```rust
123.0f64;        // type f64
0.1f64;          // type f64
0.1f32;          // type f32
12E+99_f64;      // type f64
5f32;            // type f32
let x: f64 = 2.; // type f64
```

[constant expression]: ../const_eval.md#constant-expressions
[floating-point types]: ../types/numeric.md#floating-point-types
[literal tokens]: ../tokens.md#literals
[numeric types]: ../types/numeric.md
[suffix]: ../tokens.md#suffixes
[CHAR_LITERAL]: ../tokens.md#character-literals
[STRING_LITERAL]: ../tokens.md#string-literals
[RAW_STRING_LITERAL]: ../tokens.md#raw-string-literals
[BYTE_LITERAL]: ../tokens.md#byte-literals
[BYTE_STRING_LITERAL]: ../tokens.md#byte-string-literals
[RAW_BYTE_STRING_LITERAL]: ../tokens.md#raw-byte-string-literals
[INTEGER_LITERAL]: ../tokens.md#integer-literals
[FLOAT_LITERAL]: ../tokens.md#floating-point-literals
[BOOLEAN_LITERAL]: ../tokens.md#boolean-literals
