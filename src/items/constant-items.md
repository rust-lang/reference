# Constant items

A *constant item* is a named _[constant value]_ which is not associated with a
specific memory location in the program. Constants are essentially inlined
wherever they are used, meaning that they are copied directly into the relevant
context when used. References to the same constant are not necessarily
guaranteed to refer to the same memory address.

[constant value]: expressions.html#constant-expressions

Constant values must not have destructors, and otherwise permit most forms of
data. Constants may refer to the address of other constants, in which case the
address will have elided lifetimes where applicable, otherwise – in most cases
– defaulting to the `static` lifetime. (See below on [static lifetime
elision].) The compiler is, however, still at liberty to translate the constant
many times, so the address referred to may not be stable.

[static lifetime elision]: items/static-items.html#static-lifetime-elision

Constants must be explicitly typed. The type may be any type that doesn't
implement [`Drop`] and has a `'static` lifetime: any references it contains
must have `'static` lifetimes.

[`Drop`]: the-drop-trait.html

```rust
const BIT1: u32 = 1 << 0;
const BIT2: u32 = 1 << 1;

const BITS: [u32; 2] = [BIT1, BIT2];
const STRING: &'static str = "bitstring";

struct BitsNStrings<'a> {
    mybits: [u32; 2],
    mystring: &'a str,
}

const BITS_N_STRINGS: BitsNStrings<'static> = BitsNStrings {
    mybits: BITS,
    mystring: STRING,
};
```
