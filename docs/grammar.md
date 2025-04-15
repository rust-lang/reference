# Grammar

The Reference grammar is written in markdown code blocks using a modified BNF-like syntax (with a blend of regex and other arbitrary things). The `mdbook-spec` extension parses these rules and converts them to a renderable format, including railroad diagrams.

The code block should have a lang string with the word "grammar", a comma, and the category of the grammar, like this:

~~~
```grammar,items
ProductionName -> SomeExpression
```
~~~

The category is used to group similar productions on the grammar summary page in the appendix.

## Grammar syntax

The syntax for the grammar itself is pretty close to what is described in the [Notation chapter](../src/notation.md), though there are some rendering differences.

A "root" production, marked with `@root`, is one that is not used in any other production.

The syntax for the grammar itself (written in itself, hopefully that's not too confusing) is:

```
Grammar -> Production+

BACKTICK -> U+0060

LF -> U+000A

Production -> `@root`? Name ` ->` Expression

Name -> <Alphanumeric or `_`>+

Expression -> Sequence (` `* `|` ` `* Sequence)*

Sequence -> (` `* AdornedExpr)+

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
    | Terminal
    | Charset
    | Prose
    | Group
    | NegativeExpression

Unicode -> `U+` [`A`-`Z` `0`-`9`]4..4

NonTerminal -> Name

Break -> LF ` `+

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
```

The general format is a series of productions separated by blank lines. The expressions are:

| Expression | Example | Description |
|------------|---------|-------------|
| Unicode | U+0060 | A single unicode character. |
| NonTerminal | FunctionParameters | A reference to another production by name. |
| Break | | This is used internally by the renderer to detect line breaks and indentation. |
| Terminal | \`example\` | This is a sequence of exact characters, surrounded by backticks |
| Charset | [ \`A\`-\`Z\` \`0\`-\`9\` \`_\` ] | A choice from a set of characters, space separated. There are three different forms. |
| CharacterRange | [ \`A\`-\`Z\` ] | A range of characters, each character should be in backticks.
| CharacterTerminal | [ \`x\` ] | A single character, surrounded by backticks. |
| CharacterName | [ LF ] | A nonterminal, referring to another production. |
| Prose | \<any ASCII character except CR\> | This is an English description of what should be matched, surrounded in angle brackets. |
| Group | (\`,\` Parameter)+ | This groups an expression for the purpose of precedence, such as applying a repetition operator to a sequence of other expressions.
| NegativeExpression | ~[\` \` LF] | Matches anything except the given Charset, Terminal, or Nonterminal. |
| Sequence | \`fn\` Name Parameters | A sequence of expressions, where they must match in order. |
| Alternation | Expr1 \| Expr2 | Matches only one of the given expressions, separated by the vertical pipe character. |
| Suffix | \_except \[LazyBooleanExpression\]\_  | This adds a suffix to the previous expression to provide an additional English description to it, rendered in subscript. This can have limited markdown, but try to avoid anything except basics like links. |
| Footnote | \[^extern-safe\] | This adds a footnote, which can supply some extra information that may be helpful to the user. The footnote itself should be defined outside of the code block like a normal markdown footnote. |
| Optional | Expr? | The preceding expression is optional. |
| Repeat | Expr* | The preceding expression is repeated 0 or more times. |
| Repeat (non-greedy) | Expr*? | The preceding expression is repeated 0 or more times without being greedy. |
| RepeatPlus | Expr+ | The preceding expression is repeated 1 or more times. |
| RepeatPlus (non-greedy) | Expr+? | The preceding expression is repeated 1 or more times without being greedy. |
| RepeatRange | Expr{2..4} | The preceding expression is repeated between the range of times specified. Either bounds can be excluded, which works just like Rust ranges. |

## Automatic linking

The plugin automatically adds markdown link definitions for all the production names on every page. If you want to link directly to a production name, all you need to do is surround it in square brackets, like `[ArrayExpression]`.

In some cases there might be name collisions with the automatic linking of rule names. In that case, disambiguate with the `grammar-` prefix, such as `[Type][grammar-Type]`. You can also do that if you just feel like being more explicit.
