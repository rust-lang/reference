r[macro]
# Macros

r[macro.intro]
The functionality and syntax of Rust can be extended with custom definitions
called macros. They are given names, and invoked through a consistent
syntax: `some_extension!(...)`.

There are two ways to define new macros:

* [Macros by Example] define new syntax in a higher-level, declarative way.
* [Procedural Macros] define function-like macros, custom derives, and custom
  attributes using functions that operate on input tokens.

r[macro.invocation]
## Macro Invocation

r[macro.invocation.syntax]
```grammar,macros
MacroInvocation ->
    SimplePath `!` DelimTokenTree

DelimTokenTree ->
      `(` TokenTree* `)`
    | `[` TokenTree* `]`
    | `{` TokenTree* `}`

TokenTree ->
    Token _except [delimiters][lex.token.delim]_ | DelimTokenTree

MacroInvocationSemi ->
      SimplePath `!` `(` TokenTree* `)` `;`
    | SimplePath `!` `[` TokenTree* `]` `;`
    | SimplePath `!` `{` TokenTree* `}`
```

r[macro.invocation.intro]
A macro invocation expands a macro at compile time and replaces the
invocation with the result of the macro. Macros may be invoked in the
following situations:

r[macro.invocation.expr]
* [Expressions] and [statements]

r[macro.invocation.pattern]
* [Patterns]

r[macro.invocation.type]
* [Types]

r[macro.invocation.item]
* [Items] including [associated items]

r[macro.invocation.nested]
* [`macro_rules`] transcribers

r[macro.invocation.extern]
* [External blocks]

r[macro.invocation.item-statement]
When used as an item or a statement, the [MacroInvocationSemi] form is used
where a semicolon is required at the end when not using curly braces.
[Visibility qualifiers] are never allowed before a macro invocation or
[`macro_rules`] definition.

```rust
// Used as an expression.
let x = vec![1,2,3];

// Used as a statement.
println!("Hello!");

// Used in a pattern.
macro_rules! pat {
    ($i:ident) => (Some($i))
}

if let pat!(x) = Some(1) {
    assert_eq!(x, 1);
}

// Used in a type.
macro_rules! Tuple {
    { $A:ty, $B:ty } => { ($A, $B) };
}

type N2 = Tuple!(i32, i32);

// Used as an item.
# use std::cell::RefCell;
thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

// Used as an associated item.
macro_rules! const_maker {
    ($t:ty, $v:tt) => { const CONST: $t = $v; };
}
trait T {
    const_maker!{i32, 7}
}

// Macro calls within macros.
macro_rules! example {
    () => { println!("Macro call in a macro!") };
}
// Outer macro `example` is expanded, then inner macro `println` is expanded.
example!();
```

[Macros by Example]: macros-by-example.md
[Procedural Macros]: procedural-macros.md
[associated items]: items/associated-items.md
[delimiters]: tokens.md#delimiters
[expressions]: expressions.md
[items]: items.md
[`macro_rules`]: macros-by-example.md
[patterns]: patterns.md
[statements]: statements.md
[types]: types.md
[visibility qualifiers]: visibility-and-privacy.md
[External blocks]: items/external-blocks.md
