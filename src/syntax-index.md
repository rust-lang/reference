# Syntax index

This appendix provides an index of tokens and common forms with links to where those elements are defined.

## Keywords

## Operators and punctuation

| Symbol | Name        | Usage |
|--------|-------------|-------|
| `+`    | Plus        | [addition][arith], [trait bounds], [macro Kleene matcher][macros]
| `-`    | Minus       | [subtraction][arith], [negation]
| `*`    | Star        | [multiplication][arith], [dereference], [raw pointers], [macro Kleene matcher][macros], [use wildcards]
| `/`    | Slash       | [division][arith]
| `%`    | Percent     | [remainder][arith]
| `^`    | Caret       | [bitwise and logical XOR][arith]
| `!`    | Not         | [bitwise and logical NOT][negation], [macro calls][macros], [inner attributes][attributes], [never type], [negative impls]
| `&`    | And         | [bitwise and logical AND][arith], [borrow], [references], [reference patterns]
| <code>\|</code> | Or | [bitwise and logical OR][arith], [closures], patterns in [match], [if let], [while let]
| `&&`   | AndAnd      | [lazy AND][lazy-bool], [borrow], [references], [reference patterns]
| <code>\|\|</code> | OrOr | [lazy OR][lazy-bool], [closures]
| `<<`   | Shl         | [shift left][arith], [nested generics][generics]
| `>>`   | Shr         | [shift right][arith], [nested generics][generics]
| `+=`   | PlusEq      | [addition assignment][compound]
| `-=`   | MinusEq     | [subtraction assignment][compound]
| `*=`   | StarEq      | [multiplication assignment][compound]
| `/=`   | SlashEq     | [division assignment][compound]
| `%=`   | PercentEq   | [remainder assignment][compound]
| `^=`   | CaretEq     | [bitwise XOR assignment][compound]
| `&=`   | AndEq       | [bitwise AND assignment][compound]
| <code>\|=</code> | OrEq | [bitwise OR assignment][compound]
| `<<=`  | ShlEq       | [shift left assignment][compound]
| `>>=`  | ShrEq       | [shift right assignment][compound], [nested generics][generics]
| `=`    | Eq          | [assignment], [attributes], various type definitions
| `==`   | EqEq        | [equal][comparison]
| `!=`   | Ne          | [not equal][comparison]
| `>`    | Gt          | [greater than][comparison], [generics], [paths]
| `<`    | Lt          | [less than][comparison], [generics], [paths]
| `>=`   | Ge          | [greater than or equal to][comparison], [generics]
| `<=`   | Le          | [less than or equal to][comparison]
| `@`    | At          | [subpattern binding]
| `.`    | Dot         | [field access][field], [tuple index]
| `..`   | DotDot      | [range][range], [struct expressions], [patterns], [range patterns][rangepat]
| `...`  | DotDotDot   | [variadic functions][extern], [range patterns]
| `..=`  | DotDotEq    | [inclusive range][range], [range patterns]
| `,`    | Comma       | various separators
| `;`    | Semi        | terminator for various items and statements, [array types]
| `:`    | Colon       | various separators
| `::`   | PathSep     | [path separator][paths]
| `->`   | RArrow      | [function return type][functions], [closure return type][closures], [function pointer type]
| `=>`   | FatArrow    | [match arms][match], [macros]
| `<-`   | LArrow      | The left arrow symbol has been unused since before Rust 1.0, but it is still treated as a single token.
| `#`    | Pound       | [attributes]
| `$`    | Dollar      | [macros]
| `?`    | Question    | [try propagation expressions][question], [questionably sized][sized], [macro Kleene matcher][macros]
| `~`    | Tilde       | The tilde operator has been unused since before Rust 1.0, but its token may still be used.

[arith]: expressions/operator-expr.md#arithmetic-and-logical-binary-operators
[array types]: types/array.md
[assignment]: expressions/operator-expr.md#assignment-expressions
[attributes]: attributes.md
[borrow]: expressions/operator-expr.md#borrow-operators
[closures]: expressions/closure-expr.md
[comparison]: expressions/operator-expr.md#comparison-operators
[compound]: expressions/operator-expr.md#compound-assignment-expressions
[constants]: items/constant-items.md
[dereference]: expressions/operator-expr.md#the-dereference-operator
[destructuring assignment]: expressions/underscore-expr.md
[extern crates]: items/extern-crates.md
[extern]: items/external-blocks.md
[field]: expressions/field-expr.md
[function pointer type]: types/function-pointer.md
[functions]: items/functions.md
[generics]: items/generics.md
[if let]: expressions/if-expr.md#if-let-patterns
[inferred types]: types/inferred.md
[lazy-bool]: expressions/operator-expr.md#lazy-boolean-operators
[macros]: macros-by-example.md
[match]: expressions/match-expr.md
[negation]: expressions/operator-expr.md#negation-operators
[negative impls]: items/implementations.md
[never type]: types/never.md
[paths]: paths.md
[patterns]: patterns.md
[question]: expressions/operator-expr.md#the-try-propagation-expression
[range patterns]: patterns.md#range-patterns
[range]: expressions/range-expr.md
[rangepat]: patterns.md#range-patterns
[raw pointers]: types/pointer.md#raw-pointers-const-and-mut
[reference patterns]: patterns.md#reference-patterns
[references]: types/pointer.md
[sized]: trait-bounds.md#sized
[struct expressions]: expressions/struct-expr.md
[subpattern binding]: patterns.md#identifier-patterns
[trait bounds]: trait-bounds.md
[tuple index]: expressions/tuple-expr.md#tuple-indexing-expressions
[use declarations]: items/use-declarations.md
[use wildcards]: items/use-declarations.md
[while let]: expressions/loop-expr.md#while-let-patterns
[wildcard patterns]: patterns.md#wildcard-pattern
