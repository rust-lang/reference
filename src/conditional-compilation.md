r[cfg]
# Conditional compilation

r[cfg.syntax]
```grammar,configuration
ConfigurationPredicate ->
      ConfigurationOption
    | ConfigurationAll
    | ConfigurationAny
    | ConfigurationNot
    | `true`
    | `false`

ConfigurationOption ->
    IDENTIFIER ( `=` ( STRING_LITERAL | RAW_STRING_LITERAL ) )?

ConfigurationAll ->
    `all` `(` ConfigurationPredicateList? `)`

ConfigurationAny ->
    `any` `(` ConfigurationPredicateList? `)`

ConfigurationNot ->
    `not` `(` ConfigurationPredicate `)`

ConfigurationPredicateList ->
    ConfigurationPredicate (`,` ConfigurationPredicate)* `,`?
```

r[cfg.general]
*Conditionally compiled source code* is source code that is compiled only under certain conditions.

r[cfg.attributes-macro]
Source code can be made conditionally compiled using the [`cfg`] and [`cfg_attr`] [attributes] and the built-in [`cfg` macro].

r[cfg.conditional]
Whether to compile can depend on the target architecture of the compiled crate, arbitrary values passed to the compiler, and other things further described below.

r[cfg.predicate]
Each form of conditional compilation takes a _configuration predicate_ that
evaluates to true or false. The predicate is one of the following:

r[cfg.predicate.option]
* A configuration option. The predicate is true if the option is set, and false if it is unset.

r[cfg.predicate.all]
* `all()` with a comma-separated list of configuration predicates. It is true if all of the given predicates are true, or if the list is empty.

r[cfg.predicate.any]
* `any()` with a comma-separated list of configuration predicates. It is true if at least one of the given predicates is true. If there are no predicates, it is false.

r[cfg.predicate.not]
* `not()` with a configuration predicate. It is true if its predicate is false and false if its predicate is true.

r[cfg.predicate.literal]
* `true` or `false` literals, which are always true or false respectively.

r[cfg.option-spec]
_Configuration options_ are either names or key-value pairs, and are either set or unset.

r[cfg.option-name]
Names are written as a single identifier, such as `unix`.

r[cfg.option-key-value]
Key-value pairs are written as an identifier, `=`, and then a string, such as `target_arch = "x86_64"`.

> [!NOTE]
> Whitespace around the `=` is ignored, so `foo="bar"` and `foo = "bar"` are equivalent.

r[cfg.option-key-uniqueness]
Keys do not need to be unique. For example, both `feature = "std"` and `feature = "serde"` can be set at the same time.

r[cfg.options.set]
## Set Configuration Options

r[cfg.options.general]
Which configuration options are set is determined statically during the
compilation of the crate.

r[cfg.options.target]
Some options are _compiler-set_ based on data about the compilation.

r[cfg.options.other]
Other options are _arbitrarily-set_ based on input passed to the compiler outside of the code.

r[cfg.options.crate]
It is not possible to set a
configuration option from within the source code of the crate being compiled.

> [!NOTE]
> For `rustc`, arbitrary-set configuration options are set using the [`--cfg`] flag. Configuration values for a specified target can be displayed with `rustc --print cfg --target $TARGET`.

> [!NOTE]
> Configuration options with the key `feature` are a convention used by [Cargo][cargo-feature] for specifying compile-time options and optional dependencies.

r[cfg.target_arch]
### `target_arch`

r[cfg.target_arch.gen]
Key-value option set once with the target's CPU architecture. The value is
similar to the first element of the platform's target triple, but not
identical.

r[cfg.target_arch.values]
Example values:

* `"x86"`
* `"x86_64"`
* `"mips"`
* `"powerpc"`
* `"powerpc64"`
* `"arm"`
* `"aarch64"`

r[cfg.target_feature]
### `target_feature`

r[cfg.target_feature.general]
Key-value option set for each platform feature available for the current
compilation target.

r[cfg.target_feature.values]
Example values:

* `"avx"`
* `"avx2"`
* `"crt-static"`
* `"rdrand"`
* `"sse"`
* `"sse2"`
* `"sse4.1"`

See the [`target_feature` attribute] for more details on the available
features.

r[cfg.target_feature.crt_static]
An additional feature of `crt-static` is available to the
`target_feature` option to indicate that a [static C runtime] is available.

r[cfg.target_os]
### `target_os`

r[cfg.target_os.general]
Key-value option set once with the target's operating system. This value is
similar to the second and third element of the platform's target triple.

r[cfg.target_os.values]
Example values:

* `"windows"`
* `"macos"`
* `"ios"`
* `"linux"`
* `"android"`
* `"freebsd"`
* `"dragonfly"`
* `"openbsd"`
* `"netbsd"`
* `"none"` (typical for embedded targets)

r[cfg.target_family]
### `target_family`

