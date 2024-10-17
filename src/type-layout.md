# Type Layout

r[layout]

r[layout.intro]
The layout of a type is its size, alignment, and the relative offsets of its
fields. For enums, how the discriminant is laid out and interpreted is also part
of type layout.

r[layout.guarantees]
Type layout can be changed with each compilation. Instead of trying to document
exactly what is done, we only document what is guaranteed today.

## Size and Alignment

r[layout.properties]
All values have an alignment and size.

r[layout.properties.align]
The *alignment* of a value specifies what addresses are valid to store the value
at. A value of alignment `n` must only be stored at an address that is a
multiple of n. For example, a value with an alignment of 2 must be stored at an
even address, while a value with an alignment of 1 can be stored at any address.
Alignment is measured in bytes, and must be at least 1, and always a power of 2.
The alignment of a value can be checked with the [`align_of_val`] function.

r[layout.properties.size]
The *size* of a value is the offset in bytes between successive elements in an
array with that item type including alignment padding. The size of a value is
always a multiple of its alignment. Note that some types are zero-sized; 0 is
considered a multiple of any alignment (for example, on some platforms, the type
`[u16; 0]` has size 0 and alignment 2). The size of a value can be checked with
the [`size_of_val`] function.

r[layout.properties.sized]
Types where all values have the same size and alignment, and both are known at
compile time, implement the [`Sized`] trait and can be checked with the
[`size_of`] and [`align_of`] functions. Types that are not [`Sized`] are known
as [dynamically sized types]. Since all values of a `Sized` type share the same
size and alignment, we refer to those shared values as the size of the type and
the alignment of the type respectively.

## Primitive Data Layout

r[layout.primitive]

r[layout.primitive.size]
The size of most primitives is given in this table.

| Type              | `size_of::<Type>()`|
|--                 |--                  |
| `bool`            | 1                  |
| `u8` / `i8`       | 1                  |
| `u16` / `i16`     | 2                  |
| `u32` / `i32`     | 4                  |
| `u64` / `i64`     | 8                  |
| `u128` / `i128`   | 16                 |
| `usize` / `isize` | See below          |
| `f32`             | 4                  |
| `f64`             | 8                  |
| `char`            | 4                  |

r[layout.primitive.size-int]
`usize` and `isize` have a size big enough to contain every address on the
target platform. For example, on a 32 bit target, this is 4 bytes, and on a 64
bit target, this is 8 bytes.

r[layout.primitive.align]
The alignment of primitives is platform-specific.
In most cases, their alignment is equal to their size, but it may be less.
In particular, `i128` and `u128` are often aligned to 4 or 8 bytes even though
their size is 16, and on many 32-bit platforms, `i64`, `u64`, and `f64` are only
aligned to 4 bytes, not 8.

## Pointers and References Layout

r[layout.pointer]

r[layout.pointer.intro]
Pointers and references have the same layout. Mutability of the pointer or
reference does not change the layout.

r[layout.pointer.thin]
Pointers to sized types have the same size and alignment as `usize`.

r[layout.pointer.unsized]
Pointers to unsized types are sized. The size and alignment is guaranteed to be
at least equal to the size and alignment of a pointer.

> Note: Though you should not rely on this, all pointers to
> <abbr title="Dynamically Sized Types">DSTs</abbr> are currently twice the
> size of the size of `usize` and have the same alignment.

## Array Layout

r[layout.array]

An array of `[T; N]` has a size of `size_of::<T>() * N` and the same alignment
of `T`. Arrays are laid out so that the zero-based `nth` element of the array
is offset from the start of the array by `n * size_of::<T>()` bytes.

## Slice Layout

r[layout.slice]

Slices have the same layout as the section of the array they slice.

> Note: This is about the raw `[T]` type, not pointers (`&[T]`, `Box<[T]>`,
> etc.) to slices.

## `str` Layout

r[layout.str]

String slices are a UTF-8 representation of characters that have the same layout as slices of type `[u8]`.

