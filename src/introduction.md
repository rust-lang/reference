# Introduction

This book is the primary reference for the Rust programming language.

> [!NOTE]
> For known bugs and omissions in this book, see our [GitHub issues]. If you see a case where the compiler behavior and the text here do not agree, file an issue so we can think about which is correct.

## Rust releases

Rust has a new language release every six weeks.
The first stable release of the language was Rust 1.0.0, followed by Rust 1.1.0 and so on.
Tools (`rustc`, `cargo`, etc.) and documentation ([Standard library], this book, etc.) are released with the language release.

The latest release of this book, matching the latest Rust version, can always be found at <https://doc.rust-lang.org/reference/>.
Prior versions can be found by adding the Rust version before the "reference" directory.
For example, the Reference for Rust 1.49.0 is located at <https://doc.rust-lang.org/1.49.0/reference/>.

## What *The Reference* is not

This book does not serve as an introduction to the language.
Background familiarity with the language is assumed.
A separate [book] is available to help acquire such background familiarity.

This book also does not serve as a reference to the [standard library] included in the language distribution.
Those libraries are documented separately by extracting documentation attributes from their source code.
Many of the features that one might expect to be language features are library features in Rust, so what you're looking for may be there, not here.

Similarly, this book does not usually document the specifics of `rustc` as a tool or of Cargo.
`rustc` has its own [book][rustc book].
Cargo has a [book][cargo book] that contains a [reference][cargo reference].
There are a few pages such as [linkage] that still describe how `rustc` works.

This book also only serves as a reference to what is available in stable Rust.
For unstable features being worked on, see the [Unstable Book].

Rust compilers, including `rustc`, will perform optimizations.
The reference does not specify what optimizations are allowed or disallowed.
Instead, think of the compiled program as a black box.
You can only probe by running it, feeding it input and observing its output.
Everything that happens that way must conform to what the reference says.

## How to use this book

This book does not assume you are reading this book sequentially.
Each chapter generally can be read standalone, but will cross-link to other chapters for facets of the language they refer to, but do not discuss.

There are two main ways to read this document.

The first is to answer a specific question.
If you know which chapter answers that question, you can jump to that chapter in the table of contents.
Otherwise, you can press `s` or click the magnifying glass on the top bar to search for keywords related to your question.
For example, say you wanted to know when a temporary value created in a let statement is dropped.
If you didn't already know that the [lifetime of temporaries] is defined in the [expressions chapter], you could search "temporary let" and the first search result will take you to that section.

The second is to generally improve your knowledge of a facet of the language.
In that case, just browse the table of contents until you see something you want to know more about, and just start reading.
If a link looks interesting, click it, and read about that section.

That said, there is no wrong way to read this book. Read it however you feel helps you best.

### Conventions

Like all technical books, this book has certain conventions in how it displays information.
These conventions are documented here.

* Statements that define a term contain that term in *italics*.
  Whenever that term is used outside of that chapter, it is usually a link to the section that has this definition.

  An *example term* is an example of a term being defined.

* The main text describes the latest stable edition. Differences to previous editions are separated in edition blocks:

  > [!EDITION-2018]
  > Before the 2018 edition, the behavior was this. As of the 2018 edition, the behavior is that.

* Notes that contain useful information about the state of the book or point out useful, but mostly out of scope, information are in note blocks.

  > [!NOTE]
  > This is an example note.

* Example blocks show an example that demonstrates some rule or points out some interesting aspect. Some examples may have hidden lines which can be viewed by clicking the eye icon that appears when hovering or tapping the example.

  > [!EXAMPLE]
  > This is a code example.
  > ```rust
  > println!("hello world");
  > ```

* Warnings that show unsound behavior in the language or possibly confusing interactions of language features are in a special warning box.

  > [!WARNING]
  > This is an example warning.

* Code snippets inline in the text are inside `<code>` tags.

  Longer code examples are in a syntax highlighted box that has controls for copying, executing, and showing hidden lines in the top right corner.

  ```rust
  # // This is a hidden line.
  fn main() {
      println!("This is a code example");
  }
  ```

  All examples are written for the latest edition unless otherwise stated.

* The grammar and lexical productions are described in the [Notation] chapter.

r[example.rule.label]
* Rule identifiers appear before each language rule enclosed in square brackets. These identifiers provide a way to refer to and link to a specific rule in the language ([e.g.][example rule]). The rule identifier uses periods to separate sections from most general to most specific ([destructors.scope.nesting.function-body] for example). On narrow screens, the rule name will collapse to display `[*]`.

  The rule name can be clicked to link to that rule.

  > [!WARNING]
  > The organization of the rules is currently in flux. For the time being, these identifier names are not stable between releases, and links to these rules may fail if they are changed. We intend to stabilize these once the organization has settled so that links to the rule names will not break between releases.

* Rules that have associated tests will include a `Tests` link below them (on narrow screens, the link is `[T]`). Clicking the link will pop up a list of tests, which can be clicked to view the test. For example, see [input.encoding.utf8].

  Linking rules to tests is an ongoing effort. See the [Test summary](test-summary.md) chapter for an overview.

## Contributing

We welcome contributions of all kinds.

You can contribute to this book by opening an issue or sending a pull request to [the Rust Reference repository].
If this book does not answer your question, and you think its answer is in scope of it, please do not hesitate to [file an issue] or ask about it in the `t-lang/doc` stream on [Zulip].
Knowing what people use this book for the most helps direct our attention to making those sections the best that they can be.
And of course, if you see anything that is wrong or is non-normative but not specifically called out as such, please also [file an issue].

[book]: ../book/index.html
[github issues]: https://github.com/rust-lang/reference/issues
[standard library]: std
[the Rust Reference repository]: https://github.com/rust-lang/reference/
[Unstable Book]: https://doc.rust-lang.org/nightly/unstable-book/
[cargo book]: ../cargo/index.html
[cargo reference]: ../cargo/reference/index.html
[example rule]: example.rule.label
[expressions chapter]: expressions.html
[file an issue]: https://github.com/rust-lang/reference/issues
[lifetime of temporaries]: expressions.html#temporaries
[linkage]: linkage.html
[rustc book]: ../rustc/index.html
[Notation]: notation.md
[Zulip]: https://rust-lang.zulipchat.com/#narrow/stream/237824-t-lang.2Fdoc
