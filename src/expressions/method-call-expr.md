# Method-call expressions

> **<sup>Syntax</sup>**\
> _MethodCallExpression_ :\
> &nbsp;&nbsp; [_Expression_] `.` [_PathExprSegment_] `(`[_CallParams_]<sup>?</sup> `)`

A _method call_ consists of an expression (the *receiver*) followed by a single dot, an expression path segment, and a parenthesized expression-list.
Method calls are resolved to associated [methods] on specific traits, either statically dispatching to a method if the exact `self`-type of the left-hand-side is known, or dynamically dispatching if the left-hand-side expression is an indirect [trait object](../types/trait-object.md).

```rust
let pi: Result<f32, _> = "3.14".parse();
let log_pi = pi.unwrap_or(1.0).log(2.72);
# assert!(1.14 < log_pi && log_pi < 1.15)
```

When looking up a method call, the receiver may be automatically dereferenced or borrowed in order to call a method.
This requires a more complex lookup process than for other functions, since there may be a number of possible methods to call.

The following procedure is used:

## Determining candidate types

First, a list of "candidate types" is assembled.

These types are found by taking the receiver type and iterating, following either:

* The built-in [dereference]; or
* `<T as Receiver>::Target`

to the next type. (If a step involved following the `Receiver` target, we also
note whether it would have been reachable by following `<T as
Deref::Target>` - this information is used later).

At the end, an additional candidate step may be added for
an [unsized coercion].

Each step of this chain provides a possible `self` type for methods that
might be called. The list will be used in two different ways:

* To find types that might have methods. This is used in the
  "determining candidate methods" step, described below. This considers
  the full list.
* To find types to which the receiver can be converted. This is used in the
  "picking a method from the candidates" step, also described below - in this
  case, we only consider the types reachable via `Deref` or built-in
  dereferencing.

There is a built-in implementation of `Receiver` for all `T: Deref`, so
most of the time, every step can be reached through either mechanism.
Sometimes, more types can be reached via the `Receiver` chain, and so
more types will be considered for the former usage than the latter usage.

For instance, if the receiver has type `Box<[i32;2]>`, then the candidate types
will be `Box<[i32;2]>`,`[i32; 2]` (by dereferencing), and `[i32]` (by unsized
coercion).

If `SmartPtr<T>: Receiver<Target=T>`, and the receiver type is `&SmartPtr<Foo>`,
then the candidate types would be `&SmartPtr<Foo>`, `SmartPtr<Foo>` and `Foo`.

## Determining candidate methods

This list of candidate types is then converted to a list of candidate methods.
For each step, the candidate type is used to determine what searches to perform:

* For a trait object, there is first a search for inherent candidates for
  the object, then inherent impl candidates for the type.
* For a struct, enum, or foreign type, there is a search for inherent
  impl candidates for the type.
* For a type param, there's a search for inherent candidates on the param.
* For various simpler types (listed below) there's a search for inherent
  candidates for the incoherent type.

After these occur, there's a further search for extension candidates for
traits in scope.

"Various simpler types" currently means bool, char, all numbers, str, array,
slices, raw pointers, references, never and tuple.

These searches contribute to list of all the candidate methods found;
separate lists are maintained for inherent and extension candidates
(that is, applicable candidates from traits). Only [visible] candidates
are included.

## Picking a method from the candidates

Once the list of candidate methods is assembled, the "picking" process
starts.

Once again, the candidate types are iterated. This time, only those types
are iterated which can be reached via the `Deref` trait or built-in derefs;
as noted above, this may be a shorter list than those that can be reached
using the `Receiver` trait.

For each step, picking is attempted in this order:

* First, a by-value method, where the `self` type precisely matches
  * First for inherent methods
  * Then for extension (trait) methods
* Then, a method where `self` is received by immutable reference (`&T`)
  * First for inherent methods
  * Then for extension (trait) methods
* Then, a method where `self` is received by mutable reference (`&mut T`)
  * First for inherent methods
  * Then for extension (trait) methods
* Then, a method where the `self` type is a `*const T` - this is only considered
  if the self type is `*mut T`
  * First for inherent methods
  * Then for extension (trait) methods
