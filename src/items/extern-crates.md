# Extern crate declarations

> **<sup>Syntax:<sup>**\
> _ExternCrate_ :\
> &nbsp;&nbsp; `extern` `crate` [IDENTIFIER]&nbsp;(`as` ( [IDENTIFIER] | `_` ) )<sup>?</sup> `;`

An _`extern crate` declaration_ specifies a dependency on an external crate.
The external crate is then bound into the declaring scope as the [identifier]
provided in the `extern crate` declaration.

The external crate is resolved to a specific `soname` at compile time, and a
runtime linkage requirement to that `soname` is passed to the linker for
loading at runtime. The `soname` is resolved at compile time by scanning the
compiler's library path and matching the optional `crateid` provided against
the `crateid` attributes that were declared on the external crate when it was
compiled. If no `crateid` is provided, a default `name` attribute is assumed,
equal to the [identifier] given in the `extern crate` declaration.

Three examples of `extern crate` declarations:

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

> **Note**: Crates not explicitly named with the `--extern` flag with `rustc`
> are not included in the extern prelude. As a result, additional crates
> that ship with `rustc`, such as [`proc_macro`], [`alloc`], and [`test`],
> currently aren't available in the extern prelude and must be brought into
> scope with an `extern crate` declaration, even in the 2018 edition. `use`
> paths must reference the `extern crate` item (such as using [`crate::`] or
> [`self::`] path prefixes).
>
> ```rust
> extern crate proc_macro;
> // Cannot reference `proc_macro` directly because it is not in the extern prelude.
> // use proc_macro::TokenStream;
> // Instead, you must reference the item in scope from the `extern crate`
> // declaration.
> use self::proc_macro::TokenStream;
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

The [`#[macro_use]` attribute] will work as usual and import the macro names
into the macro-use prelude.

[IDENTIFIER]: identifiers.html
[RFC 940]: https://github.com/rust-lang/rfcs/blob/master/text/0940-hyphens-considered-harmful.md
[`#[macro_use]` attribute]: attributes.html#macro-related-attributes
[`alloc`]: https://doc.rust-lang.org/alloc/
[`crate::`]: paths.html#crate
[`no_implicit_prelude`]: items/modules.html#prelude-items
[`no_std`]: crates-and-source-files.html#preludes-and-no_std
[`proc_macro`]: https://doc.rust-lang.org/proc_macro/
[`self::`]: paths.html#self
[`test`]: https://doc.rust-lang.org/test/
[use declarations]: items/use-declarations.html
