# Type Layout

The layout of a type is its size, alignment, and the relative offsets of its
fields. For enums, how the discriminant is laid out and interpreted is also part
of type layout.

Type layout can be changed with each compilation. Instead of trying to document
exactly what is done, we only document what is guaranteed today.

## Size and Alignment

All values have an alignment and size.

The *alignment* of a value specifies what addresses are valid to store the value
at. A value of alignment `n` must only be stored at an address that is a
multiple of n. For example, a value with an alignment of 2 must be stored at an
even address, while a value with an alignment of 1 can be stored at any address.
Alignment is measured in bytes, and must be at least 1, and always a power of 2.
The alignment of a value can be checked with the [`align_of_val`] function.

The *size* of a value is the offset in bytes between successive elements in an
array with that item type including alignment padding. The size of a value is
always a multiple of its alignment. The size of a value can be checked with the
[`size_of_val`] function.

Types where all values have the same size and alignment known at compile time
implement the [`Sized`] trait and can be checked with the [`size_of`] and
[`align_of`] functions. Types that are not [`Sized`] are known as [dynamically
sized types]. Since all values of a `Sized` type share the same size and
alignment, we refer to those shared values as the size of the type and the
alignment of the type respectively.

## Primitive Data Layout

The size of most primitives is given in this table.

Type | `size_of::<Type>()`
- | - | -
bool | 1
u8 | 1
u16 | 2
u32 | 4
u64 | 8
u128 | 16
i8 | 1
i16 | 2
i32 | 4
i64 | 8
i128 | 16
f32 | 4
f64 | 8
char | 4

`usize` and `isize` have a size big enough to contain every address on the
target platform. For example, on a 32 bit target, this is 4 bytes and on a 64
bit target, this is 8 bytes.

Most primitives are generally aligned to their size, although this is
platform-specific behavior. In particular, on x86 u64 and f64 are only
aligned to 32 bits.

## Pointers and References Layout

Pointers and references have the same layout. Mutability of the pointer or
reference does not change the layout.

Pointers to sized types have the same size and alignment as `usize`.

Pointers to unsized types are sized. The size and alignment is guaranteed to be
at least equal to the size and alignment of a pointer.

> Note: Though you should not rely on this, all pointers to <abbr
> title="Dynamically Sized Types">DSTs</abbr> are currently twice the size of
> the size of `usize` and have the same alignment.

## Array Layout

Arrays are laid out so that the `nth` element of the array is offset from the
start of the array by `n * the size of the type` bytes. An array of `[T; n]`
has a size of `size_of::<T>() * n` and the same alignment of `T`.

## Slice Layout

Slices have the same layout as the section of the array they slice.

> Note: This is about the raw `[T]` type, not pointers (`&[T]`, `Box<[T]>`,
> etc.) to slices.

## Tuple Layout

Tuples do not have any guarantees about their layout.

The exception to this is the unit tuple (`()`) which is guaranteed as a
zero-sized type to have a size of 0 and an alignment of 1.

## Trait Object Layout

Trait objects have the same layout as the value the trait object is of.

> Note: This is about the raw trait object types, not pointers (`&Trait`,
> `Box<Trait>`, etc.) to trait objects.

## Closure Layout

Closures have no layout guarantees.

## Representations

All user-defined composite types (`struct`s, `enum`s, and `union`s) have a
*representation* that specifies what the layout is for the type.

The possible representations for a type are the default representation, `C`, the
primitive representations, and `packed`. Multiple representations can be applied
to a single type.

The representation of a type can be changed by applying the [`repr` attribute]
to it. The following example shows a struct with a `C` representation.

```
#[repr(C)]
struct ThreeInts {
    first: i16,
    second: i8,
    third: i32
}
```

> Note: As a consequence of the representation being an attribute on the item,
> the representation does not depend on generic parameters. Any two types with
> the same name have the same representation. For example, `Foo<Bar>` and
> `Foo<Baz>` both have the same representation.

The representation of a type does not change the layout of its fields. For
example, a struct with a `C` representation that contains a struct `Inner` with
the default representation will not change the layout of Inner.

### The Default Representation

Nominal types without a `repr` attribute have the default representation.
Informally, this representation is also called the `rust` representation.

There are no guarantees of data layout made by this representation.

### The `C` Representation

The `C` representation is designed for dual purposes. One purpose is for
creating types that are interoptable with the C Language. The second purpose is
to create types that you can soundly performing operations that rely on data
layout such as reinterpreting values as a different type.

Because of this dual purpose, it is possible to create types that are not useful
for interfacing with the C programming language.

This representation can be applied to structs, unions, and enums.

#### \#[repr(C)] Structs

The alignment of the struct is the alignment of the most-aligned field in it.

The size and offset of fields is determined by the following algorithm.

Start with a current offset of 0 bytes.

