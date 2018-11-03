# Items

> **<sup>Syntax:<sup>**\
> _Item_:\
> &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup>\
> &nbsp;&nbsp; &nbsp;&nbsp; _VisItem_\
> &nbsp;&nbsp; | _MacroItem_
>
> _VisItem_:\
> &nbsp;&nbsp; [_Visibility_]<sup>?</sup>\
> &nbsp;&nbsp; (\
> &nbsp;&nbsp; &nbsp;&nbsp; &nbsp;&nbsp;  [_Module_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_ExternCrate_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_UseDeclaration_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_Function_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_TypeAlias_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_Struct_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_Enumeration_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_Union_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_ConstantItem_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_StaticItem_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_Trait_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_Implementation_]\
> &nbsp;&nbsp; &nbsp;&nbsp; | [_ExternBlock_]\
> &nbsp;&nbsp; )
>
> _MacroItem_:\
> &nbsp;&nbsp; &nbsp;&nbsp; [_MacroInvocationSemi_]\
> &nbsp;&nbsp; | [_MacroRulesDefinition_]


An _item_ is a component of a crate. Items are organized within a crate by a
nested set of [modules]. Every crate has a single "outermost" anonymous module;
all further items within the crate have [paths] within the module tree of the
crate.

Items are entirely determined at compile-time, generally remain fixed during
execution, and may reside in read-only memory.

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

Some items form an implicit scope for the declaration of sub-items. In other
words, within a function or module, declarations of items can (in many cases)
be mixed with the statements, control blocks, and similar artifacts that
otherwise compose the item body. The meaning of these scoped items is the same
as if the item was declared outside the scope &mdash; it is still a static item
&mdash; except that the item's *path name* within the module namespace is
qualified by the name of the enclosing item, or is private to the enclosing
item (in the case of functions). The grammar specifies the exact locations in
which sub-item declarations may appear.

[_ConstantItem_]: items/constant-items.html
[_Enumeration_]: items/enumerations.html
[_ExternBlock_]: items/external-blocks.html
[_ExternCrate_]: items/extern-crates.html
[_Function_]: items/functions.html
[_Implementation_]: items/implementations.html
[_MacroInvocationSemi_]: macros.html#macro-invocation
[_MacroRulesDefinition_]: macros-by-example.html
[_Module_]: items/modules.html
[_OuterAttribute_]: attributes.html
[_StaticItem_]: items/static-items.html
[_Struct_]: items/structs.html
[_Trait_]: items/traits.html
[_TypeAlias_]: items/type-aliases.html
[_Union_]: items/unions.html
[_UseDeclaration_]: items/use-declarations.html
[_Visibility_]: visibility-and-privacy.html
[`extern crate` declarations]: items/extern-crates.html
[`extern` blocks]: items/external-blocks.html
[`use` declarations]: items/use-declarations.html
[constant items]: items/constant-items.html
[enumeration definitions]: items/enumerations.html
[function definitions]: items/functions.html
[implementations]: items/implementations.html
[modules]: items/modules.html
[paths]: paths.html
[static items]: items/static-items.html
[struct definitions]: items/structs.html
[trait definitions]: items/traits.html
[type definitions]: items/type-aliases.html
[union definitions]: items/unions.html