r[cfg.target_family.general]
Key-value option providing a more generic description of a target, such as the family of the
operating systems or architectures that the target generally falls into. Any number of
`target_family` key-value pairs can be set.

r[cfg.target_family.values]
Example values:

* `"unix"`
* `"windows"`
* `"wasm"`
* Both `"unix"` and `"wasm"`

r[cfg.target_family.unix]
### `unix` and `windows`

`unix` is set if `target_family = "unix"` is set.

r[cfg.target_family.windows]
`windows` is set if `target_family = "windows"` is set.

r[cfg.target_env]
### `target_env`

r[cfg.target_env.general]
Key-value option set with further disambiguating information about the target
platform with information about the ABI or `libc` used. For historical reasons,
this value is only defined as not the empty-string when actually needed for
disambiguation. Thus, for example, on many GNU platforms, this value will be
empty. This value is similar to the fourth element of the platform's target
triple. One difference is that embedded ABIs such as `gnueabihf` will simply
define `target_env` as `"gnu"`.

r[cfg.target_env.values]
Example values:

* `""`
* `"gnu"`
* `"msvc"`
* `"musl"`
* `"sgx"`

r[cfg.target_abi]
### `target_abi`

r[cfg.target_abi.general]
Key-value option set to further disambiguate the `target_env` with information
about the target ABI.

r[cfg.target_abi.disambiguation]
For historical reasons, this value is only defined as not the empty-string when actually
 needed for disambiguation. Thus, for example, on many GNU platforms, this value will be
empty.

r[cfg.target_abi.values]
Example values:

* `""`
* `"llvm"`
* `"eabihf"`
* `"abi64"`
* `"sim"`
* `"macabi"`

r[cfg.target_endian]
### `target_endian`

Key-value option set once with either a value of "little" or "big" depending
on the endianness of the target's CPU.

r[cfg.target_pointer_width]
### `target_pointer_width`

r[cfg.target_pointer_width.general]
Key-value option set once with the target's pointer width in bits.

r[cfg.target_pointer_width.values]
Example values:

* `"16"`
* `"32"`
* `"64"`

r[cfg.target_vendor]
### `target_vendor`

r[cfg.target_vendor.general]
Key-value option set once with the vendor of the target.

r[cfg.target_vendor.values]
Example values:

* `"apple"`
* `"fortanix"`
* `"pc"`
* `"unknown"`

r[cfg.target_has_atomic]
### `target_has_atomic`

r[cfg.target_has_atomic.general]
Key-value option set for each bit width that the target supports
atomic loads, stores, and compare-and-swap operations.

r[cfg.target_has_atomic.stdlib]
When this cfg is present, all of the stable [`core::sync::atomic`] APIs are available for
the relevant atomic width.

r[cfg.target_has_atomic.values]
Possible values:

* `"8"`
* `"16"`
* `"32"`
* `"64"`
* `"128"`
* `"ptr"`

r[cfg.test]
### `test`

Enabled when compiling the test harness. Done with `rustc` by using the
[`--test`] flag. See [Testing] for more on testing support.

r[cfg.debug_assertions]
### `debug_assertions`

Enabled by default when compiling without optimizations.
This can be used to enable extra debugging code in development but not in
production.  For example, it controls the behavior of the standard library's
[`debug_assert!`] macro.

r[cfg.proc_macro]
### `proc_macro`

Set when the crate being compiled is being compiled with the `proc_macro`
[crate type].

r[cfg.panic]
### `panic`

r[cfg.panic.general]
Key-value option set depending on the [panic strategy]. Note that more values may be added in the future.

r[cfg.panic.values]
Example values:

* `"abort"`
* `"unwind"`

[panic strategy]: panic.md#panic-strategy

## Forms of conditional compilation

r[cfg.attr]
### The `cfg` attribute

r[cfg.attr.intro]
The *`cfg` [attribute]* conditionally includes the form to which it is attached based on a configuration predicate.

> [!EXAMPLE]
> ```rust
> // The function is only included in the build when compiling for macOS
> #[cfg(target_os = "macos")]
> fn macos_only() {
>   // ...
> }
>
> // This function is only included when either foo or bar is defined
> #[cfg(any(foo, bar))]
> fn needs_foo_or_bar() {
>   // ...
> }
>
> // This function is only included when compiling for a unixish OS with a 32-bit
> // architecture
> #[cfg(all(unix, target_pointer_width = "32"))]
> fn on_32bit_unix() {
>   // ...
> }
>
> // This function is only included when foo is not defined
> #[cfg(not(foo))]
> fn needs_not_foo() {
>   // ...
> }
>
> // This function is only included when the panic strategy is set to unwind
> #[cfg(panic = "unwind")]
> fn when_unwinding() {
>   // ...
> }
> ```

