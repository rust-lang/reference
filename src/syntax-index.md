# Syntax index

This appendix provides an index of tokens and common forms with links to where those elements are defined.

## Keywords

| Keyword       | Usage |
|---------------|-------|
| `_`           | [wildcard patterns], [inferred types], unnamed items in [constants], [extern crates], [use declarations], [destructuring assignment] |
| `abstract`    | [reserved keyword] |
| `as`          | [extern crate alias], [use alias], [type cast expressions], [qualified paths] |
| `async`       | [async functions], [async blocks], [async closures] |
| `await`       | [await expressions] |
| `become`      | [reserved keyword] |
| `box`         | [reserved keyword] |
| `break`       | [break expressions] |
| `const`       | [const functions], [const items], [const generics], [const blocks], [raw borrow operator], [raw pointer type], [const assembly operands] |
| `continue`    | [continue expressions] |
| `crate`       | [extern crates], [visibility], [paths] |
| `do`          | [reserved keyword] |
| `dyn`         | [trait objects] |
| `else`        | [let statements], [if expressions] |
| `enum`        | [enumerations] |
| `extern`      | [extern crate], [extern function qualifier], [external blocks], [extern function pointer types] |
| `false`       | [boolean type], [boolean expressions], [configuration predicates] |
| `final`       | [reserved keyword] |
| `fn`          | [functions], [function pointer types] |
| `for`         | [trait implementations], [iterator loops], [higher-ranked trait bounds] |
| `gen`         | [reserved keyword] |
| `if`          | [if expressions], [match guards] |
| `impl`        | [inherent impls], [trait impls], [impl traits] |
| `in`          | [visibility], [iterator loops], [assembly operands] |
| `let`         | [let statements], [`if let` patterns] |
| `loop`        | [infinite loops] |
| `macro_rules` | [macros by example] |
| `macro`       | [reserved keyword] |
| `match`       | [match expressions] |
| `mod`         | [modules] |
| `move`        | [closure expressions], [async blocks] |
| `mut`         | [borrow expressions], [identifier patterns], [reference patterns], [struct patterns], [reference types], [raw pointer types], [self parameters], [static items] |
| `override`    | [reserved keyword] |
| `priv`        | [reserved keyword] |
| `pub`         | [visibility] |
| `raw`         | [borrow expressions], [raw assembly] |
| `ref`         | [identifier patterns], [struct patterns] |
| `return`      | [return expressions] |
| `safe`        | [external block functions], [external block statics] |
| `self`        | [extern crates][items.extern-crate.self], [self parameters], [visibility], [`self` paths] |
| `Self`        | [`Self` type paths], [use bounds] |
| `static`      | [static items], [`'static` lifetimes] |
| `struct`      | [structs] |
| `super`       | [super paths], [visibility] |
| `trait`       | [trait items] |
| `true`        | [boolean type], [boolean expressions], [configuration predicates] |
| `try`         | [reserved keyword] |
| `type`        | [type aliases] |
| `typeof`      | [reserved keyword] |
| `union`       | [union items] |
| `unsafe`      | [unsafe attributes], [unsafe modules], [unsafe functions], [unsafe static items], [unsafe external blocks], [unsafe external functions], [unsafe external statics], [unsafe traits], [unsafe trait implementations] |
| `unsized`     | [reserved keyword] |
| `use`         | [use items], [use bounds] |
| `virtual`     | [reserved keyword] |
| `where`       | [where clauses] |
| `while`       | [predicate loops] |
| `yield`       | [reserved keyword] |

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

