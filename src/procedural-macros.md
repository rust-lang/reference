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

All procedural macros are compiled as a crate.

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
Cargo will naturally make procedural macro crates available to crates which
depend on them, or you can use the `--extern` argument.

### The `proc_macro` crate

Procedural macro crates almost always will link to the in-tree `proc_macro`
crate. The `proc_macro` crate is a compiler-provided crate which provides
facilities to working with the types of each procedural macro function. You can
learn more about this crate by exploring [the documentation][pm-dox].

[pm-dox]: https://doc.rust-lang.org/stable/proc_macro/

Linking to the `proc_macro` crate can currently be done with:

```rust,ignore
extern crate proc_macro;
```

In the 2018 edition, however, this statement will not be necessary.

### The `TokenStream` Type

One aspect you may notice about the `proc_macro` crate is that it doesn't
contain any AST items! Instead, it primarily contains a `TokenStream` type.
Procedural macros operate over *token streams* instead of AST nodes,
which is a far more stable interface over time for both the compiler and for
procedural macros to target.

A *token stream* is roughly equivalent to `Vec<TokenTree>` where a `TokenTree`
can roughly be thought of as lexical token. For example `foo` is an `Ident`
token, `.` is a `Punct` token, and `1.2` is a `Literal` token. The `TokenStream`
type, unlike `Vec<TokenTree>`, is cheap to clone (like `Rc<T>`).

To learn more about token streams, let's first dive into writing our first
procedural macro.

### Bang Macros

The first kind of procedural macro is the "procedural bang macro" macro. This
flavor of procedural macro is like writing `macro_rules!` only you get to
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
and the return value is the tokens that it should expand to. Let's fill in all
the pieces here with a noop macro.

First up, let's generate a skeleton project:

```sh
$ cargo new foo
$ cd foo
$ cargo new my-macro --lib
$ echo 'my-macro = { path = "my-macro" }' >> Cargo.toml
$ echo '[lib]' >> my-macro/Cargo.toml
$ echo 'proc-macro = true' >> my-macro/Cargo.toml
```

This'll set up a main binary project called `foo` along with a subcrate called
`my-macro` which is declared as a procedural macro. Next up we'll fill in:

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

and finally, build it!

```sh
$ cargo run
   Compiling my-macro v0.1.0 (file://.../foo/my-macro)
   Compiling foo v0.1.0 (file://.../foo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/foo`
the answer was: 3
```

Alright! This end-to-end example shows how we can create a macro that doesn't
do anything, so let's do something a bit more useful.

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

and reexecute (output edited slightly here):

```sh
$ cargo run
   Compiling my-macro v0.1.0 (file://.../foo/my-macro)
   Compiling foo v0.1.0 (file://.../foo)
TokenStream [
    Ident { ident: "fn", span: #0 bytes(39..41) },
    Ident { ident: "answer", span: #0 bytes(42..48) },
    Group { delimiter: Parenthesis, stream: TokenStream [], span: #0 bytes(48..50) },
    Punct { ch: '-', spacing: Joint, span: #0 bytes(51..53) },
    Punct { ch: '>', spacing: Alone, span: #0 bytes(51..53) },
    Ident { ident: "u32", span: #0 bytes(54..57) },
    Group {
        delimiter: Brace,
        stream: TokenStream [
            Literal { lit: Integer(3), suffix: None, span: #0 bytes(60..61) }
        ],
        span: #0 bytes(58..63)
    }
]
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/foo`
the answer was: 3
```

Here we can see how a procedural bang macro's input is a token stream (list) of
all the tokens provided as input to the macro itself, excluding the delimiters
used to invoke the macro. Notice how the braces and parentheses are using the
`Group` token tree which is used to enforce that macros always have balanced
delimiters.

As you may have guessed by now the macro invocation is effectively replaced by
the return value of the macro, creating the function that we provided as input.
We can see another example of this where we simply ignore the input:

```rust,ignore
// my-macro/src/lib.rs
#[proc_macro]
pub fn foo(_input: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 4 }".parse().unwrap()
}
```

and recompiling shows:

```sh
$ cargo run
   Compiling my-macro v0.1.0 (file://.../foo/my-macro)
   Compiling foo v0.1.0 (file://.../foo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/foo`
the answer was: 4
```

showing us how the input was ignored and the macro's output was used instead.

### Derive macros

[The book][procedural macros] has a tutorial on creating derive macros and here
we'll go into some of the nitty-gritty of how this works. The derive macro
feature allows you to define a new `#[derive(Foo)]` mode which often makes it
much easier to systematically implement traits, removing quite a lot of
boilerplate.

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

```sh
$ cargo run
   Compiling my-macro v0.1.0 (file://.../foo/my-macro)
   Compiling foo v0.1.0 (file://.../foo)
TokenStream [
    Ident { ident: "struct", span: #0 bytes(67..73) },
    Ident { ident: "Foo", span: #0 bytes(74..77) },
    Punct { ch: ';', spacing: Alone, span: #0 bytes(77..78) }
]
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/foo`
the answer was: 2
```

Here we can see how the input to the macro was a `TokenStream` representing
the three input tokens `struct Foo;`. While our output only contained the
`answer` function, we were still able to use `Foo` in the main program because
derive macros *append* items, they don't replace them.

Now this is a pretty wonky macro derive, and would likely be confusing to
users! Derive macros are primarily geared towards implementing traits, like
`Serialize` and `Deserialize`. The `syn` crate also has a [number of
examples][synex] of defining derive macros.

[procedural macros]: ../book/first-edition/procedural-macros.html
[synex]: https://github.com/dtolnay/syn/tree/master/examples

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

Attributes are often used to customize the behavior of derive macros, such as
the `#[serde]` attribute for the `serde` crate.

### Attribute macros

The third and final form of procedural macros is the attribute macro. Attribute
macros allow you to define a new `#[attr]`-style attribute which can be
attached to items and generate wrappers and such. These macros are defined like:

```rust,ignore
#[proc_macro_attribute]
pub fn foo(attr: TokenStream, input: TokenStream) -> TokenStream {
    // ...
}
```

The `#[proc_macro_attribute]` indicates that this macro is an attribute macro
and can only be invoked like `#[foo]`. The name of the function here will be the
name of the attribute as well.

The first input, `attr`, is the arguments to the attribute provided, which
we'll see in a moment. The second argument, `item`, is the item that the
attribute is attached to.

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

```sh
$ cargo run
   Compiling foo v0.1.0 (file://.../foo)
attr:
input: fn invoke1() { }
attr: bar
input: fn invoke2() { }
attr: crazy custom syntax
input: fn invoke3() { }
attr: delimiters
input: fn invoke4() { }
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/foo`
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

We can see what's going on here by feeding invalid syntax into the macro and
seeing what the compiler reports. Let's start off by seeing what the compiler
does normally:

```rust,ignore
// src/main.rs
fn _() {}
```

is compiled as:

```sh
$ cargo run
   Compiling foo v0.1.0 (file://.../foo)
error: expected identifier, found reserved identifier `_`
 --> src/main.rs:1:4
  |
1 | fn _() {}
  |    ^ expected identifier, found reserved identifier

error: aborting due to previous error
```

but when we feed it through our macro:

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

Controlling spans is quite a powerful feature and needs to be used with care,
misuse can lead to some excessively confusing error messages!

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