r[cfg.attr.syntax]
The syntax for the `cfg` attribute is:

```grammar,configuration
@root CfgAttribute -> `cfg` `(` ConfigurationPredicate `)`
```

r[cfg.attr.allowed-positions]
The `cfg` attribute is allowed anywhere attributes are allowed.

r[cfg.attr.duplicates]
Multiple `cfg` attributes may be specified. The form to which the attribute is attached will not be included if any of the `cfg` predicates are false except as described in [cfg.attr.crate-level-attrs].

r[cfg.attr.effect]
If the predicate is true, the form is rewritten to not have the `cfg` attribute on it. If the predicate is false, the form is removed from the source code.

r[cfg.attr.crate-level-attrs]
When a crate-level `cfg` has a false predicate, the crate itself still exists. Any crate attributes preceding the `cfg` are kept, and any crate attributes following the `cfg` are removed as well as removing all of the following crate contents.

> [!EXAMPLE]
> The behavior of not removing the preceding attributes allows you to do things such as include `#![no_std]` to avoid linking `std` even if a `#![cfg(...)]` has otherwise removed the contents of the crate. For example:
>
> <!-- ignore: test infrastructure can't handle no_std -->
> ```rust,ignore
> // This `no_std` attribute is kept even though the crate-level `cfg`
> // attribute is false.
> #![no_std]
> #![cfg(false)]
>
> // This function is not included.
> pub fn example() {}
> ```

r[cfg.cfg_attr]
### The `cfg_attr` attribute

r[cfg.cfg_attr.intro]
The *`cfg_attr` [attribute]* conditionally includes attributes based on a configuration predicate.

> [!EXAMPLE]
> The following module will either be found at `linux.rs` or `windows.rs` based on the target.
>
> <!-- ignore: `mod` needs multiple files -->
> ```rust,ignore
> #[cfg_attr(target_os = "linux", path = "linux.rs")]
> #[cfg_attr(windows, path = "windows.rs")]
> mod os;
> ```

r[cfg.cfg_attr.syntax]
The syntax for the `cfg_attr` attribute is:

```grammar,configuration
@root CfgAttrAttribute -> `cfg_attr` `(` ConfigurationPredicate `,` CfgAttrs? `)`

CfgAttrs -> Attr (`,` Attr)* `,`?
```

r[cfg.cfg_attr.allowed-positions]
The `cfg_attr` attribute is allowed anywhere attributes are allowed.

r[cfg.cfg_attr.duplicates]
Multiple `cfg_attr` attributes may be specified.

r[cfg.cfg_attr.attr-restriction]
The [`crate_type`] and [`crate_name`] attributes cannot be used with `cfg_attr`.

r[cfg.cfg_attr.behavior]
When the configuration predicate is true, `cfg_attr` expands out to the attributes listed after the predicate.

r[cfg.cfg_attr.attribute-list]
Zero, one, or more attributes may be listed. Multiple attributes will each be expanded into separate attributes.

> [!EXAMPLE]
> <!-- ignore: fake attributes -->
> ```rust,ignore
> #[cfg_attr(feature = "magic", sparkles, crackles)]
> fn bewitched() {}
>
> // When the `magic` feature flag is enabled, the above will expand to:
> #[sparkles]
> #[crackles]
> fn bewitched() {}
> ```

> [!NOTE]
> The `cfg_attr` can expand to another `cfg_attr`. For example, `#[cfg_attr(target_os = "linux", cfg_attr(feature = "multithreaded", some_other_attribute))]` is valid. This example would be equivalent to `#[cfg_attr(all(target_os = "linux", feature ="multithreaded"), some_other_attribute)]`.

r[cfg.macro]
### The `cfg` macro

The built-in `cfg` macro takes in a single configuration predicate and evaluates
to the `true` literal when the predicate is true and the `false` literal when
it is false.

For example:

```rust
let machine_kind = if cfg!(unix) {
  "unix"
} else if cfg!(windows) {
  "windows"
} else {
  "unknown"
};

println!("I'm running on a {} machine!", machine_kind);
```

[Testing]: attributes/testing.md
[`--cfg`]: ../rustc/command-line-arguments.html#--cfg-configure-the-compilation-environment
[`--test`]: ../rustc/command-line-arguments.html#--test-build-a-test-harness
[`cfg`]: #the-cfg-attribute
[`cfg` macro]: #the-cfg-macro
[`cfg_attr`]: #the-cfg_attr-attribute
[`crate_name`]: crates-and-source-files.md#the-crate_name-attribute
[`crate_type`]: linkage.md
[`target_feature` attribute]: attributes/codegen.md#the-target_feature-attribute
[attribute]: attributes.md
[attributes]: attributes.md
[cargo-feature]: ../cargo/reference/features.html
[crate type]: linkage.md
[static C runtime]: linkage.md#static-and-dynamic-c-runtimes
