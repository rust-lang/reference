r[items.extern]
# External blocks

r[items.extern.syntax]
```grammar,items
ExternBlock ->
    `unsafe`?[^unsafe-2024] `extern` Abi? `{`
        InnerAttribute*
        ExternalItem*
    `}`

ExternalItem ->
    OuterAttribute* (
        MacroInvocationSemi
      | Visibility? StaticItem
      | Visibility? Function
    )
```

[^unsafe-2024]: Starting with the 2024 Edition, the `unsafe` keyword is required semantically.

r[items.extern.intro]
External blocks provide _declarations_ of items that are not _defined_ in the
current crate and are the basis of Rust's foreign function interface. These are
akin to unchecked imports.

r[items.extern.allowed-kinds]
Two kinds of item _declarations_ are allowed in external blocks: [functions] and
[statics].

r[items.extern.safety]
Calling unsafe functions or accessing unsafe statics that are declared in external blocks is only allowed in an [`unsafe` context].

r[items.extern.namespace]
The external block defines its functions and statics in the [value namespace] of the module or block where it is located.

r[items.extern.unsafe-required]
The `unsafe` keyword is semantically required to appear before the `extern` keyword on external blocks.

r[items.extern.edition2024]
> [!EDITION-2024]
> Prior to the 2024 edition, the `unsafe` keyword is optional. The `safe` and `unsafe` item qualifiers are only allowed if the external block itself is marked as `unsafe`.

r[items.extern.fn]
## Functions

r[items.extern.fn.body]
Functions within external blocks are declared in the same way as other Rust
functions, with the exception that they must not have a body and are instead
terminated by a semicolon.

r[items.extern.fn.param-patterns]
Patterns are not allowed in parameters, only [IDENTIFIER] or `_` may be used.

r[items.extern.fn.qualifiers]
The `safe` and `unsafe` function qualifiers are
allowed, but other function qualifiers (e.g. `const`, `async`, `extern`) are
not.

r[items.extern.fn.foreign-abi]
Functions within external blocks may be called by Rust code, just like
functions defined in Rust. The Rust compiler automatically translates between
the Rust ABI and the foreign ABI.

r[items.extern.fn.safety]
A function declared in an extern block is implicitly `unsafe` unless the `safe`
function qualifier is present.

r[items.extern.fn.fn-ptr]
When coerced to a function pointer, a function declared in an extern block has
type `extern "abi" for<'l1, ..., 'lm> fn(A1, ..., An) -> R`, where `'l1`,
... `'lm` are its lifetime parameters, `A1`, ..., `An` are the declared types of
its parameters, `R` is the declared return type.

r[items.extern.static]
## Statics

r[items.extern.static.intro]
Statics within external blocks are declared in the same way as [statics] outside of external blocks,
except that they do not have an expression initializing their value.

r[items.extern.static.safety]
Unless a static item declared in an extern block is qualified as `safe`, it is `unsafe` to access that item, whether or
not it's mutable, because there is nothing guaranteeing that the bit pattern at the static's
memory is valid for the type it is declared with, since some arbitrary (e.g. C) code is in charge
of initializing the static.

r[items.extern.static.mut]
Extern statics can be either immutable or mutable just like [statics] outside of external blocks.

r[items.extern.static.read-only]
An immutable static *must* be initialized before any Rust code is executed. It is not enough for
the static to be initialized before Rust code reads from it.
Once Rust code runs, mutating an immutable static (from inside or outside Rust) is UB,
except if the mutation happens to bytes inside of an `UnsafeCell`.

r[items.extern.abi]
## ABI

r[items.extern.abi.intro]
By default external blocks assume that the library they are calling uses the
standard C ABI on the specific platform. Other ABIs may be specified using an
`abi` string, as shown here:

```rust
// Interface to the Windows API
unsafe extern "system" { }
```

r[items.extern.abi.standard]
The following ABI strings are supported on all platforms:

r[items.extern.abi.rust]
* `unsafe extern "Rust"` --- The default ABI when you write a normal `fn foo()` in any Rust code.

r[items.extern.abi.c]
* `unsafe extern "C"` --- This is the same as `extern fn foo()`; whatever the default your C compiler supports.

r[items.extern.abi.system]
* `unsafe extern "system"` --- Usually the same as `extern "C"`, except on Win32, in which case it's `"stdcall"`, or what you should use to link to the Windows API itself.

