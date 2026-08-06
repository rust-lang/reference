r[generics.types]
# Generic types

r[generics.types.intro]
A *type parameter* is a generic parameter for a type.

r[generics.types.at-least-once]
[Structs], [enumerations], and [unions] must use each of their type parameters at least once in their fields or variants.

> [!EXAMPLE]
> ```rust,compile_fail
> // ERROR: type parameter `T` is never used
> struct Unused<T>;
>
> // ERROR: type parameter `T` is never used
> enum Empty<T> { A }
> ```
>
> ```rust
> // OK: `T` appears in a field.
> struct Wrapper<T>(T);
>
> // A type parameter that does not appear directly in any field may be used
> // via `std::marker::PhantomData`.
> struct Key<T> {
>     id: u64,
>     _phantom: std::marker::PhantomData<T>,
> }
> ```

r[generics.types.sized]
Unless the `?Sized` [opt-out bound][`Sized`] is present, a type parameter has an implicit [`Sized`] bound. This means the concrete type supplied for the parameter must have a size known at compile time.

> [!EXAMPLE]
> ```rust
> // `T` implicitly requires `T: Sized`.
> fn takes_sized<T>(x: T) {}
>
> // `T` may be a dynamically sized type.
> fn takes_unsized<T: ?Sized>(x: &T) {}
> ```

r[generics.types.default-constraint]
The default type of a type parameter must satisfy all of the type parameter's [trait bounds].

> [!EXAMPLE]
> ```rust,compile_fail
> // ERROR: the default type `String` does not implement `Copy`
> struct Foo<T: Copy = String>(T);
> ```
>
> ```rust
> // OK: `i32` satisfies the `Copy` bound
> struct Bar<T: Copy = i32>(T);
> ```

[enumerations]: items.enum
[structs]: items.struct
[trait]: items.traits
[trait bounds]: bound
[unions]: items.union
