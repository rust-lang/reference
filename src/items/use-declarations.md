r[items.use]
# Use declarations

r[items.use.syntax]
```grammar,items
UseDeclaration -> `use` UseTree `;`

UseTree ->
      (SimplePath? `::`)? `*`
    | (SimplePath? `::`)? `{` (UseTree ( `,`  UseTree )* `,`?)? `}`
    | SimplePath ( `as` ( IDENTIFIER | `_` ) )?
```

r[items.use.intro]
A _use declaration_ creates one or more local name bindings synonymous with
some other [path]. Usually a `use` declaration is used to shorten the path
required to refer to a module item. These declarations may appear in [modules]
and [blocks], usually at the top.
A `use` declaration is also sometimes called an _import_, or, if it is public, a _re-export_.

[path]: ../paths.md
[modules]: modules.md
[blocks]: ../expressions/block-expr.md

r[items.use.forms]
Use declarations support a number of convenient shortcuts:

r[items.use.forms.multiple]
* Simultaneously binding a list of paths with a common prefix, using the
  brace syntax `use a::b::{c, d, e::f, g::h::i};`

r[items.use.forms.self]
* Simultaneously binding a list of paths with a common prefix and their common
  parent module, using the `self` keyword, such as `use a::b::{self, c, d::e};`

r[items.use.forms.as]
* Rebinding the target name as a new local name, using the syntax `use p::q::r
  as x;`. This can also be used with the last two features:
  `use a::b::{self as ab, c as abc}`.

r[items.use.forms.glob]
* Binding all paths matching a given prefix, using the asterisk wildcard syntax
  `use a::b::*;`.

r[items.use.forms.nesting]
* Nesting groups of the previous features multiple times, such as
  `use a::b::{self as ab, c, d::{*, e::f}};`

An example of `use` declarations:

```rust
use std::collections::hash_map::{self, HashMap};

fn foo<T>(_: T){}
fn bar(map1: HashMap<String, usize>, map2: hash_map::HashMap<String, usize>){}

fn main() {
    // use declarations can also exist inside of functions
    use std::option::Option::{Some, None};

    // Equivalent to 'foo(vec![std::option::Option::Some(1.0f64),
    // std::option::Option::None]);'
    foo(vec![Some(1.0f64), None]);

    // Both `hash_map` and `HashMap` are in scope.
    let map1 = HashMap::new();
    let map2 = hash_map::HashMap::new();
    bar(map1, map2);
}
```

r[items.use.visibility]
## `use` Visibility

r[items.use.visibility.intro]
Like items, `use` declarations are private to the containing module, by
default. Also like items, a `use` declaration can be public, if qualified by
the `pub` keyword. Such a `use` declaration serves to _re-export_ a name. A
public `use` declaration can therefore _redirect_ some public name to a
different target definition: even a definition with a private canonical path,
inside a different module.

r[items.use.visibility.unambiguous]
If a sequence of such redirections form a cycle or
cannot be resolved unambiguously, they represent a compile-time error.

An example of re-exporting:

```rust
mod quux {
    pub use self::foo::{bar, baz};
    pub mod foo {
        pub fn bar() {}
        pub fn baz() {}
    }
}

fn main() {
    quux::bar();
    quux::baz();
}
```

In this example, the module `quux` re-exports two public names defined in
`foo`.

r[items.use.path]
## `use` Paths

r[items.use.path.intro]
The [paths] that are allowed in a `use` item follow the [SimplePath] grammar and are similar to the paths that may be used in an expression.
They may create bindings for:

* Nameable [items]
* [Enum variants]
* [Built-in types]
* [Attributes]
* [Derive macros]

r[items.use.path.disallowed]
They cannot import [associated items], [generic parameters], [local variables], paths with [`Self`], or [tool attributes]. More restrictions are described below.

r[items.use.path.namespace]
`use` will create bindings for all [namespaces] from the imported entities, with the exception that a `self` import will only import from the type namespace (as described below).
For example, the following illustrates creating bindings for the same name in two namespaces:

```rust
mod stuff {
    pub struct Foo(pub i32);
}

// Imports the `Foo` type and the `Foo` constructor.
use stuff::Foo;

fn example() {
    let ctor = Foo; // Uses `Foo` from the value namespace.
    let x: Foo = ctor(123); // Uses `Foo` From the type namespace.
}
```

