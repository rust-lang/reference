# Links

This chapter explains how links should be handled by the Reference. Several of these capabilities are provided by [`mdbook-spec`](tooling/mdbook-spec.md).

See also the [linkchecker tests](tests.md#linkcheck) for testing links.

## Standard library links

You should link to the standard library without specifying a URL in a fashion similar to [rustdoc intra-doc links][intra]. Some examples:

We can link to the page on `Option`:

```markdown
[`std::option::Option`]
```

In these links, generics are ignored and can be included:

```markdown
[`std::option::Option<T>`]
```

If we don't want the full path in the text, we can write:

```markdown
[`Option`](std::option::Option)
```

Macros can end in `!`. This can be helpful for disambiguation.  For example, this refers to the macro rather than the module:

```markdown
[`alloc::vec!`]
```

Explicit namespace disambiguation is also supported:

```markdown
[`std::vec`](mod@std::vec)
```

Beware there are some limitations, for example:

- Links to rexports from `std_arch` don't work due to <https://github.com/rust-lang/rust/issues/96506>.
- Links to keywords aren't supported.
- Links to trait impls where the trait is not in the prelude doesn't work. Traits must be in scope, and there currently isn't a way to add those.
- If there are multiple generic implementations, it will link to one randomly (see <https://github.com/rust-lang/rust/issues/76895>).

When running into a rustdoc limitation, consider manually linking to the correct page using a relative link. For example, `../std/arch/macro.is_x86_feature_detected.html`.

When rendering the Reference locally, by default it uses relative links to conform with how the books are published. This probably isn't what you want, so you usually will want to set the [`SPEC_RELATIVE=0` environment variable][rel] so that the links go to the live site instead.

[intra]: https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html
[rel]: tooling/building.md#spec_relative
