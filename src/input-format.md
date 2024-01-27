# Input format

This chapter describes how a source file is interpreted as a sequence of tokens.

See [Crates and source files] for a description of how programs are organised into files.

## Source encoding

Each source file is interpreted as a sequence of Unicode characters encoded in UTF-8.
It is an error if the file is not valid UTF-8.

## CRLF normalization

Each pair of characters `U+000D` (CR) immediately followed by `U+000A` (LF) is replaced by a single `U+000A` (LF).

Other occurrences of the character `U+000D` (CR) are left in place (they are treated as [whitespace]).

## Tokenization

The resulting sequence of characters is then converted into tokens as described in the remainder of this chapter.

[Crates and source files]: crates-and-source-files.md
