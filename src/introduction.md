# Introduction

This document is the primary reference for the Rust programming language. It
provides three kinds of material:

  - Chapters that informally describe each language construct and their use.
  - Chapters that informally describe the memory model, concurrency model,
    runtime services, linkage model and debugging facilities.
  - Appendix chapters providing rationale and references to languages that
    influenced the design.

This document does not serve as an introduction to the language. Background
familiarity with the language is assumed. A separate [book] is available to
help acquire such background familiarity.

This document also does not serve as a reference to the [standard library]
included in the language distribution. Those libraries are documented
separately by extracting documentation attributes from their source code. Many
of the features that one might expect to be language features are library
features in Rust, so what you're looking for may be there, not here.

Similarly, this document does not usually document the specifics of `rustc` as a
tool or of Cargo. Cargo has a [book][cargo book] that contains a
[reference][cargo reference]. There are a few pages such as [linkage] that still
describe how `rustc` works.

This document also only serves as a reference to what is available in stable
Rust. For unstable features being worked on, see the [Unstable Book].

Finally, this document is not normative. It may include details that are
specific to `rustc` itself, and should not be taken as a specification for
the Rust language. We intend to produce such a document someday, and until then,
the reference is the closest thing we have to one.

You sould not read this document sequentially. As a reference document, you
should skim the table of contents until you find the section you are interested
in and read that section. If you are viewing this in a browser and have
JavaScript enabled, you can also press `s` or click the magnifying glass on the
top bar to open a search bar.

You may also be interested in the [grammar].

You can contribute to this document by opening an issue or sending a pull
request to [the Rust Reference repository]. If this document does not answer
your question, and you think its answer is in scope of it, please do not
hesitate to file an issue or ask about it in the Rust docs channels on IRC or
discord. Knowing what people use this document for the most helps direct our
attention to making those sections the best that they can be.

<div class="warning">

Warning: This document may be incomplete. Documenting everything takes a
while. We have a [big issue] to track documentation for every Rust feature,
so check that out if you can't find something here.

</div>

[book]: ../book/index.html
[standard library]: ../std/index.html
[grammar]: ../grammar.html
[the Rust Reference repository]: https://github.com/rust-lang-nursery/reference/
[big issue]: https://github.com/rust-lang-nursery/reference/issues/9
[Unstable Book]: https://doc.rust-lang.org/nightly/unstable-book/
[cargo book]: ../cargo/index.html
[cargo reference]: ../cargo/reference/index.html
[linkage]: linkage.html