# Extern crate declarations

> **<sup>Syntax:<sup>**\
> _ExternCrate_ :\
> &nbsp;&nbsp; `extern` `crate` _CrateRef_ _AsClause_<sup>?</sup> `;`
>
> _CrateRef_ :\
> &nbsp;&nbsp; [IDENTIFIER] | `self`
>
> _AsClause_ :\
> &nbsp;&nbsp; `as` ( [IDENTIFIER] | `_` )

An _`extern crate` declaration_ specifies a dependency on an external crate.
The external crate is then bound into the declaring scope as the [identifier]
provided in the `extern crate` declaration. The `as` clause can be used to
bind the imported crate to a different name.

The external crate is resolved to a specific `soname` at compile time, and a
runtime linkage requirement to that `soname` is passed to the linker for
loading at runtime. The `soname` is resolved at compile time by scanning the
compiler's library path and matching the optional `crateid` provided against
the `crateid` attributes that were declared on the external crate when it was
compiled. If no `crateid` is provided, a default `name` attribute is assumed,
equal to the [identifier] given in the `extern crate` declaration.

The `self` crate may be imported which creates a binding to the current crate.
In this case the `as` clause must be used to specify the name to bind it to.

Three examples of `extern crate` declarations:

<!-- ignore: requires external crates -->
```rust,ignore
extern crate pcre;

extern crate std; // equivalent to: extern crate std as std;

extern crate std as ruststd; // linking to 'std' under another name
```

When naming Rust crates, hyphens are disallowed. However, Cargo packages may
make use of them. In such case, when `Cargo.toml` doesn't specify a crate name,
Cargo will transparently replace `-` with `_` (Refer to [RFC 940] for more
details).

Here is an example:

<!-- ignore: requires external crates -->
```rust,ignore
// Importing the Cargo package hello-world
extern crate hello_world; // hyphen replaced with an underscore
```

## Extern Prelude

External crates imported with `extern crate` in the root module or provided to
the compiler (as with the `--extern` flag with `rustc`) are added to the
"extern prelude". Crates in the extern prelude are in scope in the entire
crate, including inner modules. If imported with `extern crate orig_name as
new_name`, then the symbol `new_name` is instead added to the prelude.

The `core` crate is always added to the extern prelude. The `std` crate
is added as long as the [`no_std`] attribute is not specified in the crate root.

The [`no_implicit_prelude`] attribute can be used on a module to disable
prelude lookups within that module.

> **Edition Differences**: In the 2015 edition, crates in the extern prelude
> cannot be referenced via [use declarations], so it is generally standard
> practice to include `extern crate` declarations to bring them into scope.
>
> Beginning in the 2018 edition, [use declarations] can reference crates in
> the extern prelude, so it is considered unidiomatic to use `extern crate`.

> **Note**: Additional crates that ship with `rustc`, such as [`proc_macro`],
> [`alloc`], and [`test`], are not automatically included with the `--extern`
> flag when using Cargo. They must be brought into scope with an `extern
> crate` declaration, even in the 2018 edition.
>
> ```rust
> extern crate proc_macro;
> use proc_macro::TokenStream;
> ```

<!--
The proc_macro/alloc/test limitation may be lifted if the `--extern`
flag is stabilized and used. See tracking issue
https://github.com/rust-lang/rust/issues/57288 and the unstable
`--extern` flag added in https://github.com/rust-lang/rust/pull/54116.
-->

## Underscore Imports

An external crate dependency can be declared without binding its name in scope
by using an underscore with the form `extern crate foo as _`. This may be
useful for crates that only need to be linked, but are never referenced, and
will avoid being reported as unused.

The [`macro_use` attribute] works as usual and import the macro names
into the macro-use prelude.

## The `no_link` attribute

The *`no_link` attribute* may be specified on an `extern crate` item to
prevent linking the crate into the output. This is commonly used to load a
crate to access only its macros.

[IDENTIFIER]: ../identifiers.md
[RFC 940]: https://github.com/rust-lang/rfcs/blob/master/text/0940-hyphens-considered-harmful.md
[`macro_use` attribute]: ../macros-by-example.md#the-macro_use-attribute
[`alloc`]: https://doc.rust-lang.org/alloc/
[`no_implicit_prelude`]: modules.md#prelude-items
[`no_std`]: ../crates-and-source-files.md#preludes-and-no_std
[`proc_macro`]: https://doc.rust-lang.org/proc_macro/
[`test`]: https://doc.rust-lang.org/test/
[use declarations]: use-declarations.md
