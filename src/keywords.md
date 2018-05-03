# Keywords

Rust divides keywords into three categories:

* [strict](#strict-keywords)
* [reserved](#reserved-keywords)
* [weak](#weak-keywords)

## Strict keywords

These keywords can only be used in their correct contexts. They cannot
be used as the names of:

* [Items]
* [Variables] and function parameters
* Fields and [variants]
* [Type parameters]
* Lifetime parameters or [loop labels]
* [Macros] or [attributes]
* [Macro placeholders]
* [Crates]

> **<sup>Lexer:<sup>**  
> KW_AS             : `as`  
> KW_BREAK          : `break`  
> KW_CONST          : `const`  
> KW_CONTINUE       : `continue`  
> KW_CRATE          : `crate`  
> KW_ELSE           : `else`  
> KW_ENUM           : `enum`  
> KW_EXTERN         : `extern`  
> KW_FALSE          : `false`  
> KW_FN             : `fn`  
> KW_FOR            : `for`  
> KW_IF             : `if`  
> KW_IMPL           : `impl`  
> KW_IN             : `in`  
> KW_LET            : `let`  
> KW_LOOP           : `loop`  
> KW_MATCH          : `match`  
> KW_MOD            : `mod`  
> KW_MOVE           : `move`  
> KW_MUT            : `mut`  
> KW_PUB            : `pub`  
> KW_REF            : `ref`  
> KW_RETURN         : `return`  
> KW_SELFVALUE      : `self`  
> KW_SELFTYPE       : `Self`  
> KW_STATIC         : `static`  
> KW_STRUCT         : `struct`  
> KW_SUPER          : `super`  
> KW_TRAIT          : `trait`  
> KW_TRUE           : `true`  
> KW_TYPE           : `type`  
> KW_UNSAFE         : `unsafe`  
> KW_USE            : `use`  
> KW_WHERE          : `where`  
> KW_WHILE          : `while`  

## Reserved keywords

These keywords aren't used yet, but they are reserved for future use. They have
the same restrictions as strict keywords. The reasoning behind this is to make
current programs forward compatible with future versions of Rust by forbidding
them to use these keywords.

> **<sup>Lexer</sup>**  
> KW_ABSTRACT       : `abstract`  
> KW_ALIGNOF        : `alignof`  
> KW_BECOME         : `become`  
> KW_BOX            : `box`  
> KW_DO             : `do`  
> KW_FINAL          : `final`  
> KW_MACRO          : `macro`  
> KW_OFFSETOF       : `offsetof`  
> KW_OVERRIDE       : `override`  
> KW_PRIV           : `priv`  
> KW_PURE           : `pure`  
> KW_SIZEOF         : `sizeof`  
> KW_TYPEOF         : `typeof`  
> KW_UNSIZED        : `unsized`  
> KW_VIRTUAL        : `virtual`  
> KW_YIELD          : `yield`  

## Weak keywords

These keywords have special meaning only in certain contexts. For example, it
is possible to declare a variable or method with the name `union`.

* `union` is used to declare a [union] and is only a keyword when used in a
  union declaration.
* `'static` is used for the static lifetime and cannot be used as a generic
  lifetime parameter
  
  ```compile_fail
  // error[E0262]: invalid lifetime parameter name: `'static`
  fn invalid_lifetime_parameter<'static>(s: &'static str) -> &'static str { s }
  ```
* `dyn` denotes a [trait object] and is a keyword when used in a type position
  followed by a path that does not start with `::`.

> **<sup>Lexer</sup>**  
> KW_UNION          : `union`
> KW_STATICLIFETIME : `'static`
> KW_DYN            : `dyn`

[items]: items.html
[Variables]: variables.html
[Type parameters]: types.html#type-parameters
[loop labels]: expressions/loop-expr.html#loop-labels
[Macros]: macros.html
[attributes]: attributes.html
[Macro placholders]: macros-by-example.html
[Crates]: crates-and-source-files.html
[union]: items/unions.html
[variants]: items/enumerations.html
[trait object]: types.html#trait-objects