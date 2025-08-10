r[notation]
# Notation

## Grammar

The following notations are used by the *Lexer* and *Syntax* grammar snippets:

| Notation          | Examples                      | Meaning                                   |
|-------------------|-------------------------------|-------------------------------------------|
| CAPITAL           | KW_IF, INTEGER_LITERAL        | A token produced by the lexer             |
| _ItalicCamelCase_ | _LetStatement_, _Item_        | A syntactical production                  |
| `string`          | `x`, `while`, `*`             | The exact character(s)                    |
| x<sup>?</sup>     | `pub`<sup>?</sup>             | An optional item                          |
| x<sup>\*</sup>    | _OuterAttribute_<sup>\*</sup> | 0 or more of x                            |
| x<sup>+</sup>     |  _MacroMatch_<sup>+</sup>     | 1 or more of x                            |
| x<sup>a..b</sup>  | HEX_DIGIT<sup>1..6</sup>      | a to b repetitions of x                   |
| Rule1 Rule2       | `fn` _Name_ _Parameters_      | Sequence of rules in order                |
| \|                | `u8` \| `u16`, Block \| Item  | Either one or another                     |
| \[ ]               | \[`b` `B`]                     | Any of the characters listed              |
| \[ - ]             | \[`a`-`z`]                     | Any of the characters in the range        |
| ~\[ ]              | ~\[`b` `B`]                    | Any characters, except those listed       |
| ~`string`         | ~`\n`, ~`*/`                  | Any characters, except this sequence      |
| ( )               | (`,` _Parameter_)<sup>?</sup> | Groups items                              |
| U+xxxx            | U+0060                        | A single unicode character                |
| \<text\>          | \<any ASCII char except CR\>  | An English description of what should be matched |
| Rule <sub>suffix</sub> | IDENTIFIER_OR_KEYWORD <sub>_except `crate`_</sub> | A modification to the previous rule |

Sequences have a higher precedence than `|` alternation.

## String table productions

Some rules in the grammar &mdash; notably [unary operators], [binary
operators], and [keywords] &mdash; are given in a simplified form: as a listing
of printable strings. These cases form a subset of the rules regarding the
[token][tokens] rule, and are assumed to be the result of a lexical-analysis
phase feeding the parser, driven by a <abbr title="Deterministic Finite
Automaton">DFA</abbr>, operating over the disjunction of all such string table
entries.

When such a string in `monospace` font occurs inside the grammar,
it is an implicit reference to a single member of such a string table
production. See [tokens] for more information.

## Grammar visualizations

Below each grammar block is a button to toggle the display of a [syntax diagram]. A square element is a non-terminal rule, and a rounded rectangle is a terminal.

[syntax diagram]: https://en.wikipedia.org/wiki/Syntax_diagram

## Common productions

The following are common definitions used in the grammar.

r[input.syntax]
```grammar,lexer
@root CHAR -> <a Unicode scalar value>

NUL -> U+0000

TAB -> U+0009

LF -> U+000A

CR -> U+000D
```

r[notation.tools]
## Common tools in examples

In code examples throughout this book, we use certain tools and patterns which we document below.

r[notation.tools.prove]
### `prove!`

```rust,ignore
macro_rules! prove { /* .. */ }
```

Given a set of antecedent type system predicates, assert that a set of consequent predicates are (`=>`) or are not (`?=>`) proven by rustc.

The macro accepts a set of parameters in a `for<..>` *binder*. These parameters are put into scope for the *antecedent* and *consequent*. When using the `=>` rule, the macro asserts that rustc can prove the consequent given the antecedent. When using the `?=>` rule, the macro asserts that rustc does not prove the consequent give the antecedent.

For example, if we say `prove! { for<'a, 'b, 'c> { 'a: 'b, 'b: 'c } => { 'a: 'c } }`, read that as, "for all lifetimes `'a, 'b, 'c`, assert that `'a: 'b` and `'b: 'c` implies `'a: 'c`".

Similarly, if we say `prove! { for<T> { T: ?Sized } ?=> { T: Sized } }`, we mean "for all types `T`, assert that if the `Sized` bound on `T` is relaxed then we do not prove `T: Sized`".

When asserting what is proven by rustc, lifetimes are taken into account. However, when asserting what is not proven by rustc, lifetime bounds are ignored and treated as it they always hold, so asserting lifetime relationships in the consequent with the `?=>` rule will produce misleading results.

#### Syntax

```grammar,notation
@root ProveBody ->
    ( `for<` ProveParams? `>` )? `{` ProveAntecedents? `}`
    ( `=>` | `?=>` ) `{` ProveConsequents? `}`

ProveParams -> (
        ProveLtParams ( `,` ProveTyParams )?
      | ProveTyParams
    ) `,`?

ProveLtParams -> Lifetime ( `,` Lifetime )*

ProveTyParams -> IDENTIFIER ( `,` IDENTIFIER )*

ProveAntecedents -> WhereClauseItem ( `,` WhereClauseItem )*  `,`?

ProveConsequents -> WhereClauseItem ( `,` WhereClauseItem )*  `,`?
```

<!--
For implementation simplicity, somewhat more is (incorrectly) accepted than what is described by this grammar.
-->

#### Examples

```rust
{{#rustdoc_include notation/prove.rs:-1 }}
// Assert that rustc proves that `U: From<T>` implies `T: Into<U>`.
prove! { for<T, U> { U: From<T> } => { T: Into<U> } }
//~^               ~~~~~~~~~~~~~~    ~~~~~~~~~~~~~~ Consequent.
//~|                 Antecedent.
prove! { {} => { u32: Into<u64> } }
//~^     ~~ It's OK for the antecedent to be empty.
prove! { for<'a, 'b, 'c> { 'a: 'b, 'b: 'c } => { 'a: 'c } }
//~^                     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//~|                           Transitive property.
prove! { for<T> { T: ?Sized } ?=> { T: Sized } }
//~^                          ~~~
//~| Asserts that rustc does not make this implication.
```

```rust,compile_fail
{{#rustdoc_include notation/prove.rs:-1 }}
// This is an error as it does not logically hold.
prove! { for<'a, 'b> {} => { 'a: 'b } } //~ ERROR
```

```rust,compile_fail
{{#rustdoc_include notation/prove.rs:-1 }}
// This is an error as the `?=>` rule treats lifetime bounds as
// always holding.
prove! { for<'a, 'b> {} ?=> { 'a: 'b } } //~ ERROR
```

[binary operators]: expressions/operator-expr.md#arithmetic-and-logical-binary-operators
[keywords]: keywords.md
[tokens]: tokens.md
[unary operators]: expressions/operator-expr.md#borrow-operators
