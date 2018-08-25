## Procedural Macros

*Procedural macros* allow creating syntax extensions as execution of a function.
Procedural macros currently come in one of three flavors:

* Bang macros - `my_macro!(...)`
* Derive macros - `#[derive(MyTrait)]`
* Attribute macros - `#[my_attribute]`

Procedural macros allow you to run code at compile time that operates over Rust
syntax, both consuming and producing Rust syntax. You can sort of think of
procedural macros as functions from an AST to another AST.

### Crates and procedural macros

Procedural macros must be defined in a crate with the [crate type] of
`proc-macro`.

> **Note**: When using Cargo, Procedural macro crates are defined with the
> `proc-macro` key in your manfiest:
>
> ```toml
> [lib]
> proc-macro = true
> ```

Procedural macros are always compiled with the same target as the compiler
itself. For example if you execute `cargo build --target
arm-unknown-linux-gnueabi` then procedural macros will not be compiled for ARM,
but rather your build computer (for example `x86_64-unknown-linux-gnu`).

Procedural macro crates are not currently allowed to export any items except
procedural macros (we'll see how to define these in a bit). For example this
crate is not allowed:

```rust
pub fn foo() {}
```

because the `foo` function is not a procedural macro. Procedural macros are
loaded dynamically by the compiler when they are needed during compilation.

### The `proc_macro` crate

Procedural macro crates almost always will link to the compiler-provided 
`proc_macro` crate. The `proc_macro` crate is a crate which provides
facilities to working with the types of each procedural macro function. You can
learn more about this crate by exploring [its documentation][proc macro crate].

### The `TokenStream` Type

It primarily contains a `TokenStream` type.
Procedural macros operate over *token streams* instead of AST nodes,
which is a far more stable interface over time for both the compiler and for
procedural macros to target.

A *token stream* is roughly equivalent to `Vec<TokenTree>` where a `TokenTree`
can roughly be thought of as lexical token. For example `foo` is an `Ident`
token, `.` is a `Punct` token, and `1.2` is a `Literal` token. The `TokenStream`
type, unlike `Vec<TokenTree>`, is cheap to clone (like `Rc<T>`).

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

### Derive macros

The derive macro feature allows you to define a new `#[derive(Foo)]` mode which
often makes it much easier to systematically implement traits, removing quite a
lot of boilerplate.

Custom derives are defined like so:

```rust,ignore
#[proc_macro_derive(MyTrait)]
pub fn foo(item: TokenStream) -> TokenStream {
    // ...
}
```

Here the argument to the `proc_macro_derive` attribute, `MyTrait`, is the name
of the identifier to pass to `#[derive]`. The name of the function here, `foo`,
is not currently used (but it may one day be used).

Like procedural bang macros the input to the macro here is the item that the
attribute was applied to. Unlike bang macros, however, the output is *appended*
to the program rather than replacing the item its attached to. We can see this
behavior by defining a macro like:

```rust,ignore
extern crate proc_macro;
use proc_macro::*;

#[proc_macro_derive(MyTrait)]
pub fn foo(item: TokenStream) -> TokenStream {
    println!("{:#?}", item);
    "fn answer() -> u32 { 2 }".parse().unwrap()
}
```

using it liek:

```rust,ignore
extern crate my_macro;

use my_macro::MyTrait;

#[derive(MyTrait)]
struct Foo;

fn main() {
    drop(Foo);
    println!("the answer was: {}", answer());
}
```

and compiling it:

```
the answer was: 2
```

Here we can see how the input to the macro was a `TokenStream` representing
the three input tokens `struct Foo;`. While our output only contained the
`answer` function, we were still able to use `Foo` in the main program because
derive macros *append* items, they don't replace them.

Now this is a pretty wonky macro derive, and would likely be confusing to
users! Derive macros are primarily geared towards implementing traits, like
`Serialize` and `Deserialize`.

#### Derive helper attributes

An additional feature of derive macros is that they can whitelist names
of attributes which are considered "helper attributes" and don't participate in
normal attribute macro expansion.  Taking our example from earlier we can
define:

```rust,ignore
#[proc_macro_derive(MyTrait, attributes(my_attribute))]
pub fn foo(item: TokenStream) -> TokenStream {
    // ...
}
```

The extra `attributes` key in the `proc_macro_derive` attribute contains a
comma-separated list of identifiers. Each identifier is a whitelist of
an attribute name that can be attached to items which also have
`#[derive(MyTrait)]`. These derive helper attributes will not do anything but
will be passed through to the `foo` procedural macro defined above as part of
the input.

If we change our invocation to look like:

```rust,ignore
#[derive(MyTrait)]
#[my_attribute(hello)]
struct Foo;
```

you'll see that the `#[my_attribute(hello)]` attribute is fed through to the
macro for processing.

Attributes are often used to customize the behavior of derive macros.

### Attribute macros

Attribute macros allow you to define a new `#[attr]`-style attribute which can
be attached to items and generate wrappers and such. These macros are defined
like:

```rust,ignore
#[proc_macro_attribute]
pub fn foo(attr: TokenStream, input: TokenStream) -> TokenStream {
    // ...
}
```

The `#[proc_macro_attribute]` indicates that this macro is an attribute macro
and can only be invoked like `#[foo]`. The name of the function here will be the
name of the attribute as well.

The first input, `attr`, is the arguments to the attribute provided. The second
argument, `item`, is the item that the attribute is attached to.

Like with bang macros at the beginning (and unlike derive macros), the return
value here *replaces* the input `item`.

Let's see this attribute in action:

```rust,ignore
// my-macro/src/lib.rs
extern crate proc_macro;
use proc_macro::*;

#[proc_macro_attribute]
pub fn foo(attr: TokenStream, input: TokenStream) -> TokenStream {
    println!("attr: {}", attr.to_string());
    println!("input: {}", input.to_string());
    item
}
```

and invoke it as:

```rust,ignore
// src/main.rs
extern crate my_macro;

use my_macro::foo;

#[foo]
fn invoke1() {}

#[foo(bar)]
fn invoke2() {}

#[foo(crazy custom syntax)]
fn invoke3() {}

#[foo { delimiters }]
fn invoke4() {}

fn main() {
    // ...
}
```

compiled as:

```
attr:
input: fn invoke1() { }
attr: bar
input: fn invoke2() { }
attr: crazy custom syntax
input: fn invoke3() { }
attr: delimiters
input: fn invoke4() { }
```

Here we can see how the arguments to the attribute show up in the `attr`
argument. Notably these arguments do not include the delimiters used to enclose
the arguments (like procedural bang macros. Furthermore we can see the item
continue to operate on it, either replacing it or augmenting it.

### Spans

All tokens have an associated `Span`. A `Span` is currently an opaque value you
cannot modify (but you can manufacture). `Span`s represent an extent of source
code within a program and are primarily used for error reporting currently. You
can modify the `Span` of any token, and by doing so if the compiler would like
to print an error for the token in question it'll use the `Span` that you
manufactured.

For example let's create a wonky procedural macro which swaps the spans of the
first two tokens:

```rust,ignore
// my-macro/src/lib.rs
extern crate proc_macro;

use proc_macro::*;

#[proc_macro]
pub fn swap_spans(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut a = iter.next().unwrap();
    let mut b = iter.next().unwrap();
    let a_span = a.span();
    a.set_span(b.span());
    b.set_span(a_span);
    return vec![a, b].into_iter().chain(iter).collect()
}
```

```rust,ignore
extern crate my_macro;

my_macro::swap_spans! {
    fn _() {}
}

fn main() {}
```

and compile it:

```sh
$ cargo run
   Compiling foo v0.1.0 (file://.../foo)
error: expected identifier, found reserved identifier `_`
 --> src/main.rs:4:5
  |
4 |     fn _() {}
  |     ^^ expected identifier, found reserved identifier

error: aborting due to previous error
```

notice how the error message is pointing to the wrong span! This is because we
swapped the spans of the first two tokens, giving the compiler false information
about where the tokens came from.

### Procedural macros and hygiene

Currently all procedural macros are "unhygienic". This means that all procedural
macros behave as if the output token stream was simply written inline to the
code it's next to. This means that it's affected by external items and also
affected by external imports and such.

Macro authors need to be careful to ensure their macros work in as many contexts
as possible given this limitation. This often includes using absolute paths to
items in libraries (`::std::option::Option` instead of `Option`) or
by ensuring that generated functions have names that are unlikely to clash with
other functions (like `__internal_foo` instead of `foo`).

Eventually more support for hygiene will be added to make it easier to write a
robust macro.

### Limitations of procedural macros

Procedural macros are not currently quite as powerful as `macro_rules!`-defined
macros in some respects. These limitations include:

* Procedural bang macros can only be invoked in *item* contexts. For example you
  can't define your own `format!` yet because that's only ever invoked in an
  expression context. Put another way, procedural bang macros can only expand to
  items (things like functions, impls, etc), not expressions.

  This is primarily due to the lack of hygiene with procedural macros. Once a
  better hygiene story is stabilized this restriction will likely be lifted.

* Procedural macros cannot expand to definitions of `macro_rules!` macros (none
  of them). This restriction may be lifted over time but for now this is a
  conservative limitation while compiler bugs are worked through and a design is
  agreed upon.

* Procedural attributes can only be attached to items, not expressions. For
  example `#[my_attr] fn foo() {}` is ok but `#[my_attr] return 3` is not. This
  is again due to the lack of hygiene today but this restriction may eventually
  be lifted.

* Error reporting is currently quite primitive. While an unstable diagnostic API
  exists on stable your only option is to `panic!` or to in some cases expand to
  an invocation of the `compile_error!` macro with a custom message.

[crate type]: linkage.html
[proc_macro crate]: ../proc_macro/index.html