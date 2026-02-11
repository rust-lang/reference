# Links

This chapter explains how links should be handled by the Reference. Several of these capabilities are provided by [`mdbook-spec`](tooling/mdbook-spec.md).

See also the [linkchecker tests](tests.md#linkcheck) for testing links.

## Rule links

[Rules](rules/index.md) can be linked to by their ID using Markdown, with the destination set to the rule ID. Automatic link references allow any rule to be referred to from any page in the book.

```markdown
Direct label link: [names.preludes.lang]

Destination label link (custom link text): [language prelude][names.preludes.lang]

Definition link: [namespace kinds]

[namespace kinds]: names.namespaces.kinds
```

## Standard library links

You should link to the standard library without specifying a URL, in a fashion similar to [rustdoc intra-doc links][intra]. Some examples:

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

Macros can end in `!`. This can be helpful for disambiguation. For example, this refers to the macro rather than the module:

```markdown
[`alloc::vec!`]
```

Explicit namespace disambiguation is also supported:

```markdown
[`std::vec`](mod@std::vec)
```

Beware of some limitations, for example:

- Links to reexports from `std_arch` don't work due to <https://github.com/rust-lang/rust/issues/96506>.
- Links to keywords aren't supported.
- Links to trait impls where the trait is not in the prelude don't work. Traits must be in scope, and currently there is no way to add them.
- If there are multiple generic implementations, it will link to one randomly (see <https://github.com/rust-lang/rust/issues/76895>).

When running into a rustdoc limitation, consider manually linking to the correct page using a relative link. For example, `../std/arch/macro.is_x86_feature_detected.html`.

When rendering the Reference locally, it uses relative links by default to conform with how the books are published. This probably isn't what you want, so you will usually want to set the [`SPEC_RELATIVE=0` environment variable][rel] so that the links go to the live site instead.

[intra]: https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html
[rel]: tooling/building.md#spec_relative

## Grammar links

Link definitions are automatically generated for all grammar production names. See [grammar automatic linking](grammar.md#automatic-linking) for more.

```markdown
This attribute uses the [MetaWord] syntax.

Explicit grammar links can have the `grammar-` prefix like [Type][grammar-Type].
```

## Outside book links

Links to other books published with the Reference should be relative links pointing to the corresponding book. This allows the links to point to the correct version, to work with the offline docs, and to be checked by the linkchecker. For example:

```markdown
See [`-C panic`].

[`-C panic`]: ../rustc/codegen-options/index.html#panic
```

## Internal links

When possible, internal links should use [rule links](#rule-links) or [grammar links](#grammar-links). Otherwise, links should be relative to the file path and use the `.md` extension.

```markdown
- Rule link: [language prelude][names.preludes.lang]
- Grammar link: [MetaWord]
- Internal link: [Modules](items/modules.md#attributes-on-modules)
```