## Tuple Layout

r[layout.tuple]

r[layout.tuple.general]
Tuples are laid out according to the [`Rust` representation][`Rust`].

r[layout.tuple.unit]
The exception to this is the unit tuple (`()`), which is guaranteed as a
zero-sized type to have a size of 0 and an alignment of 1.

## Trait Object Layout

r[layout.trait-object]

Trait objects have the same layout as the value the trait object is of.

> Note: This is about the raw trait object types, not pointers (`&dyn Trait`,
> `Box<dyn Trait>`, etc.) to trait objects.

## Closure Layout

r[layout.closure]

Closures have no layout guarantees.

## Aggregate Types

r[layout.aggregate]

r[layout.aggregate.intro]
Aggregate types, `struct`s and `union`s, determine the layout based on its fields, and the [repr][layout.repr] attribute. Each field is categorized by its alignment and size, and an offset within that struct which, by default, is a multiple of its alignment.

r[layout.aggregate.struct-offsets]
The fields of a `struct` have offsets such that none of the fields overlap within the `struct`.

r[layout.aggregate.struct-size-align]
The size of a `struct` is at least such that each field can be placed within the struct's storage, and the size is a multiple of its alignment. The alignment of a `struct`, unless modified by the [`repr(packed)`][layout.repr.alignment.packed] attribute, is at least the largest alignment of all fields.

r[layout.aggregate.union-offsets]
The fields of a `union` may have any offset within the `union`. In particular, `union` fields are permitted to overlap within the `union`.

> [!NOTE]
> Typically, union fields are all given offset 0 and are all allocated in the same storage. This is not necessarily guaranteed, however, for [The Rust Representation][layout.repr.rust]

r[layout.aggregate.union-size-align]
The size of a `union` is at least such that the largest field, and the size is a multiple of the alignment. The alignment of a `union`, unless modified by the [`repr(packed)`][layout.repr.alignment.packed] attribute, is at least the largest alignment of all fields.

r[layout.aggregate.repr]
The [`repr`][layout.repr] attribute may restrict the valid offsets for each field, as well as the size and alignment of the aggregate type. By [default][layout.repr.rust], the offsets of fields are unspecified.

> [!NOTE]
> The choices of the offsets of fields are made per-struct. Two `struct`s or two `union`s with the same fields using [`repr(Rust)`][layout.repr.rust] are not necessarily layed out the same

## Representations

r[layout.repr]

r[layout.repr.intro]
All user-defined composite types (`struct`s, `enum`s, and `union`s) have a
*representation* that specifies what the layout is for the type.

r[layout.repr.kinds]
The possible representations for a type are:

- [`Rust`] (default)
- [`C`]
- The [primitive representations]
- [`transparent`]

r[layout.repr.attribute]
The representation of a type can be changed by applying the `repr` attribute
to it. The following example shows a struct with a `C` representation.

```rust
#[repr(C)]
struct ThreeInts {
    first: i16,
    second: i8,
    third: i32
}
```

r[layout.repr.align-packed]
The alignment may be raised or lowered with the `align` and `packed` modifiers
respectively. They alter the representation specified in the attribute.
If no representation is specified, the default one is altered.

```rust
// Default representation, alignment lowered to 2.
#[repr(packed(2))]
struct PackedStruct {
    first: i16,
    second: i8,
    third: i32
}

// C representation, alignment raised to 8
#[repr(C, align(8))]
struct AlignedStruct {
    first: i16,
    second: i8,
    third: i32
}
```

> Note: As a consequence of the representation being an attribute on the item,
> the representation does not depend on generic parameters. Any two types with
> the same name have the same representation. For example, `Foo<Bar>` and
> `Foo<Baz>` both have the same representation.

r[layout.repr.inter-field]
The representation of a type can change the padding between fields, but does
not change the layout of the fields themselves. For example, a struct with a
`C` representation that contains a struct `Inner` with the default
representation will not change the layout of `Inner`.

