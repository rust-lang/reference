# Subtyping and Variance

Subtyping is implicit and can occur at any stage in type checking or
inference. Subtyping in Rust is very restricted and occurs only due to
variance with respect to lifetimes and between types with higher ranked
lifetimes. If we were to erase lifetimes from types, then the only subtyping
would be due to type equality.

Consider the following example: string literals always have `'static`
lifetime. Nevertheless, we can assign `s` to `t`:

```rust
fn bar<'a>() {
    let s: &'static str = "hi";
    let t: &'a str = s;
}
```

Since `'static` outlives the lifetime parameter `'a`, `&'static str` is a
subtype of `&'a str`.

[Higher-ranked]&#32;[function pointers] and [trait objects] have another
subtype relation. They are subtypes of types that are given by substitutions of
the higher-ranked lifetimes. Some examples:

```rust
// Here 'a is substituted for 'static
let subtype: &(for<'a> fn(&'a i32) -> &'a i32) = &((|x| x) as fn(&_) -> &_);
let supertype: &(fn(&'static i32) -> &'static i32) = subtype;

// This works similarly for trait objects
let subtype: &(dyn for<'a> Fn(&'a i32) -> &'a i32) = &|x| x;
let supertype: &(dyn Fn(&'static i32) -> &'static i32) = subtype;

// We can also substitute one higher-ranked lifetime for another
let subtype: &(for<'a, 'b> fn(&'a i32, &'b i32))= &((|x, y| {}) as fn(&_, &_));
let supertype: &for<'c> fn(&'c i32, &'c i32) = subtype;
```

## Variance

Variance is a property that generic types have with respect to their arguments.
A generic type's *variance* in a parameter is how the subtyping of the
parameter affects the subtyping of the type.

* `F<T>` is *covariant* over `T` if `T` being a subtype of `U` implies that
  `F<T>` is a subtype of `F<U>` (subtyping "passes through")
* `F<T>` is *contravariant* over `T` if `T` being a subtype of `U` implies that
  `F<U>` is a subtype of `F<T>`
* `F<T>` is *invariant* over `T` otherwise (no subtyping relation can be
  derived)

Variance of types is automatically determined as follows

| Type                          | Variance in `'a`  | Variance in `T`   |
|-------------------------------|-------------------|-------------------|
| `&'a T`                       | covariant         | covariant         |
| `&'a mut T`                   | covariant         | invariant         |
| `*const T`                    |                   | covariant         |
| `*mut T`                      |                   | invariant         |
| `[T]` and `[T; n]`            |                   | covariant         |
| `fn() -> T`                   |                   | covariant         |
| `fn(T) -> ()`                 |                   | contravariant     |
| `fn(T) -> T`                  |                   | invariant         |
| `std::cell::UnsafeCell<T>`    |                   | invariant         |
| `std::marker::PhantomData<T>` |                   | covariant         |
| `dyn Trait<T> + 'a`           | covariant         | invariant         |

The variance of other `struct`, `enum`, `union`, and tuple types is decided by
looking at the variance of the types of their fields. If the parameter is used
in positions with different variances then the parameter is invariant. For
example the following struct is covariant in `'a` and `T` and invariant in `'b`
and `U`.

```rust
use std::cell::UnsafeCell;
struct Variance<'a, 'b, T, U: 'a> {
    x: &'a U,               // This makes `Variance` covariant in 'a, and would
                            // make it covariant in U, but U is used later
    y: *const T,            // Covariant in T
    z: UnsafeCell<&'b f64>, // Invariant in 'b
    w: *mut U,              // Invariant in U, makes the whole struct invariant
}
```

[function pointers]: types/function-pointer.md
[Higher-ranked]: ../nomicon/hrtb.html
[trait objects]: types/trait-object.md
