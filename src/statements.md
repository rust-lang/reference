r[statement]
# Statements

r[statement.syntax]
```grammar,statements
Statement ->
      `;`
    | Item
    | LetStatement
    | ExpressionStatement
    | OuterAttribute* MacroInvocationSemi
```

r[statement.intro]
A *statement* is a component of a [block], which is in turn a component of an outer [expression] or [function].

r[statement.kind]
Rust has two kinds of statement: [declaration statements](#declaration-statements) and [expression statements](#expression-statements).

r[statement.decl]
## Declaration statements

A *declaration statement* is one that introduces one or more *names* into the enclosing statement block.
The declared names may denote new variables or new [items][item].

The two kinds of declaration statements are item declarations and `let` statements.

r[statement.item]
### Item declarations

r[statement.item.intro]
An *item declaration statement* has a syntactic form identical to an [item declaration][item] within a [module].

r[statement.item.scope]
Declaring an item within a statement block restricts its [scope] to the block containing the statement.
The item is not given a [canonical path] nor are any sub-items it may declare.

r[statement.item.associated-scope]
The exception to this is that associated items defined by [implementations] are still accessible in outer scopes as long as the item and, if applicable, trait are accessible.
It is otherwise identical in meaning to declaring the item inside a module.

r[statement.item.outer-generics]
There is no implicit capture of the containing function's generic parameters, parameters, and local variables.
For example, `inner` may not access `outer_var`.

```rust
fn outer() {
  let outer_var = true;

  fn inner() { /* outer_var is not in scope here */ }

  inner();
}
```

r[statement.let]
### `let` statements

r[statement.let.syntax]
```grammar,statements
LetStatement ->
    OuterAttribute* `let` PatternNoTopAlt ( `:` Type )?
    (
          `=` Expression
        | `=` Expression _except [LazyBooleanExpression] or end with a `}`_ `else` BlockExpression
    )? `;`
```

r[statement.let.intro]
A *`let` statement* introduces a new set of [variables], given by a [pattern].
The pattern is followed optionally by a type annotation and then either ends, or is followed by an initializer expression plus an optional `else` block.

r[statement.let.inference]
When no type annotation is given, the compiler will infer the type, or signal an error if insufficient type information is available for definite inference.

r[statement.let.scope]
Any variables introduced by a variable declaration are visible from the point of declaration until the end of the enclosing block scope, except when they are shadowed by another variable declaration.

r[statement.let.constraint]
If an `else` block is not present, the pattern must be irrefutable.
If an `else` block is present, the pattern may be refutable.

r[statement.let.behavior]
If the pattern does not match (this requires it to be refutable), the `else` block is executed.
The `else` block must always diverge (evaluate to the [never type]).

```rust
let (mut v, w) = (vec![1, 2, 3], 42); // The bindings may be mut or const
let Some(t) = v.pop() else { // Refutable patterns require an else block
    panic!(); // The else block must diverge
};
let [u, v] = [v[0], v[1]] else { // This pattern is irrefutable, so the compiler
                                 // will lint as the else block is redundant.
    panic!();
};
```

r[statement.expr]
## Expression statements

r[statement.expr.syntax]
```grammar,statements
ExpressionStatement ->
      ExpressionWithoutBlock `;`
    | ExpressionWithBlock `;`?
```

r[statement.expr.intro]
An *expression statement* is one that evaluates an [expression] and ignores its result.
As a rule, an expression statement's purpose is to trigger the effects of evaluating its expression.

r[statement.expr.restriction-semicolon]
An expression that consists of only a [block expression][block] or control flow expression, if used in a context where a statement is permitted, can omit the trailing semicolon.
This can cause an ambiguity between it being parsed as a standalone statement and as a part of another expression;
in this case, it is parsed as a statement.

r[statement.expr.constraint-block]
The type of [ExpressionWithBlock] expressions when used as statements must be the unit type.

```rust
# let mut v = vec![1, 2, 3];
v.pop();          // Ignore the element returned from pop
if v.is_empty() {
    v.push(5);
} else {
    v.remove(0);
}                 // Semicolon can be omitted.
[1];              // Separate expression statement, not an indexing expression.
```

When the trailing semicolon is omitted, the result must be type `()`.

```rust
// bad: the block's type is i32, not ()
// Error: expected `()` because of default return type
// if true {
//   1
// }

// good: the block's type is i32
if true {
  1
} else {
  2
};
```

r[statement.attribute]
## Attributes on Statements

Statements accept [outer attributes].
The attributes that have meaning on a statement are [`cfg`], and [the lint check attributes].

[block]: expressions/block-expr.md
[expression]: expressions.md
[function]: items/functions.md
[item]: items.md
[module]: items/modules.md
[never type]: types/never.md
[canonical path]: paths.md#canonical-paths
[implementations]: items/implementations.md
[variables]: variables.md
[outer attributes]: attributes.md
[`cfg`]: conditional-compilation.md
[the lint check attributes]: attributes/diagnostics.md#lint-check-attributes
[pattern]: patterns.md
[scope]: names/scopes.md
