# Field access expressions

A _field expression_ consists of an expression followed by a single dot and an
[identifier](identifiers.html), when not immediately followed by a
parenthesized expression-list (the latter is always a [method call
expression](expressions/method-call-expr.html)). A field expression denotes a field of a
[struct](types.html#struct-types) or [union](items/unions.html). To call a
function stored in a struct parentheses are needed around the field expression

```rust,ignore
mystruct.myfield;
foo().x;
(Struct {a: 10, b: 20}).a;
mystruct.method();          // Method expression
(mystruct.function_field)() // Call expression containing a field expression
```

A field access is an [lvalue](expressions.html#lvalues-and-rvalues) referring
to the location of that field. When the subexpression is
[mutable](expressions.html#mutability), the field expression is also mutable.

Also, if the type of the expression to the left of the dot is a pointer, it is
automatically dereferenced as many times as necessary to make the field access
possible. In cases of ambiguity, we prefer fewer autoderefs to more.

Finally the fields of a struct, a reference to a struct are treated as separate
entities when borrowing. If the struct does not implement
[`Drop`](the-drop-trait.html) this also applies to moving out of each of its fields
where possible. This also does not apply if automatic dereferencing is done
though user defined types.

```rust
# struct A { f1: String, f2: String, f3: String }
# let mut x = A {
#     f1: "f1".to_string(),
#     f2: "f2".to_string(),
#     f3: "f3".to_string()
# };
let a: &mut String = &mut x.f1; // x.f1 borrowed mutably
let b: &String = &x.f2;         // x.f2 borrowed immutably
let c: &String = &x.f2;         // Can borrow again
let d: String = x.f3;           // Move out of x.f3
```
