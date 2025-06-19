r[paths]
# Paths

r[paths.intro]
A *path* is a sequence of one or more path segments separated by `::` tokens.
Paths are used to refer to [items], values, [types], [macros], and [attributes].

Two examples of simple paths consisting of only identifier segments:

<!-- ignore: syntax fragment -->
```rust,ignore
x;
x::y::z;
```

## Types of paths

r[paths.simple]
### Simple Paths

r[paths.simple.syntax]
```grammar,paths
SimplePath ->
    `::`? SimplePathSegment (`::` SimplePathSegment)*

SimplePathSegment ->
    IDENTIFIER | `super` | `self` | `crate` | `$crate`
```

r[paths.simple.intro]
Simple paths are used in [visibility] markers, [attributes], [macros][mbe], and [`use`] items.
For example:

```rust
use std::io::{self, Write};
mod m {
    #[clippy::cyclomatic_complexity = "0"]
    pub (in super) fn f1() {}
}
```

r[paths.expr]
### Paths in expressions

r[paths.expr.syntax]
```grammar,paths
PathInExpression ->
    `::`? PathExprSegment (`::` PathExprSegment)*

PathExprSegment ->
    PathIdentSegment (`::` GenericArgs)?

PathIdentSegment ->
    IDENTIFIER | `super` | `self` | `Self` | `crate` | `$crate`

GenericArgs ->
      `<` `>`
    | `<` ( GenericArg `,` )* GenericArg `,`? `>`

GenericArg ->
    Lifetime | Type | GenericArgsConst | GenericArgsBinding | GenericArgsBounds

GenericArgsConst ->
      BlockExpression
    | LiteralExpression
    | `-` LiteralExpression
    | SimplePathSegment

GenericArgsBinding ->
    IDENTIFIER GenericArgs? `=` Type

GenericArgsBounds ->
    IDENTIFIER GenericArgs? `:` TypeParamBounds
```

r[paths.expr.intro]
Paths in expressions allow for paths with generic arguments to be specified. They are
used in various places in [expressions] and [patterns].

r[paths.expr.turbofish]
The `::` token is required before the opening `<` for generic arguments to avoid
ambiguity with the less-than operator. This is colloquially known as "turbofish" syntax.

```rust
(0..10).collect::<Vec<_>>();
Vec::<u8>::with_capacity(1024);
```

r[paths.expr.argument-order]
The order of generic arguments is restricted to lifetime arguments, then type
arguments, then const arguments, then equality constraints.

r[paths.expr.complex-const-params]
Const arguments must be surrounded by braces unless they are a [literal], an [inferred const], or a single segment path. An [inferred const] may not be surrounded by braces.

```rust
mod m {
    pub const C: usize = 1;
}
const C: usize = m::C;
fn f<const N: usize>() -> [u8; N] { [0; N] }

let _ = f::<1>(); // Literal.
let _: [_; 1] = f::<_>(); // Inferred const.
let _: [_; 1] = f::<(((_)))>(); // Inferred const.
let _ = f::<C>(); // Single segment path.
let _ = f::<{ m::C }>(); // Multi-segment path must be braced.
```

```rust,compile_fail
fn f<const N: usize>() -> [u8; N] { [0; _] }
let _: [_; 1] = f::<{ _ }>();
//                    ^ ERROR `_` not allowed here
```

> [!NOTE]
> In a generic argument list, an [inferred const] is parsed as an [inferred type][InferredType] but then semantically treated as a separate kind of [const generic argument].

r[paths.expr.impl-trait-params]
The synthetic type parameters corresponding to `impl Trait` types are implicit,
and these cannot be explicitly specified.

r[paths.qualified]
## Qualified paths

r[paths.qualified.syntax]
```grammar,paths
QualifiedPathInExpression -> QualifiedPathType (`::` PathExprSegment)+

QualifiedPathType -> `<` Type (`as` TypePath)? `>`

QualifiedPathInType -> QualifiedPathType (`::` TypePathSegment)+
```

