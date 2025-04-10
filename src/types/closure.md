r[type.closure]
# Closure types

r[type.closure.intro]
A [closure expression] produces a closure value with a unique, anonymous type that cannot be written out.
A closure type is approximately equivalent to a struct which contains the captured values.
For instance, the following closure:

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }
struct Rectangle { left_top: Point, right_bottom: Point }

fn f<F : FnOnce() -> String> (g: F) {
    println!("{}", g());
}

let mut rect = Rectangle {
    left_top: Point { x: 1, y: 1 },
    right_bottom: Point { x: 0, y: 0 }
};

let c = || {
    rect.left_top.x += 1;
    rect.right_bottom.x += 1;
    format!("{:?}", rect.left_top)
};
f(c); // Prints "Point { x: 2, y: 1 }".
```

generates a closure type roughly like the following:

<!-- ignore: simplified -->
```rust,ignore
// Note: This is not exactly how it is translated, this is only for
// illustration.

struct Closure<'a> {
    left_top : &'a mut Point,
    right_bottom_x : &'a mut i32,
}

impl<'a> FnOnce<()> for Closure<'a> {
    type Output = String;
    extern "rust-call" fn call_once(self, args: ()) -> String {
        self.left_top.x += 1;
        *self.right_bottom_x += 1;
        format!("{:?}", self.left_top)
    }
}
```

so that the call to `f` works as if it were:

<!-- ignore: continuation of above -->
```rust,ignore
f(Closure{ left_top: &mut rect.left_top, right_bottom_x: &mut rect.right_bottom.x });
```

r[type.closure.capture]
## Capture modes

r[type.closure.capture.intro]
A *capture mode* determines how a [place expression] from the environment is borrowed or moved into the closure.
The capture modes are:

1. Immutable borrow (`ImmBorrow`) --- The place expression is captured as a [shared reference].
2. Unique immutable borrow (`UniqueImmBorrow`) --- This is similar to an immutable borrow, but must be unique as described [below](#unique-immutable-borrows-in-captures).
3. Mutable borrow (`MutBorrow`) --- The place expression is captured as a [mutable reference].
4. Move (`ByValue`) --- The place expression is captured by [moving the value] into the closure.

r[type.closure.capture.precedence]
Place expressions from the environment are captured from the first mode that is compatible with how the captured value is used inside the closure body.
The mode is not affected by the code surrounding the closure, such as the lifetimes of involved variables or fields, or of the closure itself.

[moving the value]: ../expressions.md#moved-and-copied-types
[mutable reference]: pointer.md#mutable-references-mut
[place expression]: ../expressions.md#place-expressions-and-value-expressions
[shared reference]: pointer.md#references--and-mut

r[type.closure.capture.copy]
### `Copy` values

Values that implement [`Copy`] that are moved into the closure are captured with the `ImmBorrow` mode.

```rust
let x = [0; 1024];
let c = || {
    let y = x; // x captured by ImmBorrow
};
```

r[type.closure.async.input]
### Async input capture

Async closures always capture all input arguments, regardless of whether or not they are used within the body.

## Capture Precision

r[type.closure.capture.precision.capture-path]
A *capture path* is a sequence starting with a variable from the environment followed by zero or more place projections that were applied to that variable.

r[type.closure.capture.precision.place-projection]
A *place projection* is a [field access], [tuple index], [dereference] (and automatic dereferences), or [array or slice index] expression applied to a variable.

r[type.closure.capture.precision.intro]
The closure borrows or moves the capture path, which may be truncated based on the rules described below.

For example:

```rust
struct SomeStruct {
    f1: (i32, i32),
}
let s = SomeStruct { f1: (1, 2) };

let c = || {
    let x = s.f1.1; // s.f1.1 captured by ImmBorrow
};
c();
```

Here the capture path is the local variable `s`, followed by a field access `.f1`, and then a tuple index `.1`.
This closure captures an immutable borrow of `s.f1.1`.

[field access]: ../expressions/field-expr.md
[tuple index]: ../expressions/tuple-expr.md#tuple-indexing-expressions
[dereference]: ../expressions/operator-expr.md#the-dereference-operator
[array or slice index]: ../expressions/array-expr.md#array-and-slice-indexing-expressions

r[type.closure.capture.precision.shared-prefix]
### Shared prefix

In the case where a capture path and one of the ancestor’s of that path are both captured by a closure, the ancestor path is captured with the highest capture mode among the two captures, `CaptureMode = max(AncestorCaptureMode, DescendantCaptureMode)`, using the strict weak ordering:

`ImmBorrow < UniqueImmBorrow < MutBorrow < ByValue`

Note that this might need to be applied recursively.

```rust
// In this example, there are three different capture paths with a shared ancestor:
# fn move_value<T>(_: T){}
let s = String::from("S");
let t = (s, String::from("T"));
let mut u = (t, String::from("U"));

