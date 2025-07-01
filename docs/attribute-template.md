# Attribute template

Attributes should use the following template. Examples are given for phrasing you *should* use, but you should deviate if the attribute doesn't fit any of the examples or if they get in the way of clarity.

When an attribute (or a new attribute position in the grammar) is added, be sure to update all the "attributes on" sections which list which attributes can be used in various positions.

----

r[PARENT.example]
## The `example` attribute

r[PARENT.example.intro]
The *`example` [attribute][attributes]* ...give a high level description.

> [!EXAMPLE]
> ```rust
> // This should be a very basic example showing the attribute used in some way.
> #[example]
> fn some_meaningful_name() {}
> ```

r[PARENT.example.syntax]
Describe the accepted syntax of this attribute. You can either explain that it uses one of the pre-existing grammars like `MetaWord` or define an explicit grammar. If there are different forms, briefly describe the syntax here, and link to the appropriate rules below that explain the behavior of the different forms. Examples:

----

The `example` attribute uses the [MetaWord] syntax and thus does not take any inputs.

----

The `example` attribute uses the [MetaListPaths] syntax to specify a list of ...

----

The syntax for the `example` attribute is:

```grammar,attributes
@root ExampleAttribute -> `example` `(` ... `)`
```

r[PARENT.example.allowed-positions]
Explain the valid positions where this attribute may be used.

See [`check_attr`](https://github.com/rust-lang/rust/blob/HEAD/compiler/rustc_passes/src/check_attr.rs) and [`builtin_attrs.rs`](https://github.com/rust-lang/rust/blob/HEAD/compiler/rustc_feature/src/builtin_attrs.rs) in the compiler. Don't forget that some attributes only work as inner or outer attributes. Examples:

----

The `example` attribute may only be applied to ...

----

The `example` attribute may only be applied to the crate root.

----

The `example` attribute is allowed anywhere attributes are allowed.

----

If there are unused attribute warnings, or if rustc is incorrectly accepting some positions, include a note about these.

> [!NOTE]
> `rustc` currently warns in other positions, but this may be rejected in the future.

> [!NOTE]
> `rustc` currently warns on some positions where it is ignored, but this may become an error in the future.

----

r[PARENT.example.duplicates]
Explain the behavior if the attribute is specified multiple times on an element. See [`AttributeDuplicates`](https://github.com/rust-lang/rust/blob/40d2563ea200f9327a8cb8b99a0fb82f75a7365c/compiler/rustc_feature/src/builtin_attrs.rs#L143) in the compiler. Examples:

----

Duplicate instances of the `example` attribute on the same ... have no effect.

----

The `example` attribute may only be specified once on a [THING].

----

Only the first instance of `example` on an item is honored. Subsequent `example` attributes are ignored.

> [!NOTE]
> `rustc` currently warns on subsequent duplicate `example` attributes. This may become an error in the future.

----

Only the last instance of `example` on an item is honored. Previous `example` attributes are ignored.

> [!NOTE]
> `rustc` currently warns on preceding duplicate `example` attributes. This may become an error in the future.

---

If the `example` attribute is specified multiple times on an item, then the combination of all the specified values is used as ...explain how they are merged.

----

r[PARENT.example.ATTR_NAME]
If this attribute cannot be used with another attribute, specify each one. Do this on both attributes. Example:

----

The `example` attribute may not be used with the [`foo`] attribute.

----

r[PARENT.example.unsafe]
If this is an `unsafe` attribute, explain the safety conditions it must uphold. If not, don't include this section. Be sure to also update `attributes.safety` when adding a new unsafe attribute. Example:

----

The `example` attribute must be marked with [`unsafe`][attributes.safety] because ...

----

r[PARENT.example.stdlib]
This rule explains if the attribute is exported in the standard library. Skip this if it is not. Example:

----

The `example` attribute is exported in the standard library prelude as [`core::prelude::v1::example`].

----

r[PARENT.example.foo]
From here on, add rules explaining all the behaviors of the attribute. If the attribute is very simple, you can just have one rule called ".behavior" to explain its behavior. More complex attributes, such as those with multiple kinds of inputs or different modes should describe each as a separate rule.