r[items.use.path.edition2018]
> [!EDITION-2018]
> In the 2015 edition, `use` paths are relative to the crate root. For example:
>
> ```rust,edition2015
> mod foo {
>     pub mod example { pub mod iter {} }
>     pub mod baz { pub fn foobaz() {} }
> }
> mod bar {
>     // Resolves `foo` from the crate root.
>     use foo::example::iter;
>     // The `::` prefix explicitly resolves `foo`
>     // from the crate root.
>     use ::foo::baz::foobaz;
> }
>
> # fn main() {}
> ```
>
> The 2015 edition does not allow use declarations to reference the [extern prelude].
> Thus, [`extern crate`] declarations are still required in 2015 to reference an external crate in a `use` declaration.
> Beginning with the 2018 edition, `use` declarations can specify an external crate dependency the same way `extern crate` can.

r[items.use.as]
## `as` renames

The `as` keyword can be used to change the name of an imported entity.
For example:

```rust
// Creates a non-public alias `bar` for the function `foo`.
use inner::foo as bar;

mod inner {
    pub fn foo() {}
}
```

r[items.use.multiple-syntax]
## Brace syntax

r[items.use.multiple-syntax.intro]
Braces can be used in the last segment of the path to import multiple entities from the previous segment, or, if there are no previous segments, from the current scope.
Braces can be nested, creating a tree of paths, where each grouping of segments is logically combined with its parent to create a full path.

```rust
// Creates bindings to:
// - `std::collections::BTreeSet`
// - `std::collections::hash_map`
// - `std::collections::hash_map::HashMap`
use std::collections::{BTreeSet, hash_map::{self, HashMap}};
```

r[items.use.multiple-syntax.empty]
An empty brace does not import anything, though the leading path is validated that it is accessible.
<!-- This is slightly wrong, see: https://github.com/rust-lang/rust/issues/61826 -->

r[items.use.multiple-syntax.edition2018]
> [!EDITION-2018]
> In the 2015 edition, paths are relative to the crate root, so an import such as `use {foo, bar};` will import the names `foo` and `bar` from the crate root, whereas starting in 2018, those names are relative to the current scope.

r[items.use.self]
## `self` imports

