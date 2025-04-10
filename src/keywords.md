r[lex.keywords]
# Keywords

Rust divides keywords into three categories:

* [strict](#strict-keywords)
* [reserved](#reserved-keywords)
* [weak](#weak-keywords)

r[lex.keywords.strict]
## Strict keywords

r[lex.keywords.strict.intro]
These keywords can only be used in their correct contexts. They cannot
be used as the names of:

* [Items]
* [Variables] and function parameters
* Fields and [variants]
* [Type parameters]
* Lifetime parameters or [loop labels]
* [Macros] or [attributes]
* [Macro placeholders]
* [Crates]

r[lex.keywords.strict.list]
The following keywords are in all editions:

- `as`
- `break`
- `const`
- `continue`
- `crate`
- `else`
- `enum`
- `extern`
- `false`
- `fn`
- `for`
- `if`
- `impl`
- `in`
- `let`
- `loop`
- `match`
- `mod`
- `move`
- `mut`
- `pub`
- `ref`
- `return`
- `self`
- `Self`
- `static`
- `struct`
- `super`
- `trait`
- `true`
- `type`
- `unsafe`
- `use`
- `where`
- `while`

r[lex.keywords.strict.edition2018]
The following keywords were added beginning in the 2018 edition.

- `async`
- `await`
- `dyn`

r[lex.keywords.reserved]
## Reserved keywords

r[lex.keywords.reserved.intro]
These keywords aren't used yet, but they are reserved for future use. They have
the same restrictions as strict keywords. The reasoning behind this is to make
current programs forward compatible with future versions of Rust by forbidding
them to use these keywords.

r[lex.keywords.reserved.list]
- `abstract`
- `become`
- `box`
- `do`
- `final`
- `macro`
- `override`
- `priv`
- `typeof`
- `unsized`
- `virtual`
- `yield`

r[lex.keywords.reserved.edition2018]
The following keywords are reserved beginning in the 2018 edition.

- `try`

r[lex.keywords.reserved.edition2024]
The following keywords are reserved beginning in the 2024 edition.

- `gen`

r[lex.keywords.weak]
## Weak keywords

r[lex.keywords.weak.intro]
These keywords have special meaning only in certain contexts. For example, it
is possible to declare a variable or method with the name `union`.

- `'static`
- `macro_rules`
- `raw`
- `safe`
- `union`

r[lex.keywords.weak.macro_rules]
* `macro_rules` is used to create custom [macros].

r[lex.keywords.weak.union]
* `union` is used to declare a [union] and is only a keyword when used in a
  union declaration.

r[lex.keywords.weak.lifetime-static]
* `'static` is used for the static lifetime and cannot be used as a [generic
  lifetime parameter] or [loop label]

  ```compile_fail
  // error[E0262]: invalid lifetime parameter name: `'static`
  fn invalid_lifetime_parameter<'static>(s: &'static str) -> &'static str { s }
  ```

r[lex.keywords.weak.safe]
* `safe` is used for functions and statics, which has meaning in [external blocks].

r[lex.keywords.weak.raw]
* `raw` is used for [raw borrow operators], and is only a keyword when matching a raw borrow operator form (such as `&raw const expr` or `&raw mut expr`).

r[lex.keywords.weak.dyn.edition2018]
> [!EDITION-2018]
> In the 2015 edition, [`dyn`] is a keyword when used in a type position followed by a path that does not start with `::` or `<`, a lifetime, a question mark, a `for` keyword or an opening parenthesis.
>
> Beginning in the 2018 edition, `dyn` has been promoted to a strict keyword.

[items]: items.md
[Variables]: variables.md
[Type parameters]: types/parameters.md
[loop labels]: expressions/loop-expr.md#loop-labels
[Macros]: macros.md
[attributes]: attributes.md
[Macro placeholders]: macros-by-example.md
[Crates]: crates-and-source-files.md
[union]: items/unions.md
[variants]: items/enumerations.md
[`dyn`]: types/trait-object.md
[loop label]: expressions/loop-expr.md#loop-labels
[generic lifetime parameter]: items/generics.md
[external blocks]: items/external-blocks.md
[raw borrow operators]: expressions/operator-expr.md#raw-borrow-operators
