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
* There exists a type `V`, such that `T` is *abi compatible* with `V` an `V` is *abi compatible* with `U`,

> [!NOTE]
> These properties are respectively called "reflexivity", "symmetry", and "transitivity". They ensure that *abi compatibility* is an equivalence relation.

r[abi.compatibility.integer]
Two [integer types] are *abi compatible* if they have the same size and the same signedness

> [!NOTE]
> In particular, `usize` is *abi compatible* with `uN`, and `isize` is *abi compatible* with `iN` where `N` is the target_pointer_width.
> Two integer types with different signedness, such as `u8` and `i8` are not *abi compatible*.

```rust
#[cfg(target_pointer_width="32")]
fn foo(x: u32) -> u32{
    x
}
#[cfg(target_pointer_width="64")]
fn foo(x: u64) -> u64{
    x
}

fn main(){
    let f: fn(usize)->usize = unsafe{core::mem::transmute(foo as fn(_)->_)};
    let x = 0usize;
    let y = f(x);
    assert_eq!(x,y);
}
```

r[abi.compatibility.char]
The type [`char`] is *abi compatible* with the type [`u32`][integer types].

```rust
fn foo(x: char) -> u32{
    x as u32
}

fn main(){
    let f: fn(u32)->char = unsafe{core::mem::transmute(foo as fn(_)->_)};
    let x = b'A' as u32; // ascii character indecies are the same as Unicode character indecies
    let y = f(x);
    assert_eq!(y, 'A');
}
```

r[abi.compatibility.pointer]
Two [pointer types], `*mut T` and `*const U`, are *abi compatible* if the *metadata type*s of `T` and `U` are the same type.

> [!NOTE]
> [`core::marker::Sized`] types have a *metadata type* of `()`.

> [!NOTE]
> With transitivity, this applies regardless of the mutability of either pointer type

```rust
unsafe fn foo(x: *mut u32){
   unsafe{x.write(5);}
}

fn main(){
    let f: unsafe fn(*mut ()) = unsafe{core::mem::transmute(foo as unsafe fn(_))}; // Type Erase the function
    let mut val: u32 = 0;
    let ptr = core::ptr::addr_of_mut!(val).cast::<()>(); // Get Opaque Userdata from somewhere
    unsafe{f(ptr);}
    assert_eq!(val, 5);
}
```

r[abi.compatibility.reference-box]
The types [`&T`], [`&mut T`], [`alloc::boxed::Box<T>`], and [`core::ptr::NonNull<T>`], are *abi compatible* with `*const T`

> [!NOTE]
> With transitivity, they are also *abi compatible* with each other, and with `*mut T`, as well as references/`Box` to different types that have the same *metadata type*.

```rust
fn foo(x: &mut i32){
   *x = 5;
}

fn main(){
    let f: unsafe fn(*mut ()) = unsafe{core::mem::transmute(foo as fn(_))}; // Type Erase the function
    let mut val = 0;
    let ptr = core::ptr::addr_of_mut!(val).cast::<()>(); // Get Opaque Userdata from somewhere
    unsafe{f(ptr);}
    assert_eq!(val, 5);
}
```

r[abi.compatibility.core]
The types [`core::mem::MaybeUninit<T>`], [`core::cell::UnsafeCell<T>`], and [`core::num::NonZero<T>`], are *abi compatible* with `T`.

r[abi.compatibility.transparent]
A [`struct`] declared with the `transparent` representation is *abi compatible* with its field that does not have size 0 and alignment 1, if such a field exists.

r[abi.compatibilty.zst]
Two types, `T` and `U`, are *abi compatible* if both have size 0 and alignment 1.

