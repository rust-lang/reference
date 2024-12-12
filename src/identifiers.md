# Identifiers

r[ident]

r[ident.syntax]
> **<sup>Lexer:<sup>**\
> IDENTIFIER_OR_KEYWORD :\
> &nbsp;&nbsp; &nbsp;&nbsp; XID_Start XID_Continue<sup>\*</sup>\
> &nbsp;&nbsp; | `_` XID_Continue<sup>+</sup>
>
> RAW_IDENTIFIER : `r#` IDENTIFIER_OR_KEYWORD <sub>*Except `crate`, `self`, `super`, `Self`*</sub>
>
> NON_KEYWORD_IDENTIFIER : IDENTIFIER_OR_KEYWORD <sub>*Except a [strict] or [reserved] keyword*</sub>
>
> IDENTIFIER :\
> NON_KEYWORD_IDENTIFIER | RAW_IDENTIFIER
>
> RESERVED_RAW_IDENTIFIER : `r#_`

<!-- When updating the version, update the UAX links, too. -->
r[ident.unicode]
Identifiers follow the specification in [Unicode Standard Annex #31][UAX31] for Unicode version 16.0, with the additions described below. Some examples of identifiers:

* `foo`
* `_identifier`
* `r#true`
* `Москва`
* `東京`

r[ident.profile]
The profile used from UAX #31 is:

* Start := [`XID_Start`], plus the underscore character (U+005F)
* Continue := [`XID_Continue`]
* Medial := empty

with the additional constraint that a single underscore character is not an identifier.

> **Note**: Identifiers starting with an underscore are typically used to indicate an identifier that is intentionally unused, and will silence the unused warning in `rustc`.

r[ident.keyword]
Identifiers may not be a [strict] or [reserved] keyword without the `r#` prefix described below in [raw identifiers](#raw-identifiers).

r[ident.zero-width-chars]
Zero width non-joiner (ZWNJ U+200C) and zero width joiner (ZWJ U+200D) characters are not allowed in identifiers.

r[ident.ascii-limitations]
Identifiers are restricted to the ASCII subset of [`XID_Start`] and [`XID_Continue`] in the following situations:

* [`extern crate`] declarations
* External crate names referenced in a [path]
* [Module] names loaded from the filesystem without a [`path` attribute]
* [`no_mangle`] attributed items
* Item names in [external blocks]

## Normalization

r[ident.normalization]

Identifiers are normalized using Normalization Form C (NFC) as defined in [Unicode Standard Annex #15][UAX15]. Two identifiers are equal if their NFC forms are equal.

[Procedural][proc-macro] and [declarative][mbe] macros receive normalized identifiers in their input.

## Raw identifiers

r[ident.raw]

r[ident.raw.intro]
A raw identifier is like a normal identifier, but prefixed by `r#`. (Note that
the `r#` prefix is not included as part of the actual identifier.)

r[ident.raw.allowed]
Unlike a normal identifier, a raw identifier may be any strict or reserved
keyword except the ones listed above for `RAW_IDENTIFIER`.

r[ident.raw.reserved]
It is an error to use the RESERVED_RAW_IDENTIFIER token `r#_` in order to avoid confusion with the [_WildcardPattern_].

[_WildcardPattern_]: patterns.md#wildcard-pattern
[`extern crate`]: items/extern-crates.md
[`no_mangle`]: abi.md#the-no_mangle-attribute
[`path` attribute]: items/modules.md#the-path-attribute
[`XID_Continue`]: http://unicode.org/cldr/utility/list-unicodeset.jsp?a=%5B%3AXID_Continue%3A%5D&abb=on&g=&i=
[`XID_Start`]:  http://unicode.org/cldr/utility/list-unicodeset.jsp?a=%5B%3AXID_Start%3A%5D&abb=on&g=&i=
[external blocks]: items/external-blocks.md
[mbe]: macros-by-example.md
[module]: items/modules.md
[path]: paths.md
[proc-macro]: procedural-macros.md
[reserved]: keywords.md#reserved-keywords
[strict]: keywords.md#strict-keywords
[UAX15]: https://www.unicode.org/reports/tr15/tr15-56.html
[UAX31]: https://www.unicode.org/reports/tr31/tr31-41.html
