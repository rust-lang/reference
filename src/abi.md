r[abi]
# Application Binary Interface (ABI)

r[abi.intro]
This section documents features that affect the ABI of the compiled output of
a crate.

See *[extern functions]* for information on specifying the ABI for exporting
functions. See *[external blocks]* for information on specifying the ABI for
linking external libraries.

r[abi.used]
## The `used` attribute

r[abi.used.intro]
The *`used` attribute* can only be applied to [`static` items]. This [attribute] forces the
compiler to keep the variable in the output object file (.o, .rlib, etc. excluding final binaries)
even if the variable is not used, or referenced, by any other item in the crate.
However, the linker is still free to remove such an item.

Below is an example that shows under what conditions the compiler keeps a `static` item in the
output object file.

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

r[abi.no_mangle]
## The `no_mangle` attribute

r[abi.no_mangle.intro]
The *`no_mangle` attribute* may be used on any [item] to disable standard
symbol name mangling. The symbol for the item will be the identifier of the
item's name.

r[abi.no_mangle.publicly-exported]
Additionally, the item will be publicly exported from the produced library or
object file, similar to the [`used` attribute](#the-used-attribute).

r[abi.no_mangle.unsafe]
This attribute is unsafe as an unmangled symbol may collide with another symbol
with the same name (or with a well-known symbol), leading to undefined behavior.

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
The *`link_section` attribute* specifies the section of the object file that a
[function] or [static]'s content will be placed into.

r[abi.link_section.syntax]
The `link_section` attribute uses the [MetaNameValueStr] syntax to specify the section name.

<!-- no_run: don't link. The format of the section name is platform-specific. -->
```rust,no_run
#[unsafe(no_mangle)]
#[unsafe(link_section = ".example_section")]
pub static VAR1: u32 = 1;
```

r[abi.link_section.unsafe]
This attribute is unsafe as it allows users to place data and code into sections
of memory not expecting them, such as mutable data into read-only areas.

r[abi.link_section.edition2024]
> [!EDITION-2024]
> Before the 2024 edition it is allowed to use the `link_section` attribute without the `unsafe` qualification.

r[abi.export_name]
## The `export_name` attribute

r[abi.export_name.intro]
The *`export_name` attribute* specifies the name of the symbol that will be
exported on a [function] or [static].

r[abi.export_name.syntax]
The `export_name `attribute uses the [MetaNameValueStr] syntax to specify the symbol name.

```rust
#[unsafe(export_name = "exported_symbol_name")]
pub fn name_in_rust() { }
```

r[abi.export_name.unsafe]
This attribute is unsafe as a symbol with a custom name may collide with another
symbol with the same name (or with a well-known symbol), leading to undefined
behavior.

r[abi.export_name.edition2024]
> [!EDITION-2024]
> Before the 2024 edition it is allowed to use the `export_name` attribute without the `unsafe` qualification.

[`static` items]: items/static-items.md
[attribute]: attributes.md
[extern functions]: items/functions.md#extern-function-qualifier
[external blocks]: items/external-blocks.md
[function]: items/functions.md
[item]: items.md
[static]: items/static-items.md