r[items.use.self.intro]
The keyword `self` may be used within [brace syntax](#brace-syntax) to create a binding of the parent entity under its own name.

```rust
mod stuff {
    pub fn foo() {}
    pub fn bar() {}
}
mod example {
    // Creates a binding for `stuff` and `foo`.
    use crate::stuff::{self, foo};
    pub fn baz() {
        foo();
        stuff::bar();
    }
}
# fn main() {}
```

r[items.use.self.namespace]
`self` only creates a binding from the [type namespace] of the parent entity.
For example, in the following, only the `foo` mod is imported:

```rust,compile_fail
mod bar {
    pub mod foo {}
    pub fn foo() {}
}

// This only imports the module `foo`. The function `foo` lives in
// the value namespace and is not imported.
use bar::foo::{self};

fn main() {
    foo(); //~ ERROR `foo` is a module
}
```

> [!NOTE]
> `self` may also be used as the first segment of a path. The usage of `self` as the first segment and inside a `use` brace is logically the same; it means the current module of the parent segment, or the current module if there is no parent segment. See [`self`] in the paths chapter for more information on the meaning of a leading `self`.

r[items.use.glob]
## Glob imports

r[items.use.glob.intro]
The `*` character may be used as the last segment of a `use` path to import all importable entities from the entity of the preceding segment.
For example:

```rust
// Creates a non-public alias to `bar`.
use foo::*;

mod foo {
    fn i_am_private() {}
    enum Example {
        V1,
        V2,
    }
    pub fn bar() {
        // Creates local aliases to `V1` and `V2`
        // of the `Example` enum.
        use Example::*;
        let x = V1;
    }
}
```

r[items.use.glob.shadowing]
Items and named imports are allowed to shadow names from glob imports in the same [namespace].
That is, if there is a name already defined by another item in the same namespace, the glob import will be shadowed.
For example:

```rust
// This creates a binding to the `clashing::Foo` tuple struct
// constructor, but does not import its type because that would
// conflict with the `Foo` struct defined here.
//
// Note that the order of definition here is unimportant.
use clashing::*;
struct Foo {
    field: f32,
}

fn do_stuff() {
    // Uses the constructor from `clashing::Foo`.
    let f1 = Foo(123);
    // The struct expression uses the type from
    // the `Foo` struct defined above.
    let f2 = Foo { field: 1.0 };
    // `Bar` is also in scope due to the glob import.
    let z = Bar {};
}

mod clashing {
    pub struct Foo(pub i32);
    pub struct Bar {}
}
```

r[items.use.glob.last-segment-only]
`*` cannot be used as the first or intermediate segments.

r[items.use.glob.self-import]
`*` cannot be used to import a module's contents into itself (such as `use self::*;`).

r[items.use.glob.edition2018]
> [!EDITION-2018]
> In the 2015 edition, paths are relative to the crate root, so an import such as `use *;` is valid, and it means to import everything from the crate root. This cannot be used in the crate root itself.

r[items.use.as-underscore]
## Underscore imports

r[items.use.as-underscore.intro]
Items can be imported without binding to a name by using an underscore with
the form `use path as _`. This is particularly useful to import a trait so
that its methods may be used without importing the trait's symbol, for example
if the trait's symbol may conflict with another symbol. Another example is to
link an external crate without importing its name.

r[items.use.as-underscore.glob]
Asterisk glob imports will import items imported with `_` in their unnameable
form.

```rust
mod foo {
    pub trait Zoo {
        fn zoo(&self) {}
    }

    impl<T> Zoo for T {}
}

use self::foo::Zoo as _;
struct Zoo;  // Underscore import avoids name conflict with this item.

fn main() {
    let z = Zoo;
    z.zoo();
}
```

r[items.use.as-underscore.macro]
The unique, unnameable symbols are created after macro expansion so that
macros may safely emit multiple references to `_` imports. For example, the
following should not produce an error:

```rust
macro_rules! m {
    ($item: item) => { $item $item }
}

m!(use std as _;);
// This expands to:
// use std as _;
// use std as _;
```

r[items.use.restrictions]
## Restrictions

The following are restrictions for valid `use` declarations:

r[items.use.restrictions.crate]
* `use crate;` must use `as` to define the name to which to bind the crate root.

r[items.use.restrictions.self]
* `use {self};` is an error; there must be a leading segment when using `self`.

r[items.use.restrictions.duplicate-name]
* As with any item definition, `use` imports cannot create duplicate bindings of the same name in the same namespace in a module or block.

r[items.use.restrictions.macro-crate]
* `use` paths with `$crate` are not allowed in a [`macro_rules`] expansion.

r[items.use.restrictions.variant]
* `use` paths cannot refer to enum variants through a [type alias]. For example:
  ```rust,compile_fail
  enum MyEnum {
      MyVariant
  }
  type TypeAlias = MyEnum;

  use MyEnum::MyVariant; //~ OK
  use TypeAlias::MyVariant; //~ ERROR
  ```

r[items.use.ambiguities]
## Ambiguities

> [!NOTE]
> This section is incomplete.

r[items.use.ambiguities.intro]
Some situations are an error when there is an ambiguity as to which name a
`use` declaration refers. This happens when there are two name candidates that
do not resolve to the same entity where neither import is
[permitted](names.resolution.expansion.imports.shadowing) to shadow the other.

r[names.resolution.early.imports.errors.ambiguity.globvsglob]
* it is an error to name an item through ambiguous use declarations
    * two globs imports which both have an item matching that name where the items are different
        * this is still an error even if there is a third non glob binding resolution to an item with the same name
* it is not an error to have two glob imports which include items which would be ambiguous so long as you do not name one of those items through the ambiguous glob imports

r[items.use.ambiguities.glob]
Glob imports are allowed to import conflicting names in the same namespace as long as the name is not used.
For example:

TODO: move this section? It's documenting a situation that _isnt_ an ambiguity
error. I've been working off of a pattern I think I saw in a few other
locations, where we have specific error sections that document all of the
reference relevant error cases associated with an some part of the language.

```rust
mod foo {
    pub struct Qux;
}

mod bar {
    pub struct Qux;
}

use foo::*;
use bar::*; //~ OK, no name conflict.

fn main() {
    // This would be an error, due to the ambiguity.
    //let x = Qux;
}
```

Multiple glob imports are allowed to import the same name, and that name is allowed to be used, if the imports are of the same item (following re-exports). The visibility of the name is the maximum visibility of the imports. For example:

```rust
mod foo {
    pub struct Qux;
}

mod bar {
    pub use super::foo::Qux;
}

// These both import the same `Qux`. The visibility of `Qux`
// is `pub` because that is the maximum visibility between
// these two `use` declarations.
pub use bar::*;
use foo::*;

fn main() {
    let _: Qux = Qux;
}
```

r[names.resolution.early.imports.errors.ambiguity.builtin-attr]
* it is an error to use a user defined attribute or derive macro with the same name as a builtin attribute (e.g. inline)
    * I think we may special case this one and allow certain kinds of
      ambiguities where the builtin-attr is shadowed by a user attribute (not
      sure if this actually exists or is just proposed, TODO investigate)

```rust
use derive as inline; // OK

#[inline] // Not OK, ambiguity at use time
fn main() {}
```

<!-- ignore: test doesn't support proc-macro -->
```rust,ignore
// myinline/src/lib.rs
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn inline(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
```

<!-- ignore: requires external crates -->
```rust,ignore
// src/lib.rs
use myinline::inline;
use myinline::inline as myinline;

#[myinline::inline]
pub fn foo() {}

#[crate::inline]
pub fn bar() {}

#[myinline]
pub fn baz() {}

#[inline] // ERROR `inline` is ambiguous
pub fn quix() {}
```

r[names.resolution.early.imports.errors.ambiguity.textualvspathbasedscope]
* path-based scope bindings for macros may not shadow textual scope bindings to macros
    * This is sort of an intersection between macros and imports, because at
      least in stable rust you can only get path-based macro resolutions from
      imports of mbe macros (and presumably from proc macro crates), but you
      can only get textual scope of macros from macro declarations
    * https://doc.rust-lang.org/nightly/reference/names/namespaces.html#r-names.namespaces.sub-namespaces.use-shadow
    * [macro.decl.scope.path.ambiguity]
r[names.resolution.early.imports.errors.ambiguity.globvsouter]
* it is an error to shadow an outer name binding with a glob import
    * This seems to only apply to early resolution (duh, I documented this as part of an early resolution codepath)
        *   // Below we report various ambiguity errors.
            // We do not need to report them if we are either in speculative resolution,
            // or in late resolution when everything is already imported and expanded
            // and no ambiguities exist.
    * I attempted to produce an example using structs and it allowed the outer import to shadow the inner glob just fine

```rust
mod bar {
    pub struct Name;
}

mod baz {
    pub struct Name;
}

use baz::Name;

pub fn foo() {
    use bar::*;
    Name;
}
```

* I'd like to have a better understanding of why this doesn't trigger ambiguity errors.
    * I'm taking a guess but I think it has to do with how and when we resolve
      names during early resolution. We resolve all the imports but ambiguities
      only occur when observed, so we'd need to try to resolve Name during
      early resolution which simply won't happen because it is a struct so it
      will never be visited for resolution during expansion.
    * We will end up resolving the imports themselves, but they'll resolve fine
      because the imports themselves aren't ambiguous
    * By the time we get to late resolution we no longer expect there to be any
      ambiguities, so we will happily return the first resolution result and
      never search for additional ambiguities, so we resolve directly to
      `bar::Name` through the glob import

    * doing it with macros produced the expected error
```rust
mod bar {
    macro_rules! name {
        () => {}
    }
    pub(crate) use name;
}

mod baz {
    macro_rules! name {
        () => {}
    }
    pub(crate) use name;
}

use baz::name;

pub fn foo() {
    use bar::*;
    name!(); // ERROR `name` is ambiguous
}
```

* how does it work with imports? The same as macros, same error during early resolution

```rust
mod bar {
    pub mod foo {
        pub struct Name;
    }
}

mod baz {
    pub mod foo {
        pub struct Name;
    }
}

use baz::foo;

pub fn foo() {
    use bar::*;
    use foo::Name; // `foo` is ambiguous
}
```

r[names.resolution.early.imports.errors.ambiguity.globvsexpanded]
* Grey Area

[`extern crate`]: extern-crates.md
[`macro_rules`]: ../macros-by-example.md
[`self`]: ../paths.md#self
[associated items]: associated-items.md
[Attributes]: ../attributes.md
[Built-in types]: ../types.md
[Derive macros]: macro.proc.derive
[Enum variants]: enumerations.md
[extern prelude]: ../names/preludes.md#extern-prelude
[generic parameters]: generics.md
[items]: ../items.md
[local variables]: ../variables.md
[namespace]: ../names/namespaces.md
[namespaces]: ../names/namespaces.md
[paths]: ../paths.md
[tool attributes]: ../attributes.md#tool-attributes
[type alias]: type-aliases.md
[type namespace]: ../names/namespaces.md
[macro.decl.scope.path.ambiguity]: ../macros-by-example.md#macro.decl.scope.path.ambiguity
