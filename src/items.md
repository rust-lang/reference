r[items]
# Items

r[items.syntax]
```grammar,items
Item ->
    OuterAttribute* ( VisItem | MacroItem )

VisItem ->
    Visibility?
    (
        Module
      | ExternCrate
      | UseDeclaration
      | Function
      | TypeAlias
      | Struct
      | Enumeration
      | Union
      | ConstantItem
      | StaticItem
      | Trait
      | Implementation
      | ExternBlock
    )

MacroItem ->
      MacroInvocationSemi
    | MacroRulesDefinition
```

r[items.intro]
An _item_ is a component of a crate. Items are organized within a crate by a
nested set of [modules]. Every crate has a single "outermost" anonymous module;
all further items within the crate have [paths] within the module tree of the
crate.

r[items.static-def]
Items are entirely determined at compile-time, generally remain fixed during
execution, and may reside in read-only memory.

r[items.kinds]
There are several kinds of items:

* [modules]
* [`extern crate` declarations]
* [`use` declarations]
* [function definitions]
* [type definitions]
* [struct definitions]
* [enumeration definitions]
* [union definitions]
* [constant items]
* [static items]
* [trait definitions]
* [implementations]
* [`extern` blocks]

r[items.locations]
Items may be declared in the [root of the crate], a [module][modules], or a [block expression].

r[items.associated-locations]
A subset of items, called [associated items], may be declared in [traits] and [implementations].

r[items.extern-locations]
A subset of items, called external items, may be declared in [`extern` blocks].

r[items.decl-order]
Items may be defined in any order, with the exception of [`macro_rules`] which has its own scoping behavior.

r[items.name-resolution]
[Name resolution] of item names allows items to be defined before or after where the item is referred to in the module or block.

See [item scopes] for information on the scoping rules of items.

[`extern crate` declarations]: items/extern-crates.md
[`extern` blocks]: items/external-blocks.md
[`macro_rules`]: macros-by-example.md
[`use` declarations]: items/use-declarations.md
[associated items]: items/associated-items.md
[block expression]: expressions/block-expr.md
[constant items]: items/constant-items.md
[enumeration definitions]: items/enumerations.md
[function definitions]: items/functions.md
[implementations]: items/implementations.md
[item scopes]: names/scopes.md#item-scopes
[modules]: items/modules.md
[name resolution]: names/name-resolution.md
[paths]: paths.md
[root of the crate]: crates-and-source-files.md
[statement]: statements.md
[static items]: items/static-items.md
[struct definitions]: items/structs.md
[trait definitions]: items/traits.md
[traits]: items/traits.md
[type definitions]: items/type-aliases.md
[union definitions]: items/unions.md
