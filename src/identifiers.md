# Identifiers

> **<sup>Lexer:<sup>**  
> IDENTIFIER :  
> &nbsp;&nbsp; &nbsp;&nbsp; [`a`-`z` `A`-`Z`]&nbsp;[`a`-`z` `A`-`Z` `0`-`9` `_`]<sup>\*</sup>  
> &nbsp;&nbsp; | `_` [`a`-`z` `A`-`Z` `0`-`9` `_`]<sup>+</sup>  

An identifier is any nonempty ASCII[^non_ascii_idents] string of the following form:

Either

   * The first character is a letter
   * The remaining characters are alphanumeric or `_`

Or

   * The first character is `_`
   * The identifier is more than one character, `_` alone is not an identifier
   * The remaining characters are alphanumeric or `_`

[keywords]: keywords.html

[^non_ascii_idents] Non-ASCII characters in identifiers are currently feature-gated. See [issue #28979](https://github.com/rust-lang/rust/issues/28979).

<!-- When non-ascii identifiers are actually supported:

 > **<sup>Lexer:<sup>**  
 > IDENTIFIER :  
 > &nbsp;&nbsp; &nbsp;&nbsp; XID_start XID_continue<sup>\*</sup>  
 > &nbsp;&nbsp; | `_` XID_continue<sup>+</sup>  

An identifier is any nonempty Unicode string of the following form:

Either

   * The first character has property [`XID_start`]
   * The remaining characters have property [`XID_continue`]

Or

   * The first character is `_`
   * The identifier is more than one character, `_` alone is not an identifier
   * The remaining characters have property [`XID_continue`]

that does _not_ occur in the set of [keywords].

> **Note**: [`XID_start`] and [`XID_continue`] as character properties cover the
> character ranges used to form the more familiar C and Java language-family
> identifiers.
   
[`XID_start`]:  http://unicode.org/cldr/utility/list-unicodeset.jsp?a=%5B%3AXID_Start%3A%5D&abb=on&g=&i=
[`XID_continue`]: http://unicode.org/cldr/utility/list-unicodeset.jsp?a=%5B%3AXID_Continue%3A%5D&abb=on&g=&i=

-->
