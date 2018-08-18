# Editions

Before we get into the details of the language itself, we need to cover the
concept of an "edition."

An "edition" of Rust consists of a particular year. There are two editions of
Rust, currently:

* Rust 2015
* Rust 2018

Conforming compilers are expected to:

* Provide a flag allowing users to select an edition.
* Default to the 2015 edition if no edition is selected.

The reference follows this guideline; everything stated in this document is
part of the 2015 edition unless specifically marked otherwise.

## What editions can change

There are only two things a new edition can do:

* Change an existing deprecation into a hard error.
* Change an existing deprecation to deny by default, and leverage the
  corresponding lint setting to produce error messages as if the feature were
  removed entirely.

As a corollary to this, these changes can free up space for new features,
that would then require being tied to a particular edition. For example,
consider the desire to add a new (non-contextual) keyword. For a keyword to
be added, a previous edition must deprecate using that keyword as an
identifier. The new edition may then turn that into an error, freeing space
for the addition of the keyword.

Restricting what editions can do is important for both users and compiler
authors; if Rust were to change drastically between editions, it would be
difficult for users, who would need to switch between editions while using
Rust, and compilers would be much harder to implement.