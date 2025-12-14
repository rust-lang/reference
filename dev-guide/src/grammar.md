# Rust grammar

The Reference grammar is written in Markdown code blocks using a modified BNF-like syntax (with a blend of regex and other arbitrary things). The [`mdbook-spec`] extension parses these rules and converts them into a renderable format, including railroad diagrams.

The code block should have a lang string with the word `grammar`, a comma, and the category of the grammar, like this:

~~~
```grammar,items
ProductionName -> SomeExpression
```
~~~

The category is used to group similar productions on the grammar summary page in the appendix.

## Grammar syntax

The syntax for the grammar itself is similar to what is described in **[Notation]**, though there are some rendering differences.

A "root" production, marked with `@root`, is one that is not used in any other production.

The syntax for the grammar notation, described here using its own notation, is:

```
Grammar -> Production+

BACKTICK -> U+0060

LF -> U+000A

Production ->
    ( Comment LF )*
    `@root`? Name ` ->` Expression

Name -> <Alphanumeric or `_`>+

Expression -> Sequence (` `* `|` ` `* Sequence)*

Sequence ->
        (` `* AdornedExpr)* ` `* Cut
      | (` `* AdornedExpr)+

AdornedExpr -> ExprRepeat Suffix? Footnote?

Suffix -> ` _` <not underscore, unless in backtick>* `_`

Footnote -> `[^` ~[`]` LF]+ `]`

ExprRepeat ->
      Expr1 `?`
    | Expr1 `*?`
    | Expr1 `*`
    | Expr1 `+?`
    | Expr1 `+`
    | Expr1 `{` Range? `..` Range? `}`

Range -> [0-9]+

Expr1 ->
      Unicode
    | NonTerminal
    | Break
    | Comment
    | Terminal
    | Charset
    | Prose
    | Group
    | NegativeExpression

Unicode -> `U+` [`A`-`Z` `0`-`9`]4..4

NonTerminal -> Name

Break -> LF ` `+

Comment -> `//` ~[LF]+

Terminal -> BACKTICK ~[LF]+ BACKTICK

Charset -> `[` (` `* Characters)+ ` `* `]`

Characters ->
      CharacterRange
    | CharacterTerminal
    | CharacterName

CharacterRange -> BACKTICK <any char> BACKTICK `-` BACKTICK <any char> BACKTICK

CharacterTerminal -> Terminal

CharacterName -> Name

Prose -> `<` ~[`>` LF]+ `>`

Group -> `(` ` `* Expression ` `* `)`

NegativeExpression -> `~` ( Charset | Terminal | NonTerminal )

Cut -> `^` Sequence
```

The general format is a series of productions separated by blank lines. The expressions are as follows:

| Expression | Example | Description |
|------------|---------|-------------|
| Unicode | U+0060 | A single Unicode character. |
| NonTerminal | FunctionParameters | A reference to another production by name. |
| Break | | Used internally by the renderer to detect line breaks and indentation. |
| Comment | // Single line comment. | A comment extending to the end of the line. |
| Terminal | \`example\` | A sequence of exact characters, surrounded by backticks. |
| Charset | \[ \`A\`-\`Z\` \`0\`-\`9\` \`_\` \] | A choice from a set of characters, space-separated. There are three different forms. |
| CharacterRange | \[ \`A\`-\`Z\` \] | A range of characters; each character should be in backticks. |
| CharacterTerminal | \[ \`x\` \] | A single character, surrounded by backticks. |
| CharacterName | \[ LF \] | A nonterminal, referring to another production. |
| Prose | \<any ASCII character except CR\> | An English description of what should be matched, surrounded in angle brackets. |
| Group | (\`,\` Parameter)+ | Groups an expression for the purpose of precedence, such as applying a repetition operator to a sequence of other expressions. |
| NegativeExpression | ~\[\` \` LF\] | Matches anything except the given Charset, Terminal, or Nonterminal. |
| Cut | Expr1 ^ Expr2 \| Expr3 | The hard cut operator. Once the expressions preceding `^` in the sequence match, the rest of the sequence must match or parsing fails unconditionally --- no enclosing expression can backtrack past the cut point. |
| Sequence | \`fn\` Name Parameters | A sequence of expressions that must match in order. |
| Alternation | Expr1 \| Expr2 | Matches only one of the given expressions, separated by the vertical pipe character. |
| Suffix | \_except \[LazyBooleanExpression\]\_  | Adds a suffix to the previous expression to provide an additional English description, rendered in subscript. This can contain limited Markdown, but try to avoid anything except basics like links. |
| Footnote | \[^extern-safe\] | Adds a footnote, which can supply extra information that may be helpful to the user. The footnote itself should be defined outside of the code block like a normal Markdown footnote. |
| Optional | Expr? | The preceding expression is optional. |
| Repeat | Expr* | The preceding expression is repeated 0 or more times. |
| Repeat (non-greedy) | Expr*? | The preceding expression is repeated 0 or more times without being greedy. |
| RepeatPlus | Expr+ | The preceding expression is repeated 1 or more times. |
| RepeatPlus (non-greedy) | Expr+? | The preceding expression is repeated 1 or more times without being greedy. |
| RepeatRange | Expr{2..4} | The preceding expression is repeated between the range of times specified. Either bound can be excluded, which works just like Rust ranges. |

## Automatic linking

The [`mdbook-spec`] plugin automatically adds Markdown link definitions for all production names on every page. To link directly to a production name, simply surround it in square brackets, like `[ArrayExpression]`.

In some cases, there might be name collisions with the automatic linking of rule names. In that case, disambiguate with the `grammar-` prefix, such as `[Type][grammar-Type]`. The prefix can also be used when explicitness would aid clarity.

[`mdbook-spec`]: tooling/mdbook-spec.md
[Notation]: https://doc.rust-lang.org/nightly/reference/notation.html
