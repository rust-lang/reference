r[link]
# Linkage

> [!NOTE]
> This section is described more in terms of the compiler than of the language.

r[link.intro]
The compiler supports various methods to link crates together both
statically and dynamically. This section will explore the various methods to
link crates together, and more information about native libraries can be
found in the [FFI section of the book][ffi].

[ffi]: ../book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code

r[link.type]
In one session of compilation, the compiler can generate multiple artifacts
through the usage of either command line flags or the `crate_type` attribute.
If one or more command line flags are specified, all `crate_type` attributes will
be ignored in favor of only building the artifacts specified by command line.

r[link.bin]
* `--crate-type=bin`, `#![crate_type = "bin"]` - A runnable executable will be
  produced. This requires that there is a `main` function in the crate which
  will be run when the program begins executing. This will link in all Rust and
  native dependencies, producing a single distributable binary.
  This is the default crate type.

r[link.lib]
* `--crate-type=lib`, `#![crate_type = "lib"]` - A Rust library will be produced.
  This is an ambiguous concept as to what exactly is produced because a library
  can manifest itself in several forms. The purpose of this generic `lib` option
  is to generate the "compiler recommended" style of library. The output library
  will always be usable by rustc, but the actual type of library may change from
  time-to-time. The remaining output types are all different flavors of
  libraries, and the `lib` type can be seen as an alias for one of them (but the
  actual one is compiler-defined).

r[link.dylib]
* `--crate-type=dylib`, `#![crate_type = "dylib"]` - A dynamic Rust library will
  be produced. This is different from the `lib` output type in that this forces
  dynamic library generation. The resulting dynamic library can be used as a
  dependency for other libraries and/or executables. This output type will
  create `*.so` files on Linux, `*.dylib` files on macOS, and `*.dll` files on
  Windows.

r[link.staticlib]
* `--crate-type=staticlib`, `#![crate_type = "staticlib"]` - A static system
  library will be produced. This is different from other library outputs in that
  the compiler will never attempt to link to `staticlib` outputs. The
  purpose of this output type is to create a static library containing all of
  the local crate's code along with all upstream dependencies. This output type
  will create `*.a` files on Linux, macOS and Windows (MinGW), and `*.lib` files
  on Windows (MSVC). This format is recommended for use in situations such as
  linking Rust code into an existing non-Rust application because it will not
  have dynamic dependencies on other Rust code.

  Note that any dynamic dependencies that the static library may have (such as
  dependencies on system libraries, or dependencies on Rust libraries that are
  compiled as dynamic libraries) will have to be specified manually when
  linking that static library from somewhere. The `--print=native-static-libs` flag may help with this.

  Note that, because the resulting static library contains the code of all the
  dependencies, including the standard library, and also exports all public
  symbols of them, linking the static library into an executable or shared
  library may need special care. In case of a shared library the list of
  exported symbols will have to be limited via e.g. a linker or symbol version
  script, exported symbols list (macOS), or module definition file (Windows).
  Additionally, unused sections can be removed to remove all code of
  dependencies that is not actually used (e.g. `--gc-sections` or `-dead_strip`
  for macOS).

r[link.cdylib]
* `--crate-type=cdylib`, `#![crate_type = "cdylib"]` - A dynamic system
  library will be produced.  This is used when compiling
  a dynamic library to be loaded from another language.  This output type will
  create `*.so` files on Linux, `*.dylib` files on macOS, and `*.dll` files on
  Windows.

r[link.rlib]
* `--crate-type=rlib`, `#![crate_type = "rlib"]` - A "Rust library" file will be
  produced. This is used as an intermediate artifact and can be thought of as a
  "static Rust library". These `rlib` files, unlike `staticlib` files, are
  interpreted by the compiler in future linkage. This essentially means
  that `rustc` will look for metadata in `rlib` files like it looks for metadata
  in dynamic libraries. This form of output is used to produce statically linked
  executables as well as `staticlib` outputs.

r[link.proc-macro]
* `--crate-type=proc-macro`, `#![crate_type = "proc-macro"]` - The output
  produced is not specified, but if a `-L` path is provided to it then the
  compiler will recognize the output artifacts as a macro and it can be loaded
  for a program. Crates compiled with this crate type  must only export
  [procedural macros]. The compiler will automatically set the `proc_macro`
  [configuration option]. The crates are always compiled with the same target
  that the compiler itself was built with. For example, if you are executing
  the compiler from Linux with an `x86_64` CPU, the target will be
  `x86_64-unknown-linux-gnu` even if the crate is a dependency of another crate
  being built for a different target.

