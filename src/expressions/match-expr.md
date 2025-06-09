r[expr.match]
# `match` expressions

r[expr.match.syntax]
```grammar,expressions
MatchExpression ->
    `match` Scrutinee `{`
        InnerAttribute*
        MatchArms?
    `}`

Scrutinee ->
    Expression _except_ [StructExpression]

MatchArms ->
    ( MatchArm `=>` ( ExpressionWithoutBlock `,` | ExpressionWithBlock `,`? ) )*
    MatchArm `=>` Expression `,`?

MatchArm ->
    OuterAttribute* Pattern MatchArmGuard?

MatchArmGuard ->
    `if` MatchConditions

MatchConditions ->
     Expression
   | MatchConditionChain

MatchConditionChain ->
    MatchChainCondition ( `&&` MatchChainCondition )*

MatchChainCondition ->
     Expression _except [ExcludedMatchConditions]_
   | OuterAttribute* `let` Pattern `=` MatchGuardScrutinee

MatchGuardScrutinee -> Expression _except [ExcludedMatchConditions]_

@root ExcludedMatchConditions ->
      LazyBooleanExpression
    | RangeExpr
    | RangeFromExpr
    | RangeInclusiveExpr
    | AssignmentExpression
    | CompoundAssignmentExpression
```
<!-- TODO: The exception above isn't accurate, see https://github.com/rust-lang/reference/issues/569 -->

r[expr.match.intro]
A *`match` expression* branches on a pattern.
The exact form of matching that occurs depends on the [pattern].

r[expr.match.scrutinee]
A `match` expression has a *[scrutinee] expression*, which is the value to compare to the patterns.

r[expr.match.scrutinee-constraint]
The scrutinee expression and the patterns must have the same type.

r[expr.match.scrutinee-behavior]
A `match` behaves differently depending on whether or not the scrutinee expression is a [place expression or value expression][place expression].

r[expr.match.scrutinee-value]
If the scrutinee expression is a [value expression], it is first evaluated into a temporary location, and the resulting value is sequentially compared to the patterns in the arms until a match is found.
The first arm with a matching pattern is chosen as the branch target of the `match`, any variables bound by the pattern are assigned to local variables in the arm's block, and control enters the block.

r[expr.match.scrutinee-place]
When the scrutinee expression is a [place expression], the match does not allocate a temporary location;
however, a by-value binding may copy or move from the memory location.
When possible, it is preferable to match on place expressions, as the lifetime of these matches inherits the lifetime of the place expression rather than being restricted to the inside of the match.

An example of a `match` expression:

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
```

r[expr.match.pattern-vars]
Variables bound within the pattern are scoped to the match guard and the arm's expression.

r[expr.match.pattern-var-binding]
The [binding mode] (move, copy, or reference) depends on the pattern.

r[expr.match.or-pattern]
Multiple match patterns may be joined with the `|` operator.
Each pattern will be tested in left-to-right sequence until a successful match is found.

```rust
let x = 9;
let message = match x {
    0 | 1  => "not many",
    2 ..= 9 => "a few",
    _      => "lots"
};

assert_eq!(message, "a few");

// Demonstration of pattern match order.
struct S(i32, i32);