### <a id="the-default-representation"></a> The `Rust` Representation

r[layout.repr.rust]

r[layout.repr.rust.intro]
The `Rust` representation is the default representation for nominal types
without a `repr` attribute. Using this representation explicitly through a
`repr` attribute is guaranteed to be the same as omitting the attribute
entirely.

r[layout.repr.rust.aggregate]
The layout of aggregate types with this repr is unspecified, such that the guarantees of all `struct` and `union` types are upheld.

> [!NOTE]
> `repr(Rust)` does not provide any guarantees other than the default constraints.

#### `repr(Rust)` Enums

r[layout.repr.rust.enum]

r[layout.repr.rust.enum.intro]
Like `struct`s and `union`s with the `Rust` representation, `enum`s typically have no guarantees. The fields of individual variants are layed out with the same rules as a `repr(Rust)` struct, and the `enum` contains sufficient information to determine which variant is being represented.

r[layout.repr.rust.enum.variant-fields]
Each field of a variant has an offset within the `enum`, such that no two fields of the same variant overlap, and each field of each variant is aligned, unless modified by the [`repr(packed)`][layout.repr.alignment.packed] attribute. Fields of different variants are allowed to overlap, as though each variant appears within a `union` definition.

r[layout.repr.rust.enum.size-align]
The size of an enum is such that each field of each variant that is not uninhabited can be allocated within the enum at its given offset. The alignment of an enum, unless modified by the [`repr(packed)`][layout.repr.alignment.packed] attribute, is at least the largest alignment of all fields in variants that are inhabited.

> [!NOTE]
> A Rough approximation of a `repr(Rust)` enum is a `repr(Rust)` union containing `repr(Rust)` structs for each variant, with a discriminant field present in each struct.
> However, there is no guarantee that the discriminant field is any particular size, or exists within the union, nor stores any particular value.
> Several optimizations can also reduce the size of the `enum` beyond what is valid for a union-of-structs layout, even using the `Rust` representation.

> [!NOTE]
> It is not yet decided whether or not uninhabited variants still need to have storage allocated for their non-zero-sized fields. The documentation here conservatively assumes it is not required.

#### Discriminant Ellision

r[layout.repr.rust.option]

r[layout.repr.rust.option.intro]
Certain `repr(Rust)` enums are specially layed out when used with certain types. This layout process is known as discriminant elision.

r[layout.repr.rust.option.elligible]
An `enum` type is a *discriminant elision eligible* enum if:
* It has exactly two variants,
* One variant has exactly one field, known as the *elision candidate field*, and
* The other variant has no fields that have size greater than 0 or alignment greater than 1, known as the *elided variant*.

> [!NOTE]
> The determination of whether an `enum` type is *discriminant elision eligible* applies after mono-morphization.
> In particular, a type like [`core::result::Result<T,E>`] is *discriminant elision eligible* if either `T` or `E` is instantiated with a 1-ZST type, and [`core::option::Option<T>`] is *discriminant elision eligible* always (due to its `None` variant).

r[layout.repr.rust.option.candidates]
The following types are *elision candidate types*:
* [`&U`][type.pointer.reference.shared], if `U` is a `Sized` type
* [`&mut U`][type.pointer.reference.mut], if `U` is a `Sized` type
* [`core::ptr::NonNull<U>`], if `U` is a `Sized` type
* [`alloc::boxed::Box<U>`], if `U` is a `Sized` type
* [`core::num::NonZero<T>`] if `T` is an integer type
* A [function pointer][type.fn-pointer] type
* A `struct`, defined using [`repr(transparent)`][layout.repr.transparent], which has a field that is an *elision candidate*, and all other fields have size 0 and alignment 1.

r[layout.repr.rust.option.elision]
If the *elision candidate field* of a *discriminant elision eligible* `enum` has an *elision candidate type*, then the `enum` has the same layout as that field, except that the value consisting of all `0` bytes represents the *elided variant* of the `enum`.

