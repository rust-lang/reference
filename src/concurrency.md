r[concurrency]
# Concurrency

r[concurrency.intro]
Rust provides language and library features for writing concurrent programs. These features are designed to prevent [data races] --- situations in which multiple threads access the same memory without proper synchronization, with at least one of the accesses modifying that memory.

This chapter describes the traits, types, and concepts that Rust uses to express and enforce safe concurrency.

r[concurrency.send-and-sync]
## Send and Sync

r[concurrency.send-and-sync.intro]
The [`Send`] and [`Sync`] traits are `unsafe` [auto traits] used by the Rust type system to track which types can be safely used across thread boundaries.

These traits are [marker traits] with no methods. Implementing them asserts that a type has the intrinsic properties required for safe concurrent use. The compiler automatically implements these traits for most types when possible, but they can also be implemented manually. Because other unsafe code may rely on these traits being correctly implemented, providing an incorrect manual implementation can cause [undefined behavior].

r[concurrency.send-and-sync.non-implementors]
Some types, such as [`Rc`], [`UnsafeCell`], [`Cell`], and [`RefCell`], intentionally do not implement [`Send`] or [`Sync`]. These types enable unsynchronized shared mutable state, which would be unsafe to transfer or share across threads.

r[concurrency.send-and-sync.send]
### Send

r[concurrency.send-and-sync.send.intro]
The [`Send`] trait indicates that ownership of values of a type can be safely transferred between threads.

r[concurrency.send-and-sync.send.rules]
1. A type that implements [`Send`] can be moved to another thread and used there without causing data races or other [undefined behavior].
2. The Rust compiler automatically implements [`Send`] for types that satisfy its requirements (see the [`Send` documentation] for examples of types that automatically implement this trait).
3. Manually implementing [`Send`] is `unsafe`. Such implementations must ensure that moving a value of that type to another thread cannot violate Rust’s aliasing or mutability guarantees.

r[concurrency.send-and-sync.send.auto-implementors]
[`Send`] is an [auto trait]: the compiler automatically implements it for types that meet its requirements. Most primitive types, such as integers and booleans, are [`Send`]. Types composed entirely of [`Send`] components (such as structs and enums whose fields are all [`Send`]) are also [`Send`].

r[concurrency.send-and-sync.send.negative-implementation]
Types that manage non-thread-safe resources (such as raw pointers or unsynchronized interior mutability) may explicitly opt out of [`Send`] by providing a negative implementation (`!Send`).

```rust
struct SpecialThreadToken(u8);

impl !Send for SpecialThreadToken {}
```

r[concurrency.send-and-sync.sync]
### Sync

r[concurrency.send-and-sync.sync.intro]
The [`Sync`] trait indicates that references (`&T`) to a type can be safely shared between threads.

r[concurrency.send-and-sync.sync.rules]
1. If a type (`T`) is [`Sync`], then (`&T`) is [`Send`]: immutable references to type (`T`) can be sent to other threads and accessed there concurrently.
2. The Rust compiler automatically implements [`Sync`] for types that satisfy its requirements (see the [`Sync` documentation] for examples of types that automatically implement this trait).
3. Manually implementing [`Sync`] is `unsafe`. Such implementations must ensure that concurrent shared access to values of that type cannot lead to data races or other [undefined behavior].

r[concurrency.send-and-sync.sync.auto-implementors]
Like [`Send`], [`Sync`] is an [auto trait]: the compiler automatically implements it for types that meet its requirements. Most primitive types are [`Sync`], and types composed entirely of [`Sync`] components are also [`Sync`].

r[concurrency.send-and-sync.sync.negative-implementation]
Types with interior mutability that is not synchronized for concurrent access may explicitly opt out of [`Sync`] by providing a negative implementation (`!Sync`).

```rust
struct SpecialThreadToken(u8);

impl !Sync for SpecialThreadToken {}
```

r[concurrency.atomics]
## Atomics

r[concurrency.atomics.intro]
[Atomic types] allow multiple threads to safely read and write shared values without using explicit locks by providing atomic operations such as atomic loads, stores, and read-modify-write with configurable memory ordering.

r[concurrency.atomics.thread-safety]
Atomic operations are guaranteed to be indivisible: no other thread can observe a value half-written or perform a conflicting update in the middle of an atomic operation. Correct use of atomic types can prevent [data races], but misuse may still cause higher-level concurrency bugs such as deadlocks or livelocks.

