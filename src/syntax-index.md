# Syntax index

This appendix provides an index of tokens and common forms with links to where those elements are defined.

## Keywords

| Keyword       | Use |
|---------------|-----|
| `_`           | [wildcard pattern], [inferred const], [inferred type], [placeholder lifetime], [constant items], [extern crate], [use declarations], [destructuring assignment] |
| `abstract`    | [reserved keyword] |
| `as`          | [extern crate][items.extern-crate.as], [use declarations][items.use.forms.as], [type cast expressions], [qualified paths] |
| `async`       | [async functions], [async blocks], [async closures] |
| `await`       | [await expressions] |
| `become`      | [reserved keyword] |
| `box`         | [reserved keyword] |
| `break`       | [break expressions] |
| `const`       | [const functions], [const items], [const generics], [const blocks], [raw borrow operator], [raw pointer type], [const assembly operands] |
| `continue`    | [continue expressions] |
| `crate`       | [extern crate], [visibility], [paths] |
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
| `impl`        | [inherent impls], [trait impls], [impl trait types], [anonymous type parameters] |
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
| `self`        | [extern crate][items.extern-crate.self], [self parameters], [visibility], [`self` paths] |
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
| `unsafe`      | [unsafe blocks], [unsafe attributes], [unsafe modules], [unsafe functions], [unsafe external blocks], [unsafe external functions], [unsafe external statics], [unsafe traits], [unsafe trait implementations] |
| `unsized`     | [reserved keyword] |
| `use`         | [use items], [use bounds] |
| `virtual`     | [reserved keyword] |
| `where`       | [where clauses] |
| `while`       | [predicate loops] |
| `yield`       | [reserved keyword] |

## Operators and punctuation

| Symbol | Name        | Use |
|--------|-------------|-----|
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
| `..`   | DotDot      | [range expressions][expr.range], [struct expressions], [rest pattern], [range patterns], [struct patterns] |
| `...`  | DotDotDot   | [variadic functions], [range patterns] |
| `..=`  | DotDotEq    | [inclusive range expressions][expr.range], [range patterns] |
| `,`    | Comma       | various separators |
| `;`    | Semi        | terminator for various items and statements, [array expressions], [array types] |
| `:`    | Colon       | various separators |
| `::`   | PathSep     | [path separator][paths] |
| `->`   | RArrow      | [functions], [closures], [function pointer type] |
| `=>`   | FatArrow    | [match arms][match], [macros] |
| `<-`   | LArrow      | The left arrow symbol has been unused since before Rust 1.0, but it is still treated as a single token. |
| `#`    | Pound       | [attributes], [raw string literals], [raw byte string literals], [raw C string literals] |
| `$`    | Dollar      | [macros] |
| `?`    | Question    | [try propagation expressions][question], [relaxed trait bounds], [macro Kleene matcher] |
| `~`    | Tilde       | The tilde operator has been unused since before Rust 1.0, but its token may still be used. |

## Comments

| Comment  | Use |
|----------|-----|
| `//`     | [line comment][comments] |
| `//!`    | [inner line comment][comments] |
| `///`    | [outer line doc comment][comments] |
| `/*…*/`  | [block comment][comments] |
| `/*!…*/` | [inner block doc comment][comments] |
| `/**…*/` | [outer block doc comment][comments] |

## Other tokens

| Token        | Use |
|--------------|-----|
| `ident`      | [identifiers] |
| `r#ident`    | [raw identifiers] |
| `'ident`     | [lifetimes and loop labels] |
| `'r#ident`   | [raw lifetimes and loop labels] |
| `…u8`, `…i32`, `…f64`, `…usize`, … | [number literals] |
| `"…"`        | [string literals] |
| `r"…"`, `r#"…"#`, `r##"…"##`, … | [raw string literals] |
| `b"…"`       | [byte string literals] |
| `br"…"`, `br#"…"#`, `br##"…"##`, … | [raw byte string literals] |
| `'…'`        | [character literals] |
| `b'…'`       | [byte literals] |
| `c"…"`       | [C string literals] |
| `cr"…"`, `cr#"…"#`, `cr##"…"##`, … | [raw C string literals] |

## Macros

