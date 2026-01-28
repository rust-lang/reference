r[names.preludes]
# Preludes

r[names.preludes.intro]
A *prelude* is a collection of names that are automatically brought into scope of every module in a crate.

These prelude names are not part of the module itself: they are implicitly queried during [name resolution]. For example, even though something like [`Box`] is in scope in every module, you cannot refer to it as `self::Box` because it is not a member of the current module.

r[names.preludes.kinds]
There are several different preludes:

- [Standard library prelude]
- [Extern prelude]
- [Language prelude]
- [`macro_use` prelude]
- [Tool prelude]

r[names.preludes.std]
## Standard library prelude

r[names.preludes.std.intro]
Each crate has a standard library prelude, which consists of the names from a single standard library module.

r[names.preludes.std.module]
The module used depends on the crate's edition, and on whether the [`no_std` attribute] is applied to the crate:

Edition | `no_std` not applied        | `no_std` applied
--------| --------------------------- | ----------------------------
2015    | [`std::prelude::rust_2015`] | [`core::prelude::rust_2015`]
2018    | [`std::prelude::rust_2018`] | [`core::prelude::rust_2018`]
2021    | [`std::prelude::rust_2021`] | [`core::prelude::rust_2021`]
2024    | [`std::prelude::rust_2024`] | [`core::prelude::rust_2024`]

> [!NOTE]
> [`std::prelude::rust_2015`] and [`std::prelude::rust_2018`] have the same contents as [`std::prelude::v1`].
>
> [`core::prelude::rust_2015`] and [`core::prelude::rust_2018`] have the same contents as [`core::prelude::v1`].

> [!NOTE]
> When one of [`core::panic!`] or [`std::panic!`] is brought into scope due to the [standard library prelude], and a user-written [glob import] brings the other into scope, `rustc` currently allows use of `panic!`, even though it is ambiguous. The user-written glob import takes precedence to resolve this ambiguity.
>
> For details, see [names.resolution.expansion.imports.ambiguity.panic-hack].

r[names.preludes.extern]
## Extern prelude

r[names.preludes.extern.intro]
External crates imported with [`extern crate`] in the root module or provided to the compiler (as with the `--extern` flag with `rustc`) are added to the *extern prelude*. If imported with an alias such as `extern crate orig_name as new_name`, then the symbol `new_name` is instead added to the prelude.

r[names.preludes.extern.core]
The [`core`] crate is always added to the extern prelude.

r[names.preludes.extern.std]
The [`std`] crate is added as long as the [`no_std` attribute] is not specified in the crate root.

r[names.preludes.extern.edition2018]
> [!EDITION-2018]
> In the 2015 edition, crates in the extern prelude cannot be referenced via [use declarations], so it is generally standard practice to include `extern crate` declarations to bring them into scope.
>
> Beginning in the 2018 edition, [use declarations] can reference crates in the extern prelude, so it is considered unidiomatic to use `extern crate`.

> [!NOTE]
> Additional crates that ship with `rustc`, such as [`alloc`], and [`test`](mod@test), are not automatically included with the `--extern` flag when using Cargo. They must be brought into scope with an `extern crate` declaration, even in the 2018 edition.
>
> ```rust
> extern crate alloc;
> use alloc::rc::Rc;
> ```
>
> Cargo does bring in `proc_macro` to the extern prelude for proc-macro crates only.

<!--
See https://github.com/rust-lang/rust/issues/57288 for more about the alloc/test limitation.
-->

<!-- template:attributes -->
r[names.preludes.extern.no_std]
### The `no_std` attribute

r[names.preludes.extern.no_std.intro]
The *`no_std` [attribute][attributes]* causes the [`std`] crate to not be linked automatically and the [standard library prelude] to instead use the `core` prelude.

> [!EXAMPLE]
> <!-- ignore: test infrastructure can't handle no_std -->
> ```rust,ignore
> #![no_std]
> ```

> [!NOTE]
> Using `no_std` is useful when either the crate is targeting a platform that does not support the standard library or is purposefully not using the capabilities of the standard library. Those capabilities are mainly dynamic memory allocation (e.g. `Box` and `Vec`) and file and network capabilities (e.g. `std::fs` and `std::io`).

> [!WARNING]
> Using `no_std` does not prevent the standard library from being linked. It is still valid to write `extern crate std` in the crate or in one of its dependencies; this will cause the compiler to link the `std` crate into the program.

r[names.preludes.extern.no_std.syntax]
The `no_std` attribute uses the [MetaWord] syntax.

r[names.preludes.extern.no_std.allowed-positions]
The `no_std` attribute may only be applied to the crate root.

r[names.preludes.extern.no_std.duplicates]
The `no_std` attribute may be used any number of times on a form.

> [!NOTE]
> `rustc` lints against any use following the first.

r[names.preludes.extern.no_std.module]
The `no_std` attribute changes the [standard library prelude] to use the `core` prelude instead of the `std` prelude.

r[names.preludes.extern.no_std.edition2018]
> [!EDITION-2018]
> Before the 2018 edition, `std` is injected into the crate root by default. If `no_std` is specified, `core` is injected instead. Starting with the 2018 edition, regardless of `no_std` being specified, neither is injected into the crate root.

