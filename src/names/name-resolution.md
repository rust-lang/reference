r[names.resolution]
# Name resolution

r[names.resolution.intro]

_Name resolution_ is the process of tying paths and other identifiers to the
declarations of those entities. Names are segregated into different
[namespaces], allowing entities in different namespaces to share the same name
without conflict. Each name is valid within a [scope], or a region of source
text where that name may be referenced. Access to certain names may be
restricted based on their [visibility].

* Names are resolved at three different stages of compilation.
* [Macros] and [use declarations] are resolved during macro expansion.
    * This stage of resolution is known as "Early Resolution".
* `Type::assoc_item`, `<Type>::assoc_item`, `<Enum>::Variant` and `EnumTyAlias::Variant` are resolved during type checking
    * `Trait::assoc_item`, `<Type as Trait>::assoc_item` and `Enum::Variant` are resolved during late resolution
    * This stage of resolution is known as type-relative resolution.
        * in reality this is never talked about so I doubt it has a name yet.
* All other names are resolved during AST lowering.
    * This stage of resolution is known as "Late Resolution".
    * Note, late resolution occurs before type dependent resolution.

r[names.resolution.early]
## Early name resolution

r[names.resolution.early.intro]

* early name resolution is the part of name resolution that happens during macro expansion
* early name resolution includes the resolution of imports and macros
* early name resolution is the minimum amount of resolution required to resolve macro invocations so they can be expanded.
* resolving imports is necessary to resolve macro invocations
    * specifically for path-based scope macro resolutions, this can occur
      either because of a `#[macro_export]`, a proc macro definition, or a
      reexport of a macro in 2018 or later code.
* resolving macro invocations and tying them to macro declarations is necessary so they can be expanded
* this process is iterative and repeats until there are no remaining unexpanded macro invocations (fixed point algorithm)
* Post expansion these resolutions are checked again to ensure no new ambiguities were introduced by the expansion process
  * This causes so called time traveling ambiguities, such as when a glob import introduces an item that is ambiguous with its own base path.

TODO Document some time traveling ambiguitie examples, place in relevant ambiguity section

r[names.resolution.early.imports]

* All imports are fully resolved at this point.
    * imports of names that cannot be fully resolved during macro expansion, such as those depending on type information, are not supported and will produce an error.

TODO example of unsupported type dependent import

r[names.resolution.early.imports.shadowing]

The following is a list of situations where shadowing of use declarations is permitted:

* [use glob shadowing]
* [macro textual scope shadowing]

r[names.resolution.early.imports.errors]
r[names.resolution.early.imports.errors.ambiguity]

* shadowing and ambiguity may or may not represent the same section or one may be a subsection of the other

* Builtin Attributes
* Derive Helpers
* Textual Vs Path-based Scope
* Glob vs Outer
* Glob vs Glob
* ~~Glob vs Expanded~~ pretty certain we don't want to mention this one
* More Expanded vs Outer

r[names.resolution.expansion.imports.errors.ambiguity]
## Ambiguities

> [!NOTE]
> This section is incomplete.

r[items.use.ambiguities.intro]
Some situations are an error when there is an ambiguity as to which name a `use` declaration refers. This happens when there are two name candidates that do not resolve to the same entity.

* except where shadowing is allowed
r[names.resolution.early.imports.errors.ambiguity.globvsglob]
* it is an error to name an item through ambiguous use declarations
 * two globs imports which both have an item matching that name where the items are different
     * this is not an error even if is a third non glob binding resolution to an item with the same name
* it is not an error to have two glob imports which include items which would be ambiguous so long as you do not name one of those items through the ambiguous glob imports
    * Should this live alongside use decls item page or in the name resolution page?

r[items.use.ambiguities.glob]
Glob imports are allowed to import conflicting names in the same namespace as long as the name is not used.
For example:

TODO: move this section? It's documenting a situation that _isnt_ an ambiguity
error. I've been working off of a pattern I think I saw in a few other
locations, where we have specific error sections that document all of the
reference relevant error cases associated with an some part of the language.
    * This section does technically document globvsglob ambituity errors, but
      it does so indirectly. We never explicitly mention "you can't resolve a
      name through a glob import when there are multiple candidate glob imports
      in scope that each resolve to different entities". We just say "you can
      do that if you don't actually use the ambiguious names" and have an
      example that shows that trying to use the name would be an error.

```rust
mod foo {
    pub struct Qux;
}

mod bar {
    pub struct Qux;
}

use foo::*;
use bar::*; //~ OK, no name conflict.

fn ambiguous_use() {
    // This would be an error, due to the ambiguity.
    //let x = Qux;
}

fn ambiguous_shadow() {
    // This is permitted, since resolution is not through the ambiguous globs
    struct Qux;
    let x = Qux;
}
```


Multiple glob imports are allowed to import the same name, and that name is allowed to be used, if the imports are of the same item (following re-exports). The visibility of the name is the maximum visibility of the imports. For example:

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

r[names.resolution.early.imports.errors.ambiguity.builtin-attr]
* it is an error to use a user defined attribute or derive macro with the same name as a builtin attribute (e.g. inline)
    * I think we may special case this one and allow certain kinds of
      ambiguities where the builtin-attr is shadowed by a user attribute (not
      sure if this actually exists or is just proposed, TODO investigate)

<!-- ignore: test doesn't support proc-macro -->
```rust,ignore
// myinline/src/lib.rs
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn inline(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
```

<!-- ignore: requires external crates -->
```rust,ignore
// src/lib.rs
use myinline::inline;
use myinline::inline as myinline;

#[myinline::inline]
pub fn foo() {}

#[crate::inline]
pub fn bar() {}

#[myinline]
pub fn baz() {}

#[inline] // ERROR `inline` is ambiguous
pub fn quix() {}
```