let c = || {
    println!("{:?}", u); // u captured by ImmBorrow
    u.1.truncate(0); // u.0 captured by MutBorrow
    move_value(u.0.0); // u.0.0 captured by ByValue
};
c();
```

Overall this closure will capture `u` by `ByValue`.

r[type.closure.capture.precision.dereference-shared]
### Rightmost shared reference truncation

The capture path is truncated at the rightmost dereference in the capture path if the dereference is applied to a shared reference.

This truncation is allowed because fields that are read through a shared reference will always be read via a shared reference or a copy.
This helps reduce the size of the capture when the extra precision does not yield any benefit from a borrow checking perspective.

The reason it is the *rightmost* dereference is to help avoid a shorter lifetime than is necessary.
Consider the following example:

```rust
struct Int(i32);
struct B<'a>(&'a i32);

struct MyStruct<'a> {
   a: &'static Int,
   b: B<'a>,
}

fn foo<'a, 'b>(m: &'a MyStruct<'b>) -> impl FnMut() + 'static {
    let c = || drop(&m.a.0);
    c
}
```

If this were to capture `m`, then the closure would no longer outlive `'static`, since `m` is constrained to `'a`. Instead, it captures `(*(*m).a)` by `ImmBorrow`.

r[type.closure.capture.precision.wildcard]
### Wildcard pattern bindings

Closures only capture data that needs to be read.
Binding a value with a [wildcard pattern] does not count as a read, and thus won't be captured.
For example, the following closures will not capture `x`:

```rust
let x = String::from("hello");
let c = || {
    let _ = x;  // x is not captured
};
c();

let c = || match x {  // x is not captured
    _ => println!("Hello World!")
};
c();
```

This also includes destructuring of tuples, structs, and enums.
Fields matched with the [RestPattern] or [StructPatternEtCetera] are also not considered as read, and thus those fields will not be captured.
The following illustrates some of these:

```rust
let x = (String::from("a"), String::from("b"));
let c = || {
    let (first, ..) = x;  // captures `x.0` ByValue
};
// The first tuple field has been moved into the closure.
// The second tuple field is still accessible.
println!("{:?}", x.1);
c();
```

```rust
struct Example {
    f1: String,
    f2: String,
}

let e = Example {
    f1: String::from("first"),
    f2: String::from("second"),
};
let c = || {
    let Example { f2, .. } = e; // captures `e.f2` ByValue
};
// Field f2 cannot be accessed since it is moved into the closure.
// Field f1 is still accessible.
println!("{:?}", e.f1);
c();
```

r[type.closure.capture.precision.wildcard.array-slice]
Partial captures of arrays and slices are not supported; the entire slice or array is always captured even if used with wildcard pattern matching, indexing, or sub-slicing.
For example:

```rust,compile_fail,E0382
#[derive(Debug)]
struct Example;
let x = [Example, Example];

let c = || {
    let [first, _] = x; // captures all of `x` ByValue
};
c();
println!("{:?}", x[1]); // ERROR: borrow of moved value: `x`
```

r[type.closure.capture.precision.wildcard.initialized]
Values that are matched with wildcards must still be initialized.

```rust,compile_fail,E0381
let x: i32;
let c = || {
    let _ = x; // ERROR: used binding `x` isn't initialized
};
```

[wildcard pattern]: ../patterns.md#wildcard-pattern

r[type.closure.capture.precision.move-dereference]
### Capturing references in move contexts

Because it is not allowed to move fields out of a reference, `move` closures will only capture the prefix of a capture path that runs up to, but not including, the first dereference of a reference.
The reference itself will be moved into the closure.

```rust
struct T(String, String);

let mut t = T(String::from("foo"), String::from("bar"));
let t_mut_ref = &mut t;
let mut c = move || {
    t_mut_ref.0.push_str("123"); // captures `t_mut_ref` ByValue
};
c();
```

r[type.closure.capture.precision.raw-pointer-dereference]
### Raw pointer dereference

Because it is `unsafe` to dereference a raw pointer, closures will only capture the prefix of a capture path that runs up to, but not including, the first dereference of a raw pointer.

```rust
struct T(String, String);