> [!NOTE]
> It is valid for this optimization to be performed for types an `enum`s other than those mentioned above, however, it is only guaranteed for these types and `enum`s.

### The `C` Representation

r[layout.repr.c]

r[layout.repr.c.intro]
The `C` representation is designed for dual purposes. One purpose is for
creating types that are interoperable with the C Language. The second purpose is
to create types that you can soundly perform operations on that rely on data
layout such as reinterpreting values as a different type.

Because of this dual purpose, it is possible to create types that are not useful
for interfacing with the C programming language.

r[layout.repr.c.constraint]
This representation can be applied to structs, unions, and enums. The exception
is [zero-variant enums] for which the `C` representation is an error.

r[layout.repr.c.aggregate]
An aggregate type (`struct` or `union`) with the `C` representation is laid out by each field being given by the smallest valid offset for that field that is ascending in declaration order. That is, for a `union`, each field is at offset `0`, and for a struct, the first field is at offset `0`, then the second is at the next offset aligned to the field type. The size of such an aggregate type is the minimum value valid for the type.

r[layout.repr.c.align]
An aggregate type with the `C` representation has an alignment equal to the maximum alignment of each of its fields, unless an [alignment modifier][layout.repr.alignment] is present.

> [!NOTE]
> The layout of unions in particular is maximally compact - the size of a `C` representation `union` is the size of its largest field, rounded up to the alignment of the `union`

```rust
// Example of union layouts
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

#### `#[repr(C)]` Structs

r[layout.repr.c.struct]

r[layout.repr.c.struct.size-field-offset]
The specific algorithm for the layout of `repr(C) struct` is given as follows:

Start with a current offset of 0 bytes.

For each field in declaration order in the struct, first determine the size and
alignment of the field. If the current offset is not a multiple of the field's
alignment, then add padding bytes to the current offset until it is a multiple
of the field's alignment. The offset for the field is what the current offset
is now. Then increase the current offset by the size of the field.

Finally, the size of the struct is the current offset rounded up to the nearest
multiple of the struct's alignment.

Here is this algorithm described in pseudocode.

<!-- ignore: pseudocode -->
```rust,ignore
/// Returns the amount of padding needed after `offset` to ensure that the
/// following address will be aligned to `alignment`.
fn padding_needed_for(offset: usize, alignment: usize) -> usize {
    let misalignment = offset % alignment;
    if misalignment > 0 {
        // round up to next multiple of `alignment`
        alignment - misalignment
    } else {
        // already a multiple of `alignment`
        0
    }
}

struct.alignment = struct.fields().map(|field| field.alignment).max();

let current_offset = 0;

for field in struct.fields_in_declaration_order() {
    // Increase the current offset so that it's a multiple of the alignment
    // of this field. For the first field, this will always be zero.
    // The skipped bytes are called padding bytes.
    current_offset += padding_needed_for(current_offset, field.alignment);

    struct[field].offset = current_offset;

    current_offset += field.size;
}

struct.size = current_offset + padding_needed_for(current_offset, struct.alignment);
```

> [!WARNING]
> This pseudocode uses a naive algorithm that ignores overflow issues for the sake of clarity. To perform memory layout computations in actual code, use [`Layout`].

> [!NOTE]
> This algorithm can produce zero-sized structs. In C, an empty struct
> declaration like `struct Foo { }` is illegal. However, both gcc and clang
> support options to enable such structs, and assign them size zero. C++, in
> contrast, gives empty structs a size of 1, unless they are inherited from or
> they are fields that have the `[[no_unique_address]]` attribute, in which
> case they do not increase the overall size of the struct.

#### `#[repr(C)]` Field-less Enums

r[layout.repr.c.enum]

For [field-less enums], the `C` representation has the size and alignment of
the default `enum` size and alignment for the target platform's C ABI.

> Note: The enum representation in C is implementation defined, so this is
> really a "best guess". In particular, this may be incorrect when the C code
> of interest is compiled with certain flags.

