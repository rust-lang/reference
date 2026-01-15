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

```rust,no_run
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
use m::A::V; // ERROR: Unresolved import `m::A::V`.
use m::E::C; // ERROR: Unresolved import `m::E::C`.
```

r[names.resolution.expansion.imports.shadowing]
Names introduced via `use` declarations in an [outer scope] are shadowed by candidates in the same namespace with the same name from an inner scope except where otherwise restricted by [name resolution ambiguities].

```rust,no_run
pub mod m1 {
    pub mod ambig {
        pub const C: u8 = 1;
    }
}

pub mod m2 {
    pub mod ambig {
        pub const C: u8 = 2;
    }
}

// This introduces the name `ambig` in the outer scope.
use m1::ambig;
const _: () = {
    // This shadows `ambig` in the inner scope.
    use m2::ambig;
    // The inner candidate is selected here
    // as the resolution of `ambig`.
    use ambig::C;
    assert!(C == 2);
};
```

r[names.resolution.expansion.imports.shadowing.shared-scope]
Shadowing of names introduced via `use` declarations within a single scope is permitted in the following situations:

- [`use` glob shadowing]
- [Macro textual scope shadowing]

r[names.resolution.expansion.imports.ambiguity]
#### Ambiguities

r[names.resolution.expansion.imports.ambiguity.intro]
There are certain situations during expansion-time resolution where there are multiple macro definitions, `use` declarations, or modules an import or macro invocation's name could refer to where the compiler cannot consistently determine which candidate should shadow the other. Shadowing cannot be permitted in these situations and the compiler instead emits ambiguity errors.

r[names.resolution.expansion.imports.ambiguity.glob-vs-glob]
Names may not be resolved through ambiguous glob imports. Glob imports are allowed to import conflicting names in the same namespace as long as the name is not used. Names with conflicting candidates from ambiguous glob imports may still be shadowed by non-glob imports and used without producing an error. The errors occur at time of use, not time of import.

```rust,compile_fail,E0659
mod m1 {
    pub struct Ambig;
}

mod m2 {
    pub struct Ambig;
}

// OK: This brings conficting names in the same namespace into scope
// but they have not been used yet.
use m1::*;
use m2::*;

const _: () = {
    // The error happens when the name with the conflicting candidates
    // is used.
    let x = Ambig; // ERROR: `Ambig` is ambiguous.
};
```

```rust,no_run
# mod m1 {
#     pub struct Ambig;
# }
#
# mod m2 {
#     pub struct Ambig;
# }
#
# use m1::*;
# use m2::*; // OK: No name conflict.
const _: () = {
    // This is permitted, since resolution is not through the
    // ambiguous globs.
    struct Ambig;
    let x = Ambig; // OK.
};
```

Multiple glob imports are allowed to import the same name, and that name is allowed to be used if the imports are of the same item (following reexports). The visibility of the name is the maximum visibility of the imports.

```rust,no_run
mod m1 {
    pub struct Ambig;
}

mod m2 {
    // This reexports the same `Ambig` item from a second module.
    pub use super::m1::Ambig;
}

mod m3 {
    // These both import the same `Ambig`.
    //
    // The visibility of `Ambig` is `pub` because that is the
    // maximum visibility between these two `use` declarations.
    pub use super::m1::*;
    use super::m2::*;
}

mod m4 {
    // `Ambig` can be used through the `m3` globs and still has
    // `pub` visibility.
    pub use crate::m3::Ambig;
}

const _: () = {
    // Therefore, we can use it here.
    let _ = m4::Ambig; // OK.
};
# fn main() {}
```

r[names.resolution.expansion.imports.ambiguity.glob-vs-outer]
Names in imports and macro invocations may not be resolved through glob imports when there is another candidate available in an [outer scope].