r[concurrency.atomics.mapping]
The following table lists the atomic types and the corresponding primitive types they represent:

| Primitive Type | Atomic Type                           |
| -------------- | ------------------------------------- |
| `bool`         | [`core::sync::atomic::AtomicBool`]    |
| `i8`           | [`core::sync::atomic::AtomicI8`]      |
| `i16`          | [`core::sync::atomic::AtomicI16`]     |
| `i32`          | [`core::sync::atomic::AtomicI32`]     |
| `i64`          | [`core::sync::atomic::AtomicI64`]     |
| `isize`        | [`core::sync::atomic::AtomicIsize`]   |
| `u8`           | [`core::sync::atomic::AtomicU8`]      |
| `u16`          | [`core::sync::atomic::AtomicU16`]     |
| `u32`          | [`core::sync::atomic::AtomicU32`]     |
| `u64`          | [`core::sync::atomic::AtomicU64`]     |
| `usize`        | [`core::sync::atomic::AtomicUsize`]   |
| `*mut T`       | [`core::sync::atomic::AtomicPtr<T>`]  |

r[concurrency.atomics.usage]
Atomic types are [`Sync`], meaning references to them can be safely shared between threads. Using atomic operations correctly may require careful reasoning about memory ordering.

r[concurrency.asynchronous-computation]
## Asynchronous Computation

r[concurrency.asynchronous-computation.intro]
Rust provides asynchronous computation through the [`core::future::Future`] trait and [`core::task`] module. Asynchronous programming enables computations that may pause and resume without blocking the current thread.

r[concurrency.async.future]
A *future* represents a value that may not have finished computing yet. Any type implementing [`core::future::Future`] can be used as a future. Futures are *lazy*: calling an async function or an async closure returns a future but does not start computation until it is polled.

r[concurrency.async.await]
The result of a future is obtained in one of two ways:
1. Using an [`await` expression] (`future.await`), which implicitly polls the future until it is ready.
2. By explicitly invoking [`core::future::Future::poll`].

r[concurrency.async.rule]
Once a future has returned [`core::task::Poll::Ready`], it must not be polled again. Doing so may panic, block forever, or cause other kinds of problems.

r[concurrency.async.closures]
### Async Closures

Closures may be marked with the [`async` keyword], indicating that they produce a future when called. Calling an [`async` closure] does not perform its body immediately; instead, it returns a future representing the computation.

```rust
// An async function that accepts an async closure (something implementing AsyncFn(u64))
async fn takes_async_callback(f: impl AsyncFn(u64)) {
    f(0).await;
    f(1).await;
}

async fn example() {
    // Pass an async closure that prints its input
    takes_async_callback(async |i| {
        // This async closure just awaits on a ready-made future that returns its input
        core::future::ready(i).await;
        println!("done with {i}.");
    }).await;   // Await the entire `takes_async_callback` future to drive it to completion
}
```

r[concurrency.async.closures.edition2018]
> [!EDITION-2018]
> Async closures are available beginning with Rust 2018.

r[concurrency.async.closures.traits]
### Async Closure Traits

Async closures implement the [`AsyncFn`], [`AsyncFnMut`], and [`AsyncFnOnce`] traits in a manner analogous to how regular closures implement [`Fn`], [`FnMut`], and [`FnOnce`]. Which traits are implemented depends on how variables are captured and whether the returned future needs to hold onto those captures.

An `async` closure is said to be *lending* to its future if:
* It includes a mutable capture, or
* It captures a value by move (by value), except when the value is accessed only via a dereference projection.

If an `async` closure is lending to its future:
* It **does not** implement [`Fn`] or [`FnMut`].
* It **always** implements [`FnOnce`].

| Capture kind                                 | Traits implemented               |
| -------------------------------------------- | -------------------------------- |
| Only immutable borrows                       | [`AsyncFn`], [`AsyncFnMut`], [`AsyncFnOnce`] |
| Contains a mutable borrow                    | [`AsyncFnOnce`] only             |
| Moves (captures by value)                    | [`AsyncFnOnce`] only             |
| Moves, but values are accessed via `*` (dereference projection) | Same as immutable borrows ([`AsyncFn`], [`AsyncFnMut`], [`AsyncFnOnce`]) |

Examples:

Mutable capture preventing [`FnMut`]:
```rust
fn takes_callback<Fut: Future>(c: impl FnMut() -> Fut) {}

fn f() {
    let mut x = 1i32;
    let c = async || {
        x = 2; // `x` captured mutably
    };
    takes_callback(c); // ERROR: async closure does not implement `FnMut`
}
```

