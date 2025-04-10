r[expr.loop]
# Loops and other breakable expressions

r[expr.loop.syntax]
```grammar,expressions
LoopExpression ->
    LoopLabel? (
        InfiniteLoopExpression
      | PredicateLoopExpression
      | PredicatePatternLoopExpression
      | IteratorLoopExpression
      | LabelBlockExpression
    )
```

r[expr.loop.intro]
Rust supports five loop expressions:

*   A [`loop` expression](#infinite-loops) denotes an infinite loop.
*   A [`while` expression](#predicate-loops) loops until a predicate is false.
*   A [`while let` expression](#predicate-pattern-loops) tests a pattern.
*   A [`for` expression](#iterator-loops) extracts values from an iterator, looping until the iterator is empty.
*   A [labelled block expression](#labelled-block-expressions) runs a loop exactly once, but allows exiting the loop early with `break`.

r[expr.loop.break-label]
All five types of loop support [`break` expressions](#break-expressions), and [labels](#loop-labels).

r[expr.loop.continue-label]
All except labelled block expressions support [`continue` expressions](#continue-expressions).

r[expr.loop.explicit-result]
Only `loop` and labelled block expressions support [evaluation to non-trivial values](#break-and-loop-values).

r[expr.loop.infinite]
## Infinite loops

r[expr.loop.infinite.syntax]
```grammar,expressions
InfiniteLoopExpression -> `loop` BlockExpression
```

r[expr.loop.infinite.intro]
A `loop` expression repeats execution of its body continuously:
`loop { println!("I live."); }`.

r[expr.loop.infinite.diverging]
A `loop` expression without an associated `break` expression is diverging and has type [`!`](../types/never.md).

r[expr.loop.infinite.break]
A `loop` expression containing associated [`break` expression(s)](#break-expressions) may terminate, and must have type compatible with the value of the `break` expression(s).

r[expr.loop.while]
## Predicate loops

r[expr.loop.while.syntax]
```grammar,expressions
PredicateLoopExpression -> `while` Expression _except [StructExprStruct]_ BlockExpression
```
<!-- TODO: The exception above isn't accurate, see https://github.com/rust-lang/reference/issues/569 -->

r[expr.loop.while.intro]
A `while` loop begins by evaluating the [boolean] loop conditional operand.

r[expr.loop.while.condition]
If the loop conditional operand evaluates to `true`, the loop body block executes, then control returns to the loop conditional operand.
If the loop conditional expression evaluates to `false`, the `while` expression completes.

An example:

```rust
let mut i = 0;

while i < 10 {
    println!("hello");
    i = i + 1;
}
```

r[expr.loop.while.let]
## Predicate pattern loops

r[expr.loop.while.let.syntax]
```grammar,expressions
PredicatePatternLoopExpression ->
    `while` `let` Pattern `=` Scrutinee _except [LazyBooleanExpression]_ BlockExpression
```

r[expr.loop.while.let.intro]
A `while let` loop is semantically similar to a `while` loop but in place of a condition expression it expects the keyword `let` followed by a pattern, an `=`, a [scrutinee] expression and a block expression.

r[expr.loop.while.let.condition]
If the value of the scrutinee matches the pattern, the loop body block executes then control returns to the pattern matching statement.
Otherwise, the while expression completes.

```rust
let mut x = vec![1, 2, 3];

while let Some(y) = x.pop() {
    println!("y = {}", y);
}

while let _ = 5 {
    println!("Irrefutable patterns are always true");
    break;
}
```

r[expr.loop.while.let.desugar]
A `while let` loop is equivalent to a `loop` expression containing a [`match` expression] as follows.

<!-- ignore: expansion example -->
```rust,ignore
'label: while let PATS = EXPR {
    /* loop body */
}
```

is equivalent to

<!-- ignore: expansion example -->
```rust,ignore
'label: loop {
    match EXPR {
        PATS => { /* loop body */ },
        _ => break,
    }
}
```

r[expr.loop.while.let.or-pattern]
Multiple patterns may be specified with the `|` operator.
This has the same semantics as with `|` in `match` expressions:

```rust
let mut vals = vec![2, 3, 1, 2, 2];
while let Some(v @ 1) | Some(v @ 2) = vals.pop() {
    // Prints 2, 2, then 1
    println!("{}", v);
}
```

r[expr.loop.while.let.lazy-bool]
As is the case in [`if let` expressions], the scrutinee cannot be a [lazy boolean operator expression][expr.bool-logic].

r[expr.loop.for]
## Iterator loops

r[expr.loop.for.syntax]
```grammar,expressions
IteratorLoopExpression ->
    `for` Pattern `in` Expression _except [StructExprStruct]_ BlockExpression
```
<!-- TODO: The exception above isn't accurate, see https://github.com/rust-lang/reference/issues/569 -->

r[expr.loop.for.intro]
A `for` expression is a syntactic construct for looping over elements provided by an implementation of `std::iter::IntoIterator`.

r[expr.loop.for.condition]
If the iterator yields a value, that value is matched against the irrefutable pattern, the body of the loop is executed, and then control returns to the head of the `for` loop.
If the iterator is empty, the `for` expression completes.

An example of a `for` loop over the contents of an array:

```rust
let v = &["apples", "cake", "coffee"];

for text in v {
    println!("I like {}.", text);
}
```

An example of a for loop over a series of integers:

```rust
let mut sum = 0;
for n in 1..11 {
    sum += n;
}
assert_eq!(sum, 55);
```

r[expr.loop.for.desugar]
A `for` loop is equivalent to a `loop` expression containing a [`match` expression] as follows:

<!-- ignore: expansion example -->
```rust,ignore
'label: for PATTERN in iter_expr {
    /* loop body */
}
```

is equivalent to

<!-- ignore: expansion example -->
```rust,ignore
{
    let result = match IntoIterator::into_iter(iter_expr) {
        mut iter => 'label: loop {
            let mut next;
            match Iterator::next(&mut iter) {
                Option::Some(val) => next = val,
                Option::None => break,
            };
            let PATTERN = next;
            let () = { /* loop body */ };
        },
    };
    result
}
```

r[expr.loop.for.lang-items]
`IntoIterator`, `Iterator`, and `Option` are always the standard library items here, not whatever those names resolve to in the current scope.

The variable names `next`, `iter`, and `val` are for exposition only, they do not actually have names the user can type.

> [!NOTE]
> The outer `match` is used to ensure that any [temporary values] in `iter_expr` don't get dropped before the loop is finished. `next` is declared before being assigned because it results in types being inferred correctly more often.

r[expr.loop.label]
## Loop labels

r[expr.loop.label.syntax]
```grammar,expressions
LoopLabel -> LIFETIME_OR_LABEL `:`
```

r[expr.loop.label.intro]
A loop expression may optionally have a _label_. The label is written as a lifetime preceding the loop expression, as in `'foo: loop { break 'foo; }`, `'bar: while false {}`, `'humbug: for _ in 0..0 {}`.

r[expr.loop.label.control-flow]
If a label is present, then labeled `break` and `continue` expressions nested within this loop may exit out of this loop or return control to its head.
See [break expressions](#break-expressions) and [continue expressions](#continue-expressions).

r[expr.loop.label.ref]
Labels follow the hygiene and shadowing rules of local variables. For example, this code will print "outer loop":

```rust
'a: loop {
    'a: loop {
        break 'a;
    }
    print!("outer loop");
    break 'a;
}
```

`'_` is not a valid loop label.

r[expr.loop.break]
## `break` expressions

r[expr.loop.break.syntax]
```grammar,expressions
BreakExpression -> `break` LIFETIME_OR_LABEL? Expression?
```

r[expr.loop.break.intro]
When `break` is encountered, execution of the associated loop body is immediately terminated, for example:

```rust
let mut last = 0;
for x in 1..100 {
    if x > 12 {
        break;
    }
    last = x;
}
assert_eq!(last, 12);
```

r[expr.loop.break.label]
A `break` expression is normally associated with the innermost `loop`, `for` or `while` loop enclosing the `break` expression,
but a [label](#loop-labels) can be used to specify which enclosing loop is affected.
Example:

```rust
'outer: loop {
    while true {
        break 'outer;
    }
}
```

r[expr.loop.break.value]
A `break` expression is only permitted in the body of a loop, and has one of the forms `break`, `break 'label` or ([see below](#break-and-loop-values)) `break EXPR` or `break 'label EXPR`.

r[expr.loop.block-labels]
## Labelled block expressions

r[expr.loop.block-labels.syntax]
```grammar,expressions
LabelBlockExpression -> BlockExpression
```

r[expr.loop.block-labels.intro]
Labelled block expressions are exactly like block expressions, except that they allow using `break` expressions within the block.

r[expr.loop.block-labels.break]
Unlike loops, `break` expressions within a labelled block expression *must* have a label (i.e. the label is not optional).

r[expr.loop.block-labels.label-required]
Similarly, labelled block expressions *must* begin with a label.

```rust
# fn do_thing() {}
# fn condition_not_met() -> bool { true }
# fn do_next_thing() {}
# fn do_last_thing() {}
let result = 'block: {
    do_thing();
    if condition_not_met() {
        break 'block 1;
    }
    do_next_thing();
    if condition_not_met() {
        break 'block 2;
    }
    do_last_thing();
    3
};
```

r[expr.loop.continue]
## `continue` expressions

r[expr.loop.continue.syntax]
```grammar,expressions
ContinueExpression -> `continue` LIFETIME_OR_LABEL?
```

r[expr.loop.continue.intro]
When `continue` is encountered, the current iteration of the associated loop body is immediately terminated, returning control to the loop *head*.

r[expr.loop.continue.while]
In the case of a `while` loop, the head is the conditional expression controlling the loop.

r[expr.loop.continue.for]
In the case of a `for` loop, the head is the call-expression controlling the loop.

r[expr.loop.continue.label]
Like `break`, `continue` is normally associated with the innermost enclosing loop, but `continue 'label` may be used to specify the loop affected.

r[expr.loop.continue.in-loop-only]
A `continue` expression is only permitted in the body of a loop.

r[expr.loop.break-value]
## `break` and loop values

r[expr.loop.break-value.intro]
When associated with a `loop`, a break expression may be used to return a value from that loop, via one of the forms `break EXPR` or `break 'label EXPR`, where `EXPR` is an expression whose result is returned from the `loop`.
For example:

```rust
let (mut a, mut b) = (1, 1);
let result = loop {
    if b > 10 {
        break b;
    }
    let c = a + b;
    a = b;
    b = c;
};
// first number in Fibonacci sequence over 10:
assert_eq!(result, 13);
```

r[expr.loop.break-value.loop]
In the case a `loop` has an associated `break`, it is not considered diverging, and the `loop` must have a type compatible with each `break` expression.
`break` without an expression is considered identical to `break` with expression `()`.

[`match` expression]: match-expr.md
[boolean]: ../types/boolean.md
[scrutinee]: ../glossary.md#scrutinee
[temporary values]: ../expressions.md#temporaries
[`if let` expressions]: if-expr.md#if-let-expressions
