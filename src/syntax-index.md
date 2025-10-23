# Syntax index

This appendix provides an index of tokens and common forms with links to where those elements are defined.

## Keywords

## Operators and punctuation

| Symbol | Name        | Usage |
|--------|-------------|-------|
| `+`    | Plus        | [addition][arith], [trait bounds], [macro Kleene matcher] |
| `-`    | Minus       | [subtraction][arith], [negation] |
| `*`    | Star        | [multiplication][arith], [dereference], [raw pointers], [macro Kleene matcher], [glob imports] |
| `/`    | Slash       | [division][arith] |
| `%`    | Percent     | [remainder][arith] |
| `^`    | Caret       | [bitwise and logical XOR][arith] |
| `!`    | Not         | [bitwise and logical NOT][negation], [macro calls], [inner attributes][attributes], [never type], [negative impls] |
| `&`    | And         | [bitwise and logical AND][arith], [borrow], [references], [reference patterns] |
| <code>\|</code> | Or | [bitwise and logical OR][arith], [closures], [or patterns], [if let], [while let] |
| `&&`   | AndAnd      | [lazy AND][lazy-bool], [borrow], [references], [reference patterns] |
| <code>\|\|</code> | OrOr | [lazy OR][lazy-bool], [closures] |
| `<<`   | Shl         | [shift left][arith], [nested generics][generics] |
| `>>`   | Shr         | [shift right][arith], [nested generics][generics] |
| `+=`   | PlusEq      | [addition assignment][compound] |
| `-=`   | MinusEq     | [subtraction assignment][compound] |
| `*=`   | StarEq      | [multiplication assignment][compound] |
| `/=`   | SlashEq     | [division assignment][compound] |
| `%=`   | PercentEq   | [remainder assignment][compound] |
| `^=`   | CaretEq     | [bitwise XOR assignment][compound] |
| `&=`   | AndEq       | [bitwise AND assignment][compound] |
| <code>\|=</code> | OrEq | [bitwise OR assignment][compound] |
| `<<=`  | ShlEq       | [shift left assignment][compound] |
| `>>=`  | ShrEq       | [shift right assignment][compound], [nested generics][generics] |
| `=`    | Eq          | [assignment], [let statements], [attributes], various type definitions |
| `==`   | EqEq        | [equal][comparison] |
| `!=`   | Ne          | [not equal][comparison] |
| `>`    | Gt          | [greater than][comparison], [generics], [paths], [use bounds] |
| `<`    | Lt          | [less than][comparison], [generics], [paths], [use bounds] |
| `>=`   | Ge          | [greater than or equal to][comparison], [generics] |
| `<=`   | Le          | [less than or equal to][comparison] |
| `@`    | At          | [subpattern binding] |
| `.`    | Dot         | [field access][field], [tuple index] |
| `..`   | DotDot      | [range expressions][expr.range], [struct expressions], [rest patterns], [range patterns], [struct patterns] |
| `...`  | DotDotDot   | [variadic functions], [range patterns] |
| `..=`  | DotDotEq    | [inclusive range expressions][expr.range], [range patterns] |
| `,`    | Comma       | various separators |
| `;`    | Semi        | terminator for various items and statements, [array types] |
| `:`    | Colon       | various separators |
| `::`   | PathSep     | [path separator][paths] |
| `->`   | RArrow      | [function return type][functions], [closure return type][closures], [function pointer type] |
| `=>`   | FatArrow    | [match arms][match], [macros] |
| `<-`   | LArrow      | The left arrow symbol has been unused since before Rust 1.0, but it is still treated as a single token. |
| `#`    | Pound       | [attributes] |
| `$`    | Dollar      | [macros] |
| `?`    | Question    | [try propagation expressions][question], [questionably sized][sized], [macro Kleene matcher] |
| `~`    | Tilde       | The tilde operator has been unused since before Rust 1.0, but its token may still be used. |

[arith]: expr.arith-logic
[array types]: type.array
[assignment]: expr.assign
[borrow]: expr.operator.borrow
[closures]: expr.closure
[comparison]: expr.cmp
[compound]: expr.compound-assign
[constants]: items.const
[dereference]: expr.deref
[destructuring assignment]: expr.placeholder
[extern crates]: items.extern-crate
[extern]: items.extern
[field]: expr.field
[function pointer type]: type.fn-pointer
[functions]: items.fn
[generics]: items.generics
[glob imports]: items.use.glob
[if let]: expr.if.let
[inferred types]: type.inferred
[lazy-bool]: expr.bool-logic
[let statements]: statement.let
[macro calls]: macro.invocation
[macro Kleene matcher]: macro.decl.repetition
[macros]: macro.decl
[match]: expr.match
[negation]: expr.negate
[negative impls]: items.impl
[never type]: type.never
[or patterns]: patterns.or
[question]: expr.try
[range patterns]: patterns.range
[raw pointers]: type.pointer.raw
[reference patterns]: patterns.ref
[references]: type.pointer.reference
[rest patterns]: patterns.rest
[sized]: bound.sized
[struct expressions]: expr.struct
[struct patterns]: patterns.struct
[subpattern binding]: patterns.ident.scrutinized
[trait bounds]: bound
[tuple index]: expr.tuple-index
[use bounds]: bound.use
[use declarations]: items.use
[variadic functions]: items.extern.variadic
[while let]: expr.loop.while.let
[wildcard patterns]: patterns.wildcard
