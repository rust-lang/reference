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

r[coerce.types.unsize]
* `T` to `U` if `T: CoerceUnsized<U>`. For example:
    ```rust
    const _: &dyn std::fmt::Display = &0u8; // &u8 -> &dyn Display
    const _: &[u32] = &[0, 1, 2, 3, 4, 5];  // &[u32; 4] -> &[u32]
    ```

    See [unsized coercion](#unsized-coercions) for more details.

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

r[coerce.types.fn]
* Function item types to `fn` pointers

r[coerce.types.closure]
* Non capturing closures to `fn` pointers

r[coerce.types.never]
* `!` to any `T`

r[coerce.unsize]
### Unsized Coercions

r[coerce.unsize.intro]
The following coercions are called "unsized coercions", since their targets contain an unsized type. Unsized coercions apply to pointer-like types where some type information known about the referent at compile-time (e.g. its size or traits that it implements) can be *erased*. For example:

```rust
use std::cell::Cell;

fn main() {
    // `&[u8; 0]` can be coerced to `&[u8]`.
    //
    // Here `&_` is the pointer-like type, `[u8; 0]` is the original
    // pointee, and `[u8]` is more erased pointee (losing the length).
    let _: &[u8] = &[];

    trait A: Super {}
    impl A for () {}

    trait Super {}
    impl Super for () {}

    // `&()` can be coerced to `&dyn A`, losing the type information.
    let _: &dyn A = &();

    // `&dyn A` can be coerced to `&dyn Super`, losing the fact that
    // the underlying type (unit) implements `A` too.
    let _: &dyn Super = &() as &dyn A;

    // The same coercions work with other pointer-like types and
    // wrappers over them:
    let _: Box<[u8]> = Box::<[u8; 0]>::new([]);
    let _: Cell<Box<[u8]>> = Cell::new(Box::<[u8; 0]>::new([]));

    // The result of the coercion doesn't *have* to be the same
    // pointer-like type, although this is only allowed for certain
    // pairs of pointer-like types.
    let _: *const dyn A = &mut ();
}
```

> [!NOTE]
> The term "unsized" might be confusing, as the coercion works on sized types (the pointer-like type itself) and the source pointer might point to an unsized type in the first place (e.g. `&dyn A -> &dyn Super` in the example above).
>
> Here, "unsized" refers to the main purpose of these coercions, which is to produce (pointers to) unsized types. Since unsized types can't exist except behind a pointer, the pointers are deemphasized.

> [!NOTE]
> When doing an unsized coercion, the internal [pointer metadata] type changes. For example, when coercing `&u32` to `&dyn Debug`, the metadata type changes from `()` to `DynMetadata<dyn Debug>` (these metadata types are not yet stable, see [#81513]). This can also lead to a change in the pointer size --- `&u32` is half the size of `&dyn Debug`.

r[coerce.unsize.traits]
Three internal traits, [`Unsize`], [`CoerceUnsized`], and [`PinCoerceUnsized`] are used to assist in this process.

r[coerce.unsize.traits.unsize]
[`Unsize`] represents that the target type is layout compatible with the source type and the [pointer metadata] of the target type can be derived from the metadata of the source. This implies that a pointer to the source type can be converted to a pointer to the target type.

> [!EXAMPLE]
> Because `[T; N]` implements `Unsize<[T]>`, you can *unsize* `&[T; N]` into `&[T]`.

r[coerce.unsize.traits.coerce-unsized]
[`CoerceUnsized`] represents that a pointer-like type can be coerced to another pointer-like type when `Unsize` is implemented for the pointee of the source type.

> [!EXAMPLE]
> `&T` implements `CoerceUnsized<&U>` when `T: Unsize<U>`. So, since `u8: Unsize<dyn Display>`, `&u8: CoerceUnsized<&dyn Display>`.
>
> ```rust
> # #![ feature(coerce_unsized, unsize) ]
> # use core::{fmt::Display, marker::Unsize, ops::CoerceUnsized};
> fn f()
> where
>     // These bounds are "trivial".
>     u8: Unsize<dyn Display>,
>     for<'a> &'a u8: CoerceUnsized<&'a (dyn Display + 'a)>,
> {
>     let _: &dyn Display = &0u8;
> }
> ```
>
> ```rust
> # #![ feature(coerce_unsized, unsize) ]
> # use core::{marker::Unsize, ops::CoerceUnsized};
> fn f<T, U>(x: T)
> where
>     T: Unsize<U>,
>     for<'a> &'a T: CoerceUnsized<&'a U>,
> {
>     let _: &U = &x;
> }
> ```

r[coerce.unsize.traits.pin-coerce-unsized]
[`PinCoerceUnsized`] is an unsafe marker trait that is implemented for pointer-like types to indicate it is safe to perform an unsized coercion when the pointee is pinned (and must therefore uphold the [`Pin`] guarantees).

> [!EXAMPLE]
> ```rust
> # #![ feature(coerce_unsized, pin_coerce_unsized_trait) ]
> # use core::{ops::CoerceUnsized, pin::{Pin, PinCoerceUnsized}};
> trait Tr { fn f(); }
> impl<T, U> Tr for (T, U)
> where
>     // Assuming these are true...
>     T: CoerceUnsized<U> + PinCoerceUnsized,
>     U: PinCoerceUnsized,
> {
>     // ...we can prove this where clause:
>     fn f() where Pin<T>: CoerceUnsized<Pin<U>> {}
> }
> ```
>
> ```rust
> # #![ feature(coerce_unsized, pin_coerce_unsized_trait) ]
> # use core::{ops::CoerceUnsized, pin::{Pin, PinCoerceUnsized}};
> fn f<T, U>(x: Pin<T>)
> where
>     T: CoerceUnsized<U> + PinCoerceUnsized,
>     U: PinCoerceUnsized,
> {
>     let _: Pin<U> = x;
> }
> ```

r[coerce.unsize.built-in]
The following implementations of [`Unsize`] are built-in:

r[coerce.unsize.built-in.slice]
* `[T; n]: Unsize<[T]>`.

r[coerce.unsize.built-in.trait-object]
* `T: Unsize<dyn U>`, when `T` implements `U + Sized`, and `U` is [dyn compatible].

r[coerce.unsize.built-in.trait-upcast]
* `dyn Trait: Unsize<dyn Super>`, when `Super` is one of `Trait`'s [supertraits].
    * This allows dropping auto traits, i.e. `dyn Trait + Auto` to `dyn Super` is allowed.
    * This allows adding auto traits if the principal trait has the auto trait as a super trait, i.e. given `trait Trait: Super + Auto {}`, `dyn Trait` to `dyn Trait + Auto` or to `dyn Super + Auto` coercions are allowed.

r[coerce.unsize.built-in.composite]
* `S<A0, .., An, T0, .., Tn, B0, .., Bn>: Unsize<S<A0, .., An, .., U0, .., Un, .., B0, .., Bn>>`, when:
    * `S<..>` is a struct.
    * Where `F<..>` is the type of the last field of `S<..>`, `F<T0, .., Tn>: Unsize<F<U0, .., Un>>`.

r[coerce.unsize.pointer]
A type `S<T0, .., Tn>` *can* implement `CoerceUnsized<S<U0, .., Un>>` if both:

* Only one field of `S<T0, .., Tn>` has a different type than the same field of `S<U0, .., Un>` (ignoring fields of type `PhantomData<_>`).
* That field's type implements `CoerceUnsized<X>` where `X` is the type of the corresponding field in `S<U0, .., Un>`.

This allows `S<T0, .., Tn>` types to be coerced to `S<U0, .., Un>`.

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

[#81513]: https://github.com/rust-lang/rust/issues/81513
[RFC 401]: https://github.com/rust-lang/rfcs/blob/master/text/0401-coercions.md
[RFC 1558]: https://github.com/rust-lang/rfcs/blob/master/text/1558-closure-to-fn-coercion.md
[subtype]: subtyping.md
[dyn compatible]: items/traits.md#dyn-compatibility
[pointer metadata]: glossary.pointer-metadata
[type cast operator]: expressions/operator-expr.md#type-cast-expressions
[`Pin`]: std::pin::Pin
[`PinCoerceUnsized`]: std::pin::PinCoerceUnsized
[`Unsize`]: std::marker::Unsize
[`CoerceUnsized`]: std::ops::CoerceUnsized
[method-call expressions]: expressions/method-call-expr.md
[supertraits]: items/traits.md#supertraits
