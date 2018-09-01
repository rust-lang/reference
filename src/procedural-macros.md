## Procedural Macros

*Procedural macros* allow creating syntax extensions as execution of a function.
Procedural macros come in one of three flavors:

* Bang macros - `custom_bang!(...)`
* Derive mode macros - `#[derive(CustomMode)]`
* Attribute macros - `#[CustomAttribute]`

Procedural macros allow you to run code at compile time that operates over Rust
syntax, both consuming and producing Rust syntax. You can sort of think of
procedural macros as functions from an AST to another AST.

Procedural macros must be defined in a crate with the [crate type] of
`proc-macro`.

> **Note**: When using Cargo, Procedural macro crates are defined with the
> `proc-macro` key in your manfiest:
>
> ```toml
> [lib]
> proc-macro = true
> ```

As functions, they must either return syntax, panic, or loop endlessly. Returned
syntax either replaces or adds the syntax depending on the kind of procedural
macro. Panics are caught by the compiler and are turned into a compiler error.
Endless loops are not caught by the compiler which hangs the compiler.

Procedural macros run during compilation, and thus have the same resources that
the compiler has. For example, standard input, error, and output are the same
that the compiler has access to. Similarly, file access is the same. Because
of this, procedural macros have the same security concerns that [Cargo's
build scripts] have.

Procedural macros have two ways of reporting errors. The first is to panic. The
second is to emit a [`compile_error`] macro invocation.

### The `proc_macro` crate

Procedural macro crates almost always will link to the compiler-provided 
[`proc_macro` crate]. The `proc_macro` crate provides types required for
writing procedural macros and facilities to make it easier.

This crate primarily contains a [`TokenStream`] type. Procedural macros operate
over *token streams* instead of AST nodes, which is a far more stable interface
over time for both the compiler and for procedural macros to target. A
*token stream* is roughly equivalent to `Vec<TokenTree>` where a `TokenTree`
can roughly be thought of as lexical token. For example `foo` is an `Ident`
token, `.` is a `Punct` token, and `1.2` is a `Literal` token. The `TokenStream`
type, unlike `Vec<TokenTree>`, is cheap to clone.

All tokens have an associated `Span`. A `Span` is an opaque value that cannot
be modified but can be manufactured. `Span`s represent an extent of source
code within a program and are primarily used for error reporting. You can modify
the `Span` of any token.

### Procedural macros and hygiene

Procedural macros are *unhygienic*. This means they behave as if the output
token stream was simply written inline to the code it's next to. This means that
it's affected by external items and also affects external imports.

Macro authors need to be careful to ensure their macros work in as many contexts
as possible given this limitation. This often includes using absolute paths to
items in libraries (for example, `::std::option::Option` instead of `Option`) or
by ensuring that generated functions have names that are unlikely to clash with
other functions (like `__internal_foo` instead of `foo`).

### Limitations of procedural macros

Procedural macros are not quite as powerful as `macro_rules!`-defined macros
in certain respects. These limitations include:

* Bang macros can only be invoked in *item* contexts. For example,
  `format!` cannot yet be created in user libraries because it is only ever
  invoked in an expression context. Put another way, these macros can only
  expand to [items], not expressions.

* Procedural macros cannot expand to definitions of `macro_rules!` macros, with
  exception to derive mode macros.

* Procedural attributes can only be attached to items, not expressions. For
  example `#[my_attr] fn foo() {}` is ok but `#[my_attr] return 3` is not. This
  is again due to the lack of hygiene today but this restriction may eventually
  be lifted.

* Error reporting is currently quite primitive. While an unstable diagnostic API
  exists on stable your only option is to `panic!` or to in some cases expand to
  an invocation of the `compile_error!` macro with a custom message.

### Bang Macros

This flavor of procedural macro is like writing `macro_rules!` only you get to
execute arbitrary code over the input tokens instead of being limited to
`macro_rules!` syntax.

Procedural bang macros are defined with the `#[proc_macro]` attribute and have
the following signature:

```rust,ignore
#[proc_macro]
pub fn foo(input: TokenStream) -> TokenStream {
    // ...
}
```

This item is defining a procedural bang macro (`#[proc_macro]`) which is called
`foo`. The first argument is the input to the macro which explore in a second,
and the return value is the tokens that it should expand to. 

```rust,ignore
// my-macro/src/lib.rs
extern crate proc_macro;

use proc_macro::*;

#[proc_macro]
pub fn foo(input: TokenStream) -> TokenStream {
    input
}
```

And now let's invoke it:

```rust,ignore
// src/main.rs
extern crate my_macro;

my_macro::foo!(fn answer() -> u32 { 3 });

fn main() {
    println!("the answer was: {}", answer());
}
```

First up, let's see what the input to our macro looks like by modifying our
macro:

