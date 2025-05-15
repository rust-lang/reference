r[items.traits]
# Traits

r[items.traits.syntax]
```grammar,items
Trait ->
    `unsafe`? `trait` IDENTIFIER GenericParams? ( `:` TypeParamBounds? )? WhereClause?
    `{`
        InnerAttribute*
        AssociatedItem*
    `}`
```

r[items.traits.intro]
A _trait_ describes an abstract interface that types can implement. This
interface consists of [associated items], which come in three varieties:

- [functions](associated-items.md#associated-functions-and-methods)
- [types](associated-items.md#associated-types)
- [constants](associated-items.md#associated-constants)

r[items.traits.namespace]
The trait declaration defines a trait in the [type namespace] of the module or block where it is located.

r[items.traits.associated-item-namespaces]
Associated items are defined as members of the trait within their respective namespaces. Associated types are defined in the type namespace. Associated constants and associated functions are defined in the value namespace.

r[items.traits.self-param]
All traits define an implicit type parameter `Self` that refers to "the type
that is implementing this interface". Traits may also contain additional type
parameters. These type parameters, including `Self`, may be constrained by
other traits and so forth [as usual][generics].

r[items.traits.impls]
Traits are implemented for specific types through separate [implementations].

r[items.traits.associated-item-decls]
Trait functions may omit the function body by replacing it with a semicolon.
This indicates that the implementation must define the function. If the trait
function defines a body, this definition acts as a default for any
implementation which does not override it. Similarly, associated constants may
omit the equals sign and expression to indicate implementations must define
the constant value. Associated types must never define the type, the type may
only be specified in an implementation.

```rust
// Examples of associated trait items with and without definitions.
trait Example {
    const CONST_NO_DEFAULT: i32;
    const CONST_WITH_DEFAULT: i32 = 99;
    type TypeNoDefault;
    fn method_without_default(&self);
    fn method_with_default(&self) {}
}
```

r[items.traits.const-fn]
Trait functions are not allowed to be [`const`].

r[items.traits.bounds]
## Trait bounds

Generic items may use traits as [bounds] on their type parameters.

r[items.traits.generic]
## Generic traits

Type parameters can be specified for a trait to make it generic. These appear
after the trait name, using the same syntax used in [generic functions].

```rust
trait Seq<T> {
    fn len(&self) -> u32;
    fn elt_at(&self, n: u32) -> T;
    fn iter<F>(&self, f: F) where F: Fn(T);
}
```

<a id="object-safety"></a>
r[items.traits.dyn-compatible]
## Dyn compatibility

r[items.traits.dyn-compatible.intro]
A dyn-compatible trait can be the base trait of a [trait object]. A trait is
*dyn compatible* if it has the following qualities:

r[items.traits.dyn-compatible.supertraits]
* All [supertraits] must also be dyn compatible.

r[items.traits.dyn-compatible.sized]
* `Sized` must not be a [supertrait][supertraits]. In other words, it must not require `Self: Sized`.

r[items.traits.dyn-compatible.associated-consts]
* It must not have any associated constants.

r[items.traits.dyn-compatible.associated-types]
* It must not have any associated types with generics.

r[items.traits.dyn-compatible.associated-functions]
* All associated functions must either be dispatchable from a trait object or be explicitly non-dispatchable:
    * Dispatchable functions must:
        * Not have any type parameters (although lifetime parameters are allowed).
        * Be a [method] that does not use `Self` except in the type of the receiver.
        * Have a receiver with one of the following types:
            * `&Self` (i.e. `&self`)
            * `&mut Self` (i.e `&mut self`)
            * [`Box<Self>`]
            * [`Rc<Self>`]
            * [`Arc<Self>`]
            * [`Pin<P>`] where `P` is one of the types above
        * Not have an opaque return type; that is,
            * Not be an `async fn` (which has a hidden `Future` type).
            * Not have a return position `impl Trait` type (`fn example(&self) -> impl Trait`).
        * Not have a `where Self: Sized` bound (receiver type of `Self` (i.e. `self`) implies this).
    * Explicitly non-dispatchable functions require:
        * Have a `where Self: Sized` bound (receiver type of `Self` (i.e. `self`) implies this).

r[items.traits.dyn-compatible.async-traits]
* The [`AsyncFn`], [`AsyncFnMut`], and [`AsyncFnOnce`] traits are not dyn-compatible.

> [!NOTE]
> This concept was formerly known as *object safety*.

```rust
# use std::rc::Rc;
# use std::sync::Arc;
# use std::pin::Pin;
// Examples of dyn compatible methods.
trait TraitMethods {
    fn by_ref(self: &Self) {}
    fn by_ref_mut(self: &mut Self) {}
    fn by_box(self: Box<Self>) {}
    fn by_rc(self: Rc<Self>) {}
    fn by_arc(self: Arc<Self>) {}
    fn by_pin(self: Pin<&Self>) {}
    fn with_lifetime<'a>(self: &'a Self) {}
    fn nested_pin(self: Pin<Arc<Self>>) {}
}
# struct S;
# impl TraitMethods for S {}
# let t: Box<dyn TraitMethods> = Box::new(S);
```

```rust,compile_fail
// This trait is dyn compatible, but these methods cannot be dispatched on a trait object.
trait NonDispatchable {
    // Non-methods cannot be dispatched.
    fn foo() where Self: Sized {}
    // Self type isn't known until runtime.
    fn returns(&self) -> Self where Self: Sized;
    // `other` may be a different concrete type of the receiver.
    fn param(&self, other: Self) where Self: Sized {}
    // Generics are not compatible with vtables.
    fn typed<T>(&self, x: T) where Self: Sized {}
}

struct S;
impl NonDispatchable for S {
    fn returns(&self) -> Self where Self: Sized { S }
}
let obj: Box<dyn NonDispatchable> = Box::new(S);
obj.returns(); // ERROR: cannot call with Self return
obj.param(S);  // ERROR: cannot call with Self parameter
obj.typed(1);  // ERROR: cannot call with generic type
```

```rust,compile_fail
# use std::rc::Rc;
// Examples of dyn-incompatible traits.
trait DynIncompatible {
    const CONST: i32 = 1;  // ERROR: cannot have associated const

    fn foo() {}  // ERROR: associated function without Sized
    fn returns(&self) -> Self; // ERROR: Self in return type
    fn typed<T>(&self, x: T) {} // ERROR: has generic type parameters
    fn nested(self: Rc<Box<Self>>) {} // ERROR: nested receiver cannot be downcasted
}

struct S;
impl DynIncompatible for S {
    fn returns(&self) -> Self { S }
}
let obj: Box<dyn DynIncompatible> = Box::new(S); // ERROR
```

```rust,compile_fail
// `Self: Sized` traits are dyn-incompatible.
trait TraitWithSize where Self: Sized {}

struct S;
impl TraitWithSize for S {}
let obj: Box<dyn TraitWithSize> = Box::new(S); // ERROR
```

```rust,compile_fail
// Dyn-incompatible if `Self` is a type argument.
trait Super<A> {}
trait WithSelf: Super<Self> where Self: Sized {}

struct S;
impl<A> Super<A> for S {}
impl WithSelf for S {}
let obj: Box<dyn WithSelf> = Box::new(S); // ERROR: cannot use `Self` type parameter
```

r[items.traits.supertraits]
## Supertraits

r[items.traits.supertraits.intro]
**Supertraits** are traits that are required to be implemented for a type to
implement a specific trait. Furthermore, anywhere a [generic][generics] or [trait object]
is bounded by a trait, it has access to the associated items of its supertraits.

r[items.traits.supertraits.decl]
Supertraits are declared by trait bounds on the `Self` type of a trait and
transitively the supertraits of the traits declared in those trait bounds. It is
an error for a trait to be its own supertrait.

r[items.traits.supertraits.subtrait]
The trait with a supertrait is called a **subtrait** of its supertrait.

The following is an example of declaring `Shape` to be a supertrait of `Circle`.

```rust
trait Shape { fn area(&self) -> f64; }
trait Circle: Shape { fn radius(&self) -> f64; }
```

And the following is the same example, except using [where clauses].

```rust
trait Shape { fn area(&self) -> f64; }
trait Circle where Self: Shape { fn radius(&self) -> f64; }
```

This next example gives `radius` a default implementation using the `area`
function from `Shape`.

```rust
# trait Shape { fn area(&self) -> f64; }
trait Circle where Self: Shape {
    fn radius(&self) -> f64 {
        // A = pi * r^2
        // so algebraically,
        // r = sqrt(A / pi)
        (self.area() / std::f64::consts::PI).sqrt()
    }
}
```

This next example calls a supertrait method on a generic parameter.

```rust
# trait Shape { fn area(&self) -> f64; }
# trait Circle: Shape { fn radius(&self) -> f64; }
fn print_area_and_radius<C: Circle>(c: C) {
    // Here we call the area method from the supertrait `Shape` of `Circle`.
    println!("Area: {}", c.area());
    println!("Radius: {}", c.radius());
}
```

Similarly, here is an example of calling supertrait methods on trait objects.

```rust
# trait Shape { fn area(&self) -> f64; }
# trait Circle: Shape { fn radius(&self) -> f64; }
# struct UnitCircle;
# impl Shape for UnitCircle { fn area(&self) -> f64 { std::f64::consts::PI } }
# impl Circle for UnitCircle { fn radius(&self) -> f64 { 1.0 } }
# let circle = UnitCircle;
let circle = Box::new(circle) as Box<dyn Circle>;
let nonsense = circle.radius() * circle.area();
```

r[items.traits.safety]
## Unsafe traits

r[items.traits.safety.intro]
Traits items that begin with the `unsafe` keyword indicate that *implementing* the
trait may be [unsafe]. It is safe to use a correctly implemented unsafe trait.
The [trait implementation] must also begin with the `unsafe` keyword.

[`Sync`] and [`Send`] are examples of unsafe traits.

r[items.traits.params]
## Parameter patterns

r[items.traits.params.patterns-no-body]
Parameters in associated functions without a body only allow [IDENTIFIER] or `_` [wild card][WildcardPattern] patterns, as well as the form allowed by [SelfParam]. `mut` [IDENTIFIER] is currently allowed, but it is deprecated and will become a hard error in the future.
<!-- https://github.com/rust-lang/rust/issues/35203 -->

```rust
trait T {
    fn f1(&self);
    fn f2(x: Self, _: i32);
}
```

```rust,compile_fail,E0642
trait T {
    fn f2(&x: &i32); // ERROR: patterns aren't allowed in functions without bodies
}
```

r[items.traits.params.patterns-with-body]
Parameters in associated functions with a body only allow irrefutable patterns.

```rust
trait T {
    fn f1((a, b): (i32, i32)) {} // OK: is irrefutable
}
```

```rust,compile_fail,E0005
trait T {
    fn f1(123: i32) {} // ERROR: pattern is refutable
    fn f2(Some(x): Option<i32>) {} // ERROR: pattern is refutable
}
```

r[items.traits.params.pattern-required.edition2018]
> [!EDITION-2018]
> Prior to the 2018 edition, the pattern for an associated function parameter is optional:
>
> ```rust,edition2015
> // 2015 Edition
> trait T {
>     fn f(i32); // OK: parameter identifiers are not required
> }
> ```
>
> Beginning in the 2018 edition, patterns are no longer optional.

r[items.traits.params.restriction-patterns.edition2018]
> [!EDITION-2018]
> Prior to the 2018 edition, parameters in associated functions with a body are limited to the following kinds of patterns:
>
> * [IDENTIFIER]
> * `mut` [IDENTIFIER]
> * [`_`][WildcardPattern]
> * `&` [IDENTIFIER]
> * `&&` [IDENTIFIER]
>
> ```rust,edition2015,compile_fail,E0642
> // 2015 Edition
> trait T {
>     fn f1((a, b): (i32, i32)) {} // ERROR: pattern not allowed
> }
> ```
>
> Beginning in 2018, all irrefutable patterns are allowed as described in [items.traits.params.patterns-with-body].

r[items.traits.associated-visibility]
## Item visibility

r[items.traits.associated-visibility.intro]
Trait items syntactically allow a [Visibility] annotation, but this is
rejected when the trait is validated. This allows items to be parsed with a
unified syntax across different contexts where they are used. As an example,
an empty `vis` macro fragment specifier can be used for trait items, where the
macro rule may be used in other situations where visibility is allowed.

```rust
macro_rules! create_method {
    ($vis:vis $name:ident) => {
        $vis fn $name(&self) {}
    };
}

trait T1 {
    // Empty `vis` is allowed.
    create_method! { method_of_t1 }
}

struct S;

impl S {
    // Visibility is allowed here.
    create_method! { pub method_of_s }
}

impl T1 for S {}

fn main() {
    let s = S;
    s.method_of_t1();
    s.method_of_s();
}
```

[WildcardPattern]: ../patterns.md#wildcard-pattern
[bounds]: ../trait-bounds.md
[trait object]: ../types/trait-object.md
[associated items]: associated-items.md
[method]: associated-items.md#methods
[supertraits]: #supertraits
[implementations]: implementations.md
[generics]: generics.md
[where clauses]: generics.md#where-clauses
[generic functions]: functions.md#generic-functions
[unsafe]: ../unsafety.md
[trait implementation]: implementations.md#trait-implementations
[`Send`]: ../special-types-and-traits.md#send
[`Sync`]: ../special-types-and-traits.md#sync
[`Arc<Self>`]: ../special-types-and-traits.md#arct
[`Box<Self>`]: ../special-types-and-traits.md#boxt
[`Pin<P>`]: ../special-types-and-traits.md#pinp
[`Rc<Self>`]: ../special-types-and-traits.md#rct
[`async`]: functions.md#async-functions
[`const`]: functions.md#const-functions
[type namespace]: ../names/namespaces.md

<script>
(function() {
    var fragments = {
        "#object-safety": "traits.html#dyn-compatibility",
    };
    var target = fragments[window.location.hash];
    if (target) {
        var url = window.location.toString();
        var base = url.substring(0, url.lastIndexOf('/'));
        window.location.replace(base + "/" + target);
    }
})();
</script>
