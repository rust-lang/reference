# Trait bounds

Generic functions may use traits as _bounds_ on their type parameters. This
will have three effects:

- Only types that have the trait may instantiate the parameter.
- Within the generic function, the methods of the trait can be called on values
  that have the parameter's type. Associated types can be used in the
  function's signature, and associated constants can be used in expressions
  within the function body.
- Generic functions and types with the same or weaker bounds can use the
  generic type in the function body or signature.

For example:

```rust
# type Surface = i32;
# trait Shape { fn draw(&self, Surface); }
struct Figure<S: Shape>(S, S);
fn draw_twice<T: Shape>(surface: Surface, sh: T) {
    sh.draw(surface);
    sh.draw(surface);
}
fn draw_figure<U: Shape>(surface: Surface, Figure(sh1, sh2): Figure<U>) {
    sh1.draw(surface);
    draw_twice(surface, sh2); // Can call this since U: Shape
}
```