| Syntax                                     | Use |
|--------------------------------------------|-----|
| `ident!(…)`<br>`ident! {…}`<br>`ident![…]` | [macro invocations] |
| `$ident`                                   | [macro metavariable] |
| `$ident:kind`                              | [macro matcher fragment specifier] |
| `$(…)…`                                    | [macro repetition] |

## Attributes

| Syntax     | Use |
|------------|-----|
| `#[meta]`  | [outer attribute] |
| `#![meta]` | [inner attribute] |

## Expressions

| Expression                | Use |
|---------------------------|-----|
| <code>\|…\| expr</code><br><code>\|…\| -> Type { … }</code> | [closures] |
| `ident::…`                | [paths] |
| `::crate_name::…`         | [explicit crate paths] |
| `crate::…`                | [crate-relative paths] |
| `self::…`                 | [module-relative paths] |
| `super::…`                | [parent module paths] |
| `Type::…`<br>`<Type as Trait>::ident` | [associated items] |
| `<Type>::…`               | [qualified paths] which can be used for types without names such as `<&T>::…`, `<[T]>::…`, etc. |
| `Trait::method(…)`<br>`Type::method(…)`<br>`<Type as Trait>::method(…)` | [disambiguated method calls] |
| `method::<…>(…)`<br>`path::<…>` | [generic arguments], aka turbofish |
| `()`                      | [unit] |
| `(expr)`                  | [parenthesized expressions] |
| `(expr,)`                 | [single-element tuple expressions] |
| `(expr, …)`               | [tuple expressions] |
| `expr(expr, …)`           | [call expressions] |
| `expr.0`, `expr.1`, …     | [tuple indexing expressions] |
| `expr.ident`              | [field access expressions] |
| `{…}`                     | [block expressions] |
| `Type {…}`                | [struct expressions] |
| `Type(…)`                 | [tuple struct constructors] |
| `[…]`                     | [array expressions] |
| `[expr; len]`             | [repeat array expressions] |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]`, `expr[a..=b]`, `expr[..=b]` | [array and slice indexing expressions] |
| `if expr {…} else {…}`    | [if expressions] |
| `match expr { pattern => {…} }` | [match expressions] |
| `loop {…}`                | [infinite loop expressions] |
| `while expr {…}`          | [predicate loop expressions] |
| `for pattern in expr {…}` | [iterator loops] |
| `&expr`<br>`&mut expr`    | [borrow expressions] |
| `&raw const expr`<br>`&raw mut expr` | [raw borrow expressions] |
| `*expr`                   | [dereference expressions] |
| `expr?`                   | [try propagation expressions] |
| `-expr`                   | [negation expressions] |
| `!expr`                   | [bitwise and logical NOT expressions] |
| `expr as Type`            | [type cast expressions] |

## Items

[Items] are the components of a crate.

| Item                          | Use |
|-------------------------------|-----|
| `mod ident;`<br>`mod ident {…}` | [modules] |
| `use path;`                   | [use declarations] |
| `fn ident(…) {…}`             | [functions] |
| `type Type = Type;`           | [type aliases] |
| `struct ident {…}`            | [structs] |
| `enum ident {…}`              | [enumerations] |
| `union ident {…}`             | [unions] |
| `trait ident {…}`             | [traits] |
| `impl Type {…}`<br>`impl Type for Trait {…}` | [implementations] |
| `const ident = expr;`         | [constant items] |
| `static ident = expr;`        | [static items] |
| `extern "C" {…}`              | [external blocks] |
| `fn ident<…>(…) …`<br>`struct ident<…> {…}`<br>`enum ident<…> {…}`<br>`impl<…> Type<…> {…}` | [generic definitions] |

## Type expressions

[Type expressions] are used to refer to types.

| Type                                  | Use |
|---------------------------------------|-----|
| `bool`, `u8`, `f64`, `str`, …         | [primitive types] |
| `for<…>`                              | [higher-ranked trait bounds] |
| `T: TraitA + TraitB`                  | [trait bounds] |
| `T: 'a + 'b`                          | [lifetime bounds] |
| `T: TraitA + 'a`                      | [trait and lifetime bounds] |
| `T: ?Sized`                           | [relaxed trait bounds] |
| `[Type; len]`                         | [array types] |
| `(Type, …)`                           | [tuple types] |
| `[Type]`                              | [slice types] |
| `(Type)`                              | [parenthesized types] |
| `impl Trait`                          | [impl trait types], [anonymous type parameters] |
| `dyn Trait`                           | [trait object types] |
| `ident`<br>`ident::…`                 | [type paths] (can refer to [structs], [enumerations], [unions], [type aliases], [traits], [generics], etc.) |
| `Type<…>`<br>`Trait<…>`               | [generic arguments] (e.g. `Vec<u8>`) |
| `Trait<ident = Type>`                 | [associated type bindings] (e.g. `Iterator<Item = T>`) |
| `Trait<ident: …>`                     | [associated type bounds] (e.g. `Iterator<Item: Send>`) |
| `&Type`<br>`&mut Type`                | [reference types] |
| `*mut Type`<br>`*const Type`          | [raw pointer types] |
| `fn(…) -> Type`                       | [function pointer types] |
| `_`                                   | [inferred type], [inferred const] |
| `'_`                                  | [placeholder lifetime] |
| `!`                                   | [never type] |