> [!WARNING]
> There are crucial differences between an `enum` in the C language and Rust's [field-less enums] with this representation. An `enum` in C is mostly a `typedef` plus some named constants; in other words, an object of an `enum` type can hold any integer value. For example, this is often used for bitflags in `C`. In contrast, Rust’s [field-less enums] can only legally hold the discriminant values, everything else is [undefined behavior]. Therefore, using a field-less enum in FFI to model a C `enum` is often wrong.

#### `#[repr(C)]` Enums With Fields

r[layout.repr.c.adt]

r[layout.repr.c.adt.intro]
The representation of a `repr(C)` enum with fields is a `repr(C)` struct with
two fields, also called a "tagged union" in C:

r[layout.repr.c.adt.tag]
- a `repr(C)` version of the enum with all fields removed ("the tag")

r[layout.repr.c.adt.fields]
- a `repr(C)` union of `repr(C)` structs for the fields of each variant that had
  them ("the payload")

> Note: Due to the representation of `repr(C)` structs and unions, if a variant
> has a single field there is no difference between putting that field directly
> in the union or wrapping it in a struct; any system which wishes to manipulate
> such an `enum`'s representation may therefore use whichever form is more
> convenient or consistent for them.

```rust
// This Enum has the same representation as ...
#[repr(C)]
enum MyEnum {
    A(u32),
    B(f32, u64),
    C { x: u32, y: u8 },
    D,
 }

// ... this struct.
#[repr(C)]
struct MyEnumRepr {
    tag: MyEnumDiscriminant,
    payload: MyEnumFields,
}

// This is the discriminant enum.
#[repr(C)]
enum MyEnumDiscriminant { A, B, C, D }

// This is the variant union.
#[repr(C)]
union MyEnumFields {
    A: MyAFields,
    B: MyBFields,
    C: MyCFields,
    D: MyDFields,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct MyAFields(u32);

#[repr(C)]
#[derive(Copy, Clone)]
struct MyBFields(f32, u64);

#[repr(C)]
#[derive(Copy, Clone)]
struct MyCFields { x: u32, y: u8 }

// This struct could be omitted (it is a zero-sized type), and it must be in
// C/C++ headers.
#[repr(C)]
#[derive(Copy, Clone)]
struct MyDFields;
```

> Note: `union`s with non-`Copy` fields are unstable, see [55149].

### Primitive representations

r[layout.repr.primitive]

r[layout.repr.primitive.intro]
The *primitive representations* are the representations with the same names as
the primitive integer types. That is: `u8`, `u16`, `u32`, `u64`, `u128`,
`usize`, `i8`, `i16`, `i32`, `i64`, `i128`, and `isize`.

r[layout.repr.primitive.constraint]
Primitive representations can only be applied to enumerations and have
different behavior whether the enum has fields or no fields. It is an error
for [zero-variant enums] to have a primitive representation. Combining
two primitive representations together is an error.

#### Primitive Representation of Field-less Enums

r[layout.repr.primitive.enum]

For [field-less enums], primitive representations set the size and alignment to
be the same as the primitive type of the same name. For example, a field-less
enum with a `u8` representation can only have discriminants between 0 and 255
inclusive.

#### Primitive Representation of Enums With Fields

r[layout.repr.primitive.adt]

The representation of a primitive representation enum is a `repr(C)` union of
`repr(C)` structs for each variant with a field. The first field of each struct
in the union is the primitive representation version of the enum with all fields
removed ("the tag") and the remaining fields are the fields of that variant.

> Note: This representation is unchanged if the tag is given its own member in
> the union, should that make manipulation more clear for you (although to
> follow the C++ standard the tag member should be wrapped in a `struct`).

