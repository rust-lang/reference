# Notational conventions

## Conventions

Like all technical books, this book has certain conventions in how it displays information. These conventions are documented here.

* Statements that define a term contain that term in *italics*. Whenever that term is used outside of that chapter, it is usually a link to the section that has this definition.

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
