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
