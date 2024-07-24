# Application Binary Interface (ABI)

r[abi]

## ABI Compatibility

r[abi.compatibility]

r[abi.compatibilty.type]
Two types, `T` and `U`, can be *abi compatible*.

r[abi.compatibility.equivalence]
Two types `T` and `U` are *abi compatible* if:
* They are the same type,
* `U` is *abi compatible* with `T`, or
* There exists a type `V`, such that `T` is *abi compatible* with `V` an `V` is *abi compatuble* with `U`,

> [!NOTE]
> These properties ensure that *abi compatibility* is an equivalence relation.

r[abi.compatibility.integer]
Two integer types are *abi compatible* if they have the same size and the same signednes

> [!NOTE]
> In particular, `usize` is *abi compatible* with `uN`, and `isize` is *abi compatible* with `iN` where `N` is the target_pointer_width. 
> Two integer types with different signedness, such as `u8` and `i8` are not *abi compatible*.

r[abi.compatibility.char]
The type `char`is *abi compatible* with the type `u32`. 

r[abi.compatibility.pointer]
Two pointer types, `*mut T` and `*const U`, are *abi compatible* if the *metadata type*s of `T` and `U` are the same type. 

> [!NOTE]
> With transitivity, this applies regardless of the mutability of either pointer type

r[abi.compatibility.reference-box]
The types `&T`, `&mut T`, [`Box<T>`][core::boxed::Box], and [`NonNull<T>`][core::ptr::NonNull], are *abi compatible* with `*const T`

> [!NOTE]
> With transitivity,t hey are also *abi compatible** with each other, and with `*mut T`, as well as references/`Box` to different types that have the same *metadata type*.


r[abi.compatibility.core]
The types [`MaybeUninit<T>`][core::mem::MaybeUninit], [`UnsafeCell<T>`][core::cell::UnsafeCell], and [`NonZero<T>`][core::num::NonZero], are *abi compatible* with `T`.

r[abi.compatibility.transparent]
A `struct` declared with the `transparent` representation is *abi compatible* with its field that does not have size 0 and alignment 1, if such a field exists

r[abi.compatibilty.zst]
Two types, `T` and `U`, are *abi compatible* if both have size 0 and alignment 1.

r[abi.compatibility.option]
If `T` is a type listed in [layout.enum.option](https://doc.rust-lang.org/stable/core/option/index.html#representation), then given `S` is a type with size 0 and alignment 1, `T` is *abi compatible* with the types [`Option<T>`], [`Result<T,S>`], and [`Result<S,T>`].

r[abi.compatibility.fn-ptr]
An `fn`-ptr type `T` is compatible with an `fn`-ptr type `U` if `T` and `U` have *abi compatible* tags.

r[abi.compatibility.extern-tag]
Two abi tags are *abi compatible* if:
* They are the same string, or
* One tag is `"X"`, and the other is `"X-unwind"`

r[abi.compatibility.signature]
Two function signatures are compatible if:
* The abi tags of both signatures are *abi compatible*,
* They have the same number of parameters, excluding C-varargs,
* Each parameter of both signatures, in order, are *abi compatible*, and
* Either both signatures have C-varargs, or neither signature does.

r[abi.compatibility.simd-abi]
A type has *simd abi requirements* if:
* It is a type declared with the standard-library repr-attrbute `simd`,
* It is a aggregate type, which has a type with *simd abi requirements* as a field.

r[abi.compatibility.simd-target-feature]
A type with *simd abi requirements* may have one or more [*salient target features*][target_feature] . In the case of an aggregate type, the set of [*salient target features*][target_feature] is the union of the set of [*salient target features*][target_feature] of each field with *simd abi requirements*.

> [!TARGET-SPECIFIC]
> On x86 and x86-64, the [*salient target features*][target_feature] of the `simd` types are:
> * [`__m128`], [`__m128i`], [`__m128f`], and [`__m128d`]: `sse`
> * [`__m256`], [`__m256i`], [`__m256f`], and [`__m256d`]: `avx`
> * [`__m512`], [`__m512i`], [`__m512f`], and [`__m512d`]: `avx512f` and `avx512vl`

r[abi.compatibility.call]
A call to a function `f` via a function item or function pointer with a given signature `S` is valid only if the signature of `f` is *compatible* with the signature `S`, and:
* The ABI tag of the function is `extern "Rust"`, or
* If the type of any parameter, the return type, or the type of any argument passed via C-varargs has *simd abi requirements*, each [*salient target features*][target_feature]of that type is either set at both the definition site of the function, and at the call site, or is set at neither site.

The behaviour a call that is not valid is undefined.

> [!NOTE]
> the ABI tag `extern "Rust"` is the default when the `extern` keyword is not used (either to declare the function within an [`extern` block], or as a [function qualifier][extern functions]). Thus it is safe to call most functions that use simd types.


[`__m128`]: https://doc.rust-lang.org/stable/core/arch/x86_64/struct.__m128.html
[`__m128i`]: https://doc.rust-lang.org/stable/core/arch/x86_64/struct.__m128i.html
[`__m128f`]: https://doc.rust-lang.org/stable/core/arch/x86_64/struct.__m128f.html
[`__m128d`]: https://doc.rust-lang.org/stable/core/arch/x86_64/struct.__m128d.html
[`__m256`]: https://doc.rust-lang.org/stable/core/arch/x86_64/struct.__m256.html
[`__m256i`]: https://doc.rust-lang.org/stable/core/arch/x86_64/struct.__m256i.html
[`__m256f`]: https://doc.rust-lang.org/stable/core/arch/x86_64/struct.__m256f.html
[`__m256d`]: https://doc.rust-lang.org/stable/core/arch/x86_64/struct.__m256d.html
[`__m512`]: https://doc.rust-lang.org/stable/core/arch/x86_64/struct.__m512.html
[`__m512i`]: https://doc.rust-lang.org/stable/core/arch/x86_64/struct.__m512i.html
[`__m512f`]: https://doc.rust-lang.org/stable/core/arch/x86_64/struct.__m512f.html
[`__m512d`]: https://doc.rust-lang.org/stable/core/arch/x86_64/struct.__m512d.html

## The `used` attribute

r[abi.used]

```abnf
MetaItemUsed := "used"
```

r[abi.used.syntax]
The *`used` attribute* may be specified as a built-in attribute, using the [_MetaWord_] syntax.

r[abi.used.restriction]
The `used` attribute shall only be applied to a `static` item. It shall not be applied to a `static` item declared within an [`extern` block].

r[abi.used.application]
A `static` item with the `used` attribute is an *exported item*. 

> [!NOTE]
> *exported items* will generally appear in the output when linking a library crate, and will generally be available when linking a binary crate as a global symbol.
> The `used` attribute does not give the `static` item a *linkage name*, and thus does not disable name mangling. It may be used to place data into a given section that is referenced by the linker via the input section, without regard to the name of the symbol.
> Due to toolchain limitations, it is not guaranteed that a `#[used]` static will appear in the final output when linking a binary, or when linking an rlib/staticlib crate into a `dylib` or `cdylib`. 

``` rust
// foo.rs

// This is kept because of `#[used]`:
#[used]
static FOO: u32 = 0;

// This is removable because it is unused:
#[allow(dead_code)]
static BAR: u32 = 0;

// This is kept because it is publicly reachable:
pub static BAZ: u32 = 0;

// This is kept because it is referenced by a public, reachable function:
static QUUX: u32 = 0;

pub fn quux() -> &'static u32 {
    &QUUX
}