* And finally, a method with a `Pin` that's reborrowed, if the `pin_ergonomics`
  feature is enabled.
  * First for inherent methods
  * Then for extension (trait) methods

For each of those searches, if exactly one candidate is identified,
it's picked, and the search stops. If this results in multiple possible candidates,
then it is an error, and the user must [disambiguate][disambiguate call]
the call and convert the receiver to an appropriate receiver type.

With the example above of `SmartPtr<T>: Receiver<Target=T>`, and the receiver
type `&SmartPtr<Foo>`, this mechanism would pick:

```rust,ignore
impl Foo {
   fn method(self: &SmartPtr<Foo>) {}
}
```

but would not pick

```rust,ignore
impl Foo {
   fn method(self: &Foo) {}
}
```

because the receiver could not be converted to `&Foo` using the `Deref` chain,
only the `Receiver` chain.

## Extra details

There are a few details not considered in this overview:

* The search for candidate methods will also consider searches for
  incoherent types if `rustc_has_incoherent_inherent_impls` is active for
  a `dyn`, struct, enum, or foreign type.
* If there are multiple candidates from traits, they may in fact be
  identical, and the picking operation collapses them to a single pick to avoid
  reporting conflicts.
* Extra searches are performed to spot "shadowing" of pointee methods
  by smart pointer methods, during the picking process. If a by-value pick
  is going to be returned, an extra search is performed for a `&T` or
  `&mut T` method. Similarly, if a `&T` method is to be returned, an extra
  search is performed for `&mut T` methods. These extra searches consider
  only inherent methods, where `T` is identical, but the method is
  found from a step further along the `Receiver` chain. If any such method
  is found, an ambiguity error is emitted.
* An error is emitted if we reached a recursion limit.
* The picking process emits some adjustments which must be made to the
  receiver type in order to get to the correct `self` type. This includes
  a number of dereferences, a possible autoreferencing, a conversion from
  a mutable pointer to a constant pointer, or a pin reborrow.
* Extra lists are maintained for diagnostic purposes:
  unstable candidates, unsatisfied predicates, and static candidates.
* For diagnostic purposes, the search may be performed slightly differently,
  for instance searching all traits not just those in scope, or also noting
  inaccessible candidates.

## Net results

> The lookup is done for each type in order, which can occasionally lead to surprising results.
> The below code will print "In trait impl!", because `&self` methods are looked up first, the trait method is found before the struct's `&mut self` method is found.
>
> ```rust
> struct Foo {}
>
> trait Bar {
>   fn bar(&self);
> }
>
> impl Foo {
>   fn bar(&mut self) {
>     println!("In struct impl!")
>   }
> }
>
> impl Bar for Foo {
>   fn bar(&self) {
>     println!("In trait impl!")
>   }
> }
>
> fn main() {
>   let mut f = Foo{};
>   f.bar();
> }
> ```

> **Edition differences**: Before the 2021 edition, during the search for visible methods, if the candidate receiver type is an [array type], methods provided by the standard library [`IntoIterator`] trait are ignored.
>
> The edition used for this purpose is determined by the token representing the method name.
>
> This special case may be removed in the future.


> [!WARNING]
> For [trait objects], if there is an inherent method of the same name as a trait method, it will give a compiler error when trying to call the method in a method call expression.
> Instead, you can call the method using [disambiguating function call syntax], in which case it calls the trait method, not the inherent method.
> There is no way to call the inherent method.
> Just don't define inherent methods on trait objects with the same name as a trait method and you'll be fine.

[_CallParams_]: call-expr.md
[_Expression_]: ../expressions.md
[_PathExprSegment_]: ../paths.md#paths-in-expressions
[visible]: ../visibility-and-privacy.md
[array type]: ../types/array.md
[trait objects]: ../types/trait-object.md
[disambiguate call]: call-expr.md#disambiguating-function-calls
[disambiguating function call syntax]: call-expr.md#disambiguating-function-calls
[dereference]: operator-expr.md#the-dereference-operator
[methods]: ../items/associated-items.md#methods
[unsized coercion]: ../type-coercions.md#unsized-coercions
[`IntoIterator`]: std::iter::IntoIterator