r[link.repetition]
Note that these outputs are stackable in the sense that if multiple are
specified, then the compiler will produce each form of output without
having to recompile. However, this only applies for outputs specified by the
same method. If only `crate_type` attributes are specified, then they will all
be built, but if one or more `--crate-type` command line flags are specified,
then only those outputs will be built.

r[link.dependency]
With all these different kinds of outputs, if crate A depends on crate B, then
the compiler could find B in various different forms throughout the system. The
only forms looked for by the compiler, however, are the `rlib` format and the
dynamic library format. With these two options for a dependent library, the
compiler must at some point make a choice between these two formats. With this
in mind, the compiler follows these rules when determining what format of
dependencies will be used:

r[link.dependency-staticlib]
1. If a static library is being produced, all upstream dependencies are
   required to be available in `rlib` formats. This requirement stems from the
   reason that a dynamic library cannot be converted into a static format.

   Note that it is impossible to link in native dynamic dependencies to a static
   library, and in this case warnings will be printed about all unlinked native
   dynamic dependencies.

r[link.dependency-rlib]

2. If an `rlib` file is being produced, then there are no restrictions on what
   format the upstream dependencies are available in. It is simply required that
   all upstream dependencies be available for reading metadata from.

   The reason for this is that `rlib` files do not contain any of their upstream
   dependencies. It wouldn't be very efficient for all `rlib` files to contain a
   copy of `libstd.rlib`!

r[link.dependency-prefer-dynamic]

3. If an executable is being produced and the `-C prefer-dynamic` flag is not
   specified, then dependencies are first attempted to be found in the `rlib`
   format. If some dependencies are not available in an rlib format, then
   dynamic linking is attempted (see below).

r[link.dependency-dynamic]

4. If a dynamic library or an executable that is being dynamically linked is
   being produced, then the compiler will attempt to reconcile the available
   dependencies in either the rlib or dylib format to create a final product.

   A major goal of the compiler is to ensure that a library never appears more
   than once in any artifact. For example, if dynamic libraries B and C were
   each statically linked to library A, then a crate could not link to B and C
   together because there would be two copies of A. The compiler allows mixing
   the rlib and dylib formats, but this restriction must be satisfied.

   The compiler currently implements no method of hinting what format a library
   should be linked with. When dynamically linking, the compiler will attempt to
   maximize dynamic dependencies while still allowing some dependencies to be
   linked in via an rlib.

   For most situations, having all libraries available as a dylib is recommended
   if dynamically linking. For other situations, the compiler will emit a
   warning if it is unable to determine which formats to link each library with.

In general, `--crate-type=bin` or `--crate-type=lib` should be sufficient for
all compilation needs, and the other options are just available if more
fine-grained control is desired over the output format of a crate.

r[link.crt]
## Static and dynamic C runtimes

r[link.crt.intro]
The standard library in general strives to support both statically linked and
dynamically linked C runtimes for targets as appropriate. For example the
`x86_64-pc-windows-msvc` and `x86_64-unknown-linux-musl` targets typically come
with both runtimes and the user selects which one they'd like. All targets in
the compiler have a default mode of linking to the C runtime. Typically targets
are linked dynamically by default, but there are exceptions which are static by
default such as:

* `arm-unknown-linux-musleabi`
* `arm-unknown-linux-musleabihf`
* `armv7-unknown-linux-musleabihf`
* `i686-unknown-linux-musl`
* `x86_64-unknown-linux-musl`

r[link.crt.crt-static]
The linkage of the C runtime is configured to respect the `crt-static` target
feature. These target features are typically configured from the command line
via flags to the compiler itself. For example to enable a static runtime you
would execute:

```sh
rustc -C target-feature=+crt-static foo.rs
```

whereas to link dynamically to the C runtime you would execute:

```sh
rustc -C target-feature=-crt-static foo.rs
```

r[link.crt.ineffective]
Targets which do not support switching between linkage of the C runtime will
ignore this flag. It's recommended to inspect the resulting binary to ensure
that it's linked as you would expect after the compiler succeeds.

