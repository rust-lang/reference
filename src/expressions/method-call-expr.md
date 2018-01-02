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

When resolving method calls on an expression of type `A`, Rust will use the
following order, only looking at methods that are [visible]. If the type of `A`
is a type parameter or `Self` in a trait definitition then steps 2-4 first
consider traits from bounds on the type paramter, then the traits that are in
scope. For other types, only the traits that are in scope are considered.

1. Inherent methods, with receiver of type `A`, `&A`, `&mut A`.
1. Trait methods with receiver of type `A`.
1. Trait methods with receiver of type `&A`.
1. Trait methods with receiver of type `&mut A`.
1. If it's possible, Rust will then repeat steps 1-5 with
  `<A as std::ops::Deref>::Target`, and insert a dereference operator.
1. If `A` is now an [array] type, then repeat steps 1-4 with the corresponding
  slice type.

Note: In steps 1-4, the receiver is used, not the type of `Self` nor the
type of `A`. For example:

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
[disambiguating function call syntax]: expressions/call-expr.html#disambiguating-function-calls