r[items.extern.abi.unwind]
* `extern "C-unwind"` and `extern "system-unwind"` --- Identical to `"C"` and `"system"`, respectively, but with [different behavior][unwind-behavior] when the callee unwinds (by panicking or throwing a C++ style exception).

r[items.extern.abi.platform]
There are also some platform-specific ABI strings:

r[items.extern.abi.cdecl]
* `unsafe extern "cdecl"` --- The default for x86_32 C code.
  * Only available on x86_32 targets.

r[items.extern.abi.stdcall]
* `unsafe extern "stdcall"` --- The default for the Win32 API on x86_32.
  * Only available on x86_32 targets.

r[items.extern.abi.win64]
* `unsafe extern "win64"` --- The default for C code on x86_64 Windows.
  * Only available on x86_64 targets.

r[items.extern.abi.sysv64]
* `unsafe extern "sysv64"` --- The default for C code on non-Windows x86_64.
  * Only available on x86_64 targets.

r[items.extern.abi.aapcs]
* `unsafe extern "aapcs"` --- The default for ARM.
  * Only available on ARM32 targets.

r[items.extern.abi.fastcall]
* `unsafe extern "fastcall"` --- The `fastcall` ABI --- corresponds to MSVC's `__fastcall` and GCC and clang's `__attribute__((fastcall))`.
  * Only available on x86_32 targets.

r[items.extern.abi.thiscall]
* `unsafe extern "thiscall"` --- The default for C++ member functions on x86_32 MSVC --- corresponds to MSVC's `__thiscall` and GCC and clang's `__attribute__((thiscall))`.
  * Only available on x86_32 targets.

r[items.extern.abi.efiapi]
* `unsafe extern "efiapi"` --- The ABI used for [UEFI] functions.
  * Only available on x86 and ARM targets (32bit and 64bit).

r[items.extern.abi.platform-unwind-variants]
Like `"C"` and `"system"`, most platform-specific ABI strings also have a [corresponding `-unwind` variant][unwind-behavior]; specifically, these are:

* `"aapcs-unwind"`
* `"cdecl-unwind"`
* `"fastcall-unwind"`
* `"stdcall-unwind"`
* `"sysv64-unwind"`
* `"thiscall-unwind"`
* `"win64-unwind"`

r[items.extern.variadic]
## Variadic functions

Functions within external blocks may be variadic by specifying `...` as the
last argument. The variadic parameter may optionally be specified with an
identifier.

```rust
unsafe extern "C" {
    unsafe fn foo(...);
    unsafe fn bar(x: i32, ...);
    unsafe fn with_name(format: *const u8, args: ...);
    // SAFETY: This function guarantees it will not access
    // variadic arguments.
    safe fn ignores_variadic_arguments(x: i32, ...);
}
```

> [!WARNING]
> The `safe` qualifier should not be used on a function in an `extern` block unless that function guarantees that it will not access the variadic arguments at all. Passing an unexpected number of arguments or arguments of unexpected type to a variadic function may lead to [undefined behavior][undefined].

r[items.extern.variadic.conventions]
Variadic parameters can only be specified within `extern` blocks with the following ABI strings or their corresponding [`-unwind` variants][items.fn.extern.unwind]:

- `"aapcs"`
- `"C"`
- `"cdecl"`
- `"efiapi"`
- `"sysv64"`
- `"win64"`

r[items.extern.attributes]
## Attributes on extern blocks

r[items.extern.attributes.intro]
The following [attributes] control the behavior of external blocks.

r[items.extern.attributes.link]
### The `link` attribute

r[items.extern.attributes.link.intro]
The *`link` [attribute][attributes]* specifies the name of a native library that the compiler should link with for the items within an `extern` block.

> [!EXAMPLE]
> <!-- ignore: requires extern linking -->
> ```rust,ignore
> #[link(name = "crypto")]
> unsafe extern "C" {
>     // …
> }
>
> #[link(name = "CoreFoundation", kind = "framework")]
> unsafe extern "C" {
>     // …
> }
>
> #[link(wasm_import_module = "foo")]
> unsafe extern "C" {
>     // …
> }
> ```

r[items.extern.attributes.link.syntax]
The `link` attribute uses the [MetaListNameValueStr] syntax to specify its inputs. It accepts the following keys:

- [`name`][items.extern.attributes.link.name] --- the name of the native library to link.
- [`kind`][items.extern.attributes.link.kind] --- the kind of library.
- [`modifiers`][items.extern.attributes.link.modifiers] --- modifiers that change the behavior of how the library is linked.
- [`wasm_import_module`][items.extern.attributes.link.wasm_import_module] --- specifies the WebAssembly module name.
- [`import_name_type`][items.extern.attributes.link.import_name_type] --- on x86 Windows, this changes how functions are named.

None of the keys may be specified more than once.

r[items.extern.attributes.link.allowed-positions]
The `link` attribute may be applied to [`extern` blocks].

> [!NOTE]
> `rustc` currently warns in other positions, but this may be rejected in the future.

r[items.extern.attributes.link.duplicates]
The `link` attribute may be specified multiple times, and the corresponding linking instructions for each attribute will be passed to the linker.

r[items.extern.attributes.link.empty-block]
It is valid to add the `link` attribute on an empty extern block. You can use this to satisfy the linking requirements of extern blocks elsewhere in your code (including upstream crates) instead of adding the attribute to each extern block.

r[items.extern.attributes.link.name]
#### The `name` key

r[items.extern.attributes.link.name.intro]
The `name` key specifies the name of the library to link.

<!-- TODO: I think it would be helpful to describe how `name` works with some examples of how it works in various situations or platforms (for example, how prefixes and suffixes work). -->

<!-- TODO: Would be good to specify what happens if the same name is specified multiple times in different attributes. My understanding from https://github.com/rust-lang/rust/blob/c68340350c78eea402c4a85f8d9c1b7d3d607635/compiler/rustc_metadata/src/native_libs.rs#L551-L620 is that it de-duplicates it and uses the "last" instance. I'm not sure if we want to explicitly document that? -->

r[items.extern.attributes.link.name.requirement]
The `name` key must be included unless `wasm_import_module` is used.

r[items.extern.attributes.link.kind]
#### The `kind` key

r[items.extern.attributes.link.kind.intro]
The `kind` key specifies the kind of the library.

r[items.extern.attributes.link.kind.dylib]
The `dylib` kind indicates a dynamic library. This is the default if `kind` is not specified.

> [!EXAMPLE]
> <!-- ignore: requires extern linking -->
> ```rust,ignore
> #[link(name = "example", kind = "dylib")]
> unsafe extern "C" {}
> ```

r[items.extern.attributes.link.kind.static]
The `static` kind indicates a static library.

> [!EXAMPLE]
> <!-- ignore: requires extern linking -->
> ```rust,ignore
> #[link(name = "example", kind = "static")]
> unsafe extern "C" {}
> ```

r[items.extern.attributes.link.kind.framework]
The `framework` kind indicates a macOS framework. This is only valid for macOS targets.

> [!EXAMPLE]
> <!-- ignore: requires extern linking -->
> ```rust,ignore
> #[link(name = "CoreFoundation", kind = "framework")]
> unsafe extern "C" {}
> ```

r[items.extern.attributes.link.kind.raw-dylib]
The `raw-dylib` kind indicates a dynamic library where the compiler will generate an import library to link against (see [`dylib` versus `raw-dylib`] below for details). This is only valid for Windows targets.

> [!EXAMPLE]
> <!-- ignore: requires extern linking -->
> ```rust,ignore
> #[link(name = "example", kind = "raw-dylib")]
> unsafe extern "C" {}
> ```

> [!NOTE]
> ##### `dylib` versus `raw-dylib`
>
> On Windows, linking against a dynamic library requires that an import library is provided to the linker. This is a special static library that declares all of the symbols exported by the dynamic library in such a way that the linker knows that they have to be dynamically loaded at runtime.
>
> Specifying `kind = "dylib"` instructs the Rust compiler to link an import library based on the `name` key. The linker will then use its normal library resolution logic to find that import library. Alternatively, specifying `kind = "raw-dylib"` instructs the compiler to generate an import library during compilation and provide that to the linker instead.

r[items.extern.attributes.link.modifiers]
#### The `modifiers` key

r[items.extern.attributes.link.modifiers.intro]
The optional `modifiers` argument is a way to specify linking modifiers for the library to link.