let t = T(String::from("foo"), String::from("bar"));
let t_ptr = &t as *const T;

let c = || unsafe {
    println!("{}", (*t_ptr).0); // captures `t_ptr` by ImmBorrow
};
c();
```

r[type.closure.capture.precision.union]
### Union fields

Because it is `unsafe` to access a union field, closures will only capture the prefix of a capture path that runs up to the union itself.

```rust
union U {
    a: (i32, i32),
    b: bool,
}
let u = U { a: (123, 456) };

let c = || {
    let x = unsafe { u.a.0 }; // captures `u` ByValue
};
c();

// This also includes writing to fields.
let mut u = U { a: (123, 456) };

let mut c = || {
    u.b = true; // captures `u` with MutBorrow
};
c();
```

r[type.closure.capture.precision.unaligned]
### Reference into unaligned `struct`s

Because it is [undefined behavior] to create references to unaligned fields in a structure,
closures will only capture the prefix of the capture path that runs up to, but not including, the first field access into a structure that uses [the `packed` representation].
This includes all fields, even those that are aligned, to protect against compatibility concerns should any of the fields in the structure change in the future.

```rust
#[repr(packed)]
struct T(i32, i32);

let t = T(2, 5);
let c = || {
    let a = t.0; // captures `t` with ImmBorrow
};
// Copies out of `t` are ok.
let (a, b) = (t.0, t.1);
c();
```

Similarly, taking the address of an unaligned field also captures the entire struct:

```rust,compile_fail,E0505
#[repr(packed)]
struct T(String, String);

let mut t = T(String::new(), String::new());
let c = || {
    let a = std::ptr::addr_of!(t.1); // captures `t` with ImmBorrow
};
let a = t.0; // ERROR: cannot move out of `t.0` because it is borrowed
c();
```

but the above works if it is not packed since it captures the field precisely:

```rust
struct T(String, String);

let mut t = T(String::new(), String::new());
let c = || {
    let a = std::ptr::addr_of!(t.1); // captures `t.1` with ImmBorrow
};
// The move here is allowed.
let a = t.0;
c();
```

[undefined behavior]: ../behavior-considered-undefined.md
[the `packed` representation]: ../type-layout.md#the-alignment-modifiers

r[type.closure.capture.precision.box-deref]
### `Box` vs other `Deref` implementations

The implementation of the [`Deref`] trait for [`Box`] is treated differently from other `Deref` implementations, as it is considered a special entity.

For example, let us look at examples involving `Rc` and `Box`. The `*rc` is desugared to a call to the trait method `deref` defined on `Rc`, but since `*box` is treated differently, it is possible to do a precise capture of the contents of the `Box`.

[`Box`]: ../special-types-and-traits.md#boxt
[`Deref`]: ../special-types-and-traits.md#deref-and-derefmut

r[type.closure.capture.precision.box-non-move.not-moved]
#### `Box` with non-`move` closure

In a non-`move` closure, if the contents of the `Box` are not moved into the closure body, the contents of the `Box` are precisely captured.

```rust
struct S(String);

let b = Box::new(S(String::new()));
let c_box = || {
    let x = &(*b).0; // captures `(*b).0` by ImmBorrow
};
c_box();

// Contrast `Box` with another type that implements Deref:
let r = std::rc::Rc::new(S(String::new()));
let c_rc = || {
    let x = &(*r).0; // captures `r` by ImmBorrow
};
c_rc();
```

r[type.closure.capture.precision.box-non-move.moved]
However, if the contents of the `Box` are moved into the closure, then the box is entirely captured. This is done so the amount of data that needs to be moved into the closure is minimized.

```rust
// This is the same as the example above except the closure
// moves the value instead of taking a reference to it.

struct S(String);

let b = Box::new(S(String::new()));
let c_box = || {
    let x = (*b).0; // captures `b` with ByValue
};
c_box();
```

r[type.closure.capture.precision.box-move.read]
#### `Box` with move closure

Similarly to moving contents of a `Box` in a non-`move` closure, reading the contents of a `Box` in a `move` closure will capture the `Box` entirely.

```rust
struct S(i32);

