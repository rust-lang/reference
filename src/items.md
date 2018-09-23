# Items

> **<sup>Syntax:<sup>**\
> _Item_:\
> &nbsp;&nbsp; [_OuterAttribute_]<sup>\*</sup> [_Visibility_]<sup>?</sup>\
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
> &nbsp;&nbsp; &nbsp;&nbsp; | _Macro_\
> &nbsp;&nbsp; &nbsp;&nbsp; | _MacroDefinition_\
> &nbsp;&nbsp; )

[_OuterAttribute_]: attributes.html
[_Visibility_]: visibility-and-privacy.html
[_Module_]: items/modules.html
[_ExternCrate_]: items/extern-crates.html
[_UseDeclaration_]: items/use-declarations.html
[_Function_]: items/functions.html
[_TypeAlias_]: items/type-aliases.html
[_Struct_]: items/structs.html
[_Enumeration_]: items/enumerations.html
[_Union_]: items/unions.html
[_ConstantItem_]: items/constant-items.html
[_StaticItem_]: items/static-items.html
[_Trait_]: items/traits.html
[_Implementation_]: items/implementations.html
[_ExternBlock_]: items/extern-blocks.html

An _item_ is a component of a crate. Items are organized within a crate by a
nested set of [modules]. Every crate has a single "outermost" anonymous module;
all further items within the crate have [paths] within the module tree of the
crate.

[modules]: items/modules.html
[paths]: paths.html

Items are entirely determined at compile-time, generally remain fixed during
execution, and may reside in read-only memory.

There are several kinds of items:

* [modules](items/modules.html)
* [`extern crate` declarations](items/extern-crates.html)
* [`use` declarations](items/use-declarations.html)
* [function definitions](items/functions.html)
* [type definitions](items/type-aliases.html)
* [struct definitions](items/structs.html)
* [enumeration definitions](items/enumerations.html)
* [union definitions](items/unions.html)
* [constant items](items/constant-items.html)
* [static items](items/static-items.html)
* [trait definitions](items/traits.html)
* [implementations](items/implementations.html)
* [`extern` blocks](items/external-blocks.html)

Some items form an implicit scope for the declaration of sub-items. In other
words, within a function or module, declarations of items can (in many cases)
be mixed with the statements, control blocks, and similar artifacts that
otherwise compose the item body. The meaning of these scoped items is the same
as if the item was declared outside the scope &mdash; it is still a static item
&mdash; except that the item's *path name* within the module namespace is
qualified by the name of the enclosing item, or is private to the enclosing
item (in the case of functions). The grammar specifies the exact locations in
which sub-item declarations may appear.
