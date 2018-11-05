# Boolean type

The `bool` type is a datatype which can be either `true` or `false`. The boolean
type uses one byte of memory. It is used in comparisons and bitwise operations
like `&`, `|`, and `!`.

```rust
fn main() {
    let x = true;
    let y: bool = false; // with the boolean type annotation

    // Use of booleans in conditional expressions
    if x {
        println!("x is true");
    }
}
```
