r[names.resolution]
# Name resolution

r[names.resolution.intro]
_Name resolution_ is the process of tying paths and other identifiers to the declarations of those entities. Names are segregated into different [namespaces], allowing entities in different namespaces to share the same name without conflict. Each name is valid within a [scope], or a region of source text where that name may be referenced. Access to a name may be restricted based on its [visibility].

Name resolution is split into three stages throughout the compilation process. The first stage, *expansion-time resolution*, resolves all [`use` declarations] and [macro invocations]. The second stage, *primary resolution*, resolves all names that have not yet been resolved and that do not depend on type information to resolve. The last stage, *type-relative resolution*, resolves the remaining names once type information is available.

> [!NOTE]
> Expansion-time resolution is also known as *early resolution*. Primary resolution is also known as *late resolution*.

r[names.resolution.general]
## General

r[names.resolution.general.intro]
The rules within this section apply to all stages of name resolution.

r[names.resolution.general.scopes]
### Scopes

r[names.resolution.general.scopes.intro]
> [!NOTE]
> This is a placeholder for future expansion about resolution of names within various scopes.

r[names.resolution.expansion]
## Expansion-time name resolution

r[names.resolution.expansion.intro]
Expansion-time name resolution is the stage of name resolution necessary to complete macro expansion and fully generate a crate's [AST]. This stage requires the resolution of macro invocations and `use` declarations. Resolving `use` declarations is required for macro invocations that resolve via [path-based scope]. Resolving macro invocations is required in order to expand them.

r[names.resolution.expansion.unresolved-invocations]
After expansion-time name resolution, the AST must not contain any unexpanded macro invocations. Every macro invocation resolves to a valid definition that exists in the final AST or in an external crate.

```rust,compile_fail
m!(); // ERROR: Cannot find macro `m` in this scope.
```

r[names.resolution.expansion.expansion-order-stability]
The resolution of names must be stable. After expansion, names in the fully expanded AST must resolve to the same definition regardless of the order in which macros are expanded and imports are resolved.

r[names.resolution.expansion.speculation]
All name resolution candidates selected during macro expansion are considered speculative. Once the crate has been fully expanded, all speculative import resolutions are validated to ensure that macro expansion did not introduce any new ambiguities.

> [!NOTE]
> Due to the iterative nature of macro expansion, this causes so-called time traveling ambiguities, such as when a macro or glob import introduces an item that is ambiguous with its own base path.
>
> ```rust,compile_fail,E0659
> # fn main() {}
> macro_rules! f {
>     () => {
>         mod m {
>             pub(crate) use f;
>         }
>     }
> }
> f!();
>
> const _: () = {
>     // Initially, we speculatively resolve `m` to the module in
>     // the crate root.
>     //
>     // Expansion of `f` introduces a second `m` module inside this
>     // body.
>     //
>     // Expansion-time resolution finalizes resolutions by re-
>     // resolving all imports and macro invocations, sees the
>     // introduced ambiguity and reports it as an error.
>     m::f!(); // ERROR: `m` is ambiguous.
> };
> ```

r[names.resolution.expansion.imports]
### Imports
r[names.resolution.expansion.imports.intro]
All `use` declarations are fully resolved during this stage of resolution. [Type-relative paths] cannot be resolved at this stage and will produce an error.

```rust
mod m {
    pub const C: () = ();
    pub enum E { V }
    pub type A = E;
    impl E {
        pub const C: () = ();
    }
}

// Valid imports resolved at expansion-time:
use m::C; // OK.
use m::E; // OK.
use m::A; // OK.
use m::E::V; // OK.

// Valid expressions resolved during type-relative resolution:
let _ = m::A::V; // OK.
let _ = m::E::C; // OK.
```

```rust,compile_fail,E0432
# mod m {
#     pub const C: () = ();
#     pub enum E { V }
#     pub type A = E;
#     impl E {
#         pub const C: () = ();
#     }
# }
// Invalid type-relative imports that can't resolve at expansion-time:
use m::E::C; // ERROR: Unresolved import `m::E::C`.
use m::A::V; // ERROR: Unresolved import `m::A::V`.
```

r[names.resolution.expansion.imports.shadowing]
Names introduced via `use` declarations in an [outer scope] are shadowed by
candidates in the same namespace with the same name from an inner scope except
where otherwise restricted by [name resolution ambiguities].

```rust
pub mod foo {
    pub mod baz {
        pub struct Name;
    }
}

pub mod bar {
    pub mod baz {
        pub struct Name(pub ());
    }
}

use foo::baz;
fn f() {
    use bar::baz;
    use baz::Name
    Name(());
}
```

