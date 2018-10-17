# Static items

> **<sup>Syntax</sup>**\
> _StaticItem_ :\
> &nbsp;&nbsp; `static` `mut`<sup>?</sup> [IDENTIFIER] `:` [_Type_]
>              `=` [_Expression_] `;`

A *static item* is similar to a [constant], except that it represents a precise
memory location in the program. All references to the static refer to the same
memory location. Static items have the `static` lifetime, which outlives all
other lifetimes in a Rust program. Non-`mut` static items that contain a type
that is not [interior mutable] may be placed in read-only memory. Static items
do not call [`drop`] at the end of the program.

All access to a static is safe, but there are a number of restrictions on
statics:

* The type must have the `Sync` trait bound to allow thread-safe access.
* Statics allow using paths to statics in the [constant expression] used to
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
libraries and can also be bound from C libraries in an `extern` block.

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
type does not have to implement the `Sync` trait.

## Using Statics or Consts

It can be confusing whether or not you should use a constant item or a static
item. Constants should, in general, be preferred over statics unless one of the
following are true:

* Large amounts of data are being stored
* The single-address property of statics is required.
* Interior mutability is required.

[constant]: items/constant-items.html
[`drop`]: destructors.html
[constant expression]: const_eval.html#constant-expressions
[interior mutable]: interior-mutability.html
[IDENTIFIER]: identifiers.html
[_Type_]: types.html#type-expressions
[_Expression_]: expressions.html