r[names.preludes.lang]
## Language prelude

r[names.preludes.lang.intro]
The language prelude includes names of types and attributes that are built-in to the language. The language prelude is always in scope.

r[names.preludes.lang.entities]
It includes the following:

* [Type namespace]
    * [Boolean type] --- `bool`
    * [`char`]
    * [`str`]
    * [Integer types] --- `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`
    * [Machine-dependent integer types] --- `usize` and `isize`
    * [floating-point types] --- `f32` and `f64`
* [Macro namespace]
    * [Built-in attributes]
    * [Built-in derive macros][attributes.derive.built-in]

r[names.preludes.macro_use]
## `macro_use` prelude

r[names.preludes.macro_use.intro]
The `macro_use` prelude includes macros from external crates that were imported by the [`macro_use` attribute] applied to an [`extern crate`].

r[names.preludes.tool]
## Tool prelude

r[names.preludes.tool.intro]
The tool prelude includes tool names for external tools in the [type namespace]. See the [tool attributes] section for more details.

<!-- template:attributes -->
r[names.preludes.no_implicit_prelude]
## The `no_implicit_prelude` attribute

r[names.preludes.no_implicit_prelude.intro]
The *`no_implicit_prelude` [attribute]* is used to prevent implicit preludes from being brought into scope.

> [!EXAMPLE]
> ```rust
> // The attribute can be applied to the crate root to affect
> // all modules.
> #![no_implicit_prelude]
>
> // Or it can be applied to a module to only affect that module
> // and its descendants.
> #[no_implicit_prelude]
> mod example {
>     // ...
> }
> ```

r[names.preludes.no_implicit_prelude.syntax]
The `no_implicit_prelude` attribute uses the [MetaWord] syntax.

r[names.preludes.no_implicit_prelude.allowed-positions]
The `no_implicit_prelude` attribute may only be applied to the crate or to a module.

> [!NOTE]
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

r[names.preludes.no_implicit_prelude.duplicates]
The `no_implicit_prelude` attribute may be used any number of times on a form.

> [!NOTE]
> `rustc` lints against any use following the first.

r[names.preludes.no_implicit_prelude.excluded-preludes]
The `no_implicit_prelude` attribute prevents the [standard library prelude], [extern prelude], [`macro_use` prelude], and the [tool prelude] from being brought into scope for the module and its descendants.

r[names.preludes.no_implicit_prelude.implicitly-imported-macros]
> [!NOTE]
> Despite `#![no_implicit_prelude]`, `rustc` currently brings certain macros implicitly into scope. Those macros are:
>
> - [`assert!`]
> - [`cfg!`]
> - [`cfg_select!`]
> - [`column!`]
> - [`compile_error!`]
> - [`concat!`]
> - [`concat_bytes!`]
> - [`env!`]
> - [`file!`]
> - [`format_args!`]
> - [`include!`]
> - [`include_bytes!`]
> - [`include_str!`]
> - [`line!`]
> - [`module_path!`]
> - [`option_env!`]
> - [`panic!`]
> - [`stringify!`]
> - [`unreachable!`]
>
> E.g., this works:
>
> ```rust
> #![no_implicit_prelude]
> fn main() { assert!(true); }
> ```
>
> Don't rely on this behavior; it may be removed in the future. Always bring the items you need into scope explicitly when using `#![no_implicit_prelude]`.
>
> For details, see [Rust PR #62086](https://github.com/rust-lang/rust/pull/62086) and [Rust PR #139493](https://github.com/rust-lang/rust/pull/139493).

r[names.preludes.no_implicit_prelude.lang]
The `no_implicit_prelude` attribute does not affect the [language prelude].

r[names.preludes.no_implicit_prelude.edition2018]
> [!EDITION-2018]
> In the 2015 edition, the `no_implicit_prelude` attribute does not affect the [`macro_use` prelude], and all macros exported from the standard library are still included in the `macro_use` prelude. Starting in the 2018 edition, the attribute does remove the `macro_use` prelude.

[`char`]: ../types/char.md
[`extern crate`]: ../items/extern-crates.md
[`macro_use` attribute]: ../macros-by-example.md#the-macro_use-attribute
[`macro_use` prelude]: #macro_use-prelude
[`no_std` attribute]: #the-no_std-attribute
[`str`]: ../types/str.md
[attribute]: ../attributes.md
[Boolean type]: ../types/boolean.md
[Built-in attributes]: ../attributes.md#built-in-attributes-index
[extern prelude]: #extern-prelude
[floating-point types]: ../types/numeric.md#floating-point-types
[glob import]: items.use.glob
[Integer types]: ../types/numeric.md#integer-types
[Language prelude]: #language-prelude
[Machine-dependent integer types]: ../types/numeric.md#machine-dependent-integer-types
[Macro namespace]: namespaces.md
[name resolution]: name-resolution.md
[standard library prelude]: names.preludes.std
[tool attributes]: ../attributes.md#tool-attributes
[Tool prelude]: #tool-prelude
[Type namespace]: namespaces.md
[use declarations]: ../items/use-declarations.md
