# Coverage instrumentation attributes

The following [attributes] are used for controlling coverage instrumentation,
which can be enabled with the `-C instrument-coverage` compiler flag.

### The `coverage` attribute

The *`coverage` [attribute]* indicates whether a function should instrument code
coverage at all and show up in code coverage reports. It can only be controlled
at the function level, but it can be applied to modules, `impl` blocks, or
anything that can contain functions.

There are two ways to use the coverage attribute:

* `#[coverage(off)]` indicates that all functions within an item, recursively,
  should not be instrumented, unless specified by another attribute.
* `#[coverage(on)]` (the default) indicates that all functions within an item,
  recursively, *should* be instrumented, unless specified by another attribute.

More-specific attributes always take priority over less-specific ones, e.g.
if a crate is marked `#![coverage(off)]`, then functions inside that crate
marked `#[coverage(on)]` will still have coverage.

[attribute]: ../attributes.md
[attributes]: ../attributes.md