[`'static` lifetimes]: bound
[`if let` patterns]: expr.if.let
[`self` paths]: paths.qualifiers.mod-self
[`Self` type paths]: paths.qualifiers.type-self
[arith]: expr.arith-logic
[array types]: type.array
[assembly operands]: asm.operand-type.supported-operands.in
[assignment]: expr.assign
[async blocks]: expr.block.async
[async closures]: expr.closure.async
[async functions]: items.fn.async
[await expressions]: expr.await
[boolean expressions]: expr.literal
[boolean type]: type.bool
[borrow expressions]: expr.operator.borrow
[borrow]: expr.operator.borrow
[break expressions]: expr.loop.break
[closure expressions]: expr.closure
[closures]: expr.closure
[comparison]: expr.cmp
[compound]: expr.compound-assign
[configuration predicates]: cfg
[const assembly operands]: asm.operand-type.supported-operands.const
[const blocks]: expr.block.const
[const functions]: const-eval.const-fn
[const generics]: items.generics.const
[const items]: items.const
[constants]: items.const
[continue expressions]: expr.loop.continue
[dereference]: expr.deref
[destructuring assignment]: expr.placeholder
[enumerations]: items.enum
[extern crate alias]: items.extern-crate.as
[extern crate]: items.extern-crate
[extern crates]: items.extern-crate
[extern function pointer types]: type.fn-pointer.qualifiers
[extern function qualifier]: items.fn.extern
[extern]: items.extern
[external block functions]: items.extern.fn
[external block statics]: items.extern.static
[external blocks]: items.extern
[field]: expr.field
[function pointer type]: type.fn-pointer
[function pointer types]: type.fn-pointer
[functions]: items.fn
[generics]: items.generics
[glob imports]: items.use.glob
[higher-ranked trait bounds]: bound.higher-ranked
[identifier patterns]: patterns.ident
[if expressions]: expr.if
[if let]: expr.if.let
[impl traits]: type.impl-trait
[inferred types]: type.inferred
[infinite loops]: expr.loop.infinite
[inherent impls]: items.impl.inherent
[iterator loops]: expr.loop.for
[keywords chapter]: lex.keywords
[lazy-bool]: expr.bool-logic
[let statements]: statement.let
[macro calls]: macro.invocation
[macro Kleene matcher]: macro.decl.repetition
[macros by example]: macro.decl
[macros]: macro.decl
[match expressions]: expr.match
[match guards]: expr.match.guard
[match]: expr.match
[modules]: items.mod
[negation]: expr.negate
[negative impls]: items.impl
[never type]: type.never
[or patterns]: patterns.or
[predicate loops]: expr.loop.while
[qualified paths]: paths.qualified
[question]: expr.try
[range patterns]: patterns.range
[raw assembly]: asm.options.supported-options.raw
[raw borrow operator]: expr.borrow.raw
[raw pointer type]: type.pointer.raw
[raw pointer types]: type.pointer.raw
[raw pointers]: type.pointer.raw
[reference patterns]: patterns.ref
[reference types]: type.pointer.reference
[references]: type.pointer.reference
[reserved keyword]: lex.keywords.reserved
[rest patterns]: patterns.rest
[return expressions]: expr.return
[self parameters]: items.fn.params.self-pat
[sized]: bound.sized
[static items]: items.static
[struct expressions]: expr.struct
[struct patterns]: patterns.struct
[structs]: items.struct
[subpattern binding]: patterns.ident.scrutinized
[super paths]: paths.qualifiers.super
[trait bounds]: bound
[trait implementations]: items.impl.trait
[trait impls]: items.impl.trait
[trait items]: items.traits
[trait objects]: type.trait-object
[tuple index]: expr.tuple-index
[type aliases]: items.type
[type cast expressions]: expr.as
[union items]: items.union
[unsafe attributes]: attributes.safety
[unsafe external blocks]: unsafe.extern
[unsafe external functions]: items.extern.fn.safety
[unsafe external statics]: items.extern.static.safety
[unsafe functions]: unsafe.fn
[unsafe modules]: items.mod.unsafe
[unsafe static items]: items.static.mut.safety
[unsafe trait implementations]: items.impl.trait.safety
[unsafe traits]: items.traits.safety
[use alias]: items.use.forms.as
[use bounds]: bound.use
[use declarations]: items.use
[use items]: items.use
[variadic functions]: items.extern.variadic
[visibility]: vis
[where clauses]: items.generics.where
[while let]: expr.loop.while.let
[wildcard patterns]: patterns.wildcard
