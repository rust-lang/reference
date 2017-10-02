# Method-call expressions

A _method call_ consists of an expression followed by a single dot, an
[identifier](identifiers.html), and a parenthesized expression-list. Method
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

When resolving method calls on an expression of type `A`, Rust will use the
following order:

1. Inherent methods, with receiver of type `A`, `&A`, `&mut A`.
1. Trait methods with receiver of type `A`.
1. Trait methods with receiver of type `&A`.
1. Trait methods with receiver of type `&mut A`.
1. If it's possible, Rust will then repeat steps 1-5 with
  `<A as std::ops::Deref>::Target`, and insert a dereference operator.
1. If `A` is now an [array](types.html#array-and-slice-types) type, then
  repeat steps 1-4 with the corresponding slice type.

Note: that in steps 1-4 the receiver is used, not the type of `Self` nor the
type of `A`. For example

```rust,ignore
// `Self` is `&A`, receiver is `&A`.
impl<'a> Trait for &'a A {
    fn method(self) {}
}
// If `A` is `&B`, then `Self` is `B` and the receiver is `A`.
impl B {
    fn method(&self) {}
}
```

Another note: this process does not use the mutability or lifetime of the
receiver, or whether `unsafe` methods can currently be called to resolve
methods. These constraints instead lead to compiler errors.

If a step is reached where there is more than one possible method (where
generic methods or traits are considered the same), then it is a compiler
error. These cases require a [more specific
syntax.](expressions/call-expr.html#disambiguating-function-calls) for method
and function invocation.
