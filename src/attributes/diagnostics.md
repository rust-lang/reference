# Diagnostic attributes

The following [attributes] are used for controlling or generating diagnostic
messages during compilation.

## Lint check attributes

A lint check names a potentially undesirable coding pattern, such as
unreachable code or omitted documentation. The lint attributes `allow`,
`warn`, `deny`, and `forbid` use the [_MetaListPaths_] syntax to specify a
list of lint names to change the lint level for the entity to which the
attribute applies.

For any lint check `C`:

* `allow(C)` overrides the check for `C` so that violations will go
   unreported,
* `warn(C)` warns about violations of `C` but continues compilation.
* `deny(C)` signals an error after encountering a violation of `C`,
* `forbid(C)` is the same as `deny(C)`, but also forbids changing the lint
   level afterwards,

> Note: The lint checks supported by `rustc` can be found via `rustc -W help`,
> along with their default settings and are documented in the [rustc book].

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

### Tool lint attributes

Tool lints allows using scoped lints, to `allow`, `warn`, `deny` or `forbid`
lints of certain tools.

Currently `clippy` is the only available lint tool.

Tool lints only get checked when the associated tool is active. If a lint
attribute, such as `allow`, references a nonexistent tool lint, the compiler
will not warn about the nonexistent lint until you use the tool.

Otherwise, they work just like regular lint attributes:

```rust
// set the entire `pedantic` clippy lint group to warn
#![warn(clippy::pedantic)]
// silence warnings from the `filter_map` clippy lint
#![allow(clippy::filter_map)]

fn main() {
    // ...
}

// silence the `cmp_nan` clippy lint just for this function
#[allow(clippy::cmp_nan)]
fn foo() {
    // ...
}
```

## The `deprecated` attribute

The *`deprecated` attribute* marks an item as deprecated. `rustc` will issue
warnings on usage of `#[deprecated]` items. `rustdoc` will show item
deprecation, including the `since` version and `note`, if available.

The `deprecated` attribute has several forms:

- `deprecated` — Issues a generic message.
- `deprecated = "message"` — Includes the given string in the deprecation
  message.
- [_MetaListNameValueStr_] syntax with two optional fields:
  - `since` — Specifies a version number when the item was deprecated. `rustc`
    does not currently interpret the string, but external tools like [Clippy]
    may check the validity of the value.
  - `note` — Specifies a string that should be included in the deprecation
    message. This is typically used to provide an explanation about the
    deprecation and preferred alternatives.

The `deprecated` attribute may be applied to any [item], [trait item], [enum
variant], [struct field], or [external block item]. It cannot be applied to [trait
implementation items]. When applied to an item containing other items, such as
a [module] or [implementation], all child items inherit the deprecation attribute.
<!-- NOTE: Currently broken for macros, see https://github.com/rust-lang/rust/issues/49912
Also, it is only rejected for trait impl items (AnnotationKind::Prohibited). In all
other locations, it is silently ignored. Tuple struct fields are ignored.
-->

Here is an example:

```rust
#[deprecated(since = "5.2", note = "foo was rarely used. Users should instead use bar")]
pub fn foo() {}

pub fn bar() {}
```

The [RFC][1270-deprecation.md] contains motivations and more details.

[1270-deprecation.md]: https://github.com/rust-lang/rfcs/blob/master/text/1270-deprecation.md

## The `must_use` attribute

The *`must_use` attribute* is used to issue a diagnostic warning when a value
is not "used". It can be applied to user-defined composite types
([`struct`s][struct], [`enum`s][enum], and [`union`s][union]), [functions],
and [traits].

The `must_use` attribute may include a message by using the
[_MetaNameValueStr_] syntax such as `#[must_use = "example message"]`. The
message will be given alongside the warning.

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
// Violates the `unused_must_use` lint.
MustUse::new();
```

When used on a function, if the [expression] of an [expression statement] is a
[call expression] to that function, then the `unused_must_use` lint is
violated.

```rust
#[must_use]
fn five() -> i32 { 5i32 }

// Violates the unused_must_use lint.
five();
```

When used on a [trait declaration], a [call expression] of an [expression
statement] to a function that returns an [impl trait] of that trait violates
the `unused_must_use` lint.

```rust
#[must_use]
trait Critical {}
impl Critical for i32 {}

fn get_critical() -> impl Critical {
    4i32
}

// Violates the `unused_must_use` lint.
get_critical();
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

// Violates the `unused_must_use` lint.
5i32.use_me();
```

When used on a function in a trait implementation, the attribute does nothing.

> Note: Trivial no-op expressions containing the value will not violate the
> lint. Examples include wrapping the value in a type that does not implement
> [`Drop`] and then not using that type and being the final expression of a
> [block expression] that is not used.
>
> ```rust
> #[must_use]
> fn five() -> i32 { 5i32 }
>
> // None of these violate the unused_must_use lint.
> (five(),);
> Some(five());
> { five() };
> if true { five() } else { 0i32 };
> match true {
>     _ => five()
> };
> ```

> Note: It is idiomatic to use a [let statement] with a pattern of `_`
> when a must-used value is purposely discarded.
>
> ```rust
> #[must_use]
> fn five() -> i32 { 5i32 }
>
> // Does not violate the unused_must_use lint.
> let _ = five();
> ```

[Clippy]: https://github.com/rust-lang/rust-clippy
[_MetaListNameValueStr_]: ../attributes.md#meta-item-attribute-syntax
[_MetaListPaths_]: ../attributes.md#meta-item-attribute-syntax
[_MetaNameValueStr_]: ../attributes.md#meta-item-attribute-syntax
[`Drop`]: ../special-types-and-traits.md#drop
[attributes]: ../attributes.md
[block expression]: ../expressions/block-expr.md
[call expression]: ../expressions/call-expr.md
[enum variant]: ../items/enumerations.md
[enum]: ../items/enumerations.md
[expression statement]: ../statements.md#expression-statements
[expression]: ../expressions.md
[external block item]: ../items/external-blocks.md
[functions]: ../items/functions.md
[impl trait]: ../types/impl-trait.md
[implementation]: ../items/implementations.md
[item]: ../items.md
[let statement]: ../statements.md#let-statements
[module]: ../items/modules.md
[rustc book]: ../../rustc/lints/index.html
[struct field]: ../items/structs.md
[struct]: ../items/structs.md
[trait declaration]: ../items/traits.md
[trait implementation items]: ../items/implementations.md#trait-implementations
[trait item]: ../items/traits.md
[traits]: ../items/traits.md
[union]: ../items/unions.md
