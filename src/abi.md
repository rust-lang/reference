r[abi]
# Application binary interface (ABI)

r[abi.intro]
This section documents features that affect the ABI of the compiled output of a crate.

See *[extern functions]* for information on specifying the ABI for exporting functions. See *[external blocks]* for information on specifying the ABI for linking external libraries.

<!-- template:attributes -->
r[abi.used]
## The `used` attribute

r[abi.used.intro]
The *`used` [attribute]* forces a [static] to be kept in the output object file (.o, .rlib, etc., excluding final binaries) even if it's never used or referenced by any other item in the crate. The linker, however, is still free to remove it.

> [!EXAMPLE]
> ```rust
> // lib.rs
>
> // This is kept because of `#[used]`.
> #[used]
> static S1: u8 = 0;
>
> // This is removable because it's unused.
> #[allow(dead_code)]
> static S2: u8 = 0;
>
> // This is kept because it's publicly reachable.
> pub static S3: u8 = 0;
>
> // This is kept because it's referenced by a publicly
> // reachable function.
> static S4: u8 = 0;
> #[unsafe(no_mangle)] pub fn f4() -> &'static u8 { &S4 }
>
> // This is removable because it's referenced only by a
> // private, unused (dead) function.
> static S5: u8 = 0;
> #[allow(dead_code)]
> fn f5() -> &'static u8 { &S5 }
> ```
>
> ```console
> $ rustc -O --emit=obj --crate-type=rlib lib.rs
> $ LC_ALL=C nm -C lib.o
> 0000000000000000 R lib::S1
> 0000000000000000 R lib::S3
> 0000000000000000 r lib::S4
> 0000000000000000 T f4
> ```

r[abi.used.syntax]
The `used` attribute uses the [MetaWord] syntax.

r[abi.used.allowed-positions]
The `used` attribute may only be applied to [`static` items].

r[abi.used.duplicates]
Only the first use of `used` on an item has effect.

> [!NOTE]
> `rustc` lints against any use following the first.

r[abi.no_mangle]
## The `no_mangle` attribute

r[abi.no_mangle.intro]
The *`no_mangle` attribute* may be used on any [item] to disable standard symbol name mangling. The symbol for the item will be the identifier of the item's name.

r[abi.no_mangle.publicly-exported]
Additionally, the item will be publicly exported from the produced library or object file, similar to the [`used` attribute](#the-used-attribute).

r[abi.no_mangle.unsafe]
This attribute is unsafe as an unmangled symbol may collide with another symbol with the same name (or with a well-known symbol), leading to undefined behavior.

```rust
#[unsafe(no_mangle)]
extern "C" fn foo() {}
```

r[abi.no_mangle.edition2024]
> [!EDITION-2024]
> Before the 2024 edition it is allowed to use the `no_mangle` attribute without the `unsafe` qualification.

r[abi.link_section]
## The `link_section` attribute

r[abi.link_section.intro]
The *`link_section` attribute* specifies the section of the object file that a [function] or [static]'s content will be placed into.

r[abi.link_section.syntax]
The `link_section` attribute uses the [MetaNameValueStr] syntax to specify the section name.

<!-- no_run: don't link. The format of the section name is platform-specific. -->
```rust,no_run
# #[cfg(target_os = "linux")] {
#[unsafe(no_mangle)]
#[unsafe(link_section = ".example_section")]
pub static VAR1: u32 = 1;
# }
```

r[abi.link_section.unsafe]
This attribute is unsafe as it allows users to place data and code into sections of memory not expecting them, such as mutable data into read-only areas.

r[abi.link_section.duplicates]
Only the first use of `link_section` on an item has effect.

> [!NOTE]
> `rustc` lints against any use following the first with a future-compatibility warning. This may become an error in the future.

r[abi.link_section.edition2024]
> [!EDITION-2024]
> Before the 2024 edition it is allowed to use the `link_section` attribute without the `unsafe` qualification.

<!-- template:attributes -->
r[abi.export_name]
## The `export_name` attribute

r[abi.export_name.intro]
The *`export_name` [attribute]* specifies the name of the symbol that will be exported on a [function] or [static].

> [!EXAMPLE]
> ```rust
> #[unsafe(export_name = "exported_symbol_name")]
> pub fn name_in_rust() { }
> ```

r[abi.export_name.syntax]
The `export_name` attribute uses the [MetaNameValueStr] syntax to specify the symbol name.

r[abi.export_name.allowed-positions]
The `export_name` attribute may only be applied to:

- [Static items][items.static]
- [Free functions][items.fn]
- [Inherent associated functions][items.associated.fn]
- [Trait impl functions][items.impl.trait]

> [!NOTE]
> `rustc` currently ignores `export_name` in some positions, but this may be rejected in the future.

r[abi.export_name.duplicates]
Only the first use of `export_name` on an item has effect.

> [!NOTE]
> `rustc` lints against any use following the first with a future-compatibility warning. This may become an error in the future.

r[abi.export_name.unsafe]
The `export_name` attribute must be marked with [`unsafe`][attributes.safety] because a symbol with a custom name may collide with another symbol with the same name (or with a well-known symbol), leading to undefined behavior.

r[abi.export_name.edition2024]
> [!EDITION-2024]
> Before the 2024 edition it is allowed to use the `export_name` attribute without the `unsafe` qualification.

r[abi.export_name.no_mangle]
If `export_name` is used with [`no_mangle`][abi.no_mangle], then the `export_name` is used instead.

r[abi.export_name.publicly-exported]
The `export_name` attribute causes the symbol to be publicly exported from the produced library or object file, similar to the [`used` attribute](#the-used-attribute).

r[abi.export_name.null]
The exported name must not contain a [NUL] character.

r[abi.export_name.generic]
`export_name` has no effect on generic items.

[attribute]: attributes.md
[extern functions]: items/functions.md#extern-function-qualifier
[external blocks]: items/external-blocks.md
[function]: items/functions.md
[item]: items.md
[`static` items]: items.static
[static]: items/static-items.md
