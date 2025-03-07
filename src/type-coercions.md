r[coerce]
# Type coercions

r[coerce.intro]
**Type coercions** are implicit operations that change the type of a value.
They happen automatically at specific locations and are highly restricted in
what types actually coerce.

r[coerce.as]
Any conversions allowed by coercion can also be explicitly performed by the
[type cast operator], `as`.

Coercions are originally defined in [RFC 401] and expanded upon in [RFC 1558].

r[coerce.site]
## Coercion sites

r[coerce.site.intro]
A coercion can only occur at certain coercion sites in a program; these are
typically places where the desired type is explicit or can be derived by
propagation from explicit types (without type inference). Possible coercion
sites are:

r[coerce.site.let]
* `let` statements where an explicit type is given.

   For example, `&mut 42` is coerced to have type `&i8` in the following:

   ```rust
   let _: &i8 = &mut 42;
   ```

r[coerce.site.value]
* `static` and `const` item declarations (similar to `let` statements).

r[coerce.site.argument]
* Arguments for function calls

  The value being coerced is the actual parameter, and it is coerced to
  the type of the formal parameter.

  For example, `&mut 42` is coerced to have type `&i8` in the following:

  ```rust
  fn bar(_: &i8) { }

  fn main() {
      bar(&mut 42);
  }
  ```

  For method calls, the receiver (`self` parameter) type is coerced
  differently, see the documentation on [method-call expressions] for details.

r[coerce.site.constructor]
* Instantiations of struct, union, or enum variant fields

  For example, `&mut 42` is coerced to have type `&i8` in the following:

  ```rust
  struct Foo<'a> { x: &'a i8 }

  fn main() {
      Foo { x: &mut 42 };
  }
  ```

r[coerce.site.return]
* Function results&mdash;either the final line of a block if it is not
  semicolon-terminated or any expression in a `return` statement

  For example, `x` is coerced to have type `&dyn Display` in the following:

  ```rust
  use std::fmt::Display;
  fn foo(x: &u32) -> &dyn Display {
      x
  }
  ```

r[coerce.site.subexpr]
If the expression in one of these coercion sites is a coercion-propagating
expression, then the relevant sub-expressions in that expression are also
coercion sites. Propagation recurses from these new coercion sites.
Propagating expressions and their relevant sub-expressions are:

r[coerce.site.array]
* Array literals, where the array has type `[U; n]`. Each sub-expression in
the array literal is a coercion site for coercion to type `U`.

r[coerce.site.repeat]
* Array literals with repeating syntax, where the array has type `[U; n]`. The
repeated sub-expression is a coercion site for coercion to type `U`.

r[coerce.site.tuple]
* Tuples, where a tuple is a coercion site to type `(U_0, U_1, ..., U_n)`.
Each sub-expression is a coercion site to the respective type, e.g. the
zeroth sub-expression is a coercion site to type `U_0`.

r[coerce.site.parenthesis]
* Parenthesized sub-expressions (`(e)`): if the expression has type `U`, then
the sub-expression is a coercion site to `U`.

r[coerce.site.block]
* Blocks: if a block has type `U`, then the last expression in the block (if
it is not semicolon-terminated) is a coercion site to `U`. This includes
blocks which are part of control flow statements, such as `if`/`else`, if
the block has a known type.

r[coerce.types]
## Coercion types

r[coerce.types.intro]
Coercion is allowed between the following types:

r[coerce.types.reflexive]
* `T` to `U` if `T` is a [subtype] of `U` (*reflexive case*)

r[coerce.types.transitive]
* `T_1` to `T_3` where `T_1` coerces to `T_2` and `T_2` coerces to `T_3`
(*transitive case*)

    Note that this is not fully supported yet.

r[coerce.types.mut-reborrow]
* `&mut T` to `&T`

r[coerce.types.mut-pointer]
* `*mut T` to `*const T`

r[coerce.types.ref-to-pointer]
* `&T` to `*const T`

r[coerce.types.mut-to-pointer]
* `&mut T` to `*mut T`

r[coerce.types.deref]
* `&T` or `&mut T` to `&U` if `T` implements `Deref<Target = U>`. For example:

  ```rust
  use std::ops::Deref;

  struct CharContainer {
      value: char,
  }

  impl Deref for CharContainer {
      type Target = char;

      fn deref<'a>(&'a self) -> &'a char {
          &self.value
      }
  }

  fn foo(arg: &char) {}

  fn main() {
      let x = &mut CharContainer { value: 'y' };
      foo(x); //&mut CharContainer is coerced to &char.
  }
  ```

r[coerce.types.deref-mut]
* `&mut T` to `&mut U` if `T` implements `DerefMut<Target = U>`.