let b = Box::new(S(10));
let c_box = move || {
    let x = (*b).0; // captures `b` with ByValue
};
```

r[type.closure.unique-immutable]
## Unique immutable borrows in captures

Captures can occur by a special kind of borrow called a _unique immutable borrow_,
which cannot be used anywhere else in the language and cannot be written out explicitly.
It occurs when modifying the referent of a mutable reference, as in the following example:

```rust
let mut b = false;
let x = &mut b;
let mut c = || {
    // An ImmBorrow and a MutBorrow of `x`.
    let a = &x;
    *x = true; // `x` captured by UniqueImmBorrow
};
// The following line is an error:
// let y = &x;
c();
// However, the following is OK.
let z = &x;
```

In this case, borrowing `x` mutably is not possible, because `x` is not `mut`.
But at the same time, borrowing `x` immutably would make the assignment illegal,
because a `& &mut` reference might not be unique, so it cannot safely be used to modify a value.
So a unique immutable borrow is used: it borrows `x` immutably, but like a mutable borrow, it must be unique.

In the above example, uncommenting the declaration of `y` will produce an error because it would violate the uniqueness of the closure's borrow of `x`; the declaration of z is valid because the closure's lifetime has expired at the end of the block, releasing the borrow.

r[type.closure.call]
## Call traits and coercions

r[type.closure.call.intro]
Closure types all implement [`FnOnce`], indicating that they can be called once
by consuming ownership of the closure. Additionally, some closures implement
more specific call traits:

r[type.closure.call.fn-mut]
* A closure which does not move out of any captured variables implements
  [`FnMut`], indicating that it can be called by mutable reference.

r[type.closure.call.fn]
* A closure which does not mutate or move out of any captured variables
  implements [`Fn`], indicating that it can be called by shared reference.

> [!NOTE]
> `move` closures may still implement [`Fn`] or [`FnMut`], even though they capture variables by move. This is because the traits implemented by a closure type are determined by what the closure does with captured values, not how it captures them.

r[type.closure.non-capturing]
*Non-capturing closures* are closures that don't capture anything from their
environment. Non-async, non-capturing closures can be coerced to function pointers (e.g., `fn()`)
with the matching signature.

```rust
let add = |x, y| x + y;

let mut x = add(5,7);

type Binop = fn(i32, i32) -> i32;
let bo: Binop = add;
x = bo(5,7);
```

r[type.closure.async.traits]
### Async closure traits

r[type.closure.async.traits.fn-family]
Async closures have a further restriction of whether or not they implement [`FnMut`] or [`Fn`].

The [`Future`] returned by the async closure has similar capturing characteristics as a closure. It captures place expressions from the async closure based on how they are used. The async closure is said to be *lending* to its [`Future`] if it has either of the following properties:

- The `Future` includes a mutable capture.
- The async closure captures by value, except when the value is accessed with a dereference projection.

If the async closure is lending to its `Future`, then [`FnMut`] and [`Fn`] are *not* implemented. [`FnOnce`] is always implemented.

> **Example**: The first clause for a mutable capture can be illustrated with the following:
>
> ```rust,compile_fail
> fn takes_callback<Fut: Future>(c: impl FnMut() -> Fut) {}
>
> fn f() {
>     let mut x = 1i32;
>     let c = async || {
>         x = 2;  // x captured with MutBorrow
>     };
>     takes_callback(c);  // ERROR: async closure does not implement `FnMut`
> }
> ```
>
> The second clause for a regular value capture can be illustrated with the following:
>
> ```rust,compile_fail
> fn takes_callback<Fut: Future>(c: impl Fn() -> Fut) {}
>
> fn f() {
>     let x = &1i32;
>     let c = async move || {
>         let a = x + 2;  // x captured ByValue
>     };
>     takes_callback(c);  // ERROR: async closure does not implement `Fn`
> }
> ```
>
> The exception of the the second clause can be illustrated by using a dereference, which does allow `Fn` and `FnMut` to be implemented:
>
> ```rust
> fn takes_callback<Fut: Future>(c: impl Fn() -> Fut) {}
>
> fn f() {
>     let x = &1i32;
>     let c = async move || {
>         let a = *x + 2;
>     };
>     takes_callback(c);  // OK: implements `Fn`
> }
> ```

r[type.closure.async.traits.async-family]
Async closures implement [`AsyncFn`], [`AsyncFnMut`], and [`AsyncFnOnce`] in an analogous way as regular closures implement [`Fn`], [`FnMut`], and [`FnOnce`]; that is, depending on the use of the captured variables in its body.

r[type.closure.traits]
### Other traits

r[type.closure.traits.intro]
All closure types implement [`Sized`]. Additionally, closure types implement the
following traits if allowed to do so by the types of the captures it stores:

* [`Clone`]
* [`Copy`]
* [`Sync`]
* [`Send`]

r[type.closure.traits.behavior]
The rules for [`Send`] and [`Sync`] match those for normal struct types, while
[`Clone`] and [`Copy`] behave as if [derived]. For [`Clone`], the order of
cloning of the captured values is left unspecified.

Because captures are often by reference, the following general rules arise:

* A closure is [`Sync`] if all captured values are [`Sync`].
* A closure is [`Send`] if all values captured by non-unique immutable
  reference are [`Sync`], and all values captured by unique immutable or mutable
  reference, copy, or move are [`Send`].
* A closure is [`Clone`] or [`Copy`] if it does not capture any values by
  unique immutable or mutable reference, and if all values it captures by copy
  or move are [`Clone`] or [`Copy`], respectively.

[`Clone`]: ../special-types-and-traits.md#clone
[`Copy`]: ../special-types-and-traits.md#copy
[`Send`]: ../special-types-and-traits.md#send
[`Sized`]: ../special-types-and-traits.md#sized
[`Sync`]: ../special-types-and-traits.md#sync
[closure expression]: ../expressions/closure-expr.md
[derived]: ../attributes/derive.md

r[type.closure.drop-order]
## Drop Order

If a closure captures a field of a composite types such as structs, tuples, and enums by value, the field's lifetime would now be tied to the closure. As a result, it is possible for disjoint fields of a composite types to be dropped at different times.

```rust
{
    let tuple =
      (String::from("foo"), String::from("bar")); // --+
    { //                                               |
        let c = || { // ----------------------------+  |
            // tuple.0 is captured into the closure |  |
            drop(tuple.0); //                       |  |
        }; //                                       |  |
    } // 'c' and 'tuple.0' dropped here ------------+  |
} // tuple.1 dropped here -----------------------------+
```

r[type.closure.capture.precision.edition2018.entirety]
## Edition 2018 and before

### Closure types difference

In Edition 2018 and before, closures always capture a variable in its entirety, without its precise capture path. This means that for the example used in the [Closure types](#closure-types) section, the generated closure type would instead look something like this:

<!-- ignore: simplified -->
```rust,ignore
struct Closure<'a> {
    rect : &'a mut Rectangle,
}

