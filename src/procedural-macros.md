r[macro.proc]
# Procedural macros

r[macro.proc.intro]
*Procedural macros* allow creating syntax extensions as execution of a function.
Procedural macros come in one of three flavors:

* [Function-like macros] - `custom!(...)`
* [Derive macros] - `#[derive(CustomDerive)]`
* [Attribute macros] - `#[CustomAttribute]`

Procedural macros allow you to run code at compile time that operates over Rust
syntax, both consuming and producing Rust syntax. You can sort of think of
procedural macros as functions from an AST to another AST.

r[macro.proc.def]
Procedural macros must be defined in the root of a crate with the [crate type] of
`proc-macro`.
The macros may not be used from the crate where they are defined, and can only be used when imported in another crate.

> [!NOTE]
> When using Cargo, Procedural macro crates are defined with the `proc-macro` key in your manifest:
>
> ```toml
> [lib]
> proc-macro = true
> ```

r[macro.proc.result]
As functions, they must either return syntax, panic, or loop endlessly. Returned
syntax either replaces or adds the syntax depending on the kind of procedural
macro. Panics are caught by the compiler and are turned into a compiler error.
Endless loops are not caught by the compiler which hangs the compiler.

Procedural macros run during compilation, and thus have the same resources that
the compiler has. For example, standard input, error, and output are the same
that the compiler has access to. Similarly, file access is the same. Because
of this, procedural macros have the same security concerns that [Cargo's
build scripts] have.

r[macro.proc.error]
Procedural macros have two ways of reporting errors. The first is to panic. The
second is to emit a [`compile_error`] macro invocation.

r[macro.proc.proc_macro]
## The `proc_macro` crate

r[macro.proc.proc_macro.intro]
Procedural macro crates almost always will link to the compiler-provided
[`proc_macro` crate]. The `proc_macro` crate provides types required for
writing procedural macros and facilities to make it easier.

r[macro.proc.proc_macro.token-stream]
This crate primarily contains a [`TokenStream`] type. Procedural macros operate
over *token streams* instead of AST nodes, which is a far more stable interface
over time for both the compiler and for procedural macros to target. A
*token stream* is roughly equivalent to `Vec<TokenTree>` where a `TokenTree`
can roughly be thought of as lexical token. For example `foo` is an `Ident`
token, `.` is a `Punct` token, and `1.2` is a `Literal` token. The `TokenStream`
type, unlike `Vec<TokenTree>`, is cheap to clone.

r[macro.proc.proc_macro.span]
All tokens have an associated `Span`. A `Span` is an opaque value that cannot
be modified but can be manufactured. `Span`s represent an extent of source
code within a program and are primarily used for error reporting. While you
cannot modify a `Span` itself, you can always change the `Span` *associated*
with any token, such as through getting a `Span` from another token.

r[macro.proc.hygiene]
## Procedural macro hygiene

Procedural macros are *unhygienic*. This means they behave as if the output
token stream was simply written inline to the code it's next to. This means that
it's affected by external items and also affects external imports.

Macro authors need to be careful to ensure their macros work in as many contexts
as possible given this limitation. This often includes using absolute paths to
items in libraries (for example, `::std::option::Option` instead of `Option`) or
by ensuring that generated functions have names that are unlikely to clash with
other functions (like `__internal_foo` instead of `foo`).

r[macro.proc.function]
## Function-like procedural macros

r[macro.proc.function.intro]
*Function-like procedural macros* are procedural macros that are invoked using
the macro invocation operator (`!`).

r[macro.proc.function.def]
These macros are defined by a [public]&#32;[function] with the `proc_macro`
[attribute] and a signature of `(TokenStream) -> TokenStream`. The input
[`TokenStream`] is what is inside the delimiters of the macro invocation and the
output [`TokenStream`] replaces the entire macro invocation.

r[macro.proc.function.namespace]
The `proc_macro` attribute defines the macro in the [macro namespace] in the root of the crate.

