# Coverage instrumentation attributes

The following [attributes] are used for controlling coverage instrumentation.

> **Note**: Coverage instrumentation is controlled in `rustc` with the [`-C instrument-coverage`] compiler flag.

[`-C instrument-coverage`]: ../../rustc/instrument-coverage.html

### The `coverage` attribute

r[attributes.coverage]

r[attributes.coverage.intro]
The *`coverage` [attribute]* indicates whether a function should include instrumentation for code coverage and show up in code coverage reports.

r[attributes.coverage.syntax]
The attribute uses the [_MetaListIdents_] syntax to specify its behavior:

* `#[coverage(off)]` indicates that all functions within an item, recursively, should not be instrumented, unless specified by another attribute.
* `#[coverage(on)]` (the default) indicates that all functions within an item, recursively, *should* be instrumented, unless specified by another attribute.

```rust
#[coverage(off)]
fn example() {}

struct S;

#[coverage(off)]
impl S {
    #[coverage(on)]
    fn function_with_coverage() {}

    fn function_without_coverage() {}
}
```

r[attributes.coverage.allowed-positions]
The `coverage` attribute can only be controlled at the granularity of individual functions. It can be applied to [functions], [closures], [associated functions], [implementations], [modules], or [the crate root].

It is an error to specify the attribute on a trait function without a body.

r[attributes.coverage.trait-impl-inherit]
When specified on a trait function, the attribute only applies to the default function body. Trait implementations do not inherit the setting from the trait definition.

r[attributes.coverage.duplicates]
It is an error to specify the `#[coverage]` attribute multiple times on the same item.

r[attributes.coverage.nesting]
Coverage attributes on more deeply nested items take priority over attributes at a higher nesting level. For example, if a crate is marked `#![coverage(off)]`, then functions inside that crate marked `#[coverage(on)]` will still have coverage.

[_MetaListIdents_]: ../attributes.md#meta-item-attribute-syntax
[associated functions]: ../items/associated-items.md#associated-functions-and-methods
[attribute]: ../attributes.md
[attributes]: ../attributes.md
[closures]: ../expressions/closure-expr.md
[functions]: ../items/functions.md
[implementations]: ../items/implementations.md
[modules]: ../items/modules.md
[the crate root]: ../crates-and-source-files.md
