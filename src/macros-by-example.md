# Macros By Example

> **<sup>Syntax</sup>**\
> _MacroRulesDefinition_ :\
> &nbsp;&nbsp; `macro_rules` `!` [IDENTIFIER] _MacroRulesDef_
>
> _MacroRulesDef_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; `(` _MacroRules_ `)` `;`\
> &nbsp;&nbsp; | `[` _MacroRules_ `]` `;`\
> &nbsp;&nbsp; | `{` _MacroRules_ `}`
>
> _MacroRules_ :\
> &nbsp;&nbsp; _MacroRule_ ( `;` _MacroRule_ )<sup>\*</sup> `;`<sup>?</sup>
>
> _MacroRule_ :\
> &nbsp;&nbsp; _MacroMatcher_ `=>` _MacroTranscriber_
>
> _MacroMatcher_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; `(` _MacroMatch_<sup>\*</sup> `)`\
> &nbsp;&nbsp; | `[` _MacroMatch_<sup>\*</sup> `]`\
> &nbsp;&nbsp; | `{` _MacroMatch_<sup>\*</sup> `}`
>
> _MacroMatch_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; [_Token_]<sub>_except $ and delimiters_</sub>\
> &nbsp;&nbsp; | _MacroMatcher_\
> &nbsp;&nbsp; | `$` [IDENTIFIER] `:` _MacroFragSpec_\
> &nbsp;&nbsp; | `$` `(` _MacroMatch_<sup>+</sup> `)` _MacroRepSep_<sup>?</sup> _MacroKleeneOp_
>
> _MacroFragSpec_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; `block` | `expr` | `ident` | `item` | `lifetime` | `literal`\
> &nbsp;&nbsp; | `meta` | `pat` | `path` | `stmt` | `tt` | `ty` | `vis`
>
> _MacroRepSep_ :\
> &nbsp;&nbsp; [_Token_]<sub>_except delimiters and kleene operators_</sub>
>
> _MacroKleeneOp_<sub>2015</sub> :\
> &nbsp;&nbsp; `*` | `+`
>
> _MacroKleeneOp_<sub>2018+</sub> :\
> &nbsp;&nbsp; `*` | `+` | `?`
>
> _MacroTranscriber_ :\
> &nbsp;&nbsp; [_DelimTokenTree_]

`macro_rules` allows users to define syntax extension in a declarative way.  We
call such extensions "macros by example" or simply "macros".

Macros can expand to expressions, statements, items, types, or patterns.

The macro expander looks up macro invocations by name, and tries each macro
rule in turn. It transcribes the first successful match. Matching and
transcription are closely related to each other, and we will describe them
together.

The macro expander matches and transcribes every token that does not begin with
a `$` literally, including delimiters. For parsing reasons, delimiters must be
balanced, but they are otherwise not special.

In the matcher, `$` _name_ `:` _designator_ matches the nonterminal in the Rust
syntax named by _designator_. Valid designators are:

* `item`: an [_Item_]
* `block`: a [_BlockExpression_]
* `stmt`: a [_Statement_] without the trailing semicolon
* `pat`: a [_Pattern_]
* `expr`: an [_Expression_]
* `ty`: a [_Type_]
* `ident`: an [IDENTIFIER_OR_KEYWORD]
* `path`: a [_TypePath_] style path
* `tt`: a [_TokenTree_]&nbsp;(a single [token] or tokens in matching delimiters `()`, `[]`, or `{}`)
* `meta`: a [_MetaItem_], the contents of an attribute
* `lifetime`: a [LIFETIME_TOKEN]
* `vis`: a [_Visibility_] qualifier
* `literal`: matches `-`<sup>?</sup>[_LiteralExpression_]

[IDENTIFIER]: identifiers.md
[IDENTIFIER_OR_KEYWORD]: identifiers.md
[LIFETIME_TOKEN]: tokens.md#lifetimes-and-loop-labels
[_BlockExpression_]: expressions/block-expr.md
[_Expression_]: expressions.md
[_Item_]: items.md
[_LiteralExpression_]: expressions/literal-expr.md
[_MetaItem_]: attributes.md
[_Pattern_]: patterns.md
[_Statement_]: statements.md
[_TokenTree_]: macros.md#macro-invocation
[_TypePath_]: paths.md#paths-in-types
[_Type_]: types.md#type-expressions
[_Visibility_]: visibility-and-privacy.md
[token]: tokens.md

