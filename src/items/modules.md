# Modules

> **<sup>Syntax:<sup>**\
> _Module_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; `mod` [IDENTIFIER] `;`\
> &nbsp;&nbsp; | `mod` [IDENTIFIER] `{`\
> &nbsp;&nbsp; &nbsp;&nbsp;&nbsp;&nbsp; [_InnerAttribute_]<sup>\*</sup>\
> &nbsp;&nbsp; &nbsp;&nbsp;&nbsp;&nbsp; [_Item_]<sup>\*</sup>\
> &nbsp;&nbsp; &nbsp;&nbsp; `}`

A module is a container for zero or more [items].

A _module item_ is a module, surrounded in braces, named, and prefixed with the
keyword `mod`. A module item introduces a new, named module into the tree of
modules making up a crate. Modules can nest arbitrarily.

An example of a module:

```rust
mod math {
    type Complex = (f64, f64);
    fn sin(f: f64) -> f64 {
        /* ... */
#       unimplemented!();
    }
    fn cos(f: f64) -> f64 {
        /* ... */
#       unimplemented!();
    }
    fn tan(f: f64) -> f64 {
        /* ... */
#       unimplemented!();
    }
}
```

Modules and types share the same namespace. Declaring a named type with the
same name as a module in scope is forbidden: that is, a type definition, trait,
struct, enumeration, union, type parameter or crate can't shadow the name of a
module in scope, or vice versa. Items brought into scope with `use` also have
this restriction.

A module without a body is loaded from an external file, by default with the
same name as the module, plus the `.rs` extension. When a nested submodule is
loaded from an external file, it is loaded from a subdirectory path that
mirrors the module hierarchy.

```rust,ignore
// Load the `vec` module from `vec.rs`
mod vec;

mod thread {
    // Load the `local_data` module from `thread/local_data.rs`
    // or `thread/local_data/mod.rs`.
    mod local_data;
}
```

The directories and files used for loading external file modules can be
influenced with the `path` attribute.

```rust,ignore
#[path = "thread_files"]
mod thread {
    // Load the `local_data` module from `thread_files/tls.rs`
    #[path = "tls.rs"]
    mod local_data;
}
```

Modules implicitly have some names in scope. These name are to built-in types,
macros imported with `#[macro_use]` on an extern crate, and by the crate's
[prelude]. These names are all made of a single identifier. These names are not
part of the module, so for example, any name `name`, `self::name` is not a
valid path. The names added by the [prelude] can be removed by placing the
`no_implicit_prelude` [attribute] onto the module.

## Attributes on Modules

Modules, like all items, accept outer attributes. They also accept inner
attibutes: either after `{` for a module with a body, or at the beginning of the
source file, after the optional BOM and shebang.

The built-in attributes that have meaning on a function are [`cfg`],
[`deprecated`], [`doc`], [the lint check attributes], `path`, and
`no_implicit_prelude`. Modules also accept macro attributes.

[_InnerAttribute_]: attributes.html
[_Item_]: items.html
[_OuterAttribute_]: attributes.html
[`cfg`]: conditional-compilation.html
[`deprecated`]: attributes.html#deprecation
[`doc`]: attributes.html#documentation
[IDENTIFIER]: identifiers.html
[attribute]: attributes.html
[items]: items.html
[prelude]: crates-and-source-files.html#preludes-and-no_std
[the lint check attributes]: attributes.html#lint-check-attributes
