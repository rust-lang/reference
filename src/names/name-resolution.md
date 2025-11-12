r[names.resolution]
# Name resolution

r[names.resolution.intro]

_Name resolution_ is the process of tying paths and other identifiers to the
declarations of those entities. Names are segregated into different
[namespaces], allowing entities in different namespaces to share the same name
without conflict. Each name is valid within a [scope], or a region of source
text where that name may be referenced. Access to certain names may be
restricted based on their [visibility].

Name resolution is split into three stages throughout the compilation process.
The first stage, Expansion-time resolution, resolves all [use declarations] and
[macro invocations]. The second stage, Primary resolution, resolves all names
that have not yet been resolved that do not depend on type information to
resolve. The last stage, Type-relative resolution, resolves the remaining names
once type information is available.

> Note
>
> * Expansion-time resolution is also known as "Early Resolution"
> * Primary resolution is also known as "Late Resolution"

r[names.resolution.expansion]
## Expansion-time name resolution

r[names.resolution.expansion.intro]

Expansion-time name resolution is the stage of name resolution necessary to
complete macro expansion and fully generate a crate's AST. This stage requires
the resolution of macro invocations and use declarations. Resolving use
declarations is required to resolve [path-based scope] macro invocations.
Resolving macro invocations is required in order to expand them.

The expansion process is iterative, alternately resolving imports, resolving
and expanding macro invocations, then repeating until there are no further
macros invocations to resolve. Once this process is completed all the imports
are resolved again to ensure that the macro expansion process did not introduce
any new ambiguious imports.

TODO: do we want to talk about this? feels like an implementation detail but
also really helps to understand certain kinds of ambiguity errors that users
can run into.

> Note
>
> This causes so called time traveling ambiguities, such as when a glob import introduces an item that is ambiguous with its own base path.
>
```rust,compile_fail
macro_rules! m {
    () => { mod bar {} }
}

mod bar {
    pub(crate) use m;
}

fn f() {
    // * initially speculatively resolve bar to the module in the crate root
    // * expansion of m introduces a second bar module inside the body of f
    // * expansion-time resolution finalizes resolutions by re-resolving all
    //   imports and macro invocations, sees the introduced ambiguity
    //   and reports it as an error
    bar::m!(); // ERROR `bar` is ambiguous
}
```

TODO I would like to be able to link to a path-based scope section that
     discusses the various kinds of macros that can be invoked via path-based scope.
     Right now the section I know of off of the top of my head lives in the macros
     by example chapter.

r[names.resolution.expansion.imports]

All use declarations are fully resolved during this stage of resolution.
Type-relative paths cannot be resolved at this stage of compilation and will
produce an error.

* `Type::assoc_item`, `<Type>::assoc_item`, `<Enum>::Variant` and `EnumTyAlias::Variant` are resolved during type checking
    * `Trait::assoc_item`, `<Type as Trait>::assoc_item` and `Enum::Variant` are resolved during late resolution

```rust,compile_fail
mod my_mod {
    pub const Const: () = ();

    pub enum MyEnum {
        MyVariant
    }

    impl MyEnum {
        pub const Const: () = ();
    }

    pub type TypeAlias = MyEnum;
}

fn foo() {
    use my_mod::MyEnum; // OK
    use my_mod::MyEnum::MyVariant; // OK
    use my_mod::TypeAlias; // OK
    use my_mod::TypeAlias::MyVariant; // Doesn't work
    use my_mod::MyEnum::Const; // Doesn't work
    use my_mod::Const; // OK
    let _ = my_mod::TypeAlias::MyVariant; // OK
    let _ = my_mod::MyEnum::Const; // OK
}
```

r[names.resolution.expansion.imports.shadowing]

The following is a list of situations where shadowing of use declarations is permitted:

* [use glob shadowing]
* [macro textual scope shadowing]

r[names.resolution.expansion.imports.errors]

TODO shadowing and ambiguity may or may not represent the same section or one may be a subsection of the other

The following is a list of situations where shadowing of use declarations is
_NOT_ permitted, otherwise known as ambiguity errors:

* Builtin Attributes
* Derive Helpers
* Textual Vs Path-based Scope
* Glob vs Outer
* Glob vs Glob
* ~~Glob vs Expanded~~ pretty certain we don't want to mention this one
* More Expanded vs Outer

r[names.resolution.expansion.imports.errors.ambiguity]
## Ambiguities

r[items.use.ambiguities.intro]
Some situations are an error when there is an ambiguity as to which name a
`use` declaration refers. This happens when there are two name candidates that
do not resolve to the same entity where neither import is
[permitted] to shadow the other.

r[names.resolution.early.imports.errors.ambiguity.globvsglob]
* it is an error to name an item through ambiguous use declarations
    * two globs imports which both have an item matching that name where the items are different
        * this is not an error even if is a third non glob binding resolution to an item with the same name
* it is not an error to have two glob imports which include items which would be ambiguous so long as you do not name one of those items through the ambiguous glob imports
    * Should this live alongside use decls item page or in the name resolution page?

r[items.use.ambiguities.glob]
Glob imports are allowed to import conflicting names in the same namespace as
long as the name is not used. Names may not be resolved through ambiguous glob
statements. Conflicting names from ambiguous glob statements may still be
shadowed and used without producing an error.

For example:

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
It is an error to use a user defined attribute or derive macro with the same
name as a builtin attribute (e.g. inline)
* I think we may special case this one and allow certain kinds of ambiguities
  where the builtin-attr is shadowed by a user attribute (not sure if this
  actually exists or is just proposed, TODO investigate)

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
pub fn qux() {}
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


r[names.resolution.early.imports.errors.ambiguity.pathvstextualmacro]
Path-based scope bindings for macros may not shadow textual scope bindings to macros.

```rust,compile_fail
#[macro_export]
macro_rules! m2 {
    () => {}
}
macro_rules! m {
    () => {}
}
pub fn foo() {
    m!(); // ERROR `m` is ambiguous
    use crate::m2 as m; // in scope for entire function body
}
```

r[names.resolution.early.imports.errors.ambiguity.globvsouter]
it is an error to shadow an outer name binding with a glob import.

```rust,compile_fail
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

pub fn qux() {
    use bar::*;
    use foo::Name; // `foo` is ambiguous
}
```

```rust,compile_fail
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

> **NOTE** These ambiguity errors are specific to imports, even though they are
> only observed when those imports are used, having multiple candidates
> available for a given name during later stages of resolution is not
> considered an error, so long as none of the imports themselves are ambiguous,
> there will always be a single unambiguous closest resolution during later
> stages.
>
> ```rust
> mod bar {
>     pub struct Name;
> }
>
> mod baz {
>     pub struct Name;
> }
>
> use baz::Name;
>
> pub fn foo() {
>     use bar::*;
>     Name; // resolves to bar::Name
> }
> ```

r[macro.decl.scope.textual.ambiguity.moreexpandedvsouter]
* it is an error for name bindings from macro expansions to shadow name bindings from outside of those expansions

```rust,compile_fail
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

r[names.resolution.expansion.macros]

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

r[names.resolution.expansion.macros.errors.reserved-names]

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
[permitted]: name-resolution.md#r-names.resolution.expansion.imports.shadowing
[macro invocations]: ../macros.html#macro-invocation
[path-based scope]: ../macros-by-example.html#r-macro.decl.scope.path-based
