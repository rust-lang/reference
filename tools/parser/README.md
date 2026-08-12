# Reference grammar parser

This is a primitive interpreter that parses an input using the Reference grammar and can generate tokens or a generic tree representation of that source.

## Overview

The parser itself is fairly straightforward as it uses the Reference productions to drive an interpreter to parse some input into a tree of nodes.

There are some hard-coded handlers for some of the English-based rules such as the suffixes. Ideally the grammar should be changed to remove those and use parseable expressions (like negative lookahead).

## To lex or not to lex

The tooling is currently designed to keep lexing separate from parsing. I'm still uncertain if this is the right thing to do. It adds some complexity. For example, the parser has a `Source` abstraction so that its input can either be a string of bytes (which is used for lexing) or a sequence of tokens. An alternative is to drop the separate lexing phase, and instead somehow automatically insert "whitespace or comments" in between each expression in the non-lexer productions. However, this is not simple and itself would add its own complexity. It might be worth exploring, though.

## Token splitting

Token splitting is not implemented. That is, when the parser sees `Option<<Vec<i32>`, it will need to split the `<<` into two `<` tokens.

This is a primary blocker for getting tree-based parsing working well enough to parse a typical Rust file.

This is not an easy problem if we want to have parity with `rustc` because `rustc` does not always split tokens. It might be sufficient for a naive approach to split everything, and hope that there aren't any test cases where they diverge. Unfortunately this could cause problems with the permutation or fuzzing-based testing. Or, we could hard-code where `rustc` does split.

An alternative approach to splitting would be to change the Reference grammar so that it uses the proc-macro model where tokens keep track of their "spacing" so that you know if you can join two tokens (like `:` `:` into `::`). I believe there is desire to move `rustc` itself to this model, but the work there hasn't been done. This in itself would add some complexity, though. The Reference would also probably need to be clearer about how tokens are translated between the two models (because `macro_rules` uses the joined model whereas proc-macros use the split model). I'm not sure which approach will be easier or better.

See https://github.com/rust-lang/rust/issues/152398 for my analysis on this.