For each field in declaration order in the struct, first determine the size and
alignment of the field. If the current offset is not a multiple of the field's
alignment, then add padding bytes to the current offset until it is a multiple
of the field's alignment. The offset for the field is what the current offset
is now. Then increase the current offset by the size of the field.

Finally, the size of the struct is the current offset rounded up to the nearest
multiple of the struct's alignment.

Here is this algorithm described in pseudocode.

```rust,ignore
struct.alignment = struct.fields().map(|field| field.alignment).max();

let current_offset = 0;

for field in struct.fields_in_declaration_order() {
    // Increase the current offset so that it's a multiple of the alignment
    // of this field. For the first field, this will always be zero.
    // The skipped bytes are called padding bytes.
    current_offset += field.alignment % current_offset;

    struct[field].offset = current_offset;

    current_offset += field.size;
}

struct.size = current_offset + current_offset % struct.alignment;
```

> Note: This algorithm can produce zero-sized structs. This differs from
> C where structs without data still have a size of one byte.

#### \#[repr(C)] Unions

A union declared with `#[repr(C)]` will have the same size and alignment as an
equivalent C union declaration in the C language for the target platform.
The union will have a size of the maximum size of all of its fields rounded to
its alignment, and an alignment of the maximum alignment of all of its fields.
These maximums may come from different fields.

```
#[repr(C)]
union Union {
    f1: u16,
    f2: [u8; 4],
}

assert_eq!(std::mem::size_of::<Union>(), 4);  // From f2
assert_eq!(std::mem::align_of::<Union>(), 2); // From f1

#[repr(C)]
union SizeRoundedUp {
   a: u32,
   b: [u16; 3],
}

assert_eq!(std::mem::size_of::<SizeRoundedUp>(), 8);  // Size of 6 from b,
                                                      // rounded up to 8 from
                                                      // alignment of a.
assert_eq!(std::mem::align_of::<SizeRoundedUp>(), 4); // From a
```

#### \#[repr(C)] Enums

For [C-like enumerations], the `C` representation has the size and alignment of
the default `enum` size and alignment for the target platform's C ABI.

> Note: The enum representation in C is implementation defined, so this is
> really a "best guess". In particular, this may be incorrect when the C code
> of interest is compiled with certain flags.

<div class="warning">

Warning: There are crucial differences between an `enum` in the C language and
Rust's C-like enumerations with this representation. An `enum` in  C is
mostly a `typedef` plus some named constants; in other words, an object of an
`enum` type can hold any integer value. For example, this is often used for
bitflags in `C`. In contrast, Rust’s C-like enumerations can only legally hold
the discriminant values, everything else is undefined behaviour. Therefore,
using a C-like enumeration in FFI to model a C `enum` is often wrong.

</div>

It is an error for [zero-variant enumerations] to have the `C` representation.

For all other enumerations, the layout is unspecified.

Likewise, combining the `C` representation with a primitive representation, the
layout is unspecified.

### Primitive representations

The *primitive representations* are the representations with the same names as
the primitive integer types. That is: `u8`, `u16`, `u32`, `u64`, `usize`, `i8`,
`i16`, `i32`, `i64`, and `isize`.

Primitive representations can only be applied to enumerations.

For [C-like enumerations], they set the size and alignment to be the same as the
primitive type of the same name. For example, a C-like enumeration with a `u8`
representation can only have discriminants between 0 and 255 inclusive.

It is an error for [zero-variant enumerations] to have a primitive
representation.

For all other enumerations, the layout is unspecified.

Likewise, combining two primitive representations together is unspecified.

### The `align` Representation

The `align` representation can be used on `struct`s and `union`s to raise the
alignment of the type to a given value.

Alignment is specified as a parameter in the form of `#[repr(align(x))]`. The
alignment value must be a power of two of type `u32`. The `align` representation
can raise the alignment of a type to be greater than it's primitive alignment,
it cannot lower the alignment of a type.

The `align` and `packed` representations cannot be applied on the same type and
a `packed` type cannot transitively contain another `align`ed type.

### The `packed` Representation

The `packed` representation can only be used on `struct`s and `union`s.

It modifies the representation (either the default or `C`) by removing any
padding bytes and forcing the alignment of the type to `1`.

The `align` and `packed` representations cannot be applied on the same type and
a `packed` type cannot transitively contain another `align`ed type.

<div class="warning">

***Warning:*** Dereferencing an unaligned pointer is [undefined behavior] and
it is possible to [safely create unaligned pointers to `packed` fields][27060].
Like all ways to create undefined behavior in safe Rust, this is a bug.

</div>

### The `transparent` representation

#### Background

It's sometimes useful to add additional type safety by introducing *newtypes*.
For example, code that handles numeric quantities in different units such as
millimeters, centimeters, grams, kilograms, etc. may want to use the type system
to rule out mistakes such as adding millimeters to grams:

```rust
use std::ops::Add;

struct Millimeters(f64);
struct Grams(f64);

impl Add<Millimeters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

// Likewise: impl Add<Grams> for Grams {}
```

Other uses of newtypes include using `PhantomData` to add lifetimes to raw
pointers or to implement the "phantom types" pattern. See the [PhantomData]
documentation and [the Nomicon][nomicon-phantom] for more details.

The added type safety is especially useful when interacting with C or other
languages. However, in those cases we need to ensure the newtypes we add do not
introduce incompatibilities with the C ABI.

#### Newtypes in FFI

Luckily, `repr(C)` newtypes are laid out just like the type they wrap on all
platforms which Rust currently supports, and likely on many more. For example,
consider this C declaration:

```C
struct Object {
    double weight; //< in grams
    double height; //< in millimeters
    // ...
}

void frobnicate(struct Object *);
```

While using this C code from Rust, we could add `repr(C)` to the `Grams` and
`Millimeters` newtypes introduced above and use them to add some type safety
while staying compatible with the memory layout of `Object`:

```rust,no_run
#[repr(C)]
struct Grams(f64);

#[repr(C)]
struct Millimeters(f64);

#[repr(C)]
struct Object {
    weight: Grams,
    height: Millimeters,
    // ...
}

extern {
    fn frobnicate(_: *mut Object);
}
```

This works even when adding some `PhantomData` fields, because they are
zero-sized and therefore don't have to affect the memory layout.

However, there's more to the ABI than just memory layout: there's also the
question of how function call arguments and return values are passed. Many
common ABI treat a struct containing a single field differently from that field
itself, at least when the field is a scalar (e.g., integer or float or pointer).

To continue the above example, suppose the C library also exposes a function
like this:

```C
double calculate_weight(double height);
```

Using our newtypes on the Rust side like this will cause an ABI mismatch on many
platforms:

```rust,ignore
extern {
    fn calculate_weight(height: Millimeters) -> Grams;
}
```

For example, on x86_64 Linux, Rust will pass the argument in an integer
register, while the C function expects the argument to be in a floating-point
register. Likewise, the C function will return the result in a floating-point
register while Rust will expect it in an integer register.

Note that this problem is not specific to floats: To give another example,
32-bit x86 linux will pass and return `struct Foo(i32);` on the stack while
`i32` is placed in registers.

#### Enter `repr(transparent)`

So while `repr(C)` happens to do the right thing with respect to memory layout,
it's not quite the right tool for newtypes in FFI. Instead of declaring a C
struct, we need to communicate to the Rust compiler that our newtype is just for
type safety on the Rust side. This is what `repr(transparent)` does.

The attribute can be applied to a newtype-like structs that contains a single
field. It indicates that the newtype should be represented exactly like that
field's type, i.e., the newtype should be ignored for ABI purpopses: not only is
it laid out the same in memory, it is also passed identically in function calls.

In the above example, the ABI mismatches can be prevented by making the newtypes
`Grams` and `Millimeters` transparent like this:

```rust
#[repr(transparent)]
struct Grams(f64);

#[repr(transparent)]
struct Millimeters(f64);
```

In addition to that single field, any number of zero-sized fields are permitted,
including but not limited to `PhantomData`:

```rust
use std::marker::PhantomData;

struct Foo { /* ... */ }

#[repr(transparent)]
struct FooPtrWithLifetime<'a>(*const Foo, PhantomData<&'a Foo>);

#[repr(transparent)]
struct NumberWithUnit<T, U>(T, PhantomData<U>);

struct CustomZst;

#[repr(transparent)]
struct PtrWithCustomZst<'a> {
    ptr: FooPtrWithLifetime<'a>,
    some_marker: CustomZst,
}
```

Transparent structs can be nested: `PtrWithCustomZst` is also represented
exactly like `*const Foo`.

Because `repr(transparent)` delegates all representation concerns to another
type, it is incompatible with all other `repr(..)` attributes. It also cannot be
applied to enums, unions, empty structs, structs whose fields are all
zero-sized, or structs with *multiple* non-zero-sized fields.

[`align_of_val`]: ../std/mem/fn.align_of_val.html
[`size_of_val`]: ../std/mem/fn.size_of_val.html
[`align_of`]: ../std/mem/fn.align_of.html
[`size_of`]: ../std/mem/fn.size_of.html
[`Sized`]: ../std/marker/trait.Sized.html
[dynamically sized types]: dynamically-sized-types.html
[C-like enumerations]:  items/enumerations.html#custom-discriminant-values-for-field-less-enumerations
[zero-variant enumerations]: items/enumerations.html#zero-variant-enums
[undefined behavior]: behavior-considered-undefined.html
[27060]: https://github.com/rust-lang/rust/issues/27060
[PhantomData]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
[nomicon-phantom]: https://doc.rust-lang.org/nomicon/phantom-data.html
