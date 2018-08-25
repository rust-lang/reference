## Conditional compilation

Sometimes one wants to have different compiler outputs from the same code,
depending on build target, such as targeted operating system, or to enable
release builds.

Configuration options are boolean (on or off) and are named either with a
single identifier (e.g. `foo`) or an identifier and a string (e.g. `foo = "bar"`;
the quotes are required and spaces around the `=` are unimportant). Note that
similarly-named options, such as `foo`, `foo="bar"` and `foo="baz"` may each be
set or unset independently.

Configuration options are either provided by the compiler or passed in on the
command line using `--cfg` (e.g. `rustc main.rs --cfg foo --cfg 'bar="baz"'`).
Rust code then checks for their presence using the `#[cfg(...)]` [attribute]:

```rust
// The function is only included in the build when compiling for macOS
#[cfg(target_os = "macos")]
fn macos_only() {
  // ...
}

// This function is only included when either foo or bar is defined
#[cfg(any(foo, bar))]
fn needs_foo_or_bar() {
  // ...
}

// This function is only included when compiling for a unixish OS with a 32-bit
// architecture
#[cfg(all(unix, target_pointer_width = "32"))]
fn on_32bit_unix() {
  // ...
}

// This function is only included when foo is not defined
#[cfg(not(foo))]
fn needs_not_foo() {
  // ...
}
```

This illustrates some conditional compilation can be achieved using the
`#[cfg(...)]` [attribute]. `any`, `all` and `not` can be used to assemble
arbitrarily complex configurations through nesting.

The following configurations must be defined by the implementation:

* `target_arch = "..."` - Target CPU architecture, such as `"x86"`,
  `"x86_64"` `"mips"`, `"powerpc"`, `"powerpc64"`, `"arm"`, or
  `"aarch64"`. This value is closely related to the first element of
  the platform target triple, though it is not identical.
* `target_os = "..."` - Operating system of the target, examples
  include `"windows"`, `"macos"`, `"ios"`, `"linux"`, `"android"`,
  `"freebsd"`, `"dragonfly"`, `"bitrig"` , `"openbsd"` or
  `"netbsd"`. This value is closely related to the second and third
  element of the platform target triple, though it is not identical.
* `target_family = "..."` - Operating system family of the target, e. g.
  `"unix"` or `"windows"`. The value of this configuration option is defined
  as a configuration itself, like `unix` or `windows`.
* `unix` - See `target_family`.
* `windows` - See `target_family`.
* `target_env = ".."` - Further disambiguates the target platform with
  information about the ABI/libc. Presently this value is either
  `"gnu"`, `"msvc"`, `"musl"`, or the empty string. For historical
  reasons this value has only been defined as non-empty when needed
  for disambiguation. Thus on many GNU platforms this value will be
  empty. This value is closely related to the fourth element of the
  platform target triple, though it is not identical. For example,
  embedded ABIs such as `gnueabihf` will simply define `target_env` as
  `"gnu"`.
* `target_endian = "..."` - Endianness of the target CPU, either `"little"` or
  `"big"`.
* `target_pointer_width = "..."` - Target pointer width in bits. This is set
  to `"32"` for targets with 32-bit pointers, and likewise set to `"64"` for
  64-bit pointers.
* `target_has_atomic = "..."` - Set of integer sizes on which the target can perform
  atomic operations. Values are `"8"`, `"16"`, `"32"`, `"64"` and `"ptr"`.
* `target_vendor = "..."` - Vendor of the target, for example `apple`, `pc`, or
  simply `"unknown"`.
* `test` - Enabled when compiling the test harness (using the `--test` flag).
* `debug_assertions` - Enabled by default when compiling without optimizations.
  This can be used to enable extra debugging code in development but not in
  production.  For example, it controls the behavior of the standard library's
  `debug_assert!` macro.
* `proc_macro` - Set when the crate being compiled is being compiled with the
  `proc_macro` [crate type].

You can also set another [attribute] based on a `cfg` variable with `cfg_attr`:

```rust,ignore
#[cfg_attr(a, b)]
```

This is the same as `#[b]` if `a` is set by `cfg`, and nothing otherwise.

Lastly, configuration options can be used in expressions by invoking the `cfg!`
macro: `cfg!(a)` evaluates to `true` if `a` is set, and `false` otherwise.

[attribute]: attributes.html
[crate type]: linkage.html