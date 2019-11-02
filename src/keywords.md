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

> **<sup>Lexer:<sup>**\
> KW_AS             : [`as`](../std/keyword.as.html)\
> KW_BREAK          : [`break`](../std/keyword.break.html)\
> KW_CONST          : [`const`](../std/keyword.const.html)\
> KW_CONTINUE       : [`continue`](../std/keyword.continue.html)\
> KW_CRATE          : [`crate`](../std/keyword.crate.html)\
> KW_ELSE           : [`else`](../std/keyword.else.html)\
> KW_ENUM           : [`enum`](../std/keyword.enum.html)\
> KW_EXTERN         : [`extern`](../std/keyword.extern.html)\
> KW_FALSE          : [`false`](../std/keyword.false.html)\
> KW_FN             : [`fn`](../std/keyword.fn.html)\
> KW_FOR            : [`for`](../std/keyword.for.html)\
> KW_IF             : [`if`](../std/keyword.if.html)\
> KW_IMPL           : [`impl`](../std/keyword.impl.html)\
> KW_IN             : [`in`](../std/keyword.in.html)\
> KW_LET            : [`let`](../std/keyword.let.html)\
> KW_LOOP           : [`loop`](../std/keyword.loop.html)\
> KW_MATCH          : [`match`](../std/keyword.match.html)\
> KW_MOD            : [`mod`](../std/keyword.mod.html)\
> KW_MOVE           : [`move`](../std/keyword.move.html)\
> KW_MUT            : [`mut`](../std/keyword.mut.html)\
> KW_PUB            : [`pub`](../std/keyword.pub.html)\
> KW_REF            : [`ref`](../std/keyword.ref.html)\
> KW_RETURN         : [`return`](../std/keyword.return.html)\
> KW_SELFVALUE      : [`self`](../std/keyword.self.html)\
> KW_SELFTYPE       : [`Self`](../std/keyword.Self.html)\
> KW_STATIC         : [`static`](../std/keyword.static.html)\
> KW_STRUCT         : [`struct`](../std/keyword.struct.html)\
> KW_SUPER          : [`super`](../std/keyword.super.html)\
> KW_TRAIT          : [`trait`](../std/keyword.trait.html)\
> KW_TRUE           : [`true`](../std/keyword.true.html)\
> KW_TYPE           : [`type`](../std/keyword.type.html)\
> KW_UNSAFE         : [`unsafe`](../std/keyword.unsafe.html)\
> KW_USE            : [`use`](../std/keyword.use.html)\
> KW_WHERE          : [`where`](../std/keyword.where.html)\
> KW_WHILE          : [`while`](../std/keyword.while.html)

The following keywords were added beginning in the 2018 edition.

> **<sup>Lexer 2018+</sup>**\
> KW_ASYNC          : [`async`](../std/keyword.async.html)\
> KW_AWAIT          : [`await`](../std/keyword.await.html)\
> KW_DYN            : [`dyn`](../std/keyword.dyn.html)

## Reserved keywords

These keywords aren't used yet, but they are reserved for future use. They have
the same restrictions as strict keywords. The reasoning behind this is to make
current programs forward compatible with future versions of Rust by forbidding
them to use these keywords.

> **<sup>Lexer</sup>**\
> KW_ABSTRACT       : `abstract`\
> KW_BECOME         : `become`\
> KW_BOX            : `box`\
> KW_DO             : `do`\
> KW_FINAL          : `final`\
> KW_MACRO          : `macro`\
> KW_OVERRIDE       : `override`\
> KW_PRIV           : `priv`\
> KW_TYPEOF         : `typeof`\
> KW_UNSIZED        : `unsized`\
> KW_VIRTUAL        : `virtual`\
> KW_YIELD          : `yield`

The following keywords are reserved beginning in the 2018 edition.

> **<sup>Lexer 2018+</sup>**\
> KW_TRY   : `try`

## Weak keywords

These keywords have special meaning only in certain contexts. For example, it
is possible to declare a variable or method with the name `union`.

* [`union`](../std/keyword.union.html) is used to declare a [union]
  and is only a keyword when used in a union declaration.
* `'static` is used for the static lifetime and cannot be used as a generic
  lifetime parameter

  ```compile_fail
  // error[E0262]: invalid lifetime parameter name: `'static`
  fn invalid_lifetime_parameter<'static>(s: &'static str) -> &'static str { s }
  ```
* In the 2015 edition, [`dyn`] is a keyword when used in a type position
  followed by a path that does not start with `::`.

  Beginning in the 2018 edition, `dyn` has been promoted to a strict keyword.

> **<sup>Lexer</sup>**\
> KW_UNION          : [`union`](../std/keyword.union.html)\
> KW_STATICLIFETIME : `'static`
>
> **<sup>Lexer 2015</sup>**\
> KW_DYN            : [`dyn`](../std/keyword.dyn.html)

[items]: items.md
[Variables]: variables.md
[Type parameters]: types/parameters.md
[loop labels]: expressions/loop-expr.md#loop-labels
[Macros]: macros.md
[attributes]: attributes.md
[Macro placeholders]: macros-by-example.md
[Crates]: crates-and-source-files.md
[union]: items/unions.md
[variants]: items/enumerations.md
[`dyn`]: types/trait-object.md
