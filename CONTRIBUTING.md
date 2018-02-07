Thank you for your interest in contributing to the Rust Reference!

There are a few ways of helping with the reference: critiquing the reference,
editing the reference, fixing incorrect information, adding examples and
glossary entries, and documenting new or otherwise undocumented features in
Rust.

For awhile, the Reference was basically ignored, and Rust continued gaining new
features or changing old ones. It was also basically the introduction document
before the first edition of the Rust book, and constantly in flux from the huge
churn of the language design before v1.0.0. So there's a lot that's wrong, too
teachy for people who should have basic understanding of Rust, or is too shallow
for the Reference. As such, we have the warning saying there's work that needs
to be done. Eventually, we plan to make sure everything is well documented
enough that we can remove the warning.

## Critiquing the Reference

This is the easiest way to contribute. Basically, as you read the reference, if
you find something confusing, incorrect, or missing, then you can file an issue
against the reference explaining your concerns.

## Editing the Reference

Typos and incorrect links get through from time to time. Should you find them,
we welcome PRs to fix them. Additionally, larger editing jobs that help remove
the number of parentheticals, remove comma splices, italicize term definitions
and other similar tasks are helpful.

## Adding Examples and Glossary Entries

Examples are great. Many people will only read examples and ignore the prose.
Ideally, every facet of every feature will have an example.

Likewise, the reference has a glossary. It doesn't need to explain every facet
of every feature nor contain every definition, but it does need to be expanded
upon. Ideally entries in the glossary link to the associated documentation.

## Adding Documentation

There are a lot of features that are not documented at all or are documented
poorly. This is the hardest, but definitely most valuable. Pick something from
["Document all features"][9] or a [missing feature] tag, and write about it.

While writing, you may find it handy to have a [playpen] open to test out what
you are documenting.

Feel free to take information from the standard library and Rustonomicon as
appropriate.

Note that we don't write documentation for purely library features such as
threads and IO and we don't write about Rust in the future. Documentation is
written as if the current stable release of Rust is the last release. If you
want to write about Rust in the future, you want [the Unstable book][unstable].

## RFC Review Process

Older, stable RFCs need review to determine if they need documentation written
and, if so, need that documentation written. The ["Document all features"][9]
issue tracks the overall effort, and individual RFCs are laid out on the [RFC
Status] project. RFCs that have not yet been reviewed to scope out the work are
in the "Needs Review" column, with ones needing documentation in the "Stable,
Needs Documentation" column.

If you review an RFC and determine that there is no documentation required,
please convert the project card to an issue and then close the issue, explaining
why no documentation is required. This is so as to ensure that there is a record
and a chance for others to disagree. If you review it and determine
documentation is necessary, feel free to simply move the card into the "Stable,
Needs Documentation" column. It can be converted into an issue if it needs
discussion, or left as a text card.

For RFCs which do not stabilize all at once (for instance, because some aspects
are insta-stable), if there is any unstabilized part that needs documentation,
then the RFC should be advanced to "Stable, Needs Documentation" and converted
to an issue. On the issue, remark about which parts need documentation and which
are still unstable.

## Stabilization

Now, in order for a new RFC to be stabilized, it must have documentation
written. If this requires a change to the reference, then the necessary
documentation should be written and a PR created. Once the PR has been reviewed
(along with any necessary documentation PRs to other repositories), the feature
can be stabilized in Rust, and then the doc PRs merged. Anyone is free to write
these PRs, but they should wait until the feature is unlikely to change much
before stabilization.

RFCs needed documentation for stabilization can be added to the [RFC Status]
project, under the "Awaiting Docs for Stabilization" column.

[9]: https://github.com/rust-lang-nursery/reference/issues/9
[missing feature]: https://github.com/rust-lang-nursery/reference/issues?q=is%3Aissue+is%3Aopen+label%3A%22Missing+Feature%22
[playpen]: https://play.rust-lang.org/
[unstable]: https://doc.rust-lang.org/nightly/unstable-book/
[RFC Status]: https://github.com/rust-lang-nursery/reference/projects/1
