# Type Parameters

Functions, type aliases, structs, enumerations, unions, traits and
implementations may be *parameterized* by type. Type parameters are given as a
comma-separated list of identifiers enclosed in angle brackets (`<...>`), after
the name of the item (except for implementations, where they come directly
after `impl`) and before its definition.

The type parameters of an item are considered "part of the name", not part of
the type of the item. A referencing [path] must (in principle) provide type
arguments as a list of comma-separated types enclosed within angle brackets, in
order to refer to the type-parameterized item. In practice, the type-inference
system can usually infer such argument types from context. There are no general
type-parametric types, only type-parametric items. That is, Rust has no notion
of type abstraction: there are no higher-ranked (or "forall") types abstracted
over other types, though higher-ranked types do exist for lifetimes.

[path]: paths.html