// This is removable because it is referenced by a private, unused (dead) function:
static CORGE: u32 = 0;

#[allow(dead_code)]
fn corge() -> &'static u32 {
    &CORGE
}
```

``` console
$ rustc -O --emit=obj --crate-type=rlib foo.rs

$ nm -C foo.o
0000000000000000 R foo::BAZ
0000000000000000 r foo::FOO
0000000000000000 R foo::QUUX
0000000000000000 T foo::quux
```

## Symbol naming 

r[abi.symbol-name]

```abnf
MetaItemNoMangle := "no_mangle"
MetaItemExportName := "export_name" "=" ([STRING_LITERAL] | [RAW_STRING_LITERAL])
```

r[abi.symbol-name.names]
The *`no_mangle` attribute* and the *`export_name` attribute* shall only be applied to a `static` or `fn` item. The *`export_name` attribute* shall not be applied to an item declared within an [`extern` block]. 

> [!NOTE]
> They may be applied to an associated `fn` of an `impl` block.

r[abi.symbol-name.exported]
An item with either the *`no_mangle` attrbute* or the *`export_name` attribute* is an *exported item*.

r[abi.symbol-name.no_mangle]
The *`no_mangle` attribute* may be specified as a built-in attribute, using the [_MetaWord_] syntax. The *export name* of an item with the *`no_mangle` attribute* is the declaration name of the item.

r[abi.symbol-name.export_name]
The *`export_name` attribute* may be specified as a built-in attribute, using the [_MetaNameValueStr_] syntax. The *export name* of an item with the *`no_mangle` attribute* is the content of `STRING_LITERAL`. 



## The `link_section` attribute

r[abi.link_section]

```abnf
MetaItemLinkSection := "link_section" "=" ([STRING_LITERAL] | [RAW_STRING_LITERAL])
```

r[abi.link_section.syntax]
The *`link_section` attribute* may be specified as a built-in attribute, using the [_MetaNameValueStr_] syntax. 

r[abi.link_section.restriction]
The *`link_section` attribute* shall be aplied to a `static` or `fn` item.

r[abi.link_section.def]
An item with the *`link_section` attribute* is placed in the specified section when linking. The section specified shall not violate the constraints on section names on the target, and shall not be invalid for the item type, no diagnostic is required.

> [!NOTE]
> A section name may be invalid if it violates the requirements for the item type, for example, an `fn` item must be placed in an executable section, and a mutable static item (`static mut` or one containing an `UnsafeCell`) must be placed in a writable section.
> The required format and any restrictions on section names are target-specific.
>
> The result of using an invalid section name may be that the section is placed into the section but cannot be used as applicable, or that the section is given additional attributes that may be incompatible when linking.

<!-- no_run: don't link. The format of the section name is platform-specific. -->
```rust,no_run
#[unsafe(no_mangle)]
#[unsafe(link_section = ".example_section")]
pub static VAR1: u32 = 1;
```

> [!TARGET-SPECIFIC]
> On ELF Platforms, the standard section names, and their attributes are:
> * `.text`: Readable and Executable,
> * `.rodata`: Readable,
> * `.data`: Readable and Writable,
> * `.bss`: Readable and Writable - Uninitialized data,
> * `.tdata`: Readable and Writable - Thread-local,
> * `.tbss`: Readable and Writable - Uninitialized and Thread-local.
> 
> This is not an exhaustive list, and generally extended versions of these section names such as `.text.foo`, are also defined with the same properties as the base section.
> 
> 



[_MetaWord_]: attributes.md#meta-item-attribute-syntax
[_MetaNameValueStr_]: attributes.md#meta-item-attribute-syntax
[`static` items]: items/static-items.md
[attribute]: attributes.md
[extern functions]: items/functions.md#extern-function-qualifier
[`extern` block]: items/external-blocks.md
[function]: items/functions.md
[item]: items.md
[static]: items/static-items.md
[target_feature]: attributes/codegen.md#the-target_feature-attribute