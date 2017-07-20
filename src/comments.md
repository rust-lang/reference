# Comments

> **<sup>Lexer</sup>**  
> LINE_COMMENT :  
> &nbsp;&nbsp; `//` ~[\n\r]*  
>  
> BLOCK_COMMENT :  
> &nbsp;&nbsp; `/*` (BLOCK_COMMENT | .)* `*/`  
>  
> OUTER_DOC_LINE_COMMENT :  
> &nbsp;&nbsp; `//!` ~[\n\r]*  
>  
> OUTER_DOC_BLOCK_COMMENT :  
> &nbsp;&nbsp; `/*!` (OUTER_DOC_BLOCK_COMMENT | .)* `*/`  
>  
> INNER_DOC_LINE_COMMENT :  
> &nbsp;&nbsp; `///` ~[\n\r]*  
>  
> INNER_DOC_BLOCK_COMMENT :   
> &nbsp;&nbsp; `/**` (INNER_DOC_BLOCK_COMMENT | .)* `*/`  

Comments in Rust code follow the general C++ style of line (`//`) and
block (`/* ... */`) comment forms. Nested block comments are supported.

Line comments beginning with exactly _three_ slashes (`///`), and block
comments (`/** ... */`), are interpreted as a special syntax for `doc`
[attributes]. That is, they are equivalent to writing
`#[doc="..."]` around the body of the comment, i.e., `/// Foo` turns into
`#[doc="Foo"]`.

Line comments beginning with `//!` and block comments `/*! ... */` are
doc comments that apply to the parent of the comment, rather than the item
that follows.  That is, they are equivalent to writing `#![doc="..."]` around
the body of the comment. `//!` comments are usually used to document
modules that occupy a source file.

Non-doc comments are interpreted as a form of whitespace.

[attributes]: attributes.html
