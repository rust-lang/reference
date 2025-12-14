r[notation]
# Notation

r[notation.grammar]
## Grammar

r[notation.grammar.syntax]

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
| ^                 | `b'` ^ ASCII_FOR_CHAR         | The rest of the sequence must match or parsing fails unconditionally ([hard cut operator]) |
| U+xxxx            | U+0060                        | A single unicode character                |
| \<text\>          | \<any ASCII char except CR\>  | An English description of what should be matched |
| Rule <sub>suffix</sub> | IDENTIFIER_OR_KEYWORD <sub>_except `crate`_</sub> | A modification to the previous rule |
| // Comment. | // Single line comment. | A comment extending to the end of the line. |

Sequences have a higher precedence than `|` alternation.

r[notation.grammar.cut]
### The hard cut operator

The grammar uses ordered alternation: the parser tries alternatives left to right and takes the first that matches. If an alternative fails partway through a sequence, the parser normally backtracks and tries the next alternative. The cut operator (`^`) prevents this. Once every expression to the left of `^` in a sequence has matched, the rest of the sequence must match or parsing fails unconditionally.

Mizushima et al. introduced [cut operators][cut operator paper] to parsing expression grammars. In the PEG literature, a *soft cut* prevents backtracking only within the immediately enclosing ordered choice --- outer choices can still recover. A *hard cut* prevents all backtracking past the cut point; failure is definitive. The `^` used in this grammar is a hard cut.

The hard cut operator is necessary because some tokens in Rust begin with a prefix that is itself a valid token. For example, `c"` begins a C string literal, but `c` alone is a valid identifier. Without the cut, if `c"\0"` failed to lex as a C string literal (because null bytes are not allowed in C strings), the parser could backtrack and lex it as two tokens: the identifier `c` and the string literal `"\0"`. The [cut after `c"`] prevents this --- once the opening delimiter is recognized, the parser cannot go back. The same reasoning applies to [byte literals], [byte string literals], [raw string literals], and other literals with prefixes that are themselves valid tokens.

r[notation.grammar.string-tables]
### String table productions

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

r[notation.grammar.visualizations]
### Grammar visualizations

Below each grammar block is a button to toggle the display of a [syntax diagram]. A square element is a non-terminal rule, and a rounded rectangle is a terminal.

[binary operators]: expressions/operator-expr.md#arithmetic-and-logical-binary-operators
[byte literals]: tokens.md#r-lex.token.byte.syntax
[byte string literals]: tokens.md#r-lex.token.str-byte.syntax
[cut after `c"`]: tokens.md#r-lex.token.str-c.syntax
[cut operator paper]: https://kmizu.github.io/papers/paste513-mizushima.pdf
[hard cut operator]: notation.md#the-hard-cut-operator
[keywords]: keywords.md
[raw string literals]: tokens.md#r-lex.token.literal.str-raw.syntax
[syntax diagram]: https://en.wikipedia.org/wiki/Syntax_diagram
[tokens]: tokens.md
[unary operators]: expressions/operator-expr.md#borrow-operators
