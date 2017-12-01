# Static items

> **<sup>Syntax</sup>**  
> _StaticItem_ :  
> &nbsp;&nbsp; `static` `mut`<sup>?</sup> [IDENTIFIER] `:` [_Type_]
>              `=` [_Expression_] `;`

A *static item* is similar to a [constant], except that it represents a precise
memory location in the program. A static is never "inlined" at the usage site,
and all references to it refer to the same memory location. Static items have
the `static` lifetime, which outlives all other lifetimes in a Rust program.
Static items may be placed in read-only memory if the type is not [interior 
mutable]. Static items do not call `drop` at the end of the program.

All access to a static is safe, but there are a number of restrictions on
statics:

* The type must have the `Sync` trait bound to allow thread-safe access.
* Statics allow using paths to statics in the
  [constant-expression](expressions.html#constant-expressions) used to
  initialize them, but statics may not refer to other statics by value, only
  through a reference.
* Constants cannot refer to statics.

## Mutable statics

If a static item is declared with the `mut` keyword, then it is allowed to be
modified by the program. One of Rust's goals is to make concurrency bugs hard
to run into, and this is obviously a very large source of race conditions or
other bugs. For this reason, an `unsafe` block is required when either reading
or writing a mutable static variable. Care should be taken to ensure that
modifications to a mutable static are safe with respect to other threads
running in the same process.

Mutable statics are still very useful, however. They can be used with C
libraries and can also be bound from C libraries (in an `extern` block).

```rust
# fn atomic_add(_: &mut u32, _: u32) -> u32 { 2 }

static mut LEVELS: u32 = 0;

// This violates the idea of no shared state, and this doesn't internally
// protect against races, so this function is `unsafe`
unsafe fn bump_levels_unsafe1() -> u32 {
    let ret = LEVELS;
    LEVELS += 1;
    return ret;
}

// Assuming that we have an atomic_add function which returns the old value,
// this function is "safe" but the meaning of the return value may not be what
// callers expect, so it's still marked as `unsafe`
unsafe fn bump_levels_unsafe2() -> u32 {
    return atomic_add(&mut LEVELS, 1);
}
```

Mutable statics have the same restrictions as normal statics, except that the
type of the value does not require the `Sync` trait bound.

## `'static` lifetime elision

Both constant and static declarations of reference types have *implicit*
`'static` lifetimes unless an explicit lifetime is specified. As such, the
constant declarations involving `'static` above may be written without the
lifetimes. Returning to our previous example:

```rust
const BIT1: u32 = 1 << 0;
const BIT2: u32 = 1 << 1;

const BITS: [u32; 2] = [BIT1, BIT2];
const STRING: &str = "bitstring";

struct BitsNStrings<'a> {
    mybits: [u32; 2],
    mystring: &'a str,
}

const BITS_N_STRINGS: BitsNStrings = BitsNStrings {
    mybits: BITS,
    mystring: STRING,
};
```

Note that if the `static` or `const` items include function or closure
references, which themselves include references, the compiler will first try
the standard elision rules ([see discussion in the nomicon][elision-nomicon]).
If it is unable to resolve the lifetimes by its usual rules, it will default to
using the `'static` lifetime. By way of example:

```rust,ignore
// Resolved as `fn<'a>(&'a str) -> &'a str`.
const RESOLVED_SINGLE: fn(&str) -> &str = ..

// Resolved as `Fn<'a, 'b, 'c>(&'a Foo, &'b Bar, &'c Baz) -> usize`.
const RESOLVED_MULTIPLE: Fn(&Foo, &Bar, &Baz) -> usize = ..

// There is insufficient information to bound the return reference lifetime
// relative to the argument lifetimes, so the signature is resolved as
// `Fn(&'static Foo, &'static Bar) -> &'static Baz`.
const RESOLVED_STATIC: Fn(&Foo, &Bar) -> &Baz = ..
```

## Using Statics or Consts

In can be confusing whether or not you should use a constant item or a static
item. Constants should, in general, be preferred over statics unless one of the
following are true:

* Large amounts of data are being stored
* The single-address or non-inlining property of statics is required.
* Interior mutability is required.

[constant]: items/constant-items.html
[interior mutable]: interior_mutability.html
[IDENTIFIER]: identifiers.html
[_Type_]: types.html
[_Expression_]: expressions.html
[elision-nomicon]: ../nomicon/lifetime-elision.html