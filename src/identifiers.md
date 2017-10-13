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

[^non_ascii_idents] Non-ASCII characters in identifiers are currently feature-gated. See [issue #28979](https://github.com/rust-lang/rust/issues/28979).
