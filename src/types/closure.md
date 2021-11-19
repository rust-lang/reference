# Closure types

A [closure expression] produces a closure value with a unique, anonymous type
that cannot be written out. A closure type is approximately equivalent to a
struct which contains the captured values. For instance, the following
closure:

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
// Prints "Point { x: 2, y: 1 }".
```

generates a closure type roughly like the following:

<!-- ignore: simplified -->
```rust,ignore
struct Closure<'a> {
    left_top : &'a mut Point,
    right_bottom_x : &'a mut i32,
}

impl<'a> FnOnce<()> for Closure<'a> {
    type Output = String;
    fn call_once(self) -> String {
        self.left_top.x += 1;
        self.right_bottom_x += 1;
        format!("{:?}", self.left_top)
    }
}
```

so that the call to `f` works as if it were:

<!-- ignore: continuation of above -->
```rust,ignore
f(Closure{ left_top: rect.left_top, right_bottom_x: rect.left_top.x });
```

## Capture modes

The compiler prefers to capture a value by immutable borrow,
followed by unique immutable borrow (see below), by mutable borrow, and finally
by move. It will pick the first choice of these that is compatible with how the
captured value is used inside the closure body. The compiler does not take
surrounding code into account, such as the lifetimes of involved variables or fields, or
of the closure itself.

## Capture Precision

The precise path that gets captured is typically the full path that is used in the closure, but there are cases where we will only capture a prefix of the path.


### Shared prefix

In the case where a path and one of the ancestor’s of that path are both captured by a closure, the ancestor path is captured with the highest capture mode among the two captures,`CaptureMode = max(AncestorCaptureMode, DescendantCaptureMode)`, using the strict weak ordering

`ImmBorrow < UniqueImmBorrow < MutBorrow < ByValue`.

Note that this might need to be applied recursively.

```rust
# fn move_value<T>(_: T){}
let s = String::from("S");
let t = (s, String::from("T"));
let mut u = (t, String::from("U"));

let c = || {
    println!("{:?}", u); // u captured by ImmBorrow
    u.1.truncate(0); // u.0 captured by MutBorrow
    move_value(u.0.0); // u.0.0 captured by ByValue
};
```

Overall the closure will capture `u` by `ByValue`.

### Wild Card Patterns
Closures only capture data that needs to be read, which means the following closures will not capture `x`

```rust
let x = 10;
let c = || {
    let _ = x;
};

let c = || match x {
    _ => println!("Hello World!")
};
```

### Capturing references in move contexts

Moving fields out of references is not allowed. As a result, in the case of move closures, when values accessed through a shared references are moved into the closure body, the compiler will truncate right before a dereference.

```rust
struct T(String, String);

let mut t = T(String::from("foo"), String::from("bar"));
let t = &mut t;
let c = move || t.0.truncate(0); // closure captures `t`
```

### Raw pointer dereference
Because it is `unsafe` to dereference a raw pointer, closures will only capture the prefix of a path that runs up to, but not including, the first dereference of a raw pointer.

```rust,
struct T(String, String);

let t = T(String::from("foo"), String::from("bar"));
let t = &t as *const T;

let c = || unsafe {
    println!("{}", (*t).0); // closure captures t
};
```

### Reference into unaligned `struct`s

Because it is `unsafe` to hold references to unaligned fields in a structure, closures will only capture the prefix of the path that runs up to, but not including, the first field access into an unaligned structure.

```rust
#[repr(packed)]
struct T(String, String);

let t = T(String::from("foo"), String::from("bar"));
let c = || unsafe {
    println!("{}", t.0); // closure captures t
};
```


### `Box` vs other `Deref` implementations

The implementation of the [`Deref`] trait for [`Box`] is treated differently from other `Deref` implementations, as it is considered a special entity.

For example, let us look at examples involving `Rc` and `Box`. The `*rc` is desugared to a call to the trait method `deref` defined on `Rc`, but since `*box` is treated differently by the compiler, the compiler is able to do precise capture on contents of the `Box`.

[`Box`]: ../special-types-and-traits.md#boxt
[`Deref`]: ../special-types-and-traits.md#deref-and-derefmut

#### Non `move` closure

In a non `move` closure, if the contents of the `Box` are not moved into the closure body, the contents of the `Box` are precisely captured.

```rust
# use std::rc::Rc;

