{{#include attributes-redirect.html}}
r[attributes]
# Attributes

r[attributes.syntax]
```grammar,attributes
InnerAttribute -> `#` `!` `[` Attr `]`

OuterAttribute -> `#` `[` Attr `]`

Attr ->
      SimplePath AttrInput?
    | `unsafe` `(` SimplePath AttrInput? `)`

AttrInput ->
      DelimTokenTree
    | `=` Expression
```

r[attributes.intro]
An _attribute_ is a general, free-form metadatum that is interpreted according
to name, convention, language, and compiler version. Attributes are modeled
on Attributes in [ECMA-335], with the syntax coming from [ECMA-334] \(C#).

r[attributes.inner]
_Inner attributes_, written with a bang (`!`) after the hash (`#`), apply to the
item that the attribute is declared within. _Outer attributes_, written without
the bang after the hash, apply to the thing that follows the attribute.

r[attributes.input]
The attribute consists of a path to the attribute, followed by an optional
delimited token tree whose interpretation is defined by the attribute.
Attributes other than macro attributes also allow the input to be an equals
sign (`=`) followed by an expression. See the [meta item
syntax](#meta-item-attribute-syntax) below for more details.

r[attributes.safety]
An attribute may be unsafe to apply. To avoid undefined behavior when using
these attributes, certain obligations that cannot be checked by the compiler
must be met.  To assert these have been, the attribute is wrapped in
`unsafe(..)`, e.g. `#[unsafe(no_mangle)]`.

The following attributes are unsafe:

* [`export_name`]
* [`link_section`]
* [`naked`]
* [`no_mangle`]

r[attributes.kind]
Attributes can be classified into the following kinds:

* [Built-in attributes]
* [Proc macro attributes][attribute macros]
* [Derive macro helper attributes]
* [Tool attributes](#tool-attributes)

r[attributes.allowed-position]
Attributes may be applied to many things in the language:

* All [item declarations] accept outer attributes while [external blocks],
  [functions], [implementations], and [modules] accept inner attributes.
* Most [statements] accept outer attributes (see [Expression Attributes] for
  limitations on expression statements).
* [Block expressions] accept outer and inner attributes, but only when they are
  the outer expression of an [expression statement] or the final expression of
  another block expression.
* [Enum] variants and [struct] and [union] fields accept outer attributes.
* [Match expression arms][match expressions] accept outer attributes.
* [Generic lifetime or type parameter][generics] accept outer attributes.
* Expressions accept outer attributes in limited situations, see [Expression
  Attributes] for details.
* [Function][functions], [closure] and [function pointer]
  parameters accept outer attributes. This includes attributes on variadic parameters
  denoted with `...` in function pointers and [external blocks][variadic functions].

Some examples of attributes:

```rust
// General metadata applied to the enclosing module or crate.
#![crate_type = "lib"]

// A function marked as a unit test
#[test]
fn test_foo() {
    /* ... */
}

// A conditionally-compiled module
#[cfg(target_os = "linux")]
mod bar {
    /* ... */
}

// A lint attribute used to suppress a warning/error
#[allow(non_camel_case_types)]
type int8_t = i8;

// Inner attribute applies to the entire function.
fn some_unused_variables() {
  #![allow(unused_variables)]

  let x = ();
  let y = ();
  let z = ();
}
```

r[attributes.meta]
## Meta Item Attribute Syntax

r[attributes.meta.intro]
A "meta item" is the syntax used for the [Attr] rule by most [built-in
attributes]. It has the following grammar:

r[attributes.meta.syntax]
```grammar,attributes
@root MetaItem ->
      SimplePath
    | SimplePath `=` Expression
    | SimplePath `(` MetaSeq? `)`

MetaSeq ->
    MetaItemInner ( `,` MetaItemInner )* `,`?

MetaItemInner ->
      MetaItem
    | Expression
```

r[attributes.meta.literal-expr]
Expressions in meta items must macro-expand to literal expressions, which must not
include integer or float type suffixes. Expressions which are not literal expressions
will be syntactically accepted (and can be passed to proc-macros), but will be rejected after parsing.

r[attributes.meta.order]
Note that if the attribute appears within another macro, it will be expanded
after that outer macro. For example, the following code will expand the
`Serialize` proc-macro first, which must preserve the `include_str!` call in
order for it to be expanded:

```rust ignore
#[derive(Serialize)]
struct Foo {
    #[doc = include_str!("x.md")]
    x: u32
}
```

r[attributes.meta.order-macro]
Additionally, macros in attributes will be expanded only after all other attributes applied to the item:

```rust ignore
#[macro_attr1] // expanded first
#[doc = mac!()] // `mac!` is expanded fourth.
#[macro_attr2] // expanded second
#[derive(MacroDerive1, MacroDerive2)] // expanded third
fn foo() {}
```

r[attributes.meta.builtin]
Various built-in attributes use different subsets of the meta item syntax to
specify their inputs. The following grammar rules show some commonly used
forms:

r[attributes.meta.builtin.syntax]
```grammar,attributes
@root MetaWord ->
    IDENTIFIER

MetaNameValueStr ->
    IDENTIFIER `=` (STRING_LITERAL | RAW_STRING_LITERAL)

@root MetaListPaths ->
    IDENTIFIER `(` ( SimplePath (`,` SimplePath)* `,`? )? `)`

@root MetaListIdents ->
    IDENTIFIER `(` ( IDENTIFIER (`,` IDENTIFIER)* `,`? )? `)`

@root MetaListNameValueStr ->
    IDENTIFIER `(` ( MetaNameValueStr (`,` MetaNameValueStr)* `,`? )? `)`
```

Some examples of meta items are:

Style | Example
------|--------
[MetaWord] | `no_std`
[MetaNameValueStr] | `doc = "example"`
[MetaListPaths] | `allow(unused, clippy::inline_always)`
[MetaListIdents] | `macro_use(foo, bar)`
[MetaListNameValueStr] | `link(name = "CoreFoundation", kind = "framework")`

r[attributes.activity]
## Active and inert attributes

r[attributes.activity.intro]
An attribute is either active or inert. During attribute processing, *active
attributes* remove themselves from the thing they are on while *inert attributes*
stay on.

The [`cfg`] and [`cfg_attr`] attributes are active.
[Attribute macros] are active. All other attributes are inert.

r[attributes.tool]
## Tool attributes

r[attributes.tool.intro]
The compiler may allow attributes for external tools where each tool resides
in its own module in the [tool prelude]. The first segment of the attribute
path is the name of the tool, with one or more additional segments whose
interpretation is up to the tool.

r[attributes.tool.ignored]
When a tool is not in use, the tool's attributes are accepted without a
warning. When the tool is in use, the tool is responsible for processing and
interpretation of its attributes.

r[attributes.tool.prelude]
Tool attributes are not available if the [`no_implicit_prelude`] attribute is
used.

```rust
// Tells the rustfmt tool to not format the following element.
#[rustfmt::skip]
struct S {
}

// Controls the "cyclomatic complexity" threshold for the clippy tool.
#[clippy::cyclomatic_complexity = "100"]
pub fn f() {}
```

> [!NOTE]
> `rustc` currently recognizes the tools "clippy", "rustfmt", "diagnostic", "miri" and "rust_analyzer".

r[attributes.builtin]
## Built-in attributes index

The following is an index of all built-in attributes.

- Conditional compilation
  - [`cfg`] --- Controls conditional compilation.
  - [`cfg_attr`] --- Conditionally includes attributes.

- Testing
  - [`test`] --- Marks a function as a test.
  - [`ignore`] --- Disables a test function.
  - [`should_panic`] --- Indicates a test should generate a panic.

- Derive
  - [`derive`] --- Automatic trait implementations.
  - [`automatically_derived`] --- Marker for implementations created by
    `derive`.

- Macros
  - [`macro_export`] --- Exports a `macro_rules` macro for cross-crate usage.
  - [`macro_use`] --- Expands macro visibility, or imports macros from other
    crates.
  - [`proc_macro`] --- Defines a function-like macro.
  - [`proc_macro_derive`] --- Defines a derive macro.
  - [`proc_macro_attribute`] --- Defines an attribute macro.

- Diagnostics
  - [`allow`], [`expect`], [`warn`], [`deny`], [`forbid`] --- Alters the default lint level.
  - [`deprecated`] --- Generates deprecation notices.
  - [`must_use`] --- Generates a lint for unused values.
  - [`diagnostic::on_unimplemented`] --- Hints the compiler to emit a certain error
    message if a trait is not implemented.
  - [`diagnostic::do_not_recommend`] --- Hints the compiler to not show a certain trait impl in error messages.

- ABI, linking, symbols, and FFI
  - [`link`] --- Specifies a native library to link with an `extern` block.
  - [`link_name`] --- Specifies the name of the symbol for functions or statics
    in an `extern` block.
  - [`link_ordinal`] --- Specifies the ordinal of the symbol for functions or
    statics in an `extern` block.
  - [`no_link`] --- Prevents linking an extern crate.
  - [`repr`] --- Controls type layout.
  - [`crate_type`] --- Specifies the type of crate (library, executable, etc.).
  - [`no_main`] --- Disables emitting the `main` symbol.
  - [`export_name`] --- Specifies the exported symbol name for a function or
    static.
  - [`link_section`] --- Specifies the section of an object file to use for a
    function or static.
  - [`no_mangle`] --- Disables symbol name encoding.
  - [`used`] --- Forces the compiler to keep a static item in the output
    object file.
  - [`crate_name`] --- Specifies the crate name.

- Code generation
  - [`inline`] --- Hint to inline code.
  - [`cold`] --- Hint that a function is unlikely to be called.
  - [`naked`] --- Prevent the compiler from emitting a function prologue and epilogue.
  - [`no_builtins`] --- Disables use of certain built-in functions.
  - [`target_feature`] --- Configure platform-specific code generation.
  - [`track_caller`] --- Pass the parent call location to `std::panic::Location::caller()`.
  - [`instruction_set`] --- Specify the instruction set used to generate a functions code

- Documentation
  - `doc` --- Specifies documentation. See [The Rustdoc Book] for more
    information. [Doc comments] are transformed into `doc` attributes.

- Preludes
  - [`no_std`] --- Removes std from the prelude.
  - [`no_implicit_prelude`] --- Disables prelude lookups within a module.

- Modules
  - [`path`] --- Specifies the filename for a module.

- Limits
  - [`recursion_limit`] --- Sets the maximum recursion limit for certain
    compile-time operations.
  - [`type_length_limit`] --- Sets the maximum size of a polymorphic type.

- Runtime
  - [`panic_handler`] --- Sets the function to handle panics.
  - [`global_allocator`] --- Sets the global memory allocator.
  - [`windows_subsystem`] --- Specifies the windows subsystem to link with.

- Features
  - `feature` --- Used to enable unstable or experimental compiler features. See
    [The Unstable Book] for features implemented in `rustc`.

- Type System
  - [`non_exhaustive`] --- Indicate that a type will have more fields/variants
    added in future.

- Debugger
  - [`debugger_visualizer`] --- Embeds a file that specifies debugger output for a type.
  - [`collapse_debuginfo`] --- Controls how macro invocations are encoded in debuginfo.

[Doc comments]: comments.md#doc-comments
[ECMA-334]: https://www.ecma-international.org/publications-and-standards/standards/ecma-334/
[ECMA-335]: https://www.ecma-international.org/publications-and-standards/standards/ecma-335/
[Expression Attributes]: expressions.md#expression-attributes
[The Rustdoc Book]: ../rustdoc/the-doc-attribute.html
[The Unstable Book]: ../unstable-book/index.html
[`allow`]: attributes/diagnostics.md#lint-check-attributes
[`automatically_derived`]: attributes/derive.md#the-automatically_derived-attribute
[`cfg_attr`]: conditional-compilation.md#the-cfg_attr-attribute
[`cfg`]: conditional-compilation.md#the-cfg-attribute
[`cold`]: attributes/codegen.md#the-cold-attribute
[`collapse_debuginfo`]: attributes/debugger.md#the-collapse_debuginfo-attribute
[`crate_name`]: crates-and-source-files.md#the-crate_name-attribute
[`crate_type`]: linkage.md
[`debugger_visualizer`]: attributes/debugger.md#the-debugger_visualizer-attribute
[`deny`]: attributes/diagnostics.md#lint-check-attributes
[`deprecated`]: attributes/diagnostics.md#the-deprecated-attribute
[`derive`]: attributes/derive.md
[`export_name`]: abi.md#the-export_name-attribute
[`expect`]: attributes/diagnostics.md#lint-check-attributes
[`forbid`]: attributes/diagnostics.md#lint-check-attributes
[`global_allocator`]: runtime.md#the-global_allocator-attribute
[`ignore`]: attributes/testing.md#the-ignore-attribute
[`inline`]: attributes/codegen.md#the-inline-attribute
[`instruction_set`]: attributes/codegen.md#the-instruction_set-attribute
[`link_name`]: items/external-blocks.md#the-link_name-attribute
[`link_ordinal`]: items/external-blocks.md#the-link_ordinal-attribute
[`link_section`]: abi.md#the-link_section-attribute
[`link`]: items/external-blocks.md#the-link-attribute
[`macro_export`]: macros-by-example.md#path-based-scope
[`macro_use`]: macros-by-example.md#the-macro_use-attribute
[`must_use`]: attributes/diagnostics.md#the-must_use-attribute
[`naked`]: attributes/codegen.md#the-naked-attribute
[`no_builtins`]: attributes/codegen.md#the-no_builtins-attribute
[`no_implicit_prelude`]: names/preludes.md#the-no_implicit_prelude-attribute
[`no_link`]: items/extern-crates.md#the-no_link-attribute
[`no_main`]: crates-and-source-files.md#the-no_main-attribute
[`no_mangle`]: abi.md#the-no_mangle-attribute
[`no_std`]: names/preludes.md#the-no_std-attribute
[`non_exhaustive`]: attributes/type_system.md#the-non_exhaustive-attribute
[`panic_handler`]: panic.md#the-panic_handler-attribute
[`path`]: items/modules.md#the-path-attribute
[`proc_macro_attribute`]: procedural-macros.md#attribute-macros
[`proc_macro_derive`]: procedural-macros.md#derive-macros
[`proc_macro`]: procedural-macros.md#function-like-procedural-macros
[`recursion_limit`]: attributes/limits.md#the-recursion_limit-attribute
[`repr`]: type-layout.md#representations
[`should_panic`]: attributes/testing.md#the-should_panic-attribute
[`target_feature`]: attributes/codegen.md#the-target_feature-attribute
[`test`]: attributes/testing.md#the-test-attribute
[`track_caller`]: attributes/codegen.md#the-track_caller-attribute
[`type_length_limit`]: attributes/limits.md#the-type_length_limit-attribute
[`used`]: abi.md#the-used-attribute
[`warn`]: attributes/diagnostics.md#lint-check-attributes
[`windows_subsystem`]: runtime.md#the-windows_subsystem-attribute
[attribute macros]: procedural-macros.md#attribute-macros
[block expressions]: expressions/block-expr.md
[built-in attributes]: #built-in-attributes-index
[derive macro helper attributes]: procedural-macros.md#derive-macro-helper-attributes
[enum]: items/enumerations.md
[expression statement]: statements.md#expression-statements
[external blocks]: items/external-blocks.md
[functions]: items/functions.md
[generics]: items/generics.md
[implementations]: items/implementations.md
[item declarations]: items.md
[match expressions]: expressions/match-expr.md
[modules]: items/modules.md
[statements]: statements.md
[struct]: items/structs.md
[tool prelude]: names/preludes.md#tool-prelude
[union]: items/unions.md
[closure]: expressions/closure-expr.md
[function pointer]: types/function-pointer.md
[variadic functions]: items/external-blocks.html#variadic-functions
[`diagnostic::on_unimplemented`]: attributes/diagnostics.md#the-diagnosticon_unimplemented-attribute
[`diagnostic::do_not_recommend`]: attributes/diagnostics.md#the-diagnosticdo_not_recommend-attribute
