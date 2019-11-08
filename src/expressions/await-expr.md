# Await expressions

> **<sup>Syntax</sup>**\
> _AwaitExpression_ :\
> &nbsp;&nbsp; [_Expression_] `.` `await`

Await expressions are legal only within an [async context], like an
[`async fn`] or an [`async` block]. They operate on a [future]. Their effect
is to suspend the current computation until the given future is ready
to produce a value.

More specifically, an `<expr>.await` expression has the following effect.

1. Evaluate `<expr>` to a [future] `tmp`;
2. Pin `tmp` using [`Pin::new_unchecked`];
3. This pinned future is then polled by calling the [`Future::poll`] method and
   passing it the current [task context](#task-context);
3. If the call to `poll` returns [`Poll::Pending`], then the future
   returns `Poll::Pending`, suspending its state so that, when the
   surrounding async context is re-polled, execution returns to step
   2;
4. Otherwise the call to `poll` must have returned [`Poll::Ready`], in which case the
   value contained in the [`Poll::Ready`] variant is used as the result
   of the `await` expression itself.

[`async fn`]: ../items/functions.md#async-functions
[`async` block]: block-expr.md#async-blocks
[future]: ../../std/future/trait.Future.html
[_Expression_]: ../expressions.md
[`Future::poll`]: ../../std/future/trait.Future.html#tymethod.poll
[`Context`]: ../../std/task/struct.Context.html
[`Pin::new_unchecked`]: ../../std/pin/struct.Pin.html#method.new_unchecked
[`Poll::Pending`]: ../../std/task/enum.Poll.html#variant.Pending
[`Poll::Ready`]: ../../std/task/enum.Poll.html#variant.Ready

> **Edition differences**: Await expressions are only available beginning with
> Rust 2018.

## Task context

The task context refers to the [`Context`] which was supplied to the
current [async context] when the async context itself was
polled. Because `await` expressions are only legal in an async
context, there must be some task context available.

[`Context`]: ../../std/task/struct.Context.html
[async context]: ../expressions/block-expr.md#async-context

## Approximate desugaring

Effectively, an `<expr>.await` expression is roughly
equivalent to the following (this desugaring is not normative):

<!-- ignore: example expansion -->
```rust,ignore
match /* <expr> */ {
    mut pinned => loop {
        let mut pin = unsafe { Pin::new_unchecked(&mut pinned) };
        match Pin::future::poll(Pin::borrow(&mut pin), &mut current_context) {
            Poll::Ready(r) => break r,
            Poll::Pending => yield Poll::Pending,
        }
    }
}
```

where the `yield` pseudo-code returns `Poll::Pending` and, when
re-invoked, resumes execution from that point. The variable
`current_context` refers to the context taken from the async
environment.