struct S(i32);

let b = Box::new(S(10));
let c_box = || {
    println!("{}", (*b).0); // closure captures `(*b).0`
};

let r = Rc::new(S(10));
let c_rc = || {
    println!("{}", (*r).0); // closure caprures `r`
};
```

However, if the contents of the `Box` are moved into the closure, then the box is entirely captured. This is done so the amount of data that needs to be moved into the closure is minimized.

```rust
struct S(i32);

let b = Box::new(S(10));
let c_box = || {
    let x = (*b).0; // closure captures `b`
};
```

#### `move` closure

Similarly to moving contents of a `Box` in a non-`move` closure, reading the contents of a `Box` in a `move` closure will capture the `Box` entirely.

```rust
struct S(i32);

let b = Box::new(S(10));
let c_box = || {
    println!("{}", (*b).0); // closure captures `b`
};
```

## Unique immutable borrows in captures

Captures can occur by a special kind of borrow called a _unique immutable
borrow_, which cannot be used anywhere else in the language and cannot be
written out explicitly. It occurs when modifying the referent of a mutable
reference, as in the following example:

```rust
let mut b = false;
let x = &mut b;
{
    let mut c = || { *x = true; };
    // The following line is an error:
    // let y = &x;
    c();
}
let z = &x;
```

In this case, borrowing `x` mutably is not possible, because `x` is not `mut`.
But at the same time, borrowing `x` immutably would make the assignment illegal,
because a `& &mut` reference might not be unique, so it cannot safely be used to
modify a value. So a unique immutable borrow is used: it borrows `x` immutably,
but like a mutable borrow, it must be unique. In the above example, uncommenting
the declaration of `y` will produce an error because it would violate the
uniqueness of the closure's borrow of `x`; the declaration of z is valid because
the closure's lifetime has expired at the end of the block, releasing the borrow.


## Call traits and coercions

Closure types all implement [`FnOnce`], indicating that they can be called once
by consuming ownership of the closure. Additionally, some closures implement
more specific call traits:

* A closure which does not move out of any captured variables implements
  [`FnMut`], indicating that it can be called by mutable reference.

* A closure which does not mutate or move out of any captured variables
  implements [`Fn`], indicating that it can be called by shared reference.

> Note: `move` closures may still implement [`Fn`] or [`FnMut`], even though
> they capture variables by move. This is because the traits implemented by a
> closure type are determined by what the closure does with captured values,
> not how it captures them.

*Non-capturing closures* are closures that don't capture anything from their
environment. They can be coerced to function pointers (e.g., `fn()`)
with the matching signature.

```rust
let add = |x, y| x + y;

let mut x = add(5,7);

