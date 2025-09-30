r[items.extern-crate]
# Extern crate declarations

r[items.extern-crate.syntax]
```grammar,items
ExternCrate -> `extern` `crate` CrateRef AsClause? `;`

CrateRef -> IDENTIFIER | `self`

AsClause -> `as` ( IDENTIFIER | `_` )
```

r[items.extern-crate.intro]
An _`extern crate` declaration_ specifies a dependency on an external crate.

r[items.extern-crate.namespace]
The external crate is then bound into the declaring scope as the given [identifier] in the [type namespace].

r[items.extern-crate.extern-prelude]
Additionally, if the `extern crate` appears in the crate root, then the crate name is also added to the [extern prelude], making it automatically in scope in all modules.

r[items.extern-crate.as]
The `as` clause can be used to bind the imported crate to a different name.

r[items.extern-crate.lookup]
The external crate is resolved to a specific `soname` at compile time, and a
runtime linkage requirement to that `soname` is passed to the linker for
loading at runtime. The `soname` is resolved at compile time by scanning the
compiler's library path and matching the optional `crate_name` provided against
the [`crate_name` attributes] that were declared on the external crate when it was
compiled. If no `crate_name` is provided, a default `name` attribute is assumed,
equal to the [identifier] given in the `extern crate` declaration.

r[items.extern-crate.self]
The `self` crate may be imported which creates a binding to the current crate.
In this case the `as` clause must be used to specify the name to bind it to.

Three examples of `extern crate` declarations:

<!-- ignore: requires external crates -->
```rust,ignore
extern crate pcre;

extern crate std; // equivalent to: extern crate std as std;

extern crate std as ruststd; // linking to 'std' under another name
```

r[items.extern-crate.name-restrictions]
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

r[items.extern-crate.underscore]
## Underscore imports

r[items.extern-crate.underscore.intro]
An external crate dependency can be declared without binding its name in scope
by using an underscore with the form `extern crate foo as _`. This may be
useful for crates that only need to be linked, but are never referenced, and
will avoid being reported as unused.

r[items.extern-crate.underscore.macro_use]
The [`macro_use` attribute] works as usual and imports the macro names
into the [`macro_use` prelude].

<!-- template:attributes -->
r[items.extern-crate.no_link]
## The `no_link` attribute

r[items.extern-crate.no_link.intro]
The *`no_link` [attribute][attributes]* may be applied to an `extern crate` item to prevent linking the crate.

> [!NOTE]
> This is helpful, e.g., when only the macros of a crate are needed.

> [!EXAMPLE]
> <!-- ignore: requires external crates -->
> ```rust,ignore
> #[no_link]
> extern crate other_crate;
>
> other_crate::some_macro!();
> ```

r[items.extern-crate.no_link.syntax]
The `no_link` attribute uses the [MetaWord] syntax.

r[items.extern-crate.no_link.allowed-positions]
The `no_link` attribute may only be applied to an `extern crate` declaration.

> [!NOTE]
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

r[items.extern-crate.no_link.duplicates]
Only the first use of `no_link` on an `extern crate` declaration has effect.

> [!NOTE]
> `rustc` lints against any use following the first. This may become an error in the future.

[identifier]: ../identifiers.md
[RFC 940]: https://github.com/rust-lang/rfcs/blob/master/text/0940-hyphens-considered-harmful.md
[`macro_use` attribute]: ../macros-by-example.md#the-macro_use-attribute
[extern prelude]: ../names/preludes.md#extern-prelude
[`macro_use` prelude]: ../names/preludes.md#macro_use-prelude
[`crate_name` attributes]: ../crates-and-source-files.md#the-crate_name-attribute
[type namespace]: ../names/namespaces.md