r[paths.qualified.intro]
Fully qualified paths allow for disambiguating the path for [trait implementations] and
for specifying [canonical paths](#canonical-paths). When used in a type specification, it
supports using the type syntax specified below.

```rust
struct S;
impl S {
    fn f() { println!("S"); }
}
trait T1 {
    fn f() { println!("T1 f"); }
}
impl T1 for S {}
trait T2 {
    fn f() { println!("T2 f"); }
}
impl T2 for S {}
S::f();  // Calls the inherent impl.
<S as T1>::f();  // Calls the T1 trait function.
<S as T2>::f();  // Calls the T2 trait function.
```

r[paths.type]
### Paths in types

r[paths.type.syntax]
```grammar,paths
TypePath -> `::`? TypePathSegment (`::` TypePathSegment)*

TypePathSegment -> PathIdentSegment (`::`? (GenericArgs | TypePathFn))?

TypePathFn -> `(` TypePathFnInputs? `)` (`->` TypeNoBounds)?

TypePathFnInputs -> Type (`,` Type)* `,`?
```

r[paths.type.intro]
Type paths are used within type definitions, trait bounds, type parameter bounds,
and qualified paths.

r[paths.type.turbofish]
Although the `::` token is allowed before the generics arguments, it is not required
because there is no ambiguity like there is in [PathInExpression].

```rust
# mod ops {
#     pub struct Range<T> {f1: T}
#     pub trait Index<T> {}
#     pub struct Example<'a> {f1: &'a i32}
# }
# struct S;
impl ops::Index<ops::Range<usize>> for S { /*...*/ }
fn i<'a>() -> impl Iterator<Item = ops::Example<'a>> {
    // ...
#    const EXAMPLE: Vec<ops::Example<'static>> = Vec::new();
#    EXAMPLE.into_iter()
}
type G = std::boxed::Box<dyn std::ops::FnOnce(isize) -> isize>;
```

r[paths.qualifiers]
## Path qualifiers

Paths can be denoted with various leading qualifiers to change the meaning of
how it is resolved.

r[paths.qualifiers.global-root]
### `::`

r[paths.qualifiers.global-root.intro]
Paths starting with `::` are considered to be *global paths* where the segments of the path
start being resolved from a place which differs based on edition. Each identifier in
the path must resolve to an item.

r[paths.qualifiers.global-root.edition2018]
> [!EDITION-2018]
> In the 2015 Edition, identifiers resolve from the "crate root" (`crate::` in the 2018 edition), which contains a variety of different items, including external crates, default crates such as `std` or `core`, and items in the top level of the crate (including `use` imports).
>
> Beginning with the 2018 Edition, paths starting with `::` resolve from crates in the [extern prelude]. That is, they must be followed by the name of a crate.

```rust
pub fn foo() {
    // In the 2018 edition, this accesses `std` via the extern prelude.
    // In the 2015 edition, this accesses `std` via the crate root.
    let now = ::std::time::Instant::now();
    println!("{:?}", now);
}
```

```rust,edition2015
// 2015 Edition
mod a {
    pub fn foo() {}
}
mod b {
    pub fn foo() {
        ::a::foo(); // call `a`'s foo function
        // In Rust 2018, `::a` would be interpreted as the crate `a`.
    }
}
# fn main() {}
```

r[paths.qualifiers.mod-self]
### `self`

r[paths.qualifiers.mod-self.intro]
`self` resolves the path relative to the current module.

r[paths.qualifiers.mod-self.restriction]
`self` can only be used as the first segment, without a preceding `::`.

r[paths.qualifiers.self-pat]
In a method body, a path which consists of a single `self` segment resolves to the method's self parameter.

```rust
fn foo() {}
fn bar() {
    self::foo();
}
struct S(bool);
impl S {
  fn baz(self) {
        self.0;
    }
}
# fn main() {}
```

r[paths.qualifiers.type-self]
### `Self`

r[paths.qualifiers.type-self.intro]
`Self`, with a capital "S", is used to refer to the current type being implemented or defined. It may be used in the following situations:

r[paths.qualifiers.type-self.trait]
* In a [trait] definition, it refers to the type implementing the trait.

r[paths.qualifiers.type-self.impl]
* In an [implementation], it refers to the type being implemented.
  When implementing a tuple or unit [struct], it also refers to the constructor in the [value namespace].

r[paths.qualifiers.type-self.type]
* In the definition of a [struct], [enumeration], or [union], it refers to the type being defined.
  The definition is not allowed to be infinitely recursive (there must be an indirection).

r[paths.qualifiers.type-self.scope]
The scope of `Self` behaves similarly to a generic parameter; see the [`Self` scope] section for more details.

r[paths.qualifiers.type-self.allowed-positions]
`Self` can only be used as the first segment, without a preceding `::`.

r[paths.qualifiers.type-self.no-generics]
The `Self` path cannot include generic arguments (as in `Self::<i32>`).

```rust
trait T {
    type Item;
    const C: i32;
    // `Self` will be whatever type that implements `T`.
    fn new() -> Self;
    // `Self::Item` will be the type alias in the implementation.
    fn f(&self) -> Self::Item;
}
struct S;
impl T for S {
    type Item = i32;
    const C: i32 = 9;
    fn new() -> Self {           // `Self` is the type `S`.
        S
    }
    fn f(&self) -> Self::Item {  // `Self::Item` is the type `i32`.
        Self::C                  // `Self::C` is the constant value `9`.
    }
}

// `Self` is in scope within the generics of a trait definition,
// to refer to the type being defined.
trait Add<Rhs = Self> {
    type Output;
    // `Self` can also reference associated items of the
    // type being implemented.
    fn add(self, rhs: Rhs) -> Self::Output;
}

struct NonEmptyList<T> {
    head: T,
    // A struct can reference itself (as long as it is not
    // infinitely recursive).
    tail: Option<Box<Self>>,
}
```

r[paths.qualifiers.super]
### `super`

r[paths.qualifiers.super.intro]
`super` in a path resolves to the parent module.

r[paths.qualifiers.super.allowed-positions]
It may only be used in leading segments of the path, possibly after an initial `self` segment.

```rust
mod a {
    pub fn foo() {}
}
mod b {
    pub fn foo() {
        super::a::foo(); // call a's foo function
    }
}
# fn main() {}
```

r[paths.qualifiers.super.repetition]
`super` may be repeated several times after the first `super` or `self` to refer to
ancestor modules.

```rust
mod a {
    fn foo() {}

    mod b {
        mod c {
            fn foo() {
                super::super::foo(); // call a's foo function
                self::super::super::foo(); // call a's foo function
            }
        }
    }
}
# fn main() {}
```

r[paths.qualifiers.crate]
### `crate`

r[paths.qualifiers.crate.intro]
`crate` resolves the path relative to the current crate.

r[paths.qualifiers.crate.allowed-positions]
`crate` can only be used as the first segment, without a preceding `::`.

```rust
fn foo() {}
mod a {
    fn bar() {
        crate::foo();
    }
}
# fn main() {}
```

r[paths.qualifiers.macro-crate]
### `$crate`

r[paths.qualifiers.macro-crate.allowed-positions]
[`$crate`] is only used within [macro transcribers], and can only be used as the first
segment, without a preceding `::`.

r[paths.qualifiers.macro-crate.hygiene]
[`$crate`] will expand to a path to access items from the
top level of the crate where the macro is defined, regardless of which crate the macro is
invoked.

```rust
pub fn increment(x: u32) -> u32 {
    x + 1
}

#[macro_export]
macro_rules! inc {
    ($x:expr) => ( $crate::increment($x) )
}
# fn main() { }
```

r[paths.canonical]
## Canonical paths

r[paths.canonical.intro]
Items defined in a module or implementation have a *canonical path* that
corresponds to where within its crate it is defined.

r[paths.canonical.alias]
All other paths to these items are aliases.

r[paths.canonical.def]
The canonical path is defined as a *path prefix* appended by
the path segment the item itself defines.

r[paths.canonical.non-canonical]
[Implementations] and [use declarations] do not have canonical paths, although
the items that implementations define do have them. Items defined in
block expressions do not have canonical paths. Items defined in a module that
does not have a canonical path do not have a canonical path. Associated items
defined in an implementation that refers to an item without a canonical path,
e.g. as the implementing type, the trait being implemented, a type parameter or
bound on a type parameter, do not have canonical paths.

r[paths.canonical.module-prefix]
The path prefix for modules is the canonical path to that module.

r[paths.canonical.bare-impl-prefix]
For bare implementations, it is the canonical path of the item being implemented
surrounded by <span class="parenthetical">angle (`<>`)</span> brackets.

r[paths.canonical.trait-impl-prefix]
For [trait implementations], it is the canonical path of the item being implemented
followed by `as` followed by the canonical path to the trait all surrounded in
<span class="parenthetical">angle (`<>`)</span> brackets.

r[paths.canonical.local-canonical-path]
The canonical path is only meaningful within a given crate. There is no global
namespace across crates; an item's canonical path merely identifies it within
the crate.

```rust
// Comments show the canonical path of the item.

mod a { // crate::a
    pub struct Struct; // crate::a::Struct

    pub trait Trait { // crate::a::Trait
        fn f(&self); // crate::a::Trait::f
    }

    impl Trait for Struct {
        fn f(&self) {} // <crate::a::Struct as crate::a::Trait>::f
    }

    impl Struct {
        fn g(&self) {} // <crate::a::Struct>::g
    }
}

mod without { // crate::without
    fn canonicals() { // crate::without::canonicals
        struct OtherStruct; // None

        trait OtherTrait { // None
            fn g(&self); // None
        }

        impl OtherTrait for OtherStruct {
            fn g(&self) {} // None
        }

        impl OtherTrait for crate::a::Struct {
            fn g(&self) {} // None
        }

        impl crate::a::Trait for OtherStruct {
            fn f(&self) {} // None
        }
    }
}

# fn main() {}
```

[`$crate`]: macro.decl.hygiene.crate
[implementations]: items/implementations.md
[items]: items.md
[literal]: expressions/literal-expr.md
[use declarations]: items/use-declarations.md
[`Self` scope]: names/scopes.md#self-scope
[`use`]: items/use-declarations.md
[attributes]: attributes.md
[const generic argument]: items.generics.const.argument
[enumeration]: items/enumerations.md
[expressions]: expressions.md
[extern prelude]: names/preludes.md#extern-prelude
[implementation]: items/implementations.md
[inferred const]: items.generics.const.inferred
[macro transcribers]: macros-by-example.md
[macros]: macros.md
[mbe]: macros-by-example.md
[patterns]: patterns.md
[struct]: items/structs.md
[trait implementations]: items/implementations.md#trait-implementations
[trait]: items/traits.md
[traits]: items/traits.md
[types]: types.md
[union]: items/unions.md
[value namespace]: names/namespaces.md
[visibility]: visibility-and-privacy.md
