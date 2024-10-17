# Authoring Guide

This document serves as a guide for editors and reviewers. Some conventions and content guidelines are specified in the [introduction].

[introduction]: ../src/introduction.md

## Markdown formatting

* Use [ATX-style headings][atx] (not Setext) with [sentence case].
* Do not use tabs, only spaces.
* Files must end with a newline.
* Lines must not end with spaces. Double spaces have semantic meaning, but can be invisible. Use a trailing backslash if you need a hard line break.
* If possible, avoid double blank lines.
* Do not use indented code blocks; use 3+ backticks code blocks instead.
* Code blocks should have an explicit language tag.
* Do not wrap long lines. This helps with reviewing diffs of the source.
* Use [smart punctuation] instead of Unicode characters. For example, use `---` for em-dash instead of the Unicode character. Characters like em-dash can be difficult to see in a fixed-width editor, and some editors may not have easy methods to enter such characters.
* Links should be relative with the `.md` extension. Links to other rust-lang books that are published with the reference should also be relative so that the linkchecker can validate them.
* Links to the standard library should use rustdoc-style links described in [Standard library links](#standard-library-links).
* The use of reference links is preferred, with shortcuts if appropriate. Place the sorted link reference definitions at the bottom of the file, or at the bottom of a section if there are an unusually large number of links that are specific to the section.

    ```markdown
    Example of shortcut link: [enumerations]
    Example of reference link with label: [block expression][block]

    [block]: expressions/block-expr.md
    [enumerations]: types/enum.md
    ```
* See the [Conventions] section for formatting callouts such as notes, edition differences, and warnings.

There are automated checks for some of these rules. Run `cargo run --manifest-path style-check/Cargo.toml -- src` to run them locally.

[atx]: https://spec.commonmark.org/0.31.2/#atx-headings
[conventions]: ../src/introduction.md#conventions
[sentence case]: https://apastyle.apa.org/style-grammar-guidelines/capitalization/sentence-case
[smart punctuation]: https://rust-lang.github.io/mdBook/format/markdown.html#smart-punctuation

### Code examples

Code examples should use code blocks with triple backticks. The language should always be specified (such as `rust`).

```rust
println!("Hello!");
```

See <https://rust-lang.github.io/mdBook/format/theme/syntax-highlighting.html#supported-languages> for a list of supported languages.

Rust examples are tested via rustdoc, and should include the appropriate annotations:

* `edition2015` or `edition2018` --- If it is edition-specific (see `book.toml` for the default).
* `no_run` --- The example should compile successfully, but should not be executed.
* `should_panic` --- The example should compile and run, but produce a panic.
* `compile_fail` --- The example is expected to fail to compile.
* `ignore` --- The example shouldn't be built or tested. This should be avoided if possible. Usually this is only necessary when the testing framework does not support it (such as external crates or modules, or a proc-macro), or it contains pseudo-code which is not valid Rust. An HTML comment such as `<!-- ignore: requires extern crate -->` should be placed before the example to explain why it is ignored.
* `Exxxx` --- If the example is expected to fail to compile with a specific error code, include that code so that rustdoc will check that the expected code is used.

See the [rustdoc documentation] for more detail.

[rustdoc documentation]: https://doc.rust-lang.org/rustdoc/documentation-tests.html

## Special markdown constructs

The following are extensions provided by [`mdbook-spec`](https://github.com/rust-lang/spec/tree/main/mdbook-spec).

### Rules

Most clauses should be preceded with a rule. Rules can be specified in the markdown source with the following on a line by itself:

```markdown
r[foo.bar]
```

The rule name should be lowercase, with periods separating from most general to most specific (like `r[array.repeat.zero]`).

Rules can be linked to by their ID using markdown such as `[foo.bar]`. There are automatic link references so that any rule can be referred to from any page in the book.

In the HTML, the rules are clickable just like headers.

When assigning rules to new paragraphs, or when modifying rule names, use the following guidelines:

1. A rule applies to one core idea, which should be easily determined when reading the paragraph it is applied to.
2. Other than the "intro" paragraph, purely explanatory, expository, or exemplary content does not need a rule. If the expository paragraph isn't directly related to the previous, separate it with a hard (rendered) line break.
    * This content will be moved to `[!NOTE]` or more specific admonitions in the future.
3. Rust code examples and tests do not need their own rules.
4. Use the following guidelines for admonitions:
    * Notes: Do not include a rule.
    * Warning: Omit the rule if the warning follows from the previous paragraph or if the warning is explanatory and doesn't introduce any new rules.
    * Target specific behavior: Always include the rule.
    * Edition differences: Always include the rule.
5. The following keywords should be used to identify paragraphs when unambiguous:
    * `intro`: The beginning paragraph of each section - should explain the construct being defined overall.
    * `syntax`: Syntax definitions or explanations when BNF syntax definitions are not used.
    * `namespace`: For items only, specifies the namespace(s) the item introduces a name in. May also be used elsewhere when defining a namespace (e.g. `r[attribute.diagnostic.namespace]`).
6. When a rule doesn't fall under the above keywords, or for section rule ids, name the subrule as follows:
    * If the rule is naming a specific Rust language construct (e.g. an attribute, standard library type/function, or keyword-introduced concept), use the construct as named in the language, appropriately case-adjusted (but do not replace `_`s with `-`s).
    * Other than Rust language concepts with `_`s in the name, use `-` characters to separate words within a "subrule".
    * Whenever possible, do not repeat previous components of the rule.
    * Edition differences admonitions should typically be named by the edition referenced directly by the rule. If multiple editions are named, use the one for which the behavior is defined by the admonition, and not by a previous paragraph.
    * Target specific admonitions should typically be named by the least specific target property to which they apply (e.g. if a rule affects all x86 CPUs, the rule name should include `x86` rather than separately listing `i586`, `i686` and `x86_64`, and if a rule applies to all ELF platforms, it should be named `elf` rather than listing every ELF OS).
    * Use an appropriately descriptive, but short, name if the language does not provide one.

### Standard library links

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

[intra]: https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html

### Admonitions

Admonitions use a style similar to GitHub-flavored markdown, where the style name is placed at the beginning of a blockquote, such as:

```markdown
> [!WARNING]
> This is a warning.
```

All this does is apply a CSS class to the blockquote. You should define the color or style of the rule in the `css/custom.css` file if it isn't already defined.

## Style

Idioms and styling to avoid:

* Use American English spelling.
* Use Oxford commas.
* Avoid slashes for alternatives ("program/binary"); use conjunctions or rewrite it ("program or binary").
* Avoid qualifying something as "in Rust"; the entire reference is about Rust.

## Content guidelines

The following are guidelines for the content of the reference.

### Targets

The reference does not document which targets exist, or the properties of specific targets. The reference may refer to *platforms* or *target properties* where required by the language. Some examples:

* Conditional-compilation keys like `target_os` are specified to exist, but not what their values must be.
* The `windows_subsystem` attribute specifies that it only works on Windows platforms.
* Inline assembly and the `target_feature` attribute specify the architectures that are supported.

### Editions

The main text and flow should document only the current edition. Whenever there is a difference between editions, the differences should be called out with an "Edition differences" block.