```rust
// This enum has the same representation as ...
#[repr(u8)]
enum MyEnum {
    A(u32),
    B(f32, u64),
    C { x: u32, y: u8 },
    D,
 }

// ... this union.
#[repr(C)]
union MyEnumRepr {
    A: MyVariantA,
    B: MyVariantB,
    C: MyVariantC,
    D: MyVariantD,
}

// This is the discriminant enum.
#[repr(u8)]
#[derive(Copy, Clone)]
enum MyEnumDiscriminant { A, B, C, D }

#[repr(C)]
#[derive(Clone, Copy)]
struct MyVariantA(MyEnumDiscriminant, u32);

#[repr(C)]
#[derive(Clone, Copy)]
struct MyVariantB(MyEnumDiscriminant, f32, u64);

#[repr(C)]
#[derive(Clone, Copy)]
struct MyVariantC { tag: MyEnumDiscriminant, x: u32, y: u8 }

#[repr(C)]
#[derive(Clone, Copy)]
struct MyVariantD(MyEnumDiscriminant);
```

> Note: `union`s with non-`Copy` fields are unstable, see [55149].

#### Combining primitive representations of enums with fields and `#[repr(C)]`

r[layout.repr.primitive-c]

For enums with fields, it is also possible to combine `repr(C)` and a
primitive representation (e.g., `repr(C, u8)`). This modifies the [`repr(C)`] by
changing the representation of the discriminant enum to the chosen primitive
instead. So, if you chose the `u8` representation, then the discriminant enum
would have a size and alignment of 1 byte.

The discriminant enum from the example [earlier][`repr(C)`] then becomes:

```rust
#[repr(C, u8)] // `u8` was added
enum MyEnum {
    A(u32),
    B(f32, u64),
    C { x: u32, y: u8 },
    D,
 }

// ...

#[repr(u8)] // So `u8` is used here instead of `C`
enum MyEnumDiscriminant { A, B, C, D }

// ...
```

For example, with a `repr(C, u8)` enum it is not possible to have 257 unique
discriminants ("tags") whereas the same enum with only a `repr(C)` attribute
will compile without any problems.

Using a primitive representation in addition to `repr(C)` can change the size of
an enum from the `repr(C)` form:

```rust
#[repr(C)]
enum EnumC {
    Variant0(u8),
    Variant1,
}

#[repr(C, u8)]
enum Enum8 {
    Variant0(u8),
    Variant1,
}

#[repr(C, u16)]
enum Enum16 {
    Variant0(u8),
    Variant1,
}

// The size of the C representation is platform dependant
assert_eq!(std::mem::size_of::<EnumC>(), 8);
// One byte for the discriminant and one byte for the value in Enum8::Variant0
assert_eq!(std::mem::size_of::<Enum8>(), 2);
// Two bytes for the discriminant and one byte for the value in Enum16::Variant0
// plus one byte of padding.
assert_eq!(std::mem::size_of::<Enum16>(), 4);
```

[`repr(C)`]: #reprc-enums-with-fields

### The alignment modifiers

r[layout.repr.alignment]

r[layout.repr.alignment.intro]
The `align` and `packed` modifiers can be used to respectively raise or lower
the alignment of `struct`s and `union`s. `packed` may also alter the padding
between fields (although it will not alter the padding inside of any field).
On their own, `align` and `packed` do not provide guarantees about the order
of fields in the layout of a struct or the layout of an enum variant, although
they may be combined with representations (such as `C`) which do provide such
guarantees.

r[layout.repr.alignment.constraint-alignment]
The alignment is specified as an integer parameter in the form of
`#[repr(align(x))]` or `#[repr(packed(x))]`. The alignment value must be a
power of two from 1 up to 2<sup>29</sup>. For `packed`, if no value is given,
as in `#[repr(packed)]`, then the value is 1.

r[layout.repr.alignment.align]
For `align`, if the specified alignment is less than the alignment of the type
without the `align` modifier, then the alignment is unaffected.

r[layout.repr.alignment.packed]
For `packed`, if the specified alignment is greater than the type's alignment
without the `packed` modifier, then the alignment and layout is unaffected.