r[names.resolution.early.imports.errors.ambiguity.derivehelper]
* derive helpers used before their associated derive may not shadow other attributes or other derive helpers that are otherwise in scope after their derive
    * TODO example? This ones harder to do concisely afaik

Helper attributes may not be used before the macro that introduces them.

* What happens if two macros introduce the same helper, will the second one not
  be able to see the attribute of the first anymore, assuming their order is
  "firstmacro" "helper" "secondmacro"?

> [!NOTE]
> rustc currently allows derive helpers to be used before their attribute macro
> introduces them into scope so long as they do not shadow any other attributes
> or derive helpers that are otherwise correctly in scope. This behavior
> deprecated and slated for removal.
> <!-- ignore: requires external crates -->
> ```rust,ignore
> #[helper] // deprecated, hard error in the future
> #[derive(WithHelperAttr)]
> struct Struct {
>     field: (),
> }
> ```
>
> For more details, see [Rust issue #79202](https://github.com/rust-lang/rust/issues/79202).


r[names.resolution.early.imports.errors.ambiguity.textualvspathbasedscope]
* path-based scope bindings for macros may not shadow textual scope bindings to macros
    * This is sort of an intersection between macros and imports, because at
      least in stable rust you can only get path-based macro resolutions from
      imports of mbe macros (and presumably from proc macro crates), but you
      can only get textual scope of macros from macro declarations
    * https://doc.rust-lang.org/nightly/reference/names/namespaces.html#r-names.namespaces.sub-namespaces.use-shadow
    * [macro.decl.scope.path.ambiguity]
r[names.resolution.early.imports.errors.ambiguity.globvsouter]
* it is an error to shadow an outer name binding with a glob import
    * This seems to only apply to early resolution (duh, I documented this as part of an early resolution codepath)
        *   // Below we report various ambiguity errors.
            // We do not need to report them if we are either in speculative resolution,
            // or in late resolution when everything is already imported and expanded
            // and no ambiguities exist.
    * I attempted to produce an example using structs and it allowed the outer import to shadow the inner glob just fine

```rust
mod bar {
    pub struct Name;
}

mod baz {
    pub struct Name;
}

use baz::Name;

pub fn foo() {
    use bar::*;
    Name;
}
```

* I'd like to have a better understanding of why this doesn't trigger ambiguity errors.
    * I'm taking a guess but I think it has to do with how and when we resolve
      names during early resolution. We resolve all the imports but ambiguities
      only occur when observed, so we'd need to try to resolve Name during
      early resolution which simply won't happen because it is a struct so it
      will never be visited for resolution during expansion.
    * We will end up resolving the imports themselves, but they'll resolve fine
      because the imports themselves aren't ambiguous
    * By the time we get to late resolution we no longer expect there to be any
      ambiguities, so we will happily return the first resolution result and
      never search for additional ambiguities, so we resolve directly to
      `bar::Name` through the glob import

    * doing it with macros produced the expected error
```rust
mod bar {
    macro_rules! name {
        () => {}
    }
    pub(crate) use name;
}

mod baz {
    macro_rules! name {
        () => {}
    }
    pub(crate) use name;
}

use baz::name;

pub fn foo() {
    use bar::*;
    name!(); // ERROR `name` is ambiguous
}
```

* how does it work with imports? The same as macros, same error during early resolution

```rust
mod bar {
    pub mod foo {
        pub struct Name;
    }
}

mod baz {
    pub mod foo {
        pub struct Name;
    }
}

use baz::foo;

pub fn foo() {
    use bar::*;
    use foo::Name; // `foo` is ambiguous
}
```

r[macro.decl.scope.textual.ambiguity.moreexpandedvsouter]
* it is an error for name bindings from macro expansions to shadow name bindings from outside of those expansions

```rust
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
    name!(); // ERROR `name` is ambiguous
}
```


r[names.resolution.early.imports.errors.ambiguity.globvsexpanded]
* Grey Area

r[names.resolution.early.macros]

* .visitation-order
    * derive helpers
        * not visited when resolving derive macros in the parent scope (starting scope)
    * derive helpers compat
        * always visited
    * macro rules bindings (textual scope macros)
        * always visited
    * modules (path-based scope macros)
        * always visited
    * macrouseprelude
        * not visited in 2018 and later when `#[no_implicit_prelude]` is present
    * stdlibprelude
        * always visited for macro resolutions
        * is it? what about no-std + no-core?
    * builtinattrs
        * always visited
* .subnamespaces
    * macros are split into two subnamespaces, one for bang macros, and the other for attributes and derives. Resolution candidates from the incorrect subnamespace are ignored
        * https://doc.rust-lang.org/nightly/reference/names/namespaces.html#r-names.namespaces.sub-namespaces

r[names.resolution.early.macros.errors.reserved-names]

the names cfg and cfg_attr are reserved in the macro attribute sub-namespace

* https://doc.rust-lang.org/nightly/reference/names/namespaces.html#r-names.namespaces.sub-namespaces


r[names.resolution.late]

r[names.resolution.type-dependent]

[use glob shadowing]: ../items/use-declarations.md#r-items.use.glob.shadowing
[Macros]: ../macros.md
[use declarations]: ../items/use-declarations.md
[macro textual scope shadowing]: ../macros-by-example.md#r-macro.decl.scope.textual.shadow
[`let` bindings]: ../statements.md#let-statements
[item definitions]: ../items.md
[namespaces]: ../names/namespaces.md
[scope]: ../names/scopes.md
[visibility]: ../visibility-and-privacy.md
