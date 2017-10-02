# Enumeration Variant expressions

Enumeration variants can be constructed similarly to structs, using a path to
an enum variant instead of to a struct:

```rust
# enum Message {
#     Quit,
#     WriteString(String),
#     Move { x: i32, y: i32 },
# }
let q = Message::Quit;
let w = Message::WriteString("Some string".to_string());
let m = Message::Move { x: 50, y: 200 };
```