r[layout.repr.alignment.packed-fields]
The alignments of each field, for the purpose of positioning fields, is the
smaller of the specified alignment and the alignment of the field's type.

r[layout.repr.alignment.packed-padding]
Inter-field padding is guaranteed to be the minimum required in order to
satisfy each field's (possibly altered) alignment (although note that, on its
own, `packed` does not provide any guarantee about field ordering). An
important consequence of these rules is that a type with `#[repr(packed(1))]`
(or `#[repr(packed)]`) will have no inter-field padding.

r[layout.repr.alignment.constraint-exclusive]
The `align` and `packed` modifiers cannot be applied on the same type and a
`packed` type cannot transitively contain another `align`ed type. `align` and
`packed` may only be applied to the [`Rust`] and [`C`] representations.

r[layout.repr.alignment.enum]
The `align` modifier can also be applied on an `enum`.
When it is, the effect on the `enum`'s alignment is the same as if the `enum`
was wrapped in a newtype `struct` with the same `align` modifier.

> Note: References to unaligned fields are not allowed because it is [undefined behavior].
> When fields are unaligned due to an alignment modifier, consider the following options for using references and dereferences:
>
> ```rust
> #[repr(packed)]
> struct Packed {
>     f1: u8,
>     f2: u16,
> }
> let mut e = Packed { f1: 1, f2: 2 };
> // Instead of creating a reference to a field, copy the value to a local variable.
> let x = e.f2;
> // Or in situations like `println!` which creates a reference, use braces
> // to change it to a copy of the value.
> println!("{}", {e.f2});
> // Or if you need a pointer, use the unaligned methods for reading and writing
> // instead of dereferencing the pointer directly.
> let ptr: *const u16 = &raw const e.f2;
> let value = unsafe { ptr.read_unaligned() };
> let mut_ptr: *mut u16 = &raw mut e.f2;
> unsafe { mut_ptr.write_unaligned(3) }
> ```

### The `transparent` Representation

r[layout.repr.transparent]

r[layout.repr.transparent.constraint-field]
The `transparent` representation can only be used on a [`struct`][structs]
or an [`enum`][enumerations] with a single variant that has:
- any number of fields with size 0 and alignment 1 (e.g. [`PhantomData<T>`]), and
- at most one other field.

r[layout.repr.transparent.layout-abi]
Structs and enums with this representation have the same layout and ABI
as the only non-size 0 non-alignment 1 field, if present, or unit otherwise.

This is different than the `C` representation because
a struct with the `C` representation will always have the ABI of a `C` `struct`
while, for example, a struct with the `transparent` representation with a
primitive field will have the ABI of the primitive field.

r[layout.repr.transparent.constraint-exclusive]
Because this representation delegates type layout to another type, it cannot be
used with any other representation.

[`align_of_val`]: std::mem::align_of_val
[`size_of_val`]: std::mem::size_of_val
[`align_of`]: std::mem::align_of
[`size_of`]: std::mem::size_of
[`Sized`]: std::marker::Sized
[`Copy`]: std::marker::Copy
[dynamically sized types]: dynamically-sized-types.md
[field-less enums]: items/enumerations.md#field-less-enum
[enumerations]: items/enumerations.md
[zero-variant enums]: items/enumerations.md#zero-variant-enums
[undefined behavior]: behavior-considered-undefined.md
[55149]: https://github.com/rust-lang/rust/issues/55149
[`PhantomData<T>`]: special-types-and-traits.md#phantomdatat
[`Rust`]: #the-rust-representation
[`C`]: #the-c-representation
[primitive representations]: #primitive-representations
[structs]: items/structs.md
[`transparent`]: #the-transparent-representation
[`Layout`]: std::alloc::Layout
[type.pointer.reference.shared]: types/pointer.md#shared-references-
[type.pointer.reference.mut]: types/pointer.md#mutable-references-mut
[type.fn-pointer]: types/function-pointer.md
