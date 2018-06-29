# Paths

A *path* is a sequence of one or more path components _logically_ separated by
a namespace <span class="parenthetical">qualifier (`::`)</span>. If a path
consists of only one component, it refers to either an [item] or a [variable] in
a local control scope. If a path has multiple components, it always refers to an
item.

Two examples of simple paths consisting of only identifier components:

```rust,ignore
x;
x::y::z;
```

Path components are usually [identifiers], but they may also include
angle-bracket-enclosed lists of type arguments. In [expression] context, the
type argument list is given after a `::` namespace qualifier in order to
disambiguate it from a relational expression involving the less-than
<span class="parenthetical">symbol (`<`)</span>. In type expression context, the
final namespace qualifier is omitted.

Two examples of paths with type arguments:

```rust
# struct HashMap<K, V>(K,V);
# fn f() {
# fn id<T>(t: T) -> T { t }
type T = HashMap<i32,String>; // Type arguments used in a type expression
let x  = id::<i32>(10);       // Type arguments used in a call expression
# }
```

Paths can be denoted with various leading qualifiers to change the meaning of
how it is resolved:

* Paths starting with `::` are considered to be global paths where the
  components of the path start being resolved from the crate root. Each
  identifier in the path must resolve to an item.

```rust
mod a {
    pub fn foo() {}
}
mod b {
    pub fn foo() {
        ::a::foo(); // call a's foo function
    }
}
# fn main() {}
```

* Paths starting with the keyword `super` begin resolution relative to the
  parent module. Each further identifier must resolve to an item.

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

* Paths starting with the keyword `self` begin resolution relative to the
  current module. Each further identifier must resolve to an item.

```rust
fn foo() {}
fn bar() {
    self::foo();
}
# fn main() {}
```

Additionally keyword `super` may be repeated several times after the first
`super` or `self` to refer to ancestor modules.

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

## Canonical paths

Items defined in a module or implementation have a *canonical path* that
corresponds to where within its crate it is defined. All other paths to these
items are aliases. The canonical path is defined as a *path prefix* appended by
the path component the item itself defines.

[Implementations] and [use declarations] do not have canonical paths, although
the items that implementations define do have them. Items defined in
block expressions do not have canonical paths. Items defined in a module that
does not have a canonical path do not have a canonical path. Associated items
defined in an implementation that refers to an item without a canonical path,
e.g. as the implementing type, the trait being implemented, a type parameter or
bound on a type parameter, do not have canonical paths.

The path prefix for modules is the canonical path to that module. For bare
implementations, it is the canonical path of the item being implemented
surrounded by <span class="parenthetical">angle (`<>`)</span> brackets. For
trait implementations, it is the canonical path of the item being implemented
followed by `as` followed by the canonical path to the trait all surrounded in
<span class="parenthetical">angle (`<>`)</span> brackets.

The canonical path is only meaningful within a given crate. There is no global
namespace across crates; an item's canonical path merely identifies it within
the crate.

```rust
// Comments show the canonical path of the item.

mod a { // ::a
    pub struct Struct; // ::a::Struct

    pub trait Trait { // ::a::Trait
        fn f(&self); // a::Trait::f
    }

    impl Trait for Struct {
        fn f(&self) {} // <::a::Struct as ::a::Trait>::f
    }

    impl Struct {
        fn g(&self) {} // <::a::Struct>::g
    }
}

mod without { // ::without
    fn canonicals() { // ::without::canonicals
        struct OtherStruct; // None

        trait OtherTrait { // None
            fn g(&self); // None
        }

        impl OtherTrait for OtherStruct {
            fn g(&self) {} // None
        }

        impl OtherTrait for ::a::Struct {
            fn g(&self) {} // None
        }

        impl ::a::Trait for OtherStruct {
            fn f(&self) {} // None
        }
    }
}

# fn main() {}
```
[item]: items.html
[variable]: variables.html
[identifiers]: identifiers.html
[expression]: expressions.html
[implementations]: items/implementations.html
[modules]: items/modules.html
[use declarations]: items/use-declarations.html
