r[type.never]
# Never type

r[type.never.syntax]
```grammar,types
NeverType -> `!`
```

r[type.never.intro]
The never type `!` is a type with no values, representing the result of
computations that never complete.

r[type.never.coercion]
Expressions of type `!` can be coerced into any other type.

r[type.never.constraint]
The `!` type can **only** appear in function return types presently,
indicating it is a diverging function that never returns.

```rust
fn foo() -> ! {
    panic!("This call never returns.");
}
```

```rust
unsafe extern "C" {
    pub safe fn no_return_extern_func() -> !;
}
```
