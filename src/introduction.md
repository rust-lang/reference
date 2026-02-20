# Introduction

This book is the primary reference for the Rust programming language. It serves as a description of the syntax, semantics, and interpretation of the Rust language.

## Audience

The audience for this book includes:

- Rust users who want to learn about the specifics and details of the language.
- Tool developers who need to know the syntax and semantics of the language.
- Language designers who work on the evolution of the language.

See the [scope chapter] for a detailed explanation of what constitutes the Reference.

## How to use this book

This book does not assume you are reading it sequentially. Each chapter generally can be read standalone, but will cross-link to other chapters for facets of the language they refer to but do not discuss.

There are two main ways to read this document.

The first is to answer a specific question. If you know which chapter answers that question, you can jump to that chapter in the table of contents. Otherwise, you can press `s` or click the magnifying glass on the top bar to search for keywords related to your question. For example, say you wanted to know when a temporary value created in a `let` statement is dropped. If you didn't already know that the [lifetime of temporaries] is defined in the [expressions chapter], you could search "temporary let" and the first search result will take you to that section.

The second is to generally improve your knowledge of a facet of the language. In that case, just browse the table of contents until you see something you want to know more about, and just start reading. If a link looks interesting, click it, and read about that section.

That said, there is no wrong way to read this book. Read it however you feel helps you best.

> [!NOTE]
> For known bugs and omissions in this book, see our [GitHub issues]. If you see a case where the compiler behavior and the text here do not agree, file an issue so we can think about which is correct.

## Rust releases

Rust has a new language release every six weeks. The first stable release of the language was Rust 1.0.0, followed by Rust 1.1.0 and so on. Tools (`rustc`, `cargo`, etc.) and documentation ([Standard library], this book, etc.) are released with the language release.

The latest release of this book, matching the latest Rust version, can always be found at <https://doc.rust-lang.org/reference/>. Prior versions can be found by adding the Rust version before the "reference" directory. For example, the Reference for Rust 1.49.0 is located at <https://doc.rust-lang.org/1.49.0/reference/>.

## Contributing

We welcome contributions of all kinds.

You can contribute to this book by opening an issue or sending a pull request to [the Rust Reference repository]. If this book does not answer your question and you think its answer is in scope, please do not hesitate to [file an issue] or ask about it in the `t-lang-docs` stream on [Zulip]. Knowing what people use this book for the most helps direct our attention to making those sections the best they can be. And of course, if you see anything that is wrong or is non-normative but not specifically called out as such, please also [file an issue].

<!-- TODO: Link to contributing guide -->

[expressions chapter]: expressions.html
[file an issue]: https://github.com/rust-lang/reference/issues
[github issues]: https://github.com/rust-lang/reference/issues
[lifetime of temporaries]: expressions.html#temporaries
[scope chapter]: scope.md
[standard library]: std
[the Rust Reference repository]: https://github.com/rust-lang/reference/
[Zulip]: https://rust-lang.zulipchat.com/#narrow/channel/237824-t-lang-docs
