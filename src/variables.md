# Variables

A _variable_ is a component of a stack frame, either a named function parameter,
an anonymous [temporary](expressions.html#temporary-lifetimes), or a named local
variable.

A _local variable_ (or *stack-local* allocation) holds a value directly,
allocated within the stack's memory. The value is a part of the stack frame.

Local variables are immutable unless declared otherwise. For example: `let mut x = ...`.

Function parameters are immutable unless declared with `mut`. The `mut` keyword
applies only to the following parameter. For example: `|mut x, y|` and `fn f(mut x:
Box<i32>, y: Box<i32>)` declare one mutable variable `x` and one immutable
variable `y`.

Methods that take either `self` or `Box<Self>` can optionally place them in a
mutable variable by prefixing them with `mut` (similar to regular arguments). For example:

```rust
trait Changer: Sized {
    fn change(mut self) {}
    fn modify(mut self: Box<Self>) {}
}
```

Local variables are not initialized when allocated. Instead, the entire frame worth of
local variables are allocated, on frame-entry, in an uninitialized
state. Subsequent statements within a function may or may not initialize the
local variables. Local variables can be used only after they have been
initialized; this is enforced by the compiler.