type Binop = fn(i32, i32) -> i32;
let bo: Binop = add;
x = bo(5,7);
```

## Other traits

All closure types implement [`Sized`]. Additionally, closure types implement the
following traits if allowed to do so by the types of the captures it stores:

* [`Clone`]
* [`Copy`]
* [`Sync`]
* [`Send`]

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
[`FnMut`]: ../../std/ops/trait.FnMut.html
[`FnOnce`]: ../../std/ops/trait.FnOnce.html
[`Fn`]: ../../std/ops/trait.Fn.html
[`Send`]: ../special-types-and-traits.md#send
[`Sized`]: ../special-types-and-traits.md#sized
[`Sync`]: ../special-types-and-traits.md#sync
[closure expression]: ../expressions/closure-expr.md
[derived]: ../attributes/derive.md

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


## Overall Capture analysis algorithm

* Input:
    * Analyzing the closure C yields a mapping of `Place -> Mode` that are accessed
    * Access mode is `ref`, `ref uniq`, `ref mut`, or `by-value` (ordered least to max)
        * For a `Place` that is used in two different access modes within the same closure, the mode reported from closure analysis is the maximum access mode.
        * Note: `ByValue` use of a `Copy` type is seen as a `ref` access mode.
    * Closure mode is `ref` or `move`
* Output:
    * Minimal `(Place, Mode)` pairs that are actually captured
* Cleanup and truncation
    * Generate C' by mapping each (Mode, Place) in C:
        * `(Place1, Mode1) = ref_opt(unsafe_check(Place, Mode))`
        * `(Place2, Mode2)` = if this is a ref closure:
            * `ref_xform(Place1, Mode1)`
        * else:
            * `move_xform(Place1, Mode1)`
        * Add `(Place3, Mode3) = truncate_move_through_drop(Place2, Mode2)` to C'.
* Minimization
    * Until no rules apply:
        * For each two places (P1, M1), (P2, M2) where P1 is a prefix of P2:
            * Remove both places from the set
            * Add (P1, max(M1, M2)) into the set
* Helper functions:
    * `unsafe_check(Place, Mode) -> (Place, Mode)`
        * "Ensure unsafe accesses occur within the closure"
        * If Place contains a deref (at index `i`) of a raw pointer:
            * Let `(Place1, Mode1) = truncate_place(Place, Mode, i)`
            * Return (Place1, Mode1)
        * If Mode is `ref *` and the place contains a field of a packed struct at index `i`:
            * Let `(Place1, Mode1) = truncate_place(Place, Mode, i)`
            * Return (Place1, Mode1)
        * Else
            * Return (Place, Mode)
    * `move_xform(Place, Mode) -> (Place, Mode)` (For move closures)
        * If place contains a deref at index `i`:
            * Let `(Place1, _) = truncate_place(Place, Mode, i)`
            * Return (Place1, ByValue)
        * Else:
            * Return (Place, ByValue)
        * Note that initially we had considered an approach where "Take ownership if data being accessed is owned by the variable used to access it (or if closure attempts to move data that it doesn't own). That is when taking ownership only capture data that is found on the stack otherwise reborrow the reference.". This cause a bug around lifetimes, check [rust-lang/rust#88431](https://github.com/rust-lang/rust/issues/88431).
    * `ref_xform(Place, Mode) -> (Place, Mode)` (for ref closures)
        * "If taking ownership of data, only move data from enclosing stack frame."
        * Generate C' by mapping each (Mode, Place) in C
            * If Mode is ByValue and place contains a deref at index `i`:
                * Let `(Place1, _) = truncate_place(Place, Mode, i)`
                * Return (Place1, ByValue)
            * Else:
                * Return (Place, Mode)
    * `ref_opt(Place, Mode) -> (Place, Mode)`
        * "Optimization: borrow the ref, not data owned by ref."
        * Disjoint capture over immutable reference doesn't add too much value because the fields can either be borrowed immutably or copied.
        * Edge case: Field that is accessed via the referece lives longer than the reference.
            * Resolution: Only consider the last Deref
        * If Place is (Base, Projections), where Projections is a list of size N.
            * For all `i, 0 <= i < N`, Projections[i] != Deref
                * Return (Place, Mode)
            * If `l, 0 <= l < N` is the last/rightmost Deref Projection i.e. for any `i, l < i < N` Projection[i] != Deref,
              and `Place.type_before_projection(l) = ty::Ref(.., Mutability::Not)`
                * Let Place1 = (Base, Projections[0..=l])
                * Return (Place1, Ref)
    * `truncate_move_through_drop(Place1, Mode1) -> (Place, Mode)`
        * Rust doesn't permit moving out of a type that implements drop
        * In the case where we do a disjoint capture in a move closure, we might end up trying to move out of drop type
        * We truncate move of not-Copy types
        * If Mode1 != ByBalue
            * return (Place1, Mode1)
        * If there exists `i` such that `Place1.before_projection(i): Drop` and `Place1.ty()` doesn't impl `Copy`
            * then return `truncate_place(Place1, Mode1, i)`
        * Else return (Place1, Mode1)
        * Check [rust-lang/rust#88476](https://github.com/rust-lang/rust/issues/88476) for examples.
    * `truncate_place(Place, Mode, len) -> (Place, Mode)`
        * "Truncate the place to length `len`, i.e. upto but not including index `len`"
        * "If during truncation we drop Deref of a `&mut` and the place was being used by `ref mut`, the access to the truncated place must be unique"
        * Let (Proj_before, Proj_after) = Place.split_before(len)
            * If Mode == `ref mut` and there exists `Deref` in `Proj_after` at index `i` such that `Place.type_before_projection(len + i)` is `&mut T`
                * Return (Place(Proj_before, ..InputPlace), `ref-uniq`)
            * Else Return (Place(Proj_before, ..InputPlace), Mode)

## Key examples

### box-mut

This test shows how a `move` closure can sometimes capture values by mutable reference, if they are reached via a `&mut` reference.

```rust
struct Foo { x: i32 }

