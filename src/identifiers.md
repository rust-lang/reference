# Identifiers

> **<sup>Lexer:<sup>**\
> IDENTIFIER_OR_KEYWORD :\
> &nbsp;&nbsp; &nbsp;&nbsp; XID_start XID_continue<sup>\*</sup>\
> &nbsp;&nbsp; | `_` XID_continue<sup>+</sup>
>
> RAW_IDENTIFIER : `r#` IDENTIFIER_OR_KEYWORD <sub>*Except `crate`, `self`, `super`, `Self`*</sub>
>
> NON_KEYWORD_IDENTIFIER : IDENTIFIER_OR_KEYWORD <sub>*Except a [strict] or [reserved] keyword*</sub>
>
> IDENTIFIER :\
> NON_KEYWORD_IDENTIFIER | RAW_IDENTIFIER

An identifier is any nonempty Unicode string of the following form:

Either

* The first character has property [`XID_start`].
* The remaining characters have property [`XID_continue`].

Or

* The first character is `_`.
* The identifier is more than one character. `_` alone is not an identifier.
* The remaining characters have property [`XID_continue`].

> **Note**: [`XID_start`] and [`XID_continue`] as character properties cover the
> character ranges used to form the more familiar C and Java language-family
> identifiers.

A raw identifier is like a normal identifier, but prefixed by `r#`. (Note that
the `r#` prefix is not included as part of the actual identifier.)
Unlike a normal identifier, a raw identifier may be any strict or reserved
keyword except the ones listed above for `RAW_IDENTIFIER`.

[strict]: keywords.md#strict-keywords
[reserved]: keywords.md#reserved-keywords
[`XID_start`]:  http://unicode.org/cldr/utility/list-unicodeset.jsp?a=%5B%3AXID_Start%3A%5D&abb=on&g=&i=
[`XID_continue`]: http://unicode.org/cldr/utility/list-unicodeset.jsp?a=%5B%3AXID_Continue%3A%5D&abb=on&g=&i=
