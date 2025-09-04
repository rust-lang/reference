r[lex.whitespace]
# Whitespace

r[whitespace.syntax]
```grammar,lexer
@root WHITESPACE ->
      END_OF_LINE
    | IGNORABLE_CODE_POINT
    | HORIZONTAL_WHITESPACE

TAB -> HORIZONTAL_TAB

LF -> LINE_FEED

CR -> CARRIAGE_RETURN

END_OF_LINE ->
      LINE_FEED
    | VERTICAL_TAB
    | FORM_FEED
    | CARRIAGE_RETURN
    | NEXT_LINE
    | LINE_SEPARATOR
    | PARAGRAPH_SEPARATOR

LINE_FEED -> U+000A

VERTICAL_TAB -> U+000B

FORM_FEED -> U+000C

CARRIAGE_RETURN -> U+000D

NEXT_LINE -> U+0085

LINE_SEPARATOR -> U+2028

PARAGRAPH_SEPARATOR -> U+2029

IGNORABLE_CODE_POINT ->
      LEFT_TO_RIGHT_MARK
    | RIGHT_TO_LEFT_MARK

LEFT_TO_RIGHT_MARK -> U+200E

RIGHT_TO_LEFT_MARK -> U+200F

HORIZONTAL_WHITESPACE ->
      HORIZONTAL_TAB
    | SPACE

HORIZONTAL_TAB -> U+0009

SPACE -> U+0020
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