impl<'a> FnOnce<()> for Closure<'a> {
    type Output = String;
    extern "rust-call" fn call_once(self, args: ()) -> String {
        self.rect.left_top.x += 1;
        self.rect.right_bottom.x += 1;
        format!("{:?}", self.rect.left_top)
    }
}
```

and the call to `f` would work as follows:

<!-- ignore: continuation of above -->
```rust,ignore
f(Closure { rect: rect });
```

r[type.closure.capture.precision.edition2018.composite]
### Capture precision difference

Composite types such as structs, tuples, and enums are always captured in its entirety,
not by individual fields. As a result, it may be necessary to borrow into a local variable in order to capture a single field:

```rust
# use std::collections::HashSet;
#
struct SetVec {
    set: HashSet<u32>,
    vec: Vec<u32>
}

impl SetVec {
    fn populate(&mut self) {
        let vec = &mut self.vec;
        self.set.iter().for_each(|&n| {
            vec.push(n);
        })
    }
}
```

If, instead, the closure were to use `self.vec` directly, then it would attempt to capture `self` by mutable reference. But since `self.set` is already borrowed to iterate over, the code would not compile.

r[type.closure.capture.precision.edition2018.move]
If the `move` keyword is used, then all captures are by move or, for `Copy` types, by copy, regardless of whether a borrow would work. The `move` keyword is usually used to allow the closure to outlive the captured values, such as if the closure is being returned or used to spawn a new thread.

r[type.closure.capture.precision.edition2018.wildcard]
Regardless of if the data will be read by the closure, i.e. in case of wild card patterns, if a variable defined outside the closure is mentioned within the closure the variable will be captured in its entirety.

r[type.closure.capture.precision.edition2018.drop-order]
### Drop order difference

As composite types are captured in their entirety, a closure which captures one of those composite types by value would drop the entire captured variable at the same time as the closure gets dropped.

```rust
{
    let tuple =
      (String::from("foo"), String::from("bar"));
    {
        let c = || { // --------------------------+
            // tuple is captured into the closure |
            drop(tuple.0); //                     |
        }; //                                     |
    } // 'c' and 'tuple' dropped here ------------+
}
```
