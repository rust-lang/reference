# Attributes

> **<sup>Syntax</sup>**\
> _Attribute_ :\
> &nbsp;&nbsp; _InnerAttribute_ | _OuterAttribute_
>
> _InnerAttribute_ :\
> &nbsp;&nbsp; `#![` MetaItem `]`
>
> _OuterAttribute_ :\
> &nbsp;&nbsp; `#[` MetaItem `]`
>
> _MetaItem_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; IDENTIFIER\
> &nbsp;&nbsp; | IDENTIFIER `=` LITERAL\
> &nbsp;&nbsp; | IDENTIFIER `(` _MetaSeq_ `)`
>
> _MetaSeq_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; EMPTY\
> &nbsp;&nbsp; | _MetaItem_\
> &nbsp;&nbsp; | LITERAL\
> &nbsp;&nbsp; | _MetaItem_ `,` _MetaSeq_

An _attribute_ is a general, free-form metadatum that is interpreted according
to name, convention, and language and compiler version. Attributes are modeled
on Attributes in [ECMA-335], with the syntax coming from [ECMA-334] \(C#).

Attributes may appear as any of:

* A single identifier, the _attribute name_
* An identifier followed by the equals sign '=' and a literal, providing a
  key/value pair
* An identifier followed by a parenthesized list of sub-attribute arguments
  which include literals

_Inner attributes_, written with a bang ("!") after the hash ("#"), apply to the
item that the attribute is declared within. _Outer attributes_, written without
the bang after the hash, apply to the item or generic parameter that follow the
attribute.

Attributes may be applied to many things in the language:

* All [item declarations] accept outer attributes while [external blocks],
  [functions], [implementations], and [modules] accept inner attributes.
* [Statements] accept outer attributes.
* [Block expressions] accept outer and inner attributes, but only when they are
  the outer expression of an [expression statement] or the final expression of
  another block expression.
* [Enum] variants and [struct] and [union] fields accept outer attributes.
* [Match expression arms][match expressions] accept outer attributes.
* [Generic lifetime or type parameter][generics] accept outer attributes.

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

// Outer attribute applies to the entire function.
fn some_unused_variables() {
  #![allow(unused_variables)]
  
  let x = ();
  let y = ();
  let z = ();
}
```

The rest of this page describes or links to descriptions of which attribute
names have meaning.

## Crate-only attributes

- `crate_name` - specify the crate's crate name.
- `crate_type` - see [linkage](linkage.html).
- `no_builtins` - disable optimizing certain code patterns to invocations of
                  library functions that are assumed to exist
- `no_main` - disable emitting the `main` symbol. Useful when some other
   object being linked to defines `main`.
- `no_start` - disable linking to the `native` crate, which specifies the
  "start" language item.
- `no_std` - disable linking to the `std` crate.
- `recursion_limit` - Sets the maximum depth for potentially
                      infinitely-recursive compile-time operations like
                      auto-dereference or macro expansion. The default is
                      `#![recursion_limit="64"]`.
- `windows_subsystem` - Indicates that when this crate is linked for a Windows
                        target it will configure the resulting binary's
                        [subsystem] via the linker. Valid values for this
                        attribute are `console` and `windows`, corresponding to
                        those two respective subsystems. More subsystems may be
                        allowed in the future, and this attribute is ignored on
                        non-Windows targets.

[subsystem]: https://msdn.microsoft.com/en-us/library/fcc1zstk.aspx

## Module-only attributes

- `no_implicit_prelude` - disable injecting `use std::prelude::*` in this
  module.
- `path` - specifies the file to load the module from. `#[path="foo.rs"] mod
  bar;` is equivalent to `mod bar { /* contents of foo.rs */ }`. The path is
  taken relative to the directory that the current module is in.

## Function-only attributes

- `test` - indicates that this function is a test function, to only be compiled
  in case of `--test`.
  - `ignore` - indicates that this test function is disabled.
- `should_panic` - indicates that this test function should panic, inverting the
  success condition.

## FFI attributes

On an `extern` block, the following attributes are interpreted:

- `link_args` - specify arguments to the linker, rather than just the library
  name and type. This is feature gated and the exact behavior is
  implementation-defined (due to variety of linker invocation syntax).
- `link` - indicate that a native library should be linked to for the
  declarations in this block to be linked correctly. `link` supports an optional
  `kind` key with three possible values: `dylib`, `static`, and `framework`. See
  [external blocks](items/external-blocks.html) for more about external blocks.
  Two examples: `#[link(name = "readline")]` and
  `#[link(name = "CoreFoundation", kind = "framework")]`.
- `linked_from` - indicates what native library this block of FFI items is
  coming from. This attribute is of the form `#[linked_from = "foo"]` where
  `foo` is the name of a library in either `#[link]` or a `-l` flag. This
  attribute is currently required to export symbols from a Rust dynamic library
  on Windows, and it is feature gated behind the `linked_from` feature.

On declarations inside an `extern` block, the following attributes are
interpreted:

- `link_name` - the name of the symbol that this function or static should be
  imported as.
- `linkage` - on a static, this specifies the [linkage
  type](http://llvm.org/docs/LangRef.html#linkage-types).

See [type layout](type-layout.html) for documentation on the `repr` attribute
which can be used to control type layout.

## Macro-related attributes

- `macro_use` on a `mod` — macros defined in this module will be visible in the
  module's parent, after this module has been included.

- `macro_use` on an `extern crate` — load macros from this crate.  An optional
  list of names `#[macro_use(foo, bar)]` restricts the import to just those
  macros named.  The `extern crate` must appear at the crate root, not inside
  `mod`, which ensures proper function of the [`$crate` macro
  variable](../book/first-edition/macros.html#the-variable-crate).

- `macro_reexport` on an `extern crate` — re-export the named macros.

- `macro_export` - export a macro for cross-crate usage.

- `no_link` on an `extern crate` — even if we load this crate for macros, don't
  link it into the output.

See the [macros section of the first edition of the
book](../book/first-edition/macros.html#scoping-and-macro-importexport) for more
information on macro scope.

## Miscellaneous attributes

- `export_name` - on statics and functions, this determines the name of the
  exported symbol.
- `global_allocator` - when applied to a static item implementing the
  `GlobalAlloc` trait, sets the global allocator.
- `link_section` - on statics and functions, this specifies the section of the
  object file that this item's contents will be placed into.
- `no_mangle` - on any item, do not apply the standard name mangling. Set the
  symbol for this item to its identifier.

### Deprecation

The `deprecated` attribute marks an item as deprecated. It has two optional
fields, `since` and `note`.

- `since` expects a version number, as in `#[deprecated(since = "1.4.1")]`
    - `rustc` doesn't know anything about versions, but external tools like
      `clippy` may check the validity of this field.
- `note` is a free text field, allowing you to provide an explanation about
  the deprecation and preferred alternatives.

Only [public items](visibility-and-privacy.html) can be given the
`#[deprecated]` attribute. `#[deprecated]` on a module is inherited by all
child items of that module.

`rustc` will issue warnings on usage of `#[deprecated]` items. `rustdoc` will
show item deprecation, including the `since` version and `note`, if available.

Here's an example.

```rust
#[deprecated(since = "5.2", note = "foo was rarely used. Users should instead use bar")]
pub fn foo() {}

pub fn bar() {}
```

The [RFC][1270-deprecation.md] contains motivations and more details.

[1270-deprecation.md]: https://github.com/rust-lang/rfcs/blob/master/text/1270-deprecation.md

### Documentation

The `doc` attribute is used to document items and fields. [Doc comments]
are transformed into `doc` attributes.

See [The Rustdoc Book] for reference material on this attribute.

### Conditional compilation

The `cfg` and `cfg_attr` attributes control conditional compilation of [items]
and attributes. See the [conditional compilation] section for reference material
on these attributes.

### Lint check attributes

A lint check names a potentially undesirable coding pattern, such as
unreachable code or omitted documentation, for the static entity to which the
attribute applies.

For any lint check `C`:

* `allow(C)` overrides the check for `C` so that violations will go
   unreported,
* `deny(C)` signals an error after encountering a violation of `C`,
* `forbid(C)` is the same as `deny(C)`, but also forbids changing the lint
   level afterwards,
* `warn(C)` warns about violations of `C` but continues compilation.

The lint checks supported by the compiler can be found via `rustc -W help`,
along with their default settings.  [Compiler
plugins][unstable book plugin] can provide additional lint checks.

```rust
pub mod m1 {
    // Missing documentation is ignored here
    #[allow(missing_docs)]
    pub fn undocumented_one() -> i32 { 1 }

    // Missing documentation signals a warning here
    #[warn(missing_docs)]
    pub fn undocumented_too() -> i32 { 2 }

    // Missing documentation signals an error here
    #[deny(missing_docs)]
    pub fn undocumented_end() -> i32 { 3 }
}
```

This example shows how one can use `allow` and `warn` to toggle a particular
check on and off:

```rust
#[warn(missing_docs)]
pub mod m2{
    #[allow(missing_docs)]
    pub mod nested {
        // Missing documentation is ignored here
        pub fn undocumented_one() -> i32 { 1 }

        // Missing documentation signals a warning here,
        // despite the allow above.
        #[warn(missing_docs)]
        pub fn undocumented_two() -> i32 { 2 }
    }

    // Missing documentation signals a warning here
    pub fn undocumented_too() -> i32 { 3 }
}
```

This example shows how one can use `forbid` to disallow uses of `allow` for
that lint check:

```rust,compile_fail
#[forbid(missing_docs)]
pub mod m3 {
    // Attempting to toggle warning signals an error here
    #[allow(missing_docs)]
    /// Returns 2.
    pub fn undocumented_too() -> i32 { 2 }
}
```

#### `must_use` Attribute

The `must_use` attribute can be used on user-defined composite types
([`struct`s][struct], [`enum`s][enum], and [`union`s][union]) and [functions].

When used on user-defined composite types, if the [expression] of an
[expression statement] has that type, then the `unused_must_use` lint is
violated.

```rust
#[must_use]
struct MustUse {
  // some fields
}

# impl MustUse {
#   fn new() -> MustUse { MustUse {} }
# }
#
fn main() {
  // Violates the `unused_must_use` lint.
  MustUse::new();
}
```

When used on a function, if the [expression] of an
[expression statement] is a [call expression] to that function, then the
`unused_must_use` lint is violated. The exceptions to this is if the return type
of the function is `()`, `!`, or a [zero-variant enum], in which case the
attribute does nothing.

```rust
#[must_use]
fn five() -> i32 { 5i32 }

fn main() {
  // Violates the unused_must_use lint.
  five();
}
```

When used on a function in a trait declaration, then the behavior also applies
when the call expression is a function from an implementation of the trait.

```rust
trait Trait {
  #[must_use]
  fn use_me(&self) -> i32;
}

impl Trait for i32 {
  fn use_me(&self) -> i32 { 0i32 }
}

fn main() {
  // Violates the `unused_must_use` lint.
  5i32.use_me();
}
```

When used on a function in an implementation, the attribute does nothing.

> Note: Trivial no-op expressions containing the value will not violate the
> lint. Examples include wrapping the value in a type that does not implement
> [`Drop`] and then not using that type and being the final expression of a
> [block expression] that is not used.
>
> ```rust
> #[must_use]
> fn five() -> i32 { 5i32 }
>
> fn main() {
>   // None of these violate the unused_must_use lint.
>   (five(),);
>   Some(five());
>   { five() };
>   if true { five() } else { 0i32 };
>   match true {
>     _ => five()
>   };
> }
> ```

> Note: It is idiomatic to use a [let statement] with a pattern of `_`
> when a must-used value is purposely discarded.
>
> ```rust
> #[must_use]
> fn five() -> i32 { 5i32 }
>
> fn main() {
>   // Does not violate the unused_must_use lint.
>   let _ = five();
> }
> ```

The `must_use` attribute may also include a message by using
`#[must_use = "message"]`. The message will be given alongside the warning.

### Optimization Hints

The `cold` and `inline` attributes give suggestions to the compiler to compile
your code in a way that may be faster than what it would do without the hint.
The attributes are only suggestions, and the compiler may choose to ignore it.

#### `inline` Attribute

The *`inline` attribute* suggests to the compiler that it should place a copy of
the attributed function in the caller, rather than generating code to call the
function where it is defined.

This attribute can be used on [functions] and function prototypes, although it
does not do anything on function prototypes. When this attribute is applied to
a function in a [trait], it applies only to that function when used as a default
function for a trait implementation and not to all trait implementations.

> ***Note***: The compiler automatically inlines functions based on internal
> heuristics. Incorrectly inlining functions can actually make the program
> slower, so this attibute should be used with care.

There are three ways of using the inline attribute:

* `#[inline]` hints the compiler to perform an inline expansion.
* `#[inline(always)]` asks the compiler to always perform an inline expansion.
* `#[inline(never)]` asks the compiler to never perform an inline expansion.

#### `cold` Attribute

The *`cold` attribute* suggests to the compiler that the attributed function is
unlikely to be called.

This attribute can be used on [functions] and function prototypes, although it
does not do anything on function prototypes. When this attribute is applied to
a function in a [trait], it applies only to that function when used as a default
function for a trait implementation and not to all trait implementations.

### `derive`

The `derive` attribute allows certain traits to be automatically implemented
for data structures. For example, the following will create an `impl` for the
`PartialEq` and `Clone` traits for `Foo`, the type parameter `T` will be given
the `PartialEq` or `Clone` constraints for the appropriate `impl`:

```rust
#[derive(PartialEq, Clone)]
struct Foo<T> {
    a: i32,
    b: T,
}
```

The generated `impl` for `PartialEq` is equivalent to

```rust
# struct Foo<T> { a: i32, b: T }
impl<T: PartialEq> PartialEq for Foo<T> {
    fn eq(&self, other: &Foo<T>) -> bool {
        self.a == other.a && self.b == other.b
    }

    fn ne(&self, other: &Foo<T>) -> bool {
        self.a != other.a || self.b != other.b
    }
}
```

You can implement `derive` for your own traits through [procedural macros].

[Doc comments]: comments.html#doc-comments
[The Rustdoc Book]: ../rustdoc/the-doc-attribute.html
[procedural macros]: procedural-macros.html
[struct]: items/structs.html
[enum]: items/enumerations.html
[union]: items/unions.html
[functions]: items/functions.html
[expression]: expressions.html
[expression statement]: statements.html#expression-statements
[call expression]: expressions/call-expr.html
[block expression]: expressions/block-expr.html
[block expressions]: expressions/block-expr.html
[`Drop`]: special-types-and-traits.html#drop
[let statement]: statements.html#let-statements
[unstable book plugin]: ../unstable-book/language-features/plugin.html#lint-plugins
[zero-variant enum]: items/enumerations.html#zero-variant-enums
[ECMA-334]: https://www.ecma-international.org/publications/standards/Ecma-334.htm
[ECMA-335]: https://www.ecma-international.org/publications/standards/Ecma-335.htm
[item declarations]: items.html
[generics]: items/generics.html
[implementations]: items/implementations.html
[modules]: items/modules.html
[statements]: statements.html
[match expressions]: expressions/match-expr.html
[external blocks]: items/external-blocks.html
[items]: items.html
[conditional compilation]: conditional-compilation.html
[trait]: items/traits.html