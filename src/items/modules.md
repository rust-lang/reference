r[items.mod]
# Modules

r[items.mod.syntax]
```grammar,items
Module ->
      `unsafe`? `mod` IDENTIFIER `;`
    | `unsafe`? `mod` IDENTIFIER `{`
        InnerAttribute*
        Item*
      `}`
```

r[items.mod.intro]
A module is a container for zero or more [items].

r[items.mod.def]
A _module item_ is a module, surrounded in braces, named, and prefixed with the
keyword `mod`. A module item introduces a new, named module into the tree of
modules making up a crate.

r[items.mod.nesting]
Modules can nest arbitrarily.

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

r[items.mod.namespace]
Modules are defined in the [type namespace] of the module or block where they are located.

r[items.mod.multiple-items]
It is an error to define multiple items with the same name in the same namespace within a module.
See the [scopes chapter] for more details on restrictions and shadowing behavior.

r[items.mod.unsafe]
The `unsafe` keyword is syntactically allowed to appear before the `mod`
keyword, but it is rejected at a semantic level. This allows macros to consume
the syntax and make use of the `unsafe` keyword, before removing it from the
token stream.

r[items.mod.outlined]
## Module Source Filenames

r[items.mod.outlined.intro]
A module without a body is loaded from an external file. When the module does
not have a `path` attribute, the path to the file mirrors the logical [module
path].

r[items.mod.outlined.search]
Ancestor module path components are directories, and the module's
contents are in a file with the name of the module plus the `.rs` extension.
For example, the following module structure can have this corresponding
filesystem structure:

Module Path               | Filesystem Path  | File Contents
------------------------- | ---------------  | -------------
`crate`                   | `lib.rs`         | `mod util;`
`crate::util`             | `util.rs`        | `mod config;`
`crate::util::config`     | `util/config.rs` |

r[items.mod.outlined.search-mod]
Module filenames may also be the name of the module as a directory with the
contents in a file named `mod.rs` within that directory. The above example can
alternately be expressed with `crate::util`'s contents in a file named
`util/mod.rs`. It is not allowed to have both `util.rs` and `util/mod.rs`.

> [!NOTE]
> Prior to `rustc` 1.30, using `mod.rs` files was the way to load a module with nested children. It is encouraged to use the new naming convention as it is more consistent, and avoids having many files named `mod.rs` within a project.

r[items.mod.outlined.path]
### The `path` attribute

r[items.mod.outlined.path.intro]
The directories and files used for loading external file modules can be
influenced with the `path` attribute.

r[items.mod.outlined.path.search]
For `path` attributes on modules not inside inline module blocks, the file
path is relative to the directory the source file is located. For example, the
following code snippet would use the paths shown based on where it is located:

<!-- ignore: requires external files -->
```rust,ignore
#[path = "foo.rs"]
mod c;
```

Source File    | `c`'s File Location | `c`'s Module Path
-------------- | ------------------- | ----------------------
`src/a/b.rs`   | `src/a/foo.rs`      | `crate::a::b::c`
`src/a/mod.rs` | `src/a/foo.rs`      | `crate::a::c`

r[items.mod.outlined.path.search-nested]
For `path` attributes inside inline module blocks, the relative location of
the file path depends on the kind of source file the `path` attribute is
located in. "mod-rs" source files are root modules (such as `lib.rs` or
`main.rs`) and modules with files named `mod.rs`. "non-mod-rs" source files
are all other module files. Paths for `path` attributes inside inline module
blocks in a mod-rs file are relative to the directory of the mod-rs file
including the inline module components as directories. For non-mod-rs files,
it is the same except the path starts with a directory with the name of the
non-mod-rs module. For example, the following code snippet would use the paths
shown based on where it is located:

<!-- ignore: requires external files -->
```rust,ignore
mod inline {
    #[path = "other.rs"]
    mod inner;
}
```

Source File    | `inner`'s File Location   | `inner`'s Module Path
-------------- | --------------------------| ----------------------------
`src/a/b.rs`   | `src/a/b/inline/other.rs` | `crate::a::b::inline::inner`
`src/a/mod.rs` | `src/a/inline/other.rs`   | `crate::a::inline::inner`

An example of combining the above rules of `path` attributes on inline modules
and nested modules within (applies to both mod-rs and non-mod-rs files):

<!-- ignore: requires external files -->
```rust,ignore
#[path = "thread_files"]
mod thread {
    // Load the `local_data` module from `thread_files/tls.rs` relative to
    // this source file's directory.
    #[path = "tls.rs"]
    mod local_data;
}
```

r[items.mod.attributes]
## Attributes on Modules

r[items.mod.attributes.intro]
Modules, like all items, accept outer attributes. They also accept inner
attributes: either after `{` for a module with a body, or at the beginning of the
source file, after the optional BOM and shebang.

r[items.mod.attributes.supported]
The built-in attributes that have meaning on a module are [`cfg`],
[`deprecated`], [`doc`], [the lint check attributes], [`path`], and
[`no_implicit_prelude`]. Modules also accept macro attributes.

[`cfg`]: ../conditional-compilation.md
[`deprecated`]: ../attributes/diagnostics.md#the-deprecated-attribute
[`doc`]: ../../rustdoc/the-doc-attribute.html
[`no_implicit_prelude`]: ../names/preludes.md#the-no_implicit_prelude-attribute
[`path`]: #the-path-attribute
[attribute]: ../attributes.md
[items]: ../items.md
[module path]: ../paths.md
[scopes chapter]: ../names/scopes.md
[the lint check attributes]: ../attributes/diagnostics.md#lint-check-attributes
[type namespace]: ../names/namespaces.md

<script>
(function() {
    var fragments = {
        "#prelude-items": "../names/preludes.html",
    };
    var target = fragments[window.location.hash];
    if (target) {
        var url = window.location.toString();
        var base = url.substring(0, url.lastIndexOf('/'));
        window.location.replace(base + "/" + target);
    }
})();
</script>