For example, the following macro definition ignores its input and outputs a
function `answer` into its scope.

<!-- ignore: test doesn't support proc-macro -->
```rust,ignore
# #![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
```

And then we use it in a binary crate to print "42" to standard output.

<!-- ignore: requires external crates -->
```rust,ignore
extern crate proc_macro_examples;
use proc_macro_examples::make_answer;

make_answer!();

fn main() {
    println!("{}", answer());
}
```

r[macro.proc.function.invocation]
Function-like procedural macros may be invoked in any macro invocation
position, which includes [statements], [expressions], [patterns], [type
expressions], [item] positions, including items in [`extern` blocks], inherent
and trait [implementations], and [trait definitions].

<!-- template:attributes -->
r[macro.proc.derive]
## The `proc_macro_derive` attribute

r[macro.proc.derive.intro]
Applying the *`proc_macro_derive` [attribute]* to a function defines a *derive macro* that can be invoked by the [`derive` attribute]. These macros are given the token stream of a [struct], [enum], or [union] definition and can emit new [items] after it. They can also declare and use [derive macro helper attributes].

> [!EXAMPLE]
> This derive macro ignores its input and appends tokens that define a function.
>
> <!-- ignore: test doesn't support proc-macro -->
> ```rust,ignore
> # #![crate_type = "proc-macro"]
> extern crate proc_macro;
> use proc_macro::TokenStream;
>
> #[proc_macro_derive(AnswerFn)]
> pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
>     "fn answer() -> u32 { 42 }".parse().unwrap()
> }
> ```
>
> To use it, we might write:
>
> <!-- ignore: requires external crates -->
> ```rust,ignore
> extern crate proc_macro_examples;
> use proc_macro_examples::AnswerFn;
>
> #[derive(AnswerFn)]
> struct Struct;
>
> fn main() {
>     assert_eq!(42, answer());
> }
> ```

r[macro.proc.derive.syntax]
The syntax for the `proc_macro_derive` attribute is:

```grammar,attributes
@root ProcMacroDeriveAttribute ->
    `proc_macro_derive` `(` DeriveMacroName ( `,` DeriveMacroAttributes )? `,`? `)`

DeriveMacroName -> IDENTIFIER

DeriveMacroAttributes ->
    `attributes` `(` ( IDENTIFIER (`,` IDENTIFIER)* `,`?)? `)`
```

The name of the derive macro is given by [DeriveMacroName]. The optional `attributes` argument is described in [macro.proc.derive.attributes].

r[macro.proc.derive.allowed-positions]
The `proc_macro_derive` attribute may only be applied to a `pub` function with the [Rust ABI][items.fn.extern] defined in the root of the crate with a type of `fn(TokenStream) -> TokenStream`  where [`TokenStream`] comes from the [`proc_macro` crate]. The function may be `const` and may use `extern` to explicitly specify the Rust ABI, but it may not use any other [qualifiers][FunctionQualifiers] (e.g. it may not be `async` or `unsafe`).

r[macro.proc.derive.duplicates]
The `proc_macro_derive` attribute may be used only once on a function.

r[macro.proc.derive.namespace]
The `proc_macro_derive` attribute publicly defines the derive macro in the [macro namespace] in the root of the crate.

r[macro.proc.derive.output]
The input [`TokenStream`] is the token stream of the item to which the `derive` attribute is applied. The output [`TokenStream`] must be a (possibly empty) set of items. These items are appended following the input item within the same [module] or [block].

r[macro.proc.derive.attributes]
### Derive macro helper attributes

r[macro.proc.derive.attributes.intro]
Derive macros can declare *derive macro helper attributes* to be used within the scope of the [item] to which the derive macro is applied. These [attributes] are [inert]. While their purpose is to be used by the macro that declared them, they can be seen by any macro.