match S(1, 2) {
    S(z @ 1, _) | S(_, z @ 2) => assert_eq!(z, 1),
    _ => panic!(),
}
```

> [!NOTE]
> The `2..=9` is a [Range Pattern], not a [Range Expression]. Thus, only those types of ranges supported by range patterns can be used in match arms.

r[expr.match.or-patterns-restriction]
Every binding in each `|` separated pattern must appear in all of the patterns in the arm.

r[expr.match.binding-restriction]
Every binding of the same name must have the same type, and have the same binding mode.

r[expr.match.guard]
## Match guards

r[expr.match.guard.intro]
Match arms can accept _match guards_ to further refine the criteria for matching a case.

r[expr.match.guard.type]
Pattern guards appear after the pattern and consist of a `bool`-typed expression following the `if` keyword.

r[expr.match.guard.behavior]
When the pattern matches successfully, the pattern guard expression is executed.
If the expression evaluates to true, the pattern is successfully matched against.

r[expr.match.guard.next]
Otherwise, the next pattern, including other matches with the `|` operator in the same arm, is tested.

```rust
# let maybe_digit = Some(0);
# fn process_digit(i: i32) { }
# fn process_other(i: i32) { }
let message = match maybe_digit {
    Some(x) if x < 10 => process_digit(x),
    Some(x) => process_other(x),
    None => panic!(),
};
```

> [!NOTE]
> Multiple matches using the `|` operator can cause the pattern guard and the side effects it has to execute multiple times. For example:
>
> ```rust
> # use std::cell::Cell;
> let i : Cell<i32> = Cell::new(0);
> match 1 {
>     1 | _ if { i.set(i.get() + 1); false } => {}
>     _ => {}
> }
> assert_eq!(i.get(), 2);
> ```

r[expr.match.guard.bound-variables]
A pattern guard may refer to the variables bound within the pattern they follow.

r[expr.match.guard.shared-ref]
Before evaluating the guard, a shared reference is taken to the part of the scrutinee the variable matches on.
While evaluating the guard, this shared reference is then used when accessing the variable.

r[expr.match.guard.value]
Only when the guard evaluates to true is the value moved, or copied, from the scrutinee into the variable.
This allows shared borrows to be used inside guards without moving out of the scrutinee in case guard fails to match.

r[expr.match.guard.no-mutation]
Moreover, by holding a shared reference while evaluating the guard, mutation inside guards is also prevented.

r[expr.match.if.let.guard]
## If-let Guards
Match arms can have additional conditions using *if-let guards*. These allow using `if let` expressions within match guard clauses, enabling conditional pattern matching directly in the guard.

r[expr.match.if.let.guard.syntax]
```rust
enum Command {
    Run(String),
    Stop,
}

match cmd {
    Command::Run(name) if let Some(first_char) = name.chars().next() => {
        // Both `name` and `first_char` are available here
        println!("Running: {} (starts with '{}')", name, first_char);
    }
    Command::Run(name) => {
        println!("Cannot run command: {}", name);
    }
    _ => {}
}
```
Here, the guard condition `let Some(first_char) = name.chars().next()` is evaluated. If the `if let` expression successfully matches (i.e., the string has at least one character), the arm's body is executed with both `name` and `first_char` available. Otherwise, pattern matching continues to the next arm.

The key point is that the `if let` guard creates a new binding (`first_char`) that's only available if the guard succeeds, and this binding can be used alongside the original pattern bindings (`name`) in the arm's body.

r[expr.match.if.let.guard.behavior]
When the pattern matches successfully, the `if let` expression in the guard is evaluated:
  * The guard proceeds if the inner pattern (`subpattern`) matches the result of `guard_expr`.
  * Otherwise, the next arm is tested.

r[expr.match.if.let.guard.scope]

For detailed information about variable scope and drop behavior, see the [scope and drop section].

```rust,ignore
# fn take<T>(value: T) -> Option<T> { Some(value) }

let val = Some(vec![1, 2, 3]);
match val {
    Some(v) if let Some(_) = take(v) => "moved", // ERROR: cannot move `v`
    Some(v) if let Some(_) = take(v.clone()) => "cloned", // OK
    _ => "none",
}
```
> [!NOTE]
> Multiple matches using the `|` operator can cause the pattern guard and the side effects it has to execute multiple times. For example:
> ```rust
> use std::cell::Cell;
>
> let i: Cell<i32> = Cell::new(0);
> match 1 {
>     1 | _ if let Some(_) = { i.set(i.get() + 1); Some(1) } => {}
>     _ => {}
> }
> assert_eq!(i.get(), 2); // Guard is executed twice
> ```

r[expr.match.attributes]
## Attributes on match arms

r[expr.match.attributes.outer]
Outer attributes are allowed on match arms.
The only attributes that have meaning on match arms are [`cfg`] and the [lint check attributes].

r[expr.match.attributes.inner]
[Inner attributes] are allowed directly after the opening brace of the match expression in the same expression contexts as [attributes on block expressions].

[`cfg`]: ../conditional-compilation.md
[attributes on block expressions]: block-expr.md#attributes-on-block-expressions
[binding mode]: ../patterns.md#binding-modes
[Inner attributes]: ../attributes.md
[lint check attributes]: ../attributes/diagnostics.md#lint-check-attributes
[pattern]: ../patterns.md
[place expression]: ../expressions.md#place-expressions-and-value-expressions
[Range Expression]: range-expr.md
[Range Pattern]: ../patterns.md#range-patterns
[scrutinee]: ../glossary.md#scrutinee
[value expression]: ../expressions.md#place-expressions-and-value-expressions
[scope and drop section]: ../destructors.md#match-guards-and-pattern-binding