r[coerce.types.unsize]
* TyCtor(`T`) to TyCtor(`U`), where TyCtor(`T`) is one of
    - `&T`
    - `&mut T`
    - `*const T`
    - `*mut T`
    - `Box<T>`

    and where `U` can be obtained from `T` by [unsized coercion](#unsized-coercions).

    <!--In the future, coerce_inner will be recursively extended to tuples and
    structs. In addition, coercions from subtraits to supertraits will be
    added. See [RFC 401] for more details.-->

r[coerce.types.fn]
* Function item types to `fn` pointers

r[coerce.types.closure]
* Non capturing closures to `fn` pointers

r[coerce.types.never]
* `!` to any `T`

r[coerce.unsize]
### Unsized Coercions

r[coerce.unsize.intro]
The following coercions are called `unsized coercions`, since they
relate to converting types to unsized types, and are permitted in a few
cases where other coercions are not, as described above. They can still happen
anywhere else a coercion can occur.

r[coerce.unsize.trait]
Two traits, [`Unsize`] and [`CoerceUnsized`], are used
to assist in this process and expose it for library use. The following
coercions are built-ins and, if `T` can be coerced to `U` with one of them, then
an implementation of `Unsize<U>` for `T` will be provided:

r[coerce.unsize.slice]
* `[T; n]` to `[T]`.

r[coerce.unsize.trait-object]
* `T` to `dyn U`, when `T` implements `U + Sized`, and `U` is [dyn compatible].

r[coerce.unsize.trait-upcast]
* `dyn T` to `dyn U`, when `U` is one of `T`'s [supertraits].
    * This allows dropping auto traits, i.e. `dyn T + Auto` to `dyn U` is allowed.
    * This allows adding auto traits if the principal trait has the auto trait as a super trait, i.e. given `trait T: U + Send {}`, `dyn T` to `dyn T + Send` or to `dyn U + Send` coercions are allowed.

r[coerce.unsized.composite]
* `Foo<..., T, ...>` to `Foo<..., U, ...>`, when:
    * `Foo` is a struct.
    * `T` implements `Unsize<U>`.
    * The last field of `Foo` has a type involving `T`.
    * If that field has type `Bar<T>`, then `Bar<T>` implements `Unsize<Bar<U>>`.
    * T is not part of the type of any other fields.

r[coerce.unsized.pointer]
Additionally, a type `Foo<T>` can implement `CoerceUnsized<Foo<U>>` when `T`
implements `Unsize<U>` or `CoerceUnsized<Foo<U>>`. This allows it to provide an
unsized coercion to `Foo<U>`.

> [!NOTE]
> While the definition of the unsized coercions and their implementation has been stabilized, the traits themselves are not yet stable and therefore can't be used directly in stable Rust.

r[coerce.least-upper-bound]
## Least upper bound coercions

r[coerce.least-upper-bound.intro]
In some contexts, the compiler must coerce together multiple types to try and
find the most general type. This is called a "Least Upper Bound" coercion.
LUB coercion is used and only used in the following situations:

+ To find the common type for a series of if branches.
+ To find the common type for a series of match arms.
+ To find the common type for array elements.
+ To find the type for the return type of a closure with multiple return statements.
+ To check the type for the return type of a function with multiple return statements.

r[coerce.least-upper-bound.target]
In each such case, there are a set of types `T0..Tn` to be mutually coerced
to some target type `T_t`, which is unknown to start.

r[coerce.least-upper-bound.computation]
Computing the LUB
coercion is done iteratively. The target type `T_t` begins as the type `T0`.
For each new type `Ti`, we consider whether

r[coerce.least-upper-bound.computation-identity]
+ If `Ti` can be coerced to the current target type `T_t`, then no change is made.

r[coerce.least-upper-bound.computation-replace]
+ Otherwise, check whether `T_t` can be coerced to `Ti`; if so, the `T_t` is
changed to `Ti`. (This check is also conditioned on whether all of the source
expressions considered thus far have implicit coercions.)

r[coerce.least-upper-bound.computation-unify]
+ If not, try to compute a mutual supertype of `T_t` and `Ti`, which will become the new target type.

### Examples:

```rust
# let (a, b, c) = (0, 1, 2);
// For if branches
let bar = if true {
    a
} else if false {
    b
} else {
    c
};

// For match arms
let baw = match 42 {
    0 => a,
    1 => b,
    _ => c,
};

// For array elements
let bax = [a, b, c];

// For closure with multiple return statements
let clo = || {
    if true {
        a
    } else if false {
        b
    } else {
        c
    }
};
let baz = clo();

// For type checking of function with multiple return statements
fn foo() -> i32 {
    let (a, b, c) = (0, 1, 2);
    match 42 {
        0 => a,
        1 => b,
        _ => c,
    }
}
```

In these examples, types of the `ba*` are found by LUB coercion. And the
compiler checks whether LUB coercion result of `a`, `b`, `c` is `i32` in the
processing of the function `foo`.

### Caveat

This description is obviously informal. Making it more precise is expected to
proceed as part of a general effort to specify the Rust type checker more
precisely.

[RFC 401]: https://github.com/rust-lang/rfcs/blob/master/text/0401-coercions.md
[RFC 1558]: https://github.com/rust-lang/rfcs/blob/master/text/1558-closure-to-fn-coercion.md
[subtype]: subtyping.md
[dyn compatible]: items/traits.md#dyn-compatibility
[type cast operator]: expressions/operator-expr.md#type-cast-expressions
[`Unsize`]: std::marker::Unsize
[`CoerceUnsized`]: std::ops::CoerceUnsized
[method-call expressions]: expressions/method-call-expr.md
[supertraits]: items/traits.md#supertraits
