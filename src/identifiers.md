# Identifiers

> **<sup>Lexer:<sup>**\
> IDENTIFIER_OR_KEYWORD :\
> &nbsp;&nbsp; &nbsp;&nbsp; \[`a`-`z` `A`-`Z`]&nbsp;\[`a`-`z` `A`-`Z` `0`-`9` `_`]<sup>\*</sup>\
> &nbsp;&nbsp; | `_` \[`a`-`z` `A`-`Z` `0`-`9` `_`]<sup>+</sup>
>
> RAW_IDENTIFIER : `r#` IDENTIFIER_OR_KEYWORD <sub>*Except `crate`, `self`, `super`, `Self`*</sub>
>
> NON_KEYWORD_IDENTIFIER : IDENTIFIER_OR_KEYWORD <sub>*Except a [strict] or [reserved] keyword*</sub>
>
> IDENTIFIER :\
> NON_KEYWORD_IDENTIFIER | RAW_IDENTIFIER

An identifier is any nonempty ASCII string of the following form:

Either

* The first character is a letter.
* The remaining characters are alphanumeric or `_`.

Or

* The first character is `_`.
* The identifier is more than one character. `_` alone is not an identifier.
* The remaining characters are alphanumeric or `_`.

A raw identifier is like a normal identifier, but prefixed by `r#`. (Note that
the `r#` prefix is not included as part of the actual identifier.)
Unlike a normal identifier, a raw identifier may be any strict or reserved
keyword except the ones listed above for `RAW_IDENTIFIER`.

[strict]: keywords.md#strict-keywords
[reserved]: keywords.md#reserved-keywords