By-value capture preventing [`Fn`]:
```rust
fn takes_callback<Fut: Future>(c: impl Fn() -> Fut) {}

fn f() {
    let x = &1i32;
    let c = async move || {
        let a = x + 2; // `x` captured by value
    };
    takes_callback(c); // ERROR: async closure does not implement `Fn`
}
```

Dereference projection allowing [`Fn`]:
```rust
fn takes_callback<Fut: Future>(c: impl Fn() -> Fut) {}

fn f() {
    let x = &1i32;
    let c = async move || {
        let a = *x + 2; // accessed via dereference
    };
    takes_callback(c); // OK: implements `Fn`
}
```

r[concurrency.async.unsafe]
### Combining `async` and `unsafe`

It is legal to declare a function that is both `async` and `unsafe`. Such a function is `unsafe` to call and, like any other [`async` function], returns a future. The returned future is an ordinary future, and no `unsafe` context is required to `await` it.

The safety requirements of an `async unsafe fn` apply from the point of the call until the returned future has completed. This is because the body of an async function is suspended across yield points, so callers must ensure that all `unsafe` preconditions remain true for the entire duration of the returned future.

```rust
// Returns a future that, when awaited, dereferences `x`.
//
// Soundness condition: `x` must remain valid to dereference
// until the resulting future is complete.
async unsafe fn unsafe_example(x: *const i32) -> i32 {
    *x
}

async fn safe_example() {
    let p = 22;

    // An `unsafe` block is required to invoke the function:
    let future = unsafe { unsafe_example(&p) };

    // No `unsafe` block is required to await the future:
    let q = future.await;
}
```

This behavior follows from the desugaring of an `async fn` into a function that returns an `impl Future`. The `unsafe` qualifier applies to the call of that function, not to operations on the returned future.

[data races]: glossary.md#data_race
[`Send`]: special-types-and-traits.md#Send
[`Sync`]: special-types-and-traits.md#Sync
[auto traits]: special-types-and-traits.md#auto-traits
[marker traits]: glossary.md#marker-trait
[undefined behavior]: glossary.md#undefined_behavior
[`Rc`]: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html
[`UnsafeCell`]: https://doc.rust-lang.org/stable/std/cell/struct.UnsafeCell.html
[`Cell`]: https://doc.rust-lang.org/stable/std/cell/struct.Cell.html
[`RefCell`]: https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html
[`Send` documentation]: https://doc.rust-lang.org/stable/core/marker/trait.Send.html#synthetic-implementors
[`Sync` documentation]: https://doc.rust-lang.org/stable/core/marker/trait.Sync.html#synthetic-implementors
[Atomic types]: glossary.md#atomic_types
[`Mutex`]: https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html
[`RwLock`]: https://doc.rust-lang.org/stable/std/sync/struct.RwLock.html
[`core::future::Future`]: https://doc.rust-lang.org/stable/core/future/trait.Future.html
[`core::task`]: https://doc.rust-lang.org/stable/core/task/index.html
[`core::future::Future`]: https://doc.rust-lang.org/stable/core/future/trait.Future.html
[`await` expression]: expressions/await-expr.md#await-expressions
[`core::future::Future::poll`]: https://doc.rust-lang.org/stable/core/future/trait.Future.html#tymethod.poll
[`core::task::Poll::Ready`]: https://doc.rust-lang.org/stable/core/task/enum.Poll.html#variant.Ready
[`async` keyword]: https://doc.rust-lang.org/std/keyword.async.html
[`async` closure]: expressions/closure-expr.md#async-closures
[`AsyncFn`]: https://doc.rust-lang.org/stable/core/ops/trait.AsyncFn.html
[`AsyncFnMut`]: https://doc.rust-lang.org/stable/core/ops/trait.AsyncFnMut.html
[`AsyncFnOnce`]: https://doc.rust-lang.org/stable/core/ops/trait.AsyncFnOnce.html
[`Fn`]: https://doc.rust-lang.org/stable/core/ops/trait.Fn.html
[`FnMut`]: https://doc.rust-lang.org/stable/core/ops/trait.FnMut.html
[`FnOnce`]: https://doc.rust-lang.org/stable/core/ops/trait.FnOnce.html
[`async` function]: items/functions.md#async-functions
