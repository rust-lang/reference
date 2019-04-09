# Destructors

When an [initialized]&#32;[variable] in Rust goes out of scope or a [temporary]
is no longer needed its _destructor_ is run. [Assignment] also runs the
destructor of its left-hand operand, unless it's an uninitialized variable. If a
[struct] variable has been partially initialized, only its initialized fields
are dropped.

The destructor of a type consists of

1. Calling its [`std::ops::Drop::drop`] method, if it has one.
2. Recursively running the destructor of all of its fields.
    * The fields of a [struct], [tuple] or [enum variant] are dropped in
      declaration order. \*
    * The elements of an [array] or owned [slice][array] are dropped from the
      first element to the last. \*
    * The captured values of a [closure] are dropped in an unspecified order.
    * [Trait objects] run the destructor of the underlying type.
    * Other types don't result in any further drops.

\* This order was stabilized in [RFC 1857].

Variables are dropped in reverse order of declaration. Variables declared in
the same pattern drop in an unspecified ordered.

If a destructor must be run manually, such as when implementing your own smart
pointer, [`std::ptr::drop_in_place`] can be used.

Some examples:

```rust
struct ShowOnDrop(&'static str);

impl Drop for ShowOnDrop {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

{
    let mut overwritten = ShowOnDrop("Drops when overwritten");
    overwritten = ShowOnDrop("drops when scope ends");
}
# println!("");
{
    let declared_first = ShowOnDrop("Dropped last");
    let declared_last = ShowOnDrop("Dropped first");
}
# println!("");
{
    // Tuple elements drop in forwards order
    let tuple = (ShowOnDrop("Tuple first"), ShowOnDrop("Tuple second"));
}
# println!("");
loop {
    // Tuple expression doesn't finish evaluating so temporaries drop in reverse order:
    let partial_tuple = (ShowOnDrop("Temp first"), ShowOnDrop("Temp second"), break);
}
# println!("");
{
    let moved;
    // No destructor run on assignment.
    moved = ShowOnDrop("Drops when moved");
    // drops now, but is then uninitialized
    moved;
    let uninitialized: ShowOnDrop;
    // Only first element drops
    let mut partially_initialized: (ShowOnDrop, ShowOnDrop);
    partially_initialized.0 = ShowOnDrop("Partial tuple first");
}
```

## Not running destructors

Not running destructors in Rust is safe even if it has a type that isn't
`'static`. [`std::mem::ManuallyDrop`] provides a wrapper to prevent a
variable or field from being dropped automatically.

[initialized]: glossary.html#initialized
[variable]: variables.html
[temporary]: expressions.html#temporary-lifetimes
[Assignment]: expressions/operator-expr.html#assignment-expressions
[`std::ops::Drop::drop`]: ../std/ops/trait.Drop.html
[RFC 1857]: https://github.com/rust-lang/rfcs/blob/master/text/1857-stabilize-drop-order.md
[struct]: types/struct.html
[tuple]: types/tuple.html
[enum variant]: types/enum.html
[array]: types/array.html
[closure]: types/closure.html
[Trait objects]: types/trait-object.html
[`std::ptr::drop_in_place`]: ../std/ptr/fn.drop_in_place.html
[`std::mem::ManuallyDrop`]: ../std/mem/struct.ManuallyDrop.html
