r[lex.whitespace]
# Whitespace

r[whitespace.syntax]
```grammar,lexer
@root WHITESPACE ->
    // end of line
      LF
    | U+000B // vertical tabulation
    | U+000C // form feed
    | CR
    | U+0085 // Unicode next line
    | U+2028 // Unicode LINE SEPARATOR
    | U+2029 // Unicode PARAGRAPH SEPARATOR
    // Ignorable Code Point
    | U+200E // Unicode LEFT-TO-RIGHT MARK
    | U+200F // Unicode RIGHT-TO-LEFT MARK
    // horizontal whitespace
    | TAB
    | U+0020  // space ' '

TAB -> U+0009  // horizontal tab ('\t')

LF -> U+000A  // line feed ('\n')

CR -> U+000D  // carriage return ('\r')
```

r[lex.whitespace.intro]
Whitespace is any non-empty string containing only characters that have the
[`Pattern_White_Space`] Unicode property.

r[lex.whitespace.token-sep]
Rust is a "free-form" language, meaning that all forms of whitespace serve only
to separate _tokens_ in the grammar, and have no semantic significance.

r[lex.whitespace.replacement]
A Rust program has identical meaning if each whitespace element is replaced
with any other legal whitespace element, such as a single space character.

[`Pattern_White_Space`]: https://www.unicode.org/reports/tr31/
