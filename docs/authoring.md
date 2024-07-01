# Authoring Guide

## Markdown formatting

* Use [ATX-style headings][atx] (not Setext) with [sentence case].
* Do not use tabs, only spaces.
* Files must end with a newline.
* Lines must not end with spaces. Double spaces have semantic meaning, but can be invisible. Use a trailing backslash if you need a hard line break.
* If possible, avoid double blank lines.
* Do not use indented code blocks, use 3+ backticks code blocks instead.
* Code blocks should have an explicit language tag.
* Do not wrap long lines. This helps with reviewing diffs of the source.
* Use [smart punctuation] instead of Unicode characters. For example, use `---` for em-dash instead of the Unicode character. Characters like em-dash can be difficult to see in a fixed-width editor, and some editors may not have easy methods to enter such characters.

There are automated checks for some of these rules. Run `cargo run --manifest-path style-check/Cargo.toml -- spec` to run them locally.

[atx]: https://spec.commonmark.org/0.31.2/#atx-headings
[sentence case]: https://apastyle.apa.org/style-grammar-guidelines/capitalization/sentence-case
[smart punctuation]: https://rust-lang.github.io/mdBook/format/markdown.html#smart-punctuation

## Special markdown constructs

### Rules

Most clauses should be preceded with a rule.
Rules can be specified in the markdown source with the following on a line by itself:

```
r[foo.bar]
```

The rule name should be lowercase, with periods separating from most general to most specific (like `r[array.repeat.zero]`).

Rules can be linked to by their ID using markdown such as `[foo.bar]`. There are automatic link references so that any rule can be referred to from any page in the book.

In the HTML, the rules are clickable just like headers.

### Standard library links

You should link to the standard library without specifying a URL in a fashion similar to [rustdoc intra-doc links][intra]. Some examples:

```
Link to Option is [`std::option::Option`]

You can include generics, they are ignored, like [`std::option::Option<T>`]

You can shorthand things if you don't want the full path in the text,
like [`Option`](std::option::Option).

Macros can use `!`, which also works for disambiguation,
like [`alloc::vec!`] is the macro, not the module.

Explicit namespace disambiguation is also supported, such as [`std::vec`](mod@std::vec).
```

[intra]: https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html

### Admonitions

Admonitions use a style similar to GitHub-flavored markdown, where the style name is placed at the beginning of a blockquote, such as:

```
> [!WARNING]
> This is a warning.
```

All this does is apply a CSS class to the blockquote. You should define the color or style of the rule in the `css/custom.css` file if it isn't already defined.

## Content guidelines

The following are guidelines for the content of the spec.

### Targets

The spec does not document which targets exist, or the properties of specific targets. The spec may refer to *platforms* or *target properties* where required by the language. Some examples:

* Conditional-compilation keys like `target_os` are specified to exist, but not what their values must be.
* The `windows_subsystem` attribute specifies that it only works on Windows platforms.
* Inline assembly and the `target_feature` attribute specify the architectures that are supported.