## Patterns

[Patterns] are used to match values.

| Pattern                           | Use |
|-----------------------------------|-----|
| `"foo"`, `'a'`, `123`, `2.4`, …   | [literal patterns] |
| `ident`                           | [identifier patterns] |
| `_`                               | [wildcard pattern] |
| `..`                              | [rest pattern] |
| `a..`, `..b`, `a..b`, `a..=b`, `..=b` | [range patterns] |
| `&pattern`<br>`&mut pattern`      | [reference patterns] |
| `path {…}`                        | [struct patterns] |
| `path(…)`                         | [tuple struct patterns] |
| `(pattern, …)`                    | [tuple patterns] |
| `(pattern)`                       | [grouped patterns] |
| `[pattern, …]`                    | [slice patterns] |
| `CONST`, `Enum::Variant`, …       | [path patterns] |

[`'static` lifetimes]: bound
[`if let` patterns]: expr.if.let
[`self` paths]: paths.qualifiers.mod-self
[`Self` type paths]: paths.qualifiers.type-self
[anonymous type parameters]: type.impl-trait.param
[arith]: expr.arith-logic
[array and slice indexing expressions]: expr.array.index
[array expressions]: expr.array
[array types]: type.array
[assembly operands]: asm.operand-type.supported-operands.in
[assignment]: expr.assign
[associated items]: items.associated
[associated type bindings]: paths.expr
[associated type bounds]: paths.expr
[async blocks]: expr.block.async
[async closures]: expr.closure.async
[async functions]: items.fn.async
[await expressions]: expr.await
[bitwise and logical NOT expressions]: expr.negate
[block expressions]: expr.block
[boolean expressions]: expr.literal
[boolean type]: type.bool
[borrow expressions]: expr.operator.borrow
[borrow]: expr.operator.borrow
[break expressions]: expr.loop.break
[byte literals]: lex.token.byte
[byte string literals]: lex.token.str-byte
[C string literals]: lex.token.str-c
[call expressions]: expr.call
[character literals]: lex.token.literal.char
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
[constant items]: items.const
[continue expressions]: expr.loop.continue
[crate-relative paths]: paths.qualifiers.crate
[dereference expressions]: expr.deref
[dereference]: expr.deref
[destructuring assignment]: expr.placeholder
[disambiguated method calls]: expr.call.desugar
[enumerations]: items.enum
[explicit crate paths]: paths.qualifiers.global-root
[extern crate]: items.extern-crate
[extern function pointer types]: type.fn-pointer.qualifiers
[extern function qualifier]: items.fn.extern
[external block functions]: items.extern.fn
[external block statics]: items.extern.static
[external blocks]: items.extern
[field access expressions]: expr.field
[field]: expr.field
[function pointer type]: type.fn-pointer
[function pointer types]: type.fn-pointer
[functions]: items.fn
[generic arguments]: items.generics
[generic definitions]: items.generics
[generics]: items.generics
[glob imports]: items.use.glob
[grouped patterns]: patterns.paren
[higher-ranked trait bounds]: bound.higher-ranked
[identifier patterns]: patterns.ident
[identifiers]: ident
[if expressions]: expr.if
[if let]: expr.if.let
[impl trait types]: type.impl-trait.return
[implementations]: items.impl
[inferred const]: items.generics.const.inferred
[inferred type]: type.inferred
[infinite loop expressions]: expr.loop.infinite
[infinite loops]: expr.loop.infinite
[inherent impls]: items.impl.inherent
[inner attribute]: attributes.inner
[iterator loops]: expr.loop.for
[lazy-bool]: expr.bool-logic
[let statements]: statement.let
[lifetime bounds]: bound.lifetime
[lifetimes and loop labels]: lex.token.life
[literal patterns]: patterns.literal
[macro calls]: macro.invocation
[macro invocations]: macro.invocation
[macro Kleene matcher]: macro.decl.repetition
[macro matcher fragment specifier]: macro.decl.meta.specifier
[macro metavariable]: macro.decl.meta
[macro repetition]: macro.decl.repetition
[macros by example]: macro.decl
[macros]: macro.decl
[match expressions]: expr.match
[match guards]: expr.match.guard
[match]: expr.match
[module-relative paths]: paths.qualifiers.mod-self
[modules]: items.mod
[negation expressions]: expr.negate
[negation]: expr.negate
[negative impls]: items.impl
[never type]: type.never
[number literals]: lex.token.literal.num
[or patterns]: patterns.or
[outer attribute]: attributes.outer
[parent module paths]: paths.qualifiers.super
[parenthesized expressions]: expr.paren
[parenthesized types]: type.name.parenthesized
[path patterns]: patterns.path
[placeholder lifetime]: lifetime-elision.function.explicit-placeholder
[predicate loop expressions]: expr.loop.while
[predicate loops]: expr.loop.while
[primitive types]: type.kinds
[qualified paths]: paths.qualified
[question]: expr.try
[range patterns]: patterns.range
[raw assembly]: asm.options.supported-options.raw
[raw borrow expressions]: expr.borrow.raw
[raw borrow operator]: expr.borrow.raw
[raw byte string literals]: lex.token.str-byte-raw
[raw C string literals]: lex.token.str-c-raw
[raw identifiers]: ident.raw
[raw lifetimes and loop labels]: lex.token.life
[raw pointer type]: type.pointer.raw
[raw pointer types]: type.pointer.raw
[raw pointers]: type.pointer.raw
[raw string literals]: lex.token.literal.str-raw
[reference patterns]: patterns.ref
[reference types]: type.pointer.reference
[references]: type.pointer.reference
[relaxed trait bounds]: bound.sized
[repeat array expressions]: expr.array
[reserved keyword]: lex.keywords.reserved
[rest pattern]: patterns.rest
[return expressions]: expr.return
[self parameters]: items.fn.params.self-pat
[single-element tuple expressions]: expr.tuple
[slice patterns]: patterns.slice
[slice types]: type.slice
[static items]: items.static
[string literals]: lex.token.literal.str
[struct expressions]: expr.struct
[struct patterns]: patterns.struct
[structs]: items.struct
[subpattern binding]: patterns.ident.scrutinized
[super paths]: paths.qualifiers.super
[trait and lifetime bounds]: bound
[trait bounds]: bound
[trait implementations]: items.impl.trait
[trait impls]: items.impl.trait
[trait items]: items.traits
[trait object types]: type.trait-object
[trait objects]: type.trait-object
[traits]: items.traits
[try propagation expressions]: expr.try
[tuple expressions]: expr.tuple
[tuple index]: expr.tuple-index
[tuple indexing expressions]: expr.tuple-index
[tuple patterns]: patterns.tuple
[tuple struct constructors]: items.struct.tuple
[tuple struct patterns]: patterns.tuple-struct
[tuple types]: type.tuple
[type aliases]: items.type
[type cast expressions]: expr.as
[Type expressions]: type.name
[type paths]: type.name.path
[union items]: items.union
[unions]: items.union
[unit]: type.tuple.unit
[unsafe attributes]: attributes.safety
[unsafe blocks]: expr.block.unsafe
[unsafe external blocks]: unsafe.extern
[unsafe external functions]: items.extern.fn.safety
[unsafe external statics]: items.extern.static.safety
[unsafe functions]: unsafe.fn
[unsafe modules]: items.mod.unsafe
[unsafe trait implementations]: items.impl.trait.safety
[unsafe traits]: items.traits.safety
[use bounds]: bound.use
[use declarations]: items.use
[use items]: items.use
[variadic functions]: items.extern.variadic
[visibility]: vis
[where clauses]: items.generics.where
[while let]: expr.loop.while.let
[wildcard pattern]: patterns.wildcard
