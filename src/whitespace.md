r[lex.whitespace]
# Whitespace

r[whitespace.syntax]
```grammar,lexer
@root WHITESPACE ->
      U+0009 // Horizontal tab, `'\t'`
    | U+000A // Line feed, `'\n'`
    | U+000B // Vertical tab
    | U+000C // Form feed
    | U+000D // Carriage return, `'\r'`
    | U+0020 // Space, `' '`
    | U+0085 // Next line
    | U+200E // Left-to-right mark
    | U+200F // Right-to-left mark
    | U+2028 // Line separator
    | U+2029 // Paragraph separator

TAB -> U+0009 // Horizontal tab, `'\t'`

LF -> U+000A  // Line feed, `'\n'`

CR -> U+000D  // Carriage return, `'\r'`
```

r[lex.whitespace.intro]
Whitespace is any non-empty string containing only characters that have the [`Pattern_White_Space`] Unicode property.

r[lex.whitespace.token-sep]
Rust is a "free-form" language, meaning that all forms of whitespace serve only to separate _tokens_ in the grammar, and have no semantic significance.

r[lex.whitespace.replacement]
A Rust program has identical meaning if each whitespace element is replaced with any other legal whitespace element, such as a single space character.

[`Pattern_White_Space`]: https://www.unicode.org/reports/tr31/
