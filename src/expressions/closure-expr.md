r[expr.closure]
# Closure expressions

r[expr.closure.syntax]
```grammar,expressions
ClosureExpression ->
    `async`?[^cl-async-edition]
    `move`?
    ( `||` | `|` ClosureParameters? `|` )
    (Expression | `->` TypeNoBounds BlockExpression)

ClosureParameters -> ClosureParam (`,` ClosureParam)* `,`?

ClosureParam -> OuterAttribute* PatternNoTopAlt ( `:` Type )?
```

[^cl-async-edition]: The `async` qualifier is not allowed in the 2015 edition.

r[expr.closure.intro]
A *closure expression*, also known as a lambda expression or a lambda, defines a [closure type] and evaluates to a value of that type.
The syntax for a closure expression is an optional `async` keyword, an optional `move` keyword, then a pipe-symbol-delimited (`|`) comma-separated list of [patterns], called the *closure parameters* each optionally followed by a `:` and a type, then an optional `->` and type, called the *return type*, and then an expression, called the *closure body operand*.

r[expr.closure.param-type]
The optional type after each pattern is a type annotation for the pattern.

r[expr.closure.explicit-type-body]
If there is a return type, the closure body must be a [block].

r[expr.closure.parameter-restriction]
A closure expression denotes a function that maps a list of parameters onto the expression that follows the parameters.
Just like a [`let` binding], the closure parameters are irrefutable [patterns], whose type annotation is optional and will be inferred from context if not given.

r[expr.closure.unique-type]
Each closure expression has a unique, anonymous type.

r[expr.closure.captures]
Significantly, closure expressions _capture their environment_, which regular [function definitions] do not.

r[expr.closure.capture-inference]
Without the `move` keyword, the closure expression [infers how it captures each variable from its environment](../types/closure.md#capture-modes), preferring to capture by shared reference, effectively borrowing all outer variables mentioned inside the closure's body.

r[expr.closure.capture-mut-ref]
If needed the compiler will infer that instead mutable references should be taken, or that the values should be moved or copied (depending on their type) from the environment.

r[expr.closure.capture-move]
A closure can be forced to capture its environment by copying or moving values by prefixing it with the `move` keyword.
This is often used to ensure that the closure's lifetime is `'static`.

r[expr.closure.trait-impl]
## Closure trait implementations

Which traits the closure type implement depends on how variables are captured, the types of the captured variables, and the presence of `async`.
See the [call traits and coercions] chapter for how and when a closure implements `Fn`, `FnMut`, and `FnOnce`.
The closure type implements [`Send`] and [`Sync`] if the type of every captured variable also implements the trait.

r[expr.closure.async]
## Async closures

r[expr.closure.async.intro]
Closures marked with the `async` keyword indicate that they are asynchronous in an analogous way to an [async function][items.fn.async].

r[expr.closure.async.future]
Calling the async closure does not perform any work, but instead evaluates to a value that implements [`Future`] that corresponds to the computation of the body of the closure.

```rust
async fn takes_async_callback(f: impl AsyncFn(u64)) {
    f(0).await;
    f(1).await;
}

async fn example() {
    takes_async_callback(async |i| {
        core::future::ready(i).await;
        println!("done with {i}.");
    }).await;
}
```

r[expr.closure.async.edition2018]
> [!EDITION-2018]
> Async closures are only available beginning with Rust 2018.

## Example

In this example, we define a function `ten_times` that takes a higher-order function argument, and we then call it with a closure expression as an argument, followed by a closure expression that moves values from its environment.

```rust
fn ten_times<F>(f: F) where F: Fn(i32) {
    for index in 0..10 {
        f(index);
    }
}

ten_times(|j| println!("hello, {}", j));
// With type annotations
ten_times(|j: i32| -> () { println!("hello, {}", j) });

let word = "konnichiwa".to_owned();
ten_times(move |j| println!("{}, {}", word, j));
```

## Attributes on closure parameters

r[expr.closure.param-attributes]
Attributes on closure parameters follow the same rules and restrictions as [regular function parameters].

[`let` binding]: ../statements.md#let-statements
[`Send`]: ../special-types-and-traits.md#send
[`Sync`]: ../special-types-and-traits.md#sync
[block]: block-expr.md
[call traits and coercions]: ../types/closure.md#call-traits-and-coercions
[closure type]: ../types/closure.md
[function definitions]: ../items/functions.md
[patterns]: ../patterns.md
[regular function parameters]: ../items/functions.md#attributes-on-function-parameters
