# Trait and lifetime bounds

> **<sup>Syntax</sup>**\
> _TypeParamBounds_ :\
> &nbsp;&nbsp; _TypeParamBound_ ( `+` _TypeParamBound_ )<sup>\*</sup> `+`<sup>?</sup>
>
> _TypeParamBound_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; _Lifetime_ | _TraitBound_
>
> _TraitBound_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; `?`<sup>?</sup>
> [_ForLifetimes_]<sup>?</sup> [_TypePath_]\
> &nbsp;&nbsp; | `(` `?`<sup>?</sup>
> [_ForLifetimes_]<sup>?</sup> [_TypePath_] `)`
>
> _LifetimeBounds_ :\
> &nbsp;&nbsp; _Lifetime_ ( `+` _Lifetime_ )<sup>\*</sup> `+`<sup>?</sup>
>
> _Lifetime_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; [LIFETIME_OR_LABEL]\
> &nbsp;&nbsp; | `'static`\
> &nbsp;&nbsp; | `'_`

[Trait] and lifetime bounds provide a way for [generic items][generic] to
restrict which types and lifetimes are used as their parameters. Bounds can be
provided on any type in a [where clause]. There are also shorter forms for
certain common cases:

* Bounds written after declaring a [generic parameter][generic]:
  `fn f<A: Copy>() {}` is the same as `fn f<A> where A: Copy () {}`.
* In trait declarations as [supertraits]: `trait Circle : Shape {}` is
  equivalent to `trait Circle where Self : Shape {}`.
* In trait declarations as bounds on [associated types]:
  `trait A { type B: Copy; }` is equivalent to
  `trait A where Self::B: Copy { type B; }`.

Bounds on an item must be satisfied when using the item. When type checking and
borrow checking a generic item, the bounds can be used to determine that a
trait is implemented for a type. For example, given `Ty: Trait`

* In the body of a generic function, methods from `Trait` can be called on `Ty`
  values. Likewise associated constants on the `Trait` can be used.
* Associated types from `Trait` can be used.
* Generic functions and types with a `T: Trait` bounds can be used with `Ty`
  being used for `T`.

```rust
# type Surface = i32;
trait Shape {
    fn draw(&self, Surface);
    fn name() -> &'static str;
}

fn draw_twice<T: Shape>(surface: Surface, sh: T) {
    sh.draw(surface);           // Can call method because T: Shape
    sh.draw(surface);
}

fn copy_and_draw_twice<T: Copy>(surface: Surface, sh: T) where T: Shape {
    let shape_copy = sh;        // doesn't move sh because T: Copy
    draw_twice(surface, sh);    // Can use generic function because T: Shape
}

struct Figure<S: Shape>(S, S);

fn name_figure<U: Shape>(
    figure: Figure<U>,          // Type Figure<U> is well-formed because U: Shape
) {
    println!(
        "Figure of two {}",
        U::name(),              // Can use associated function
    );
}
```

Trait and lifetime bounds are also used to name [trait objects].

## `?Sized`

`?` is only used to declare that the [`Sized`] trait may not be
implemented for a type parameter or associated type. `?Sized` may
not be used as a bound for other types.

## Lifetime bounds

Lifetime bounds can be applied to types or other lifetimes. The bound `'a: 'b`
is usually read as `'a` *outlives* `'b`. `'a: 'b` means that `'a` lasts longer
than `'b`, so a reference `&'a ()` is valid whenever `&'b ()` is valid.

```rust
fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) where 'a: 'b {
    y = x;                      // &'a i32 is a subtype of &'b i32 because 'a: 'b
    let r: &'b &'a i32 = &&0;   // &'b &'a i32 is well formed because 'a: 'b
}
```

`T: 'a` means that all lifetime parameters of `T` outlive `'a`. For example if
`'a` is an unconstrained lifetime parameter then `i32: 'static` and
`&'static str: 'a` are satisfied but `Vec<&'a ()>: 'static` is not.

## Higher-ranked trait bounds

Type bounds may be *higher ranked* over lifetimes. These bounds specify a bound
is true *for all* lifetimes. For example, a bound such as `for<'a> &'a T:
PartialEq<i32>` would require an implementation like

```rust
# struct T;
impl<'a> PartialEq<i32> for &'a T {
    // ...
#    fn eq(&self, other: &i32) -> bool {true}
}
```

and could then be used to compare a `&'a T` with any lifetime to an `i32`.

Only a higher-ranked bound can be used here as the lifetime of the reference is
shorter than a lifetime parameter on the function:

```rust
fn call_on_ref_zero<F>(f: F) where for<'a> F: Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}
```

Higher-ranked lifetimes may also be specified just before the trait, the only
difference is the scope of the lifetime parameter, which extends only to the
end of the following trait instead of the whole bound. This function is
equivalent to the last one.

```rust
fn call_on_ref_zero<F>(f: F) where F: for<'a> Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}
```

[LIFETIME_OR_LABEL]: tokens.md#lifetimes-and-loop-labels
[_TypePath_]: paths.md#paths-in-types
[`Sized`]: special-types-and-traits.md#sized
[_ForLifetimes_]: items/generics.html#where-clauses

[associated types]: items/associated-items.md#associated-types
[supertraits]: items/traits.md#supertraits
[generic]: items/generics.md
[Trait]: items/traits.md#trait-bounds
[trait objects]: types/trait-object.md
[where clause]: items/generics.md#where-clauses