r[macro.proc.derive.attributes.decl]
A helper attribute for a derive macro is declared by adding its identifier to the `attributes` list in the `proc_macro_derive` attribute.

> [!EXAMPLE]
> This declares a helper attribute and then ignores it.
>
> <!-- ignore: test doesn't support proc-macro -->
> ```rust,ignore
> # #![crate_type="proc-macro"]
> # extern crate proc_macro;
> # use proc_macro::TokenStream;
> #
> #[proc_macro_derive(WithHelperAttr, attributes(helper))]
> pub fn derive_with_helper_attr(_item: TokenStream) -> TokenStream {
>     TokenStream::new()
> }
> ```
>
> To use it, we might write:
>
> <!-- ignore: requires external crates -->
> ```rust,ignore
> #[derive(WithHelperAttr)]
> struct Struct {
>     #[helper] field: (),
> }
> ```

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

r[macro.proc.attribute]
## Attribute macros

r[macro.proc.attribute.intro]
*Attribute macros* define new [outer attributes][attributes] which can be
attached to [items], including items in [`extern` blocks], inherent and trait
[implementations], and [trait definitions].

r[macro.proc.attribute.def]
Attribute macros are defined by a [public]&#32;[function] with the
`proc_macro_attribute` [attribute] that has a signature of `(TokenStream,
TokenStream) -> TokenStream`. The first [`TokenStream`] is the delimited token
tree following the attribute's name, not including the outer delimiters. If
the attribute is written as a bare attribute name, the attribute
[`TokenStream`] is empty. The second [`TokenStream`] is the rest of the [item]
including other [attributes] on the [item]. The returned [`TokenStream`]
replaces the [item] with an arbitrary number of [items].

r[macro.proc.attribute.namespace]
The `proc_macro_attribute` attribute defines the attribute in the [macro namespace] in the root of the crate.

For example, this attribute macro takes the input stream and returns it as is,
effectively being the no-op of attributes.

<!-- ignore: test doesn't support proc-macro -->
```rust,ignore
# #![crate_type = "proc-macro"]
# extern crate proc_macro;
# use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn return_as_is(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
```

This following example shows the stringified [`TokenStream`s] that the attribute
macros see. The output will show in the output of the compiler. The output is
shown in the comments after the function prefixed with "out:".

<!-- ignore: test doesn't support proc-macro -->
```rust,ignore
// my-macro/src/lib.rs
# extern crate proc_macro;
# use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    item
}
```

<!-- ignore: requires external crates -->
```rust,ignore
// src/lib.rs
extern crate my_macro;

use my_macro::show_streams;

// Example: Basic function
#[show_streams]
fn invoke1() {}
// out: attr: ""
// out: item: "fn invoke1() {}"

// Example: Attribute with input
#[show_streams(bar)]
fn invoke2() {}
// out: attr: "bar"
// out: item: "fn invoke2() {}"

// Example: Multiple tokens in the input
#[show_streams(multiple => tokens)]
fn invoke3() {}
// out: attr: "multiple => tokens"
// out: item: "fn invoke3() {}"

// Example:
#[show_streams { delimiters }]
fn invoke4() {}
// out: attr: "delimiters"
// out: item: "fn invoke4() {}"
```

r[macro.proc.token]
## Declarative macro tokens and procedural macro tokens

r[macro.proc.token.intro]
Declarative `macro_rules` macros and procedural macros use similar, but
different definitions for tokens (or rather [`TokenTree`s].)

r[macro.proc.token.macro_rules]
Token trees in `macro_rules` (corresponding to `tt` matchers) are defined as
- Delimited groups (`(...)`, `{...}`, etc)
- All operators supported by the language, both single-character and
  multi-character ones (`+`, `+=`).
    - Note that this set doesn't include the single quote `'`.
- Literals (`"string"`, `1`, etc)
    - Note that negation (e.g. `-1`) is never a part of such literal tokens,
      but a separate operator token.
- Identifiers, including keywords (`ident`, `r#ident`, `fn`)
- Lifetimes (`'ident`)
- Metavariable substitutions in `macro_rules` (e.g. `$my_expr` in
  `macro_rules! mac { ($my_expr: expr) => { $my_expr } }` after the `mac`'s
  expansion, which will be considered a single token tree regardless of the
  passed expression)