> [!EXAMPLE]
> <!-- ignore: requires extern linking -->
> ```rust,ignore
> #[link(name = "mylib", kind = "static", modifiers = "+whole-archive")]
> unsafe extern "C" {}
> ```

r[items.extern.attributes.link.modifiers.syntax]
Modifiers are specified as a comma-delimited string with each modifier prefixed with either a `+` or `-` to indicate that the modifier is enabled or disabled, respectively.

r[items.extern.attributes.link.modifiers.once]
The `modifiers` argument may only be specified once.

r[items.extern.attributes.link.modifiers.duplicates]
Duplicate modifiers are not allowed within a `modifiers` argument.

r[items.extern.attributes.link.wasm_import_module]
#### The `wasm_import_module` key

r[items.extern.attributes.link.wasm_import_module.behavior]
The `wasm_import_module` key may be used to specify the [WebAssembly module] name for the items within an `extern` block when importing symbols from the host environment. The default module name is `env` if `wasm_import_module` is not specified.

r[items.extern.attributes.link.modifiers.bundle]
#### Linking modifiers: `bundle`

r[items.extern.attributes.link.modifiers.bundle.allowed-kinds]
This modifier is only compatible with the `static` linking kind. Using any other kind will result in a compiler error.

r[items.extern.attributes.link.modifiers.bundle.behavior]
When building a rlib or staticlib `+bundle` means that the native static library will be packed into the rlib or staticlib archive, and then retrieved from there during linking of the final binary.

r[items.extern.attributes.link.modifiers.bundle.behavior-negative]
When building a rlib `-bundle` means that the native static library is registered as a dependency of that rlib "by name", and object files from it are included only during linking of the final binary, the file search by that name is also performed during final linking.

When building a staticlib `-bundle` means that the native static library is simply not included into the archive and some higher level build system will need to add it later during linking of the final binary.

r[items.extern.attributes.link.modifiers.bundle.no-effect]
This modifier has no effect when building other targets like executables or dynamic libraries.

r[items.extern.attributes.link.modifiers.bundle.default]
The default for this modifier is `+bundle`.

More implementation details about this modifier can be found in [`bundle` documentation for rustc].

r[items.extern.attributes.link.modifiers.whole-archive]
#### Linking modifiers: `whole-archive`

r[items.extern.attributes.link.modifiers.whole-archive.allowed-kinds]
This modifier is only compatible with the `static` linking kind. Using any other kind will result in a compiler error.

r[items.extern.attributes.link.modifiers.whole-archive.behavior]
`+whole-archive` means that the static library is linked as a whole archive without throwing any object files away.

r[items.extern.attributes.link.modifiers.whole-archive.default]
The default for this modifier is `-whole-archive`.

More implementation details about this modifier can be found in [`whole-archive` documentation for rustc].

r[items.extern.attributes.link.modifiers.verbatim]
#### Linking modifiers: `verbatim`

r[items.extern.attributes.link.modifiers.verbatim.allowed-kinds]
This modifier is compatible with all linking kinds.

r[items.extern.attributes.link.modifiers.verbatim.behavior]
`+verbatim` means that rustc itself won't add any target-specified library prefixes or suffixes (like `lib` or `.a`) to the library name, and will try its best to ask for the same thing from the linker.

r[items.extern.attributes.link.modifiers.verbatim.behavior-negative]
`-verbatim` means that rustc will either add a target-specific prefix and suffix to the library name before passing it to linker, or won't prevent linker from implicitly adding it.

r[items.extern.attributes.link.modifiers.verbatim.default]
The default for this modifier is `-verbatim`.

More implementation details about this modifier can be found in [`verbatim` documentation for rustc].




r[items.extern.attributes.link.import_name_type]
#### The `import_name_type` key

r[items.extern.attributes.link.import_name_type.intro]
On x86 Windows, names of functions are "decorated" (i.e., have a specific prefix and/or suffix added) to indicate their calling convention. For example, a `stdcall` calling convention function with the name `fn1` that has no arguments would be decorated as `_fn1@0`. However, the [PE Format] does also permit names to have no prefix or be undecorated. Additionally, the MSVC and GNU toolchains use different decorations for the same calling conventions which means, by default, some Win32 functions cannot be called using the `raw-dylib` link kind via the GNU toolchain.

r[items.extern.attributes.link.import_name_type.values]
To allow for these differences, when using the `raw-dylib` link kind you may also specify the `import_name_type` key with one of the following values to change how functions are named in the generated import library:

