# Field access expressions

> **<sup>Syntax</sup>**\
> _FieldExpression_ :\
> &nbsp;&nbsp; [_Expression_] `.` [IDENTIFIER]

A _field expression_ consists of an expression followed by a single dot and an [identifier], when not immediately followed by a parenthesized expression-list (the latter is always a [method call expression]).
A field expression denotes a field of a [struct] or [union].
To call a function stored in a struct, parentheses are needed around the field expression.

<!-- ignore: needs lots of support code -->
```rust,ignore
mystruct.myfield;
foo().x;
(Struct {a: 10, b: 20}).a;
mystruct.method();          // Method expression
(mystruct.function_field)() // Call expression containing a field expression
```

A field access is a [place expression] referring to the location of that field.
When the subexpression is [mutable], the field expression is also mutable.

Also, if the type of the expression to the left of the dot is a pointer, it is automatically dereferenced as many times as necessary to make the field access possible.
In cases of ambiguity, we prefer fewer autoderefs to more.

Finally, the fields of a struct or a reference to a struct are treated as separate entities when borrowing.
If the struct does not implement [`Drop`](../special-types-and-traits.md#drop) and is stored in a local variable, this also applies to moving out of each of its fields.
This also does not apply if automatic dereferencing is done though user defined types other than [`Box`](../special-types-and-traits.html#boxt).

```rust
struct A { f1: String, f2: String, f3: String }
let mut x: A;
# x = A {
#     f1: "f1".to_string(),
#     f2: "f2".to_string(),
#     f3: "f3".to_string()
# };
let a: &mut String = &mut x.f1; // x.f1 borrowed mutably
let b: &String = &x.f2;         // x.f2 borrowed immutably
let c: &String = &x.f2;         // Can borrow again
let d: String = x.f3;           // Move out of x.f3
```

[_Expression_]: ../expressions.md
[IDENTIFIER]: ../identifiers.md
[method call expression]: method-call-expr.md
[struct]: ../items/structs.md
[union]: ../items/unions.md
[place expression]: ../expressions.md#place-expressions-and-value-expressions
[mutable]: ../expressions.md#mutability
