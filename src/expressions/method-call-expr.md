# Method-call expressions

A _method call_ consists of an expression followed by a single dot, an
[identifier], and a parenthesized expression-list. Method
calls are resolved to methods on specific traits, either statically dispatching
to a method if the exact `self`-type of the left-hand-side is known, or
dynamically dispatching if the left-hand-side expression is an indirect [trait
object](types.html#trait-objects). Method call expressions will automatically
take a shared or mutable borrow of the receiver if needed.

```rust
let pi: Result<f32, _> = "3.14".parse();
let log_pi = pi.unwrap_or(1.0).log(2.72);
# assert!(1.14 < log_pi && log_pi < 1.15)
```

When resolving method calls on an expression of type `A`, Rust looks up methods
both on the type itself and the traits in implements. Additionally, unlike with
non-method function calls, the `self` parameter is special and may be
automatically dereferenced in order to resolve it. Rust uses the following
process to resolve method calls.

First, Rust will attempt to build a list of candidate receiver types. It obtains
these by repeatedly [dereferencing][dereference] the type, adding each type
encountered to the list, then finally attempting an [unsized coercion] at the
end, and adding the result type if that is successful. Then, for each candidate
`T`, Rust adds `&T` and `&mut T` to the list immediately afterward.

So, for instance, if `A` is `Box<[i32;2]>`, then the candidate types will be
`Box<[i32;2]>`, `&Box<[i32;2]>`, `&mut Box<[i32;2]>`, `[i32; 2]` (by
dereferencing), `&[i32; 2]`, `&mut [i32; 2]`, `[i32]` (by unsized coercion),
`&[i32]`, and finally `&mut [i32]`.

Then, for each candidate type `T`, Rust will search for a [visible] method with
a receiver of that type in the following places:

1. `T`'s inherent methods (methods implemented directly on `T`).
1. Any of the methods provided by a trait implemented by `T`. If `T` is
   a type parameter (including the `Self` parameter of a trait), then only
   methods from the trait constraints on `T` are available for lookup. If `T` is
   not, then methods from any in-scope trait are available.

Note that the lookup is done for each type in order, which can occasionally lead
to surprising results. The below code will print "In trait impl!", because
`&self` methods are looked up first, the trait method is found before the
struct's `&mut self` method is found.

```rust
struct Foo {}

trait Bar {
  fn bar(&self);
}

impl Foo {
  fn bar(&mut self) {
    println!("In struct impl!")
  }
}

impl Bar for Foo {
  fn bar(&self) {
    println!("In trait impl!")
  }
}

fn main() {
  let mut f = Foo{};
  f.bar();
}
```

If this results in multiple possible candidates, then it is an error, and the
receiver must be [converted][disambiguate call] to an appropriate receiver type
to make the method call.

The lookup process does not take into account the mutability or lifetime of the
receiver, or whether a method is `unsafe`. Once a method is looked up.

If a step is reached where there is more than one possible method, such as where
generic methods or traits are considered the same, then it is a compiler
error. These cases require a [disambiguating function call syntax] for method
and function invocation.

> Warning: For [trait objects], if there is an inherent method of the same name
> as a trait method, it will give a compiler error when trying to call the
> method in a method call expression. Instead, you can call the method using
> [disambiguating function call syntax], in which case it calls the trait
> method, not the inherent method. There is no way to call the inherent method.
> Just don't define inherent methods on trait objects with the same name a trait
> method and you'll be fine.

[IDENTIFIER]: identifiers.html
[visible]: visibility-and-privacy.html
[array]: types.html#array-and-slice-types
[trait objects]: types.html#trait-objects
[disambiguate call]: expressions/call-expr.html#disambiguating-function-calls
[dereference]: expressions/operator-expr.html#the-dereference-operator