r[names.resolution.expansion.imports.shadowing.shared-scope]
Shadowing of names introduced via `use` declarations within a single scope is permitted in the following situations:

- [`use` glob shadowing]
- [Macro textual scope shadowing]

r[names.resolution.expansion.imports.ambiguity]
#### Ambiguities

r[names.resolution.expansion.imports.ambiguity.intro]
Some situations are an error when there is an ambiguity as to which macro definition, `use` declaration, or module an import or macro invocation's name refers to. This happens when there are two name candidates that do not resolve to the same entity where neither candidate is [permitted] to shadow the other.

r[names.resolution.expansion.imports.ambiguity.glob-vs-glob]
Names may not be resolved through ambiguous glob imports. Glob imports are allowed to import conflicting names in the same namespace as long as the name is not used. Names with conflicting candidates from ambiguous glob imports may still be shadowed by non glob imports and used without producing an error. The errors occur at time of use, not time of import.

For example:

```rust
mod foo {
    pub struct Qux;
}

mod bar {
    pub struct Qux;
}

use foo::*;
use bar::*; // OK, no name conflict.

fn ambiguous_use() {
    // This would be an error, due to the ambiguity.
    //let x = Qux;
}

fn ambiguous_shadow() {
    // This is permitted, since resolution is not through the ambiguous globs.
    struct Qux;
    let x = Qux;
}
```

Multiple glob imports are allowed to import the same name, and that name is allowed to be used if the imports are of the same item (following re-exports). The visibility of the name is the maximum visibility of the imports. For example:

```rust
mod foo {
    pub struct Qux;
}

mod bar {
    pub use super::foo::Qux;
}

// These both import the same `Qux`. The visibility of `Qux`
// is `pub` because that is the maximum visibility between
// these two `use` declarations.
pub use bar::*;
use foo::*;

fn main() {
    let _: Qux = Qux;
}
```

r[names.resolution.expansion.imports.ambiguity.glob-vs-outer]
Names may not be resolved through glob imports when there is another candidate available in an [outer scope].

```rust,compile_fail,E0659
mod bar {
    pub mod foo {
//          ^-- glob `foo` candidate
        pub struct Name;
    }
}

pub mod foo {
//      ^-- outer `foo` candidate
    pub struct Name;
}

pub fn qux() {
    use bar::*;
    use foo::Name; // ERROR: `foo` is ambiguous
}
```

```rust,compile_fail,E0659
pub mod bar {
    #[macro_export]
    macro_rules! m {
        () => {};
    }

    macro_rules! m2 {
        () => {};
    }
    pub(crate) use m2 as m;
}

pub fn qux() {
    use bar::*;
    m!(); // ERROR: `m` is ambiguous
}
```

> [!NOTE]
> These ambiguity errors are specific to imports, even though they are only observed when those imports are used. Having multiple candidates available for a given name during later stages of resolution is not considered an error. So long as none of the imports themselves are ambiguous, there will always be a single unambiguous closest resolution.
>
> ```rust
> mod bar {
>     pub const NAME: bool = true;
> }
>
> mod baz {
>     pub const NAME: bool = false;
> }
>
> use baz::NAME;
>
> pub fn foo() {
>     use bar::*;
>     assert!(NAME);
>     //      ^--- This `NAME` is resolved during primary resolution.
> }
> ```

r[names.resolution.expansion.imports.ambiguity.path-vs-textual-macro]
Path-based scope bindings for macros may not shadow textual scope bindings to macros.

```rust,compile_fail,E0659
macro_rules! m2 {
    () => {}
}
macro_rules! m {
    () => {}
}
pub fn foo() {
    m!(); // ERROR: `m` is ambiguous
    use m2 as m; // In scope for entire function body.
}
```

r[names.resolution.expansion.macros]
### Macros

r[names.resolution.expansion.macros.intro]
Macros are resolved by iterating through the available scopes to find the available candidates. Macros are split into two sub-namespaces, one for bang macros, and the other for attributes and derives. Resolution candidates from the incorrect sub-namespace are ignored.

r[names.resolution.expansion.macros.visitation-order]
The available scopes are visited in the following order.

* [Derive helpers]
* [Textual scope macros]
* [Path-based scope macros]
* [`macro_use` prelude]
* [Standard library prelude]
* [Builtin attributes]

> [!NOTE]
>
> The compiler will attempt to resolve derive helpers that are used before their associated macro introduces them into scope. This scope is visited after the scope for resolving derive helper candidates that are correctly in scope. This behavior is slated for removal.
>
> For more info see [derive helper scope].