r[macro.proc.token.tree]
Token trees in procedural macros are defined as
- Delimited groups (`(...)`, `{...}`, etc)
- All punctuation characters used in operators supported by the language (`+`,
  but not `+=`), and also the single quote `'` character (typically used in
  lifetimes, see below for lifetime splitting and joining behavior)
- Literals (`"string"`, `1`, etc)
    - Negation (e.g. `-1`) is supported as a part of integer
      and floating point literals.
- Identifiers, including keywords (`ident`, `r#ident`, `fn`)

r[macro.proc.token.conversion.intro]
Mismatches between these two definitions are accounted for when token streams
are passed to and from procedural macros. \
Note that the conversions below may happen lazily, so they might not happen if
the tokens are not actually inspected.

r[macro.proc.token.conversion.to-proc_macro]
When passed to a proc-macro
- All multi-character operators are broken into single characters.
- Lifetimes are broken into a `'` character and an identifier.
- The keyword metavariable [`$crate`] is passed as a single identifier.
- All other metavariable substitutions are represented as their underlying
  token streams.
    - Such token streams may be wrapped into delimited groups ([`Group`]) with
      implicit delimiters ([`Delimiter::None`]) when it's necessary for
      preserving parsing priorities.
    - `tt` and `ident` substitutions are never wrapped into such groups and
      always represented as their underlying token trees.

r[macro.proc.token.conversion.from-proc_macro]
When emitted from a proc macro
- Punctuation characters are glued into multi-character operators
  when applicable.
- Single quotes `'` joined with identifiers are glued into lifetimes.
- Negative literals are converted into two tokens (the `-` and the literal)
  possibly wrapped into a delimited group ([`Group`]) with implicit delimiters
  ([`Delimiter::None`]) when it's necessary for preserving parsing priorities.

r[macro.proc.token.doc-comment]
Note that neither declarative nor procedural macros support doc comment tokens
(e.g. `/// Doc`), so they are always converted to token streams representing
their equivalent `#[doc = r"str"]` attributes when passed to macros.

[Attribute macros]: #attribute-macros
[Cargo's build scripts]: ../cargo/reference/build-scripts.html
[Derive macros]: macro.proc.derive
[Function-like macros]: #function-like-procedural-macros
[`$crate`]: macro.decl.hygiene.crate
[`Delimiter::None`]: proc_macro::Delimiter::None
[`Group`]: proc_macro::Group
[`TokenStream`]: proc_macro::TokenStream
[`TokenStream`s]: proc_macro::TokenStream
[`TokenTree`s]: proc_macro::TokenTree
[`derive` attribute]: attributes/derive.md
[`extern` blocks]: items/external-blocks.md
[`macro_rules`]: macros-by-example.md
[`proc_macro` crate]: proc_macro
[attribute]: attributes.md
[attributes]: attributes.md
[block]: expressions/block-expr.md
[crate type]: linkage.md
[derive macro helper attributes]: #derive-macro-helper-attributes
[enum]: items/enumerations.md
[expressions]: expressions.md
[function]: items/functions.md
[implementations]: items/implementations.md
[inert]: attributes.md#active-and-inert-attributes
[item]: items.md
[items]: items.md
[macro namespace]: names/namespaces.md
[module]: items/modules.md
[patterns]: patterns.md
[public]: visibility-and-privacy.md
[statements]: statements.md
[struct]: items/structs.md
[trait definitions]: items/traits.md
[type expressions]: types.md#type-expressions
[type]: types.md
[union]: items/unions.md