```rust,ignore
// my-macro/src/lib.rs
#[proc_macro]
pub fn foo(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    input
}
```

The macro invocation is effectively replaced by
the return value of the macro, creating the function that we provided as input.
We can see another example of this where we simply ignore the input:

```rust,ignore
// my-macro/src/lib.rs
#[proc_macro]
pub fn foo(_input: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 4 }".parse().unwrap()
}
```

```
the answer was: 4
```

### Derive mode macros

*Derive mode macros* define new modes for the `derive` attribute. The macros
define new items given the token stream of a [struct], [enum], or [union]. They
also define derive mode helper attributes.

Custom derivers are defined by a [public] [function] with the `proc_maco_derive`
attribute and a signature of `(TokenStream) -> TokenStream`.

The input [`TokenStream`] is the token stream of the item that has the `derive`
attribute on it. The output [`TokenStream`] must be a set of items that are
then appended to the [module] or [block] that the item from the input
[`TokenStream`] is in.

The following is an example of a derive mode macro. Instead of doing anything
useful with its input, it just appends a function `answer`.

```rust,ignore
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(AnswerFn)]
pub fn foo(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
```

And then using said derive mode:

```rust,ignore
extern crate proc_macro_examples;
use proc_macro_examples::AnswerFn;

#[derive(AnswerFn)]
struct Struct;

fn main() {
    assert_eq!(42, answer());
}
```

#### Derive mode helper attributes

Derive mode macros can add additional [attributes] into the scope of the item
they are on. Said attributes are called *derive mode helper attributes*. These
attributes are inert, and their only purpose is to be fed into the derive
mode macro that defined them. That said, they can be seen by all macros.

The way to define helper attributes is to put an `attributes` key in the
`proc_macro_derive` macro with a comma separated list of identifiers that are
the names of the helper attributes.

For example, the following derive mode macro defines a helper attribute
`helper`, but ultimately doesn't do anything with it.

```rust, ignore
# extern crate proc_macro;
# use proc_macro::TokenStream;

#[proc_macro_derive(HelperAttr, attributes(helper))]
pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new();
}
```

And then usage on the derive mode on a struct:

```
# extern crate proc_macro_examples;
# use proc_macro_examples::HelperAttr;

#[derive(HelperAttr)]
struct Struct {
    #[helper] field: ()
}
```

### Attribute macros

*Attribute macros* define new [attributes] which can be attached to [items].

Attribute macros are defined by a [public] [function] with the
`proc_macro_attribute` attribute that a signature of `(TokenStream, TokenStream)
-> TokenStream`. The first [`TokenStream`] is the attribute's metaitems, not
including the delimiters. If the attribute is written without a metaitem, the
attribute [`TokenStream`] is empty. The second [`TokenStream`] is of the rest of
the item including other attributes on the item. The returned [`TokenStream`]
replaces the item. It may contain an arbitrary number of items. The returned
[`TokenStream`] cannot include any [macro] definitions.

For example, this attribute macro takes the input stream and returns it as is,
effectively being the no-op of attributes.

```rust
#![crate_type = "proc_macro"]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn return_as_is(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
```

This following example shows the stringified [`TokenStream`s] that the attribute
macros see. The output will show in the output of the compiler. The output is
shown in the comments after the function prefixed with "out:".

```rust,ignore
// my-macro/src/lib.rs
# extern crate proc_macro;
# use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, input: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"', input.to_string());
    item
}
```

```rust,ignore
// src/lib.rs
extern crate my_macro;

use my_macro::show_streams;

// Example: Basic function
#[show_streams]
fn invoke1() {}
// out: attr: ""
// out: item: "fn invoke1() { }"

// Example: Attribute has a metaitem
#[show_streams(bar)]
fn invoke2() {}
// out: attr: "bar"
// out: item: "fn invoke2() {}"

// Example: Multiple words in metaitem
#[show_streams(multiple words)]
fn invoke3() {}
// out: attr: "multiple words"
// out: item: "fn invoke3() {}"

// Example: 
#[show_streams { delimiters }]
fn invoke4() {}
// out: "delimiters"
// out: "fn invoke4() {}"
```

[`TokenStream`]: ../proc_macro/struct.TokenStream.html
[`TokenStream`s]: ../proc_macro/struct.TokenStream.html
[`compile_error`]: ../std/macro.compile_error.html
[`derive`]: attributes.html#derive
[`proc_macro` crate]: ../proc_macro/index.html
[Cargo's build scripts]: ../cargo/reference/build-scripts.html
[attributes]: attributes.html
[custom attributes]: attributes.html
[crate type]: linkage.html
[item]: items.html
[function]: items/functions.html
[macro]: macros.html
[module]: items/modules.html
[procedural macro tutorial]: ../book/2018-edition/appendix-04-macros.html#procedural-macros-for-custom-derive
[public]: visibility-and-privacy.html