r[names.resolution.expansion.imports.ambiguity.panic-hack]
> [!NOTE]
> When one of [`core::panic!`] or [`std::panic!`] is brought into scope due to the [standard library prelude], and a user-written [glob import] brings the other into scope, `rustc` currently allows use of `panic!`, even though it is ambiguous. The user-written glob import takes precedence to resolve this ambiguity.
>
> In Rust 2021 and later, [`core::panic!`] and [`std::panic!`] operate identically. But in earlier editions, they differ; only [`std::panic!`] accepts a [`String`] as the format argument.
>
> E.g., this is an error:
>
> ```rust,edition2018,compile_fail,E0308
> extern crate core;
> use ::core::prelude::v1::*;
> fn main() {
>     panic!(std::string::String::new()); // ERROR.
> }
> ```
>
> And this is accepted:
>
> <!-- ignore: Can't test with `no_std`. -->
> ```rust,edition2018,ignore
> #![no_std]
> extern crate std;
> use ::std::prelude::v1::*;
> fn main() {
>     panic!(std::string::String::new()); // OK.
> }
> ```
>
> Don't rely on this behavior; the plan is to remove it.
>
> For details, see [Rust issue #147319](https://github.com/rust-lang/rust/issues/147319).

```rust,compile_fail,E0659
mod glob {
    pub mod ambig {
        pub struct Name;
    }
}

// Outer `ambig` candidate.
pub mod ambig {
    pub struct Name;
}

const _: () = {
    // Cannot resolve `ambig` through this glob
    // because of the outer `ambig` candidate above.
    use glob::*;
    use ambig::Name; // ERROR: `ambig` is ambiguous.
};
```

```rust,compile_fail,E0659
// As above, but with macros.
pub mod m {
    macro_rules! f {
        () => {};
    }
    pub(crate) use f;
}
pub mod glob {
    macro_rules! f {
        () => {};
    }
    pub(crate) use f as ambig;
}

use m::f as ambig;

const _: () = {
    use glob::*;
    ambig!(); // ERROR: `ambig` is ambiguous.
};
```

> [!NOTE]
> These ambiguity errors are specific to expansion-time resolution. Having multiple candidates available for a given name during later stages of resolution is not considered an error. So long as none of the imports themselves are ambiguous, there will always be a single unambiguous closest resolution.
>
> ```rust,no_run
> mod glob {
>     pub const AMBIG: u8 = 1;
> }
>
> mod outer {
>     pub const AMBIG: u8 = 2;
> }
>
> use outer::AMBIG;
>
> const C: () = {
>     use glob::*;
>     assert!(AMBIG == 1);
>     //      ^---- This `AMBIG` is resolved during primary resolution.
> };
> ```

r[names.resolution.expansion.imports.ambiguity.path-vs-textual-macro]
Names may not be resolved through ambiguous macro reexports. Macro reexports are ambiguous when they would shadow a textual macro candidate for the same name in an [outer scope].

```rust,compile_fail,E0659
// Textual macro candidate.
macro_rules! ambig {
    () => {}
}

// Path-based macro candidate.
macro_rules! path_based {
    () => {}
}

pub fn f() {
    // This reexport of the `path_based` macro definition
    // as `ambig` may not shadow the `ambig` macro definition
    // which is resolved via textual macro scope.
    use path_based as ambig;
    ambig!(); // ERROR: `ambig` is ambiguous.
}
```

> [!NOTE]
> This restriction is needed due to implementation details in the compiler, specifically the current scope visitation logic and the complexity of supporting this behavior. This ambiguity error may be removed in the future.

r[names.resolution.expansion.macros]
### Macros

r[names.resolution.expansion.macros.intro]
Macros are resolved by iterating through the available scopes to find the available candidates. Macros are split into two sub-namespaces, one for function-like macros, and the other for attributes and derives. Resolution candidates from the incorrect sub-namespace are ignored.

r[names.resolution.expansion.macros.visitation-order]
The available scope kinds are visited in the following order. Each of these scope kinds represent one or more scopes.

* [Derive helpers]
* [Textual scope macros]
* [Path-based scope macros]
* [`macro_use` prelude]
* [Standard library prelude]
* [Builtin attributes]

> [!NOTE]
> The compiler will attempt to resolve derive helpers that are used before their associated macro introduces them into scope. This scope is visited after the scope for resolving derive helper candidates that are correctly in scope. This behavior is slated for removal.
>
> For more info see [derive helper scope].

> [!NOTE]
> This visitation order may change in the future, such as interleaving the visitation of textual and path-based scope candidates based on their lexical scopes.

> [!EDITION-2018]
> Starting in edition 2018 the `#[macro_use]` prelude is not visited when [`#[no_implicit_prelude]`][names.preludes.no_implicit_prelude] is present.

r[names.resolution.expansion.macros.reserved-names]
The names `cfg` and `cfg_attr` are reserved in the macro attribute [sub-namespace].

r[names.resolution.expansion.macros.ambiguity]
#### Ambiguities

r[names.resolution.expansion.macros.ambiguity.more-expanded-vs-outer]
Names may not be resolved through ambiguous candidates inside of macro expansions. Candidates inside of macro expansions are ambiguous when they would shadow a candidate for the same name from outside of the first candidate's macro expansion and the invocation of the name being resolved is also from outside of the first candidate's macro expansion.

```rust,compile_fail,E0659
macro_rules! define_ambig {
    () => {
        macro_rules! ambig {
            () => {}
        }
    }
}

// Introduce outer candidate definition for `ambig` macro invocation.
macro_rules! ambig {
    () => {}
}

// Introduce a second candidate definition for `ambig` inside of a
// macro expansion.
define_ambig!();

// The definition of `ambig` from the second invocation
// of `define_ambig` is the innermost canadidate.
//
// The definition of `ambig` from the first invocation of
// `define_ambig` is the second candidate.
//
// The compiler checks that the first candidate is inside of a macro
// expansion, that the second candidate is not from within the same
// macro expansion, and that the name being resolved is not from
// within the same macro expansion.
ambig!(); // ERROR: `ambig` is ambiguous.
```

The reverse is not considered ambiguous.

```rust,no_run
# macro_rules! define_ambig {
#     () => {
#         macro_rules! ambig {
#             () => {}
#         }
#     }
# }
// Swap order of definitions.
define_ambig!();
macro_rules! ambig {
    () => {}
}
// The innermost candidate is now less expanded so it may shadow more
// the macro expanded definition above it.
ambig!();
```

Nor is it ambiguous if the invocation being resolved is within the innermost candidate's expansion.

```rust,no_run
macro_rules! ambig {
    () => {}
}

macro_rules! define_and_invoke_ambig {
    () => {
        // Define innermost candidate.
        macro_rules! ambig {
            () => {}
        }

        // Invocation of `ambig` is in the same expansion as the
        // innermost candidate.
        ambig!(); // OK
    }
}

define_and_invoke_ambig!();
```

It doesn't matter if both definitions come from invocations of the same macro; the outermost candidate is still considered "less expanded" because it is not within the expansion containing the innermost candidate's definition.

```rust,compile_fail,E0659
# macro_rules! define_ambig {
#     () => {
#         macro_rules! ambig {
#             () => {}
#         }
#     }
# }
define_ambig!();
define_ambig!();
ambig!(); // ERROR: `ambig` is ambiguous.
```

This also applies to imports so long as the innermost candidate for the name is from within a macro expansion.

```rust,compile_fail,E0659
macro_rules! define_ambig {
    () => {
        mod ambig {
            pub struct Name;
        }
    }
}

mod ambig {
    pub struct Name;
}

const _: () = {
    // Introduce innermost candidate for
    // `ambig` mod in this macro expansion.
    define_ambig!();
    use ambig::Name; // ERROR: `ambig` is ambiguous.
};
```

r[names.resolution.expansion.macros.ambiguity.built-in-attr]
User-defined attributes or derive macros may not shadow built-in non-macro attributes (e.g. inline).

<!-- ignore: test doesn't support proc-macro -->
```rust,ignore
// with-helper/src/lib.rs
# use proc_macro::TokenStream;
#[proc_macro_derive(WithHelperAttr, attributes(non_exhaustive))]
//                                             ^^^^^^^^^^^^^^
//                                   User-defined attribute candidate.
// ...
# pub fn derive_with_helper_attr(_item: TokenStream) -> TokenStream {
#     TokenStream::new()
# }
```

<!-- ignore: requires external crates -->
```rust,ignore
// src/lib.rs
#[derive(with_helper::WithHelperAttr)]
#[non_exhaustive] // ERROR: `non_exhaustive` is ambiguous.
struct S;
```

> [!NOTE]
> This applies regardless of the name the built-in attribute is a candidate for:
>
> <!-- ignore: test doesn't support proc-macro -->
> ```rust,ignore
> // with-helper/src/lib.rs
> # use proc_macro::TokenStream;
> #
> #[proc_macro_derive(WithHelperAttr, attributes(helper))]
> //                                             ^^^^^^
> //                                 User-defined attribute candidate.
> // ...
> # pub fn derive_with_helper_attr(_item: TokenStream) -> TokenStream {
> #     TokenStream::new()
> # }
> ```
>
> <!-- ignore: requires external crates -->
> ```rust,ignore
> // src/lib.rs
> use inline as helper;
> //            ^----- Built-in attribute candidate via reexport.
>
> #[derive(with_helper::WithHelperAttr)]
> #[helper] // ERROR: `helper` is ambiguous.
> struct S;
> ```

r[names.resolution.primary]
## Primary name resolution
> [!NOTE]
> This is a placeholder for future expansion about primary name resolution.

r[names.resolution.type-relative]
## Type-relative resolution
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
[glob import]: items.use.glob
[item definitions]: ../items.md
[macro invocations]: ../macros.md#macro-invocation
[macro textual scope shadowing]: ../macros-by-example.md#r-macro.decl.scope.textual.shadow
[name resolution ambiguities]: #r-names.resolution.expansion.imports.ambiguity
[namespaces]: ../names/namespaces.md
[outer scope]: #r-names.resolution.general.scopes
[path-based scope]: ../macros.md#r-macro.invocation.name-resolution
[scope]: ../names/scopes.md
[sub-namespace]: ../names/namespaces.md#r-names.namespaces.sub-namespaces
[type-relative paths]: names.resolution.type-relative
[visibility]: ../visibility-and-privacy.md
