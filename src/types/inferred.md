r[type.inferred]
# Inferred type

r[type.inferred.syntax]
```grammar,types
InferredType -> `_`
```

r[type.inferred.intro]
The inferred type asks the compiler to infer the type if possible based on the
surrounding information available.

r[type.inferred.constraint]
It cannot be used in item signatures.

It is often used in generic arguments:

```rust
let x: Vec<_> = (0..10).collect();
```

<!--
  What else should be said here?
  The only documentation I am aware of is https://rustc-dev-guide.rust-lang.org/type-inference.html
  There should be a broader discussion of type inference somewhere.
-->