fn box_mut() {
    let mut s = Foo { x: 0 } ;

    let px = &mut s;
    let bx = Box::new(px);


    let c = move || bx.x += 10;
    // Mutable reference to this place:
    //   (*(*bx)).x
    //    ^ ^
    //    | a Box
    //    a &mut
}
```

<!-- ignore: Omit error about unterminated string literal when representing c_prime -->
```ignore
Closure mode = move
C_in = {
    (ref mut, (*(*bx)).x)
}
C_out = C_in
```

Output is the same: `C' = C`

### Packed-field-ref-and-move

When you have a closure that both references a packed field (which is unsafe) and moves from it (which is safe) we capture the entire struct, rather than just moving the field. This is to aid in predictability, so that removing the move doesn't make the closure become unsafe:

```rust
#[repr(packed)]
struct Packed { x: String }

# fn use_ref<T>(_: &T) {}
# fn move_value<T>(_: T) {}

fn main() {
    let packed = Packed { x: String::new() };

    let c = || {
        use_ref(&packed.x);
        move_value(packed.x);
    };

    c();
}
```

<!-- ignore: Omit error about unterminated string literal when representing c_prime -->
```ignore
Closure mode = ref
C_in = {
    (ref mut, packed)
}
C_out = C_in
```

### Optimization-Edge-Case

This test shows an interesting edge case. Normally, when we see a borrow of something behind a shared reference (`&T`), we truncate to capture the entire reference, because that is more efficient (and we can always use that reference to reach all the data it refers to). However, in the case where we are dereferencing two shared references, we have to be sure to preserve the full path, since otherwise the resulting closure could have a shorter lifetime than is necessary.

```edition2021
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

<!-- ignore: Omit error about unterminated string literal when reprenting c_prime -->
```ignore
Closure mode = ref
C_in = {
    (ref mut, *m.a)
}
C_out = C_in
```

# Edition 2018 and before

## Closure types difference

In Edition 2018 and before, a closure would capture variables in its entirety. This means that for the example used in the [Closure types](#closure-types) section, the generated closure type would instead look something like this:

<!-- ignore: simplified -->
```rust,ignore
struct Closure<'a> {
    rect : &'a mut Rectangle,
}

impl<'a> FnOnce<()> for Closure<'a> {
    type Output = String;
    fn call_once(self) -> String {
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

## Capture precision difference

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

If, instead, the closure were to use `self.vec` directly, then it would attempt
to capture `self` by mutable reference. But since `self.set` is already
borrowed to iterate over, the code would not compile.

If the `move` keyword is used, then all captures are by move or, for `Copy`
types, by copy, regardless of whether a borrow would work. The `move` keyword is
usually used to allow the closure to outlive the captured values, such as if the
closure is being returned or used to spawn a new thread.

Regardless of if the data will be read by the closure, i.e. in case of wild card patterns, if a variable defined outside the closure is mentioned within the closure the variable will be captured in its entirety.

## Drop order difference

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