r[link.crt.target_feature]
Crates may also learn about how the C runtime is being linked. Code on MSVC, for
example, needs to be compiled differently (e.g. with `/MT` or `/MD`) depending
on the runtime being linked. This is exported currently through the
[`cfg` attribute `target_feature` option]:

```rust
#[cfg(target_feature = "crt-static")]
fn foo() {
    println!("the C runtime should be statically linked");
}

#[cfg(not(target_feature = "crt-static"))]
fn foo() {
    println!("the C runtime should be dynamically linked");
}
```

Also note that Cargo build scripts can learn about this feature through
[environment variables][cargo]. In a build script you can detect the linkage
via:

```rust
use std::env;

fn main() {
    let linkage = env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or(String::new());

    if linkage.contains("crt-static") {
        println!("the C runtime will be statically linked");
    } else {
        println!("the C runtime will be dynamically linked");
    }
}
```

[cargo]: ../cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts

To use this feature locally, you typically will use the `RUSTFLAGS` environment
variable to specify flags to the compiler through Cargo. For example to compile
a statically linked binary on MSVC you would execute:

```sh
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-pc-windows-msvc
```

r[link.foreign-code]
## Mixed Rust and foreign codebases

r[link.foreign-code.foreign-linkers]
If you are mixing Rust with foreign code (e.g. C, C++) and wish to make a single
binary containing both types of code, you have two approaches for the final
binary link:

* Use `rustc`. Pass any non-Rust libraries using `-L <directory>` and `-l<library>`
  rustc arguments, and/or `#[link]` directives in your Rust code. If you need to
  link against `.o` files you can use `-Clink-arg=file.o`.
* Use your foreign linker. In this case, you first need to generate a Rust `staticlib`
  target and pass that into your foreign linker invocation. If you need to link
  multiple Rust subsystems, you will need to generate a _single_ `staticlib`
  perhaps using lots of `extern crate` statements to include multiple Rust `rlib`s.
  Multiple Rust `staticlib` files are likely to conflict.

Passing `rlib`s directly into your foreign linker is currently unsupported.

> [!NOTE]
> Rust code compiled or linked with a different instance of the Rust runtime counts as "foreign code" for the purpose of this section.

r[link.unwinding]
### Prohibited linkage and unwinding

r[link.unwinding.intro]
Panic unwinding can only be used if the binary is built consistently according to the following rules.

r[link.unwinding.potential]
A Rust artifact is called *potentially unwinding* if any of the following conditions is met:
- The artifact uses the [`unwind` panic handler][panic.panic_handler].
- The artifact contains a crate built with the `unwind` [panic strategy] that makes a call to a function using a `-unwind` ABI.
- The artifact makes a `"Rust"` ABI call to code running in another Rust artifact that has a separate copy of the Rust runtime, and that other artifact is potentially unwinding.

> [!NOTE]
> This definition captures whether a `"Rust"` ABI call inside a Rust artifact can ever unwind.

r[link.unwinding.prohibited]
If a Rust artifact is potentially unwinding, then all its crates must be built with the `unwind` [panic strategy]. Otherwise, unwinding can cause undefined behavior.

> [!NOTE]
> If you are using `rustc` to link, these rules are enforced automatically. If you are *not* using `rustc` to link, you must take care to ensure that unwinding is handled consistently across the entire binary. Linking without `rustc` includes using `dlopen` or similar facilities where linking is done by the system runtime without `rustc` being involved. This can only happen when mixing code with different [`-C panic`] flags, so most users do not have to be concerned about this.

> [!NOTE]
> To guarantee that a library will be sound (and linkable with `rustc`) regardless of the panic runtime used at link-time, the [`ffi_unwind_calls` lint] may be used. The lint flags any calls to `-unwind` foreign functions or function pointers.

[`cfg` attribute `target_feature` option]: conditional-compilation.md#target_feature
[`ffi_unwind_calls` lint]: ../rustc/lints/listing/allowed-by-default.html#ffi-unwind-calls
[configuration option]: conditional-compilation.md
[procedural macros]: procedural-macros.md
[panic strategy]: panic.md#panic-strategy
[`-C panic`]: ../rustc/codegen-options/index.html#panic