In the transcriber, the
designator is already known, and so only the name of a matched nonterminal comes
after the dollar sign.

In both the matcher and transcriber, the Kleene star-like operator indicates
repetition. The Kleene star operator consists of `$` and parentheses,
optionally followed by a separator token, followed by `*`, `+`, or `?`. `*`
means zero or more repetitions; `+` means _at least_ one repetition; `?` means
at most one repetition. The parentheses are not matched or transcribed. On the
matcher side, a name is bound to _all_ of the names it matches, in a structure
that mimics the structure of the repetition encountered on a successful match.
The job of the transcriber is to sort that structure out. Also, `?`, unlike `*`
and `+`, does _not_ allow a separator, since one could never match against it
anyway.

> **Edition Differences**: The `?` Kleene operator did not exist before the
> 2018 edition.

> **Edition Differences**: Prior to the 2018 Edition, `?` was an allowed
> separator token, rather than a Kleene operator. It is no longer allowed as a
> separator as of the 2018 edition. This avoids ambiguity with the `?` Kleene
> operator.

The rules for transcription of these repetitions are called "Macro By Example".
Essentially, one "layer" of repetition is discharged at a time, and all of them
must be discharged by the time a name is transcribed. Therefore, `( $( $i:ident
),* ) => ( $i )` is an invalid macro, but `( $( $i:ident ),* ) => ( $( $i:ident
),*  )` is acceptable (if trivial).

When Macro By Example encounters a repetition, it examines all of the `$`
_name_ s that occur in its body. At the "current layer", they all must repeat
the same number of times, so ` ( $( $i:ident ),* ; $( $j:ident ),* ) => ( $(
($i,$j) ),* )` is valid if given the argument `(a,b,c ; d,e,f)`, but not
`(a,b,c ; d,e)`. The repetition walks through the choices at that layer in
lockstep, so the former input transcribes to `(a,d), (b,e), (c,f)`.

Nested repetitions are allowed.

### Parsing limitations

The parser used by the macro system is reasonably powerful, but the parsing of
Rust syntax is restricted in two ways:

1. Macro definitions are required to include suitable separators after parsing
   expressions and other bits of the Rust grammar. This implies that
   a macro definition like `$i:expr [ , ]` is not legal, because `[` could be part
   of an expression. A macro definition like `$i:expr,` or `$i:expr;` would be legal,
   however, because `,` and `;` are legal separators. See [RFC 550] for more information.
   Specifically:

   * `expr` and `stmt` may only be followed by one of `=>`, `,`, or `;`.
   * `pat` may only be followed by one of `=>`, `,`, `=`, `|`, `if`, or `in`.
   * `path` and `ty` may only be followed by one of `=>`, `,`, `=`, `|`, `;`,
     `:`, `>`, `>>`, `[`, `{`, `as`, `where`, or a macro variable of `block`
     fragment type.
   * `vis` may only be followed by one of `,`, `priv`, a raw identifier, any
     token that can begin a type, or a macro variable of `ident`, `ty`, or
     `path` fragment type.
   * All other fragment types have no restrictions.

2. The parser must have eliminated all ambiguity by the time it reaches a `$`
   _name_ `:` _designator_. This requirement most often affects name-designator
   pairs when they occur at the beginning of, or immediately after, a `$(...)*`;
   requiring a distinctive token in front can solve the problem. For example:

   ```rust
   // The matcher `$($i:ident)* $e:expr` would be ambiguous because the parser
   // would be forced to choose between an identifier or an expression. Use some
   // token to distinguish them.
   macro_rules! example {
       ($(I $i:ident)* E $e:expr) => { ($($i)-*) * $e };
   }
   let foo = 2;
   let bar = 3;
   // The following expands to `(foo - bar) * 5`
   example!(I foo I bar E 5);
   ```

[RFC 550]: https://github.com/rust-lang/rfcs/blob/master/text/0550-macro-future-proofing.md
[_DelimTokenTree_]: macros.md
[_Token_]: tokens.md
