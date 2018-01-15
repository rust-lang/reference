# Functions

A _function_ consists of a [block], along with a name and a set of parameters.
Other than a name, all these are optional. Functions are declared with the
keyword `fn`. Functions may declare a set of *input* [*variables*][variables]
as parameters, through which the caller passes arguments into the function, and
the *output* [*type*][type] of the value the function will return to its caller
on completion.

[block]: expressions/block-expr.html
[variables]: variables.html
[type]: types.html

When referred to, a _function_ yields a first-class *value* of the
corresponding zero-sized [*function item type*][function item type], which
when called evaluates to a direct call to the function.

[function item type]: types.html#function-item-types

For example, this is a simple function:
```rust
fn answer_to_life_the_universe_and_everything() -> i32 {
    return 42;
}
```

As with `let` bindings, function arguments are irrefutable patterns, so any
pattern that is valid in a let binding is also valid as an argument:

```rust
fn first((value, _): (i32, i32)) -> i32 { value }
```

The block of a function is conceptually wrapped in a block that binds the
argument patterns and then `return`s the value of the function's block. This
means that the tail expression of the block, if evaluated, ends up being
returned to the caller. As usual, an explicit return expression within
the body of the function will short-cut that implicit return, if reached.

For example, the function above behaves as if it was written as:

```rust,ignore
// argument_0 is the actual first argument passed from the caller
let (value, _) = argument_0;
return {
    value
};
```

## Generic functions

A _generic function_ allows one or more _parameterized types_ to appear in its
signature. Each type parameter must be explicitly declared in an
angle-bracket-enclosed and comma-separated list, following the function name.

```rust
// foo is generic over A and B

fn foo<A, B>(x: A, y: B) {
# }
```

Inside the function signature and body, the name of the type parameter can be
used as a type name. [Trait](items/traits.html) bounds can be specified for type
parameters to allow methods with that trait to be called on values of that
type. This is specified using the `where` syntax:

```rust
# use std::fmt::Debug;
fn foo<T>(x: T) where T: Debug {
# }
```

When a generic function is referenced, its type is instantiated based on the
context of the reference. For example, calling the `foo` function here:

```rust
use std::fmt::Debug;

fn foo<T>(x: &[T]) where T: Debug {
    // details elided
}

foo(&[1, 2]);
```

will instantiate type parameter `T` with `i32`.

The type parameters can also be explicitly supplied in a trailing [path]
component after the function name. This might be necessary if there is not
sufficient context to determine the type parameters. For example,
`mem::size_of::<u32>() == 4`.

[path]: paths.html

## Diverging functions

A special kind of function can be declared with a `!` character where the
output type would normally be. For example:

```rust
fn my_err(s: &str) -> ! {
    println!("{}", s);
    panic!();
}
```

We call such functions "diverging" because they never return a value to the
caller. Every control path in a diverging function must end with a `panic!()`,
a loop expression without an associated break expression, or a call to another
diverging function on every control path. The `!` annotation does *not* denote
a type.

It might be necessary to declare a diverging function because as mentioned
previously, the typechecker checks that every control path in a function ends
with a [`return`] or diverging expression. So, if `my_err` were declared
without the `!` annotation, the following code would not typecheck:

[`return`]: expressions/return-expr.html

```rust
# fn my_err(s: &str) -> ! { panic!() }

fn f(i: i32) -> i32 {
    if i == 42 {
        return 42;
    }
    else {
        my_err("Bad number!");
    }
}
```

This will not compile without the `!` annotation on `my_err`, since the `else`
branch of the conditional in `f` does not return an `i32`, as required by the
signature of `f`. Adding the `!` annotation to `my_err` informs the typechecker
that, should control ever enter `my_err`, no further type judgments about `f`
need to hold, since control will never resume in any context that relies on
those judgments. Thus the return type on `f` only needs to reflect the `if`
branch of the conditional.

## Extern functions

Extern functions are part of Rust's foreign function interface, providing the
opposite functionality to [external blocks]. Whereas external
blocks allow Rust code to call foreign code, extern functions with bodies
defined in Rust code _can be called by foreign code_. They are defined in the
same way as any other Rust function, except that they have the `extern`
modifier.

```rust
// Declares an extern fn, the ABI defaults to "C"
extern fn new_i32() -> i32 { 0 }

// Declares an extern fn with "stdcall" ABI
# #[cfg(target_arch = "x86_64")]
extern "stdcall" fn new_i32_stdcall() -> i32 { 0 }
```

Unlike normal functions, extern fns have type `extern "ABI" fn()`. This is the
same type as the functions declared in an extern block.

```rust
# extern fn new_i32() -> i32 { 0 }
let fptr: extern "C" fn() -> i32 = new_i32;
```

As non-Rust calling conventions do not support unwinding, unwinding past the end
of an extern function will cause the process to abort. In LLVM, this is
implemented by executing an illegal instruction.

[external blocks]: items/external-blocks.html