> [!EDITION-2018]
>
> Starting in edition 2018 the `#[macro_use]` prelude is not visited when `#[no_implicit_prelude]` is present.

r[names.resolution.expansion.macros.derivehelpers]
Derive helper scopes are not visited when resolving derive macros in the parent scope (starting scope).

r[names.resolution.expansion.macros.reserved-names]
The names `cfg` and `cfg_attr` are reserved in the macro attribute [sub-namespace].

r[names.resolution.expansion.macros.ambiguity]
#### Ambiguities

r[names.resolution.expansion.macros.ambiguity.more-expanded-vs-outer]
Name bindings from macro expansions may not shadow name bindings from outside of those expansions.

```rust,compile_fail,E0659
macro_rules! name {
    () => {}
}

macro_rules! define_name {
    () => {
        macro_rules! name {
            () => {}
        }
    }
}

fn foo() {
    define_name!();
    name!(); // ERROR: `name` is ambiguous
}
```

r[names.resolution.expansion.macros.ambiguity.builtin-attr]
User defined attributes or derive macros may not shadow builtin non-macro attributes (e.g. inline).

<!-- ignore: test doesn't support proc-macro -->
```rust,ignore
// with-helper/src/lib.rs
# use proc_macro::TokenStream;
#
#[proc_macro_derive(WithHelperAttr, attributes(non_exhaustive))]
//                                             ^------------- user attr candidate
...
# pub fn derive_with_helper_attr(_item: TokenStream) -> TokenStream {
#     TokenStream::new()
# }
```

<!-- ignore: requires external crates -->
```rust,ignore
// src/lib.rs
#[derive(with_helper::WithHelperAttr)]
#[non_exhaustive] // ERROR: `non_exhaustive` is ambiguous
struct S;
```

> [!NOTE]
> This applies regardless of the name the builtin attribute is a candidate for:
>
> <!-- ignore: test doesn't support proc-macro -->
> ```rust,ignore
> // with-helper/src/lib.rs
> # use proc_macro::TokenStream;
> #
> #[proc_macro_derive(WithHelperAttr, attributes(helper))]
> //                                             ^----- user attr candidate
> ...
> # pub fn derive_with_helper_attr(_item: TokenStream) -> TokenStream {
> #     TokenStream::new()
> # }
> ```
>
> <!-- ignore: requires external crates -->
> ```rust,ignore
> // src/lib.rs
> use inline as helper;
> //            ^----- builtin attr candidate via re-export
>
> #[derive(with_helper::WithHelperAttr)]
> #[helper] // ERROR: `helper` is ambiguous
> struct S;
> ```

r[names.resolution.primary]
## Primary name resolution
> [!NOTE]
> This is a placeholder for future expansion about primary name resolution.

r[names.resolution.type-relative]
# Type-relative resolution
> [!NOTE]
> This is a placeholder for future expansion about type-dependent resolution.

[AST]: glossary.ast
[Builtin attributes]: ./preludes.md#r-names.preludes.lang
[Derive helpers]: ../procedural-macros.md#r-macro.proc.derive.attributes
[Macros]: ../macros.md
[Path-based scope macros]: ../macros.md#r-macro.invocation.name-resolution
[Standard library prelude]: ./preludes.md#r-names.preludes.std
[Textual scope macros]: ../macros-by-example.md#r-macro.decl.scope.textual
[`let` bindings]: ../statements.md#let-statements
[`macro_use` prelude]: ./preludes.md#r-names.preludes.macro_use
[`use` declarations]: ../items/use-declarations.md
[`use` glob shadowing]: ../items/use-declarations.md#r-items.use.glob.shadowing
[derive helper scope]: ../procedural-macros.md#r-macro.proc.derive.attributes.scope
[item definitions]: ../items.md
[macro invocations]: ../macros.md#macro-invocation
[macro textual scope shadowing]: ../macros-by-example.md#r-macro.decl.scope.textual.shadow
[name resolution ambiguities]: #r-names.resolution.expansion.imports.ambiguity
[namespaces]: ../names/namespaces.md
[outer scope]: #r-names.resolution.general.scopes
[path-based scope]: ../macros.md#r-macro.invocation.name-resolution
[permitted]: name-resolution.md#r-names.resolution.expansion.imports.shadowing
[scope]: ../names/scopes.md
[sub-namespace]: ../names/namespaces.md#r-names.namespaces.sub-namespaces
[type-relative paths]: names.resolution.type-relative
[visibility]: ../visibility-and-privacy.md