* `decorated`: The function name will be fully-decorated using the MSVC toolchain format.
* `noprefix`: The function name will be decorated using the MSVC toolchain format, but skipping the leading `?`, `@`, or optionally `_`.
* `undecorated`: The function name will not be decorated.

r[items.extern.attributes.link.import_name_type.default]
If the `import_name_type` key is not specified, then the function name will be fully-decorated using the target toolchain's format.

r[items.extern.attributes.link.import_name_type.variables]
Variables are never decorated and so the `import_name_type` key has no effect on how they are named in the generated import library.

r[items.extern.attributes.link.import_name_type.platform-specific]
The `import_name_type` key is only supported on x86 Windows. Using it when targeting other platforms will result in a compiler error.

<!-- template:attributes -->
r[items.extern.attributes.link_name]
### The `link_name` attribute

r[items.extern.attributes.link_name.intro]
The *`link_name` [attribute][attributes]* may be applied to declarations inside an `extern` block to specify the symbol to import for the given function or static.

> [!EXAMPLE]
> ```rust
> unsafe extern "C" {
>     #[link_name = "actual_symbol_name"]
>     safe fn name_in_rust();
> }
> ```

r[items.extern.attributes.link_name.syntax]
The `link_name` attribute uses the [MetaNameValueStr] syntax.

r[items.extern.attributes.link_name.allowed-positions]
The `link_name` attribute may only be applied to a function or static item in an `extern` block.

> [!NOTE]
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

r[items.extern.attributes.link_name.duplicates]
Only the last use of `link_name` on an item has effect.

> [!NOTE]
> `rustc` lints against any use preceding the last. This may become an error in the future.

r[items.extern.attributes.link_name.link_ordinal]
The `link_name` attribute may not be used with the [`link_ordinal`] attribute.

r[items.extern.attributes.link_ordinal]
### The `link_ordinal` attribute

r[items.extern.attributes.link_ordinal.intro]
The *`link_ordinal` attribute* can be applied on declarations inside an `extern`
block to indicate the numeric ordinal to use when generating the import library
to link against. An ordinal is a unique number per symbol exported by a dynamic
library on Windows and can be used when the library is being loaded to find
that symbol rather than having to look it up by name.

> [!WARNING]
> `link_ordinal` should only be used in cases where the ordinal of the symbol is known to be stable: if the ordinal of a symbol is not explicitly set when its containing binary is built then one will be automatically assigned to it, and that assigned ordinal may change between builds of the binary.

```rust
# #[cfg(all(windows, target_arch = "x86"))]
#[link(name = "exporter", kind = "raw-dylib")]
unsafe extern "stdcall" {
    #[link_ordinal(15)]
    safe fn imported_function_stdcall(i: i32);
}
```

r[items.extern.attributes.link_ordinal.allowed-kinds]
This attribute is only used with the `raw-dylib` linking kind.
Using any other kind will result in a compiler error.

r[items.extern.attributes.link_ordinal.exclusive]
Using this attribute with the `link_name` attribute will result in a
compiler error.

r[items.extern.attributes.fn-parameters]
### Attributes on function parameters

Attributes on extern function parameters follow the same rules and
restrictions as [regular function parameters].

[PE Format]: https://learn.microsoft.com/windows/win32/debug/pe-format#import-name-type
[UEFI]: https://uefi.org/specifications
[WebAssembly module]: https://webassembly.github.io/spec/core/syntax/modules.html
[`bundle` documentation for rustc]: ../../rustc/command-line-arguments.html#linking-modifiers-bundle
[`dylib` versus `raw-dylib`]: #dylib-versus-raw-dylib
[`unsafe` context]: ../unsafe-keyword.md
[`verbatim` documentation for rustc]: ../../rustc/command-line-arguments.html#linking-modifiers-verbatim
[`whole-archive` documentation for rustc]: ../../rustc/command-line-arguments.html#linking-modifiers-whole-archive
[attributes]: ../attributes.md
[functions]: functions.md
[regular function parameters]: functions.md#attributes-on-function-parameters
[statics]: static-items.md
[unwind-behavior]: functions.md#unwinding
[value namespace]: ../names/namespaces.md
[`link_ordinal`]: items.extern.attributes.link_ordinal
[`extern` blocks]: external-blocks.md
