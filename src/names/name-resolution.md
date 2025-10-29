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
```rust
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

```rust
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
r[names.resolution.expansion.imports.errors.ambiguity]

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
