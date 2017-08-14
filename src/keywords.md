# Keywords

Rust divides keywords in three categories:
  - [strict](#strict-keywords)
  - [weak](#weak-keywords)
  - [reserved](#reserved-keywords)

## Strict keywords

These keywords can only be used in their correct contexts. For example, it is
not allowed to declare a variable with name `struct`.

> **<sup>Lexer:<sup>**  
> KW_AS             : `as`  
> KW_BOX            : `box`  
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
> KW_WHERE          : `wher`  
> KW_WHILE          : `while`  

## Weak keywords

These keywords have special meaning only in certain contexts. For example,
it is possible to declare a variable or method with the name `union`.

> **<sup>Lexer</sup>**  
> KW_CATCH          : `catch`  
> KW_DEFAULT        : `default`  
> KW_UNION          : `union`  
> KW_STATICLIFETIME : `'static`  

## Reserved keywords

These keywords aren't used yet, but they are reserved for future use.
The reasoning behind this is to make current programs forward compatible with
future versions of rust by forbiding them to use these keywords.

> **<sup>Lexer</sup>**  
> KW_ABSTRACT       : `abstract`  
> KW_ALIGNOF        : `alignof`  
> KW_BECOME         : `become`  
> KW_DO             : `do`  
> KW_FINAL          : `final`  
> KW_MACRO          : `macro`  
> KW_OFFSETOF       : `offsetof`  
> KW_OVERRIDE       : `override`  
> KW_PRIV           : `priv`  
> KW_PROC           : `proc`  
> KW_PURE           : `pure`  
> KW_SIZEOF         : `sizeof`  
> KW_TYPEOF         : `typeof`  
> KW_UNSIZED        : `unsized`  
> KW_VIRTUAL        : `virtual`  
> KW_YIELD          : `yield`  
