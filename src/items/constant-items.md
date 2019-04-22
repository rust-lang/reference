# Constant items

> **<sup>Syntax</sup>**\
> _ConstantItem_ :\
> &nbsp;&nbsp; `const` [IDENTIFIER] `:` [_Type_] `=` [_Expression_] `;`

A *constant item* is a named _[constant value]_ which is not associated with a
specific memory location in the program. Constants are essentially inlined
wherever they are used, meaning that they are copied directly into the relevant
context when used. References to the same constant are not necessarily
guaranteed to refer to the same memory address.

Constants must be explicitly typed. The type must have a `'static` lifetime: any
references it contains must have `'static` lifetimes.

Constants may refer to the address of other constants, in which case the
address will have elided lifetimes where applicable, otherwise – in most cases
– defaulting to the `static` lifetime. (See [static lifetime
elision].) The compiler is, however, still at liberty to translate the constant
many times, so the address referred to may not be stable.

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

## Constants with Destructors

Constants can contain destructors. Destructors are run when the value goes out
of scope.

```rust
struct TypeWithDestructor(i32);

impl Drop for TypeWithDestructor {
    fn drop(&mut self) {
        println!("Dropped. Held {}.", self.0);
    }
}

const ZERO_WITH_DESTRUCTOR: TypeWithDestructor = TypeWithDestructor(0);

fn create_and_drop_zero_with_destructor() {
    let x = ZERO_WITH_DESTRUCTOR;
    // x gets dropped at end of function, calling drop.
    // prints "Dropped. Held 0.".
}
```

[constant value]: const_eval.html#constant-expressions
[static lifetime elision]: lifetime-elision.html#static-lifetime-elision
[IDENTIFIER]: identifiers.html
[_Type_]: types.html#type-expressions
[_Expression_]: expressions.html