r[abi.compatibility.option]
If `T` is a type listed in [layout.enum.option](https://doc.rust-lang.org/stable/core/option/index.html#representation), then given `S` is a type with size 0 and alignment 1, `T` is *abi compatible* with the types [`core::option::Option<T>`], [`core::result::Result<T,S>`], and [`core::result::Result<S,T>`].


r[abi.compatibility.fn-ptr]
An [`fn`-ptr type] `T` is *abi compatible* with an [`fn`-ptr type] `U` if `T` and `U` have *abi compatible* tags.

r[abi.compatibility.extern-tag]
Two [abi tags][abi tag] are *abi compatible* if:
* They are the same string, or
* One tag is `"X"`, and the other is `"X-unwind"`

> [!NOTE]
> e.g. `extern "C"` and `extern "C-unwind"` are compatible with each other.

r[abi.compatibility.signature]
Two function signatures are compatible if:
* The [abi tags][abi tag] of both signatures are *abi compatible*,
* They have the same number of parameters, excluding C-varargs,
* Each parameter of both signatures, in order, are *abi compatible*, and
* Either both signatures have C-varargs, or neither signature does.

> [!NOTE]
> A signature is compatible with itself.

r[abi.compatibility.simd-abi]
A type has *simd abi requirements* if:
* It is a type declared with the standard-library repr-attrbute `simd`,
* It is a aggregate type[^1], which has a type with *simd abi requirements* as a field.

> [!NOTE]
> The `repr(simd)` attribute cannot be used by Rust code, only by the standard library.

r[abi.compatibility.simd-target-feature]
A type with *simd abi requirements* may have one or more [*salient target features*][target_feature] . In the case of an aggregate type, the set of [*salient target features*][target_feature] is the union of the set of [*salient target features*][target_feature] of each field with *simd abi requirements*.

> [!TARGET-SPECIFIC]
> On x86 and x86-64, the [*salient target features*][target_feature] of the `simd` types are:
> * [`__m128`], [`__m128i`], [`__m128f`], and [`__m128d`] (128-bit vector types): `sse`
> * [`__m256`], [`__m256i`], [`__m256f`], and [`__m256d`] (256-bit vector types): `avx`
> * [`__m512`], [`__m512i`], [`__m512f`], and [`__m512d`] (512-bit vector types): `avx512f` and `avx512vl`

r[abi.compatibility.call]
A call to a function `f` via a function item or function pointer with a given signature `S` is valid if and only if the signature of the definition `f` is *compatible* with the signature `S`, and:
* The ABI tag of the signature is `extern "Rust"`, or
* If any parameter type, the return type, or the type of any argument passed via C-varargs has *simd abi requirements*, each [*salient target features*][target_feature] of that type is either set at both the definition site of the function, and at the call site, or is set at neither site.

The behaviour a call that is not valid is undefined.

> [!NOTE]
> When parameter/return types do not exactly match, they are converted as though by calling [`core::mem::transmute`]. The representation and validity requirements of the type in the definition/return site still apply, for example, passing `0` to a function pointer `fn(u32)` that points to a function declared as `fn foo(x: NonZeroU32)` is undefined behaviour.

> [!NOTE]
> the ABI tag `extern "Rust"` is the default when the `extern` keyword is not used (either to declare the function within an [`extern` block], or as a [function qualifier][extern functions]). Thus it is safe to call most functions that use simd types.

[^1]: The aggregate types, for the purposes of this clause, are [`struct`] types, [`enum`] types, [`union`] types, and [array] types.

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

```rust
#[used]
static FOO: u32 = 0;
```

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

```rust
#[no_mangle]
extern "C" fn foo(x: i32) -> i32 {
    x + 1
}

#[export_name = "bar"]
extern "C" fn baz(x: i32) -> i32 {
    x + 2
}
```

```rust,compile_fail
extern "C" {
    #[export_name = "foo"]
    fn __foo(x: i32) -> i32;
}
```

> [!NOTE]
> They may be applied to an associated `fn` of an `impl` block.


r[abi.symbol-name.exported]
An item with either the *`no_mangle` attrbute* or the *`export_name` attribute* is an *exported item*.

r[abi.symbol-name.no_mangle]
The *`no_mangle` attribute* may be specified as a built-in attribute, using the [_MetaWord_] syntax. The *export name* of an item with the *`no_mangle` attribute* is the declaration name of the item.

```rust
extern "C" {
    fn bar() -> i32;
}
mod inner{
    #[no_mangle]
    extern "C" fn bar() -> i32 {
        0
    }
}

fn main() {
    let y = unsafe {bar()};
    assert_eq!(y,0);
}
```

r[abi.symbol-name.export_name]
The *`export_name` attribute* may be specified as a built-in attribute, using the [_MetaNameValueStr_] syntax. The *export name* of an item with the *`no_mangle` attribute* is the content of `STRING_LITERAL`.

```rust
extern "C" {
    fn bar() -> i32;
}
mod inner{
    #[export_name = "bar"]
    extern "C" fn __some_other_item_name() -> i32 {
        0
    }
}

fn main(){
    let y = unsafe {bar()};
    assert_eq!(y,0);
}
```

r[abi.symbol-name.safety]
These attributes are unsafe as an unmangled symbol may collide with another symbol
with the same name (or with a well-known symbol), leading to undefined behavior.

```rust
#[unsafe(no_mangle)]
extern "C" fn foo() {}
```

## The `link_section` attribute

r[abi.link_section]

```abnf
MetaItemLinkSection := "link_section" "=" ([STRING_LITERAL] | [RAW_STRING_LITERAL])
```

r[abi.link_section.syntax]
The *`link_section` attribute* may be specified as a built-in attribute, using the [_MetaNameValueStr_] syntax.

r[abi.link_section.application]
The *`link_section` attribute* shall be applied to a `static` or `fn` item.


r[abi.link_section.def]
An item with the *`link_section` attribute* is placed in the specified section when linking. The section specified shall not violate the constraints on section names on the target, and shall not be invalid for the item type, no diagnostic is required.

> [!NOTE]
> A section name may be invalid if it violates the requirements for the item type, for example, an `fn` item must be placed in an executable section, and a mutable static item (`static mut` or one containing an `UnsafeCell`) must be placed in a writable section.
> The required format and any restrictions on section names are target-specific.
>
> The result of using an invalid section name may be that the section is placed into the section but cannot be used as applicable, or that the section is given additional attributes that may be incompatible when linking.

r[abi.link_section.safety]
This attribute is unsafe as it allows users to place data and code into sections of memory not expecting them, such as mutable data into read-only areas.

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


[_MetaWord_]: attributes.md#meta-item-attribute-syntax
[_MetaNameValueStr_]: attributes.md#meta-item-attribute-syntax
[`static` items]: items/static-items.md
[attribute]: attributes.md
[extern functions]: items/functions.md#extern-function-qualifier
[`extern` block]: items/external-blocks.md
[abi tag]: items/external-blocks.md#abi
[function]: items/functions.md
[`fn`-ptr type]: types/function-pointer.md
[integer types]: types/numeric.md#integer-types
[`char`]: types/textual.md
[pointer types]: types/pointer.md#raw-pointers-const-and-mut
[`&T`]: types/pointer.md#shared-references-
[`&mut T`]: types/pointer.md#mutable-references-mut
[`struct`]: types/struct.md
[`enum`]: types/enum.md
[`union`]: types/union.md
[array]: types/array.md
[item]: items.md
[static]: items/static-items.md
[target_feature]: attributes/codegen.md#the-target_feature-attribute
