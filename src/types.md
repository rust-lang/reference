# Types

Every variable, item and value in a Rust program has a type. The _type_ of a
*value* defines the interpretation of the memory holding it.

Built-in types are tightly integrated into the language, in nontrivial ways
that are not possible to emulate in user-defined types. User-defined types have
limited capabilities.

## Primitive types

Some types are defined by the language, rather than as part of the standard
library, these are called _primitive types_. Some of these are individual
types:

* The boolean type `bool` with values `true` and `false`.
* The [machine types] (integer and floating-point).
* The [machine-dependent integer types].
* The [textual types] `char` and `str`.

There are also some primitive constructs for generic types built in to the
language:

* [Tuples]
* [Arrays]
* [Slices]
* [Function pointers]
* [References]
* [Pointers]

[machine types]: #machine-types
[machine-dependent integer types]: #machine-dependent-integer-types
[textual types]: #textual-types
[Tuples]: #tuple-types
[Arrays]: #array-and-slice-types
[Slices]: #array-and-slice-types
[References]: #pointer-types
[Pointers]: #raw-pointers-const-and-mut
[Function pointers]: #function-pointer-types
[function]: #function-types
[closure]: #closure-types

## Numeric types

### Machine types

The machine types are the following:

* The unsigned word types `u8`, `u16`, `u32` and `u64`, with values drawn from
  the integer intervals [0, 2^8 - 1], [0, 2^16 - 1], [0, 2^32 - 1] and
  [0, 2^64 - 1] respectively.

* The signed two's complement word types `i8`, `i16`, `i32` and `i64`, with
  values drawn from the integer intervals [-(2^(7)), 2^7 - 1],
  [-(2^(15)), 2^15 - 1], [-(2^(31)), 2^31 - 1], [-(2^(63)), 2^63 - 1]
  respectively.

* The IEEE 754-2008 `binary32` and `binary64` floating-point types: `f32` and
  `f64`, respectively.

### Machine-dependent integer types

The `usize` type is an unsigned integer type with the same number of bits as the
platform's pointer type. It can represent every memory address in the process.

The `isize` type is a signed integer type with the same number of bits as the
platform's pointer type. The theoretical upper bound on object and array size
is the maximum `isize` value. This ensures that `isize` can be used to calculate
differences between pointers into an object or array and can address every byte
within an object along with one byte past the end.

## Textual types

The types `char` and `str` hold textual data.

A value of type `char` is a [Unicode scalar value](
http://www.unicode.org/glossary/#unicode_scalar_value) (i.e. a code point that
is not a surrogate), represented as a 32-bit unsigned word in the 0x0000 to
0xD7FF or 0xE000 to 0x10FFFF range. A `[char]` is effectively a UCS-4 / UTF-32
string.

A value of type `str` is a Unicode string, represented as an array of 8-bit
unsigned bytes holding a sequence of UTF-8 code points. Since `str` is a
[dynamically sized type], it is not a _first-class_ type, but can only be
instantiated through a pointer type, such as `&str`.

## Tuple types

A tuple *type* is a heterogeneous product of other types, called the *elements*
of the tuple. It has no nominal name and is instead structurally typed.

Tuple types and values are denoted by listing the types or values of their
elements, respectively, in a parenthesized, comma-separated list.

Because tuple elements don't have a name, they can only be accessed by
pattern-matching or by using `N` directly as a field to access the `N`th
element.

An example of a tuple type and its use:

```rust
type Pair<'a> = (i32, &'a str);
let p: Pair<'static> = (10, "ten");
let (a, b) = p;

assert_eq!(a, 10);
assert_eq!(b, "ten");
assert_eq!(p.0, 10);
assert_eq!(p.1, "ten");
```

For historical reasons and convenience, the tuple type with no elements (`()`)
is often called ‘unit’ or ‘the unit type’.

## Array, and Slice types

Rust has two different types for a list of items:

* `[T; N]`, an 'array'
* `[T]`, a 'slice'

An array has a fixed size, and can be allocated on either the stack or the
heap.

A slice is a [dynamically sized type] representing a 'view' into an array. To
use a slice type it generally has to be used behind a pointer for example as

* `&[T]`, a 'shared slice', often just called a 'slice', it doesn't own the
  data it points to, it borrows it.
* `&mut [T]`, a 'mutable slice', mutably borrows the data it points to.
* `Box<[T]>`, a 'boxed slice'

Examples:

```rust
// A stack-allocated array
let array: [i32; 3] = [1, 2, 3];

// A heap-allocated array, coerced to a slice
let boxed_array: Box<[i32]> = Box::new([1, 2, 3]);

// A (shared) slice into an array
let slice: &[i32] = &boxed_array[..];
```

All elements of arrays and slices are always initialized, and access to an
array or slice is always bounds-checked in safe methods and operators.

> Note: The [`Vec<T>`] standard library type provides a heap-allocated resizable
> array type.

## Struct types

A `struct` *type* is a heterogeneous product of other types, called the
*fields* of the type.[^structtype]

New instances of a `struct` can be constructed with a [struct
expression](expressions/struct-expr.html).

The memory layout of a `struct` is undefined by default to allow for compiler
optimizations like field reordering, but it can be fixed with the
`#[repr(...)]` attribute. In either case, fields may be given in any order in a
corresponding struct *expression*; the resulting `struct` value will always
have the same memory layout.

The fields of a `struct` may be qualified by [visibility
modifiers](visibility-and-privacy.html), to allow access to data in a struct
outside a module.

A _tuple struct_ type is just like a struct type, except that the fields are
anonymous.

A _unit-like struct_ type is like a struct type, except that it has no fields.
The one value constructed by the associated [struct expression] is the only
value that inhabits such a type.

[^structtype]: `struct` types are analogous to `struct` types in C, the
    *record* types of the ML family, or the *struct* types of the Lisp family.

## Enumerated types

An *enumerated type* is a nominal, heterogeneous disjoint union type, denoted
by the name of an [`enum` item](items/enumerations.html). [^enumtype]

An [`enum` item](items/enumerations.html) declares both the type and a number
of *variants*, each of which is independently named and has the syntax of a
struct, tuple struct or unit-like struct.

New instances of an `enum` can be constructed in an [enumeration variant
expression](expressions/enum-variant-expr.html).

Any `enum` value consumes as much memory as the largest variant for its
corresponding `enum` type, as well as the size needed to store a discriminant.

Enum types cannot be denoted *structurally* as types, but must be denoted by
named reference to an [`enum` item](items/enumerations.html).

[^enumtype]: The `enum` type is analogous to a `data` constructor declaration in
             ML, or a *pick ADT* in Limbo.

## Union types

A *union type* is a nominal, heterogeneous C-like union, denoted by the name of
a [`union` item](items/unions.html).

A union contains the value of any one of its fields. Since the accessing the
wrong field can cause unexpected or undefined behaviour, `unsafe` is required
to read from a union field or to write to a field that doesn't implement
[`Copy`].

The memory layout of a `union` is undefined by default, but the `#[repr(...)]`
attribute can be used to fix a layout.

[`Copy`]: special-types-and-traits.html#copy

## Recursive types

Nominal types &mdash; [structs](#struct-types),
[enumerations](#enumerated-types) and [unions](#union-types) &mdash; may be
recursive. That is, each `enum` variant or `struct` or `union` field may refer,
directly or indirectly, to the enclosing `enum` or `struct` type itself. Such
recursion has restrictions:

* Recursive types must include a nominal type in the recursion (not mere [type
  definitions](../grammar.html#type-definitions), or other structural types
  such as [arrays](#array-and-slice-types) or [tuples](#tuple-types)). So
  `type Rec = &'static [Rec]` is not allowed.
* The size of a recursive type must be finite; in other words the recursive
  fields of the type must be [pointer types](#pointer-types).
* Recursive type definitions can cross module boundaries, but not module
  *visibility* boundaries, or crate boundaries (in order to simplify the module
  system and type checker).

An example of a *recursive* type and its use:

```rust
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

let a: List<i32> = List::Cons(7, Box::new(List::Cons(13, Box::new(List::Nil))));
```

## Pointer types

All pointers in Rust are explicit first-class values. They can be moved or
copied, stored into data structs, and returned from functions.

### Shared references (`&`)

These point to memory _owned by some other value_. When a shared reference to a
value is created it prevents direct mutation of the value. [Interior
mutability](interior-mutability.html) provides an exception for this in certain
circumstances. As the name suggests, any number of shared references to a value
may exit. A shared reference type is written `&type`, or `&'a type` when you
need to specify an explicit lifetime. Copying a reference is a "shallow"
operation: it involves only copying the pointer itself, that is, pointers are
`Copy`. Releasing a reference has no effect on the value it points to, but
referencing of a [temporary value](expressions.html#temporary-lifetimes) will
keep it alive during the scope of the reference itself.

### Mutable references (`&mut`)

These also point to memory owned by some other value. A mutable reference type
is written `&mut type` or `&'a mut type`. A mutable reference (that hasn't been
borrowed) is the only way to access the value it points to, so is not `Copy`.

### Raw pointers (`*const` and `*mut`)

Raw pointers are pointers without safety or liveness guarantees. Raw pointers
are written as `*const T` or `*mut T`, for example `*const i32` means a raw
pointer to a 32-bit integer. Copying or dropping a raw pointer has no effect on
the lifecycle of any other value. Dereferencing a raw pointer is an [`unsafe`
operation](unsafe-functions.html), this can also be used to convert a raw
pointer to a reference by reborrowing it (`&*` or `&mut *`). Raw pointers are
generally discouraged in Rust code; they exist to support interoperability with
foreign code, and writing performance-critical or low-level functions.

When comparing pointers they are compared by their address, rather than by what
they point to. When comparing pointers to [dynamically sized
types](dynamically-sized-types.html) they also have their addition data
compared.

### Smart Pointers

The standard library contains additional 'smart pointer' types beyond references
and raw pointers.

## Function item types

When referred to, a function item, or the constructor of a tuple-like struct or
enum variant, yields a zero-sized value of its _function item type_. That type
explicitly identifies the function - its name, its type arguments, and its
early-bound lifetime arguments (but not its late-bound lifetime arguments,
which are only assigned when the function is called) - so the value does not
need to contain an actual function pointer, and no indirection is needed when
the function is called.

There is no syntax that directly refers to a function item type, but the
compiler will display the type as something like `fn(u32) -> i32 {fn_name}` in
error messages.

Because the function item type explicitly identifies the function, the item
types of different functions - different items, or the same item with different
generics - are distinct, and mixing them will create a type error:

```rust,compile_fail,E0308
fn foo<T>() { }
let x = &mut foo::<i32>;
*x = foo::<u32>; //~ ERROR mismatched types
```

However, there is a [coercion] from function items to [function
pointers](#function-pointer-types) with the same signature, which is triggered
not only when a function item is used when a function pointer is directly
expected, but also when different function item types with the same signature
meet in different arms of the same `if` or `match`:

[coercion]: type-coercions.html

```rust
# let want_i32 = false;
# fn foo<T>() { }

// `foo_ptr_1` has function pointer type `fn()` here
let foo_ptr_1: fn() = foo::<i32>;

// ... and so does `foo_ptr_2` - this type-checks.
let foo_ptr_2 = if want_i32 {
    foo::<i32>
} else {
    foo::<u32>
};
```

All function items implement [Fn], [FnMut], [FnOnce], [Copy], [Clone], [Send], 
and [Sync].

## Function pointer types

Function pointer types, written using the `fn` keyword, refer to a function
whose identity is not necessarily known at compile-time. They can be created
via a coercion from both [function items](#function-item-types) and
non-capturing [closures](#closure-types).

A function pointer type consists of a possibly-empty set of function-type
modifiers (such as `unsafe` or `extern`), a sequence of input types and an
output type.

An example where `Binop` is defined as a function pointer type:

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

let mut x = add(5,7);

type Binop = fn(i32, i32) -> i32;
let bo: Binop = add;
x = bo(5,7);
```

## Closure types

A [closure expression] produces a closure value with a unique, anonymous type
that cannot be written out. A closure type is approximately equivalent to a
struct which contains the captured variables. For instance, the following
closure:

```rust
fn f<F : FnOnce() -> String> (g: F) {
    println!("{}", g());
}

let mut s = String::from("foo");
let t = String::from("bar");

f(|| {
    s += t;
    s
});
// Prints "foobar".
```

generates a closure type roughly like the following:

```rust
struct Closure<'a> {
    s : String
    t : &'a String
}

impl<'a> FnOnce() -> String for Closure<'a> {
    fn call_once(self) -> String {
        self.s += self.t;
        self.s
    }
}
```

so that the call to `f` works as if it were:

```rust,ignore
f(Closure{s: s, t: &t});
```

The compiler prefers to capture a closed-over variable by immutable borrow,
followed by mutable borrow and finally by move (or copy, for [`Copy`] types). It
will pick the first choice of these that allows the closure to compile. If the
`move` keyword is used, then all captures are by move or copy, regardless of
whether a borrow would work. The `move` keyword is usually used to allow the
closure to outlive the captured values, such as if the closure is being returned
or used to spawn a new thread.

Structs and tuples are always captured entirely, not by individual fields. It
may be necessary to borrow into a local variable in order to capture a single
field:

```rust
struct SetVec {
    set: HashSet<u32>,
    vec: Vec<u32>
}

impl Pair {
    fn populate(&mut self) {
        let vec = &mut self.vec;
        self.set.iter().for_each(|&n| {
            vec.push(n);
        })
    }
}
```

If, instead, the closure were to use `self.vec` directly, then it would attempt
to capture `self` by mutable reference. But since `self.set` is already
borrowed to iterate over, the closure would not compile.

### Call traits and coercions

Closure types all implement `[FnOnce]`, indicating that they can be called once
by consuming ownership of the closure. Additionally, some closures implement
more specific call traits:

* A closure which does not move out of any captured variables implements
  `[FnMut]`, indicating that it can be called by mutable reference.

* A closure which does not mutate or move out of any captured variables
  implements `[Fn]`, indicating that it can be called by shared reference.

> Note that `move` closures may still implement `[Fn]` or `[FnMut]`, even
> though they capture variables by move. This is because the traits
> implemented by a closure type are determined by what the closure does with
> captured values, not how it captures them.

In addition to the call traits, *non-capturing closures*---those that don't
capture anything from their environment---can be coerced to function pointers
(`fn`) with the matching signature.

```rust
let add = |x, y| x + y;

let mut x = add(5,7);

type Binop = fn(i32, i32) -> i32;
let bo: Binop = add;
x = bo(5,7);
```

### Other traits

Closure types implement the following traits, if allowed to do so by the
captured values:

* `[Sized]`
* `[Send]`
* `[Sync]`
* `[Clone]`
* `[Copy]`

`[Sized]` is always implemented (local variables are all sized, so all captured
values must be too). The rules for `[Send]` and `[Sync]` match those for normal
struct types, while `[Clone]` and `[Copy]` behave as if [derived][derive]. For
`[Clone]`, the order of cloning of the captured variables is left unspecified.

Because captures are often by reference, the following general rules arise:

* All closures are `[Sized]`.
* A closure is `[Sync]` if all values captured by mutable reference, move, or
  copy are `[Sync]`.
* A closure is `[Send]` if all values captured by shared reference are `[Sync]`,
  and all values captured by mutable reference, move, or copy are `[Send]`.
* A closure is `[Clone]` or `[Copy]` if it does not capture any values by
  mutable reference, and if all values it captures by move or copy are `[Clone]`
  or `[Copy]`, respectively.

## Trait objects

A *trait object* is an opaque value of another type that implements a set of
traits. The set of traits is made up of an [object safe] *base trait* plus any
number of [auto traits].

Trait objects implement the base trait, its auto traits, and any super traits
of the base trait.

Trait objects are written as the path to the base trait followed by the list
of auto traits followed optionally by a lifetime bound all separated by `+`. For
example, given a trait `Trait`, the following are all trait objects: `Trait`,
`Trait + Send`, `Trait + Send + Sync`, `Trait + 'static`,
`Trait + Send + 'static`.

Two trait object types alias each other if the base traits alias each other and
if the sets of auto traits are the same and the lifetime bounds are the same.
For example, `Trait + Send + UnwindSafe` is the same as
`Trait + Unwindsafe + Send`.

> Warning: With two trait object types, even when the complete set of traits is
> the same, if the base traits differ, the type is different. For example,
> `Send + Sync` is a different type from `Sync + Send`. See [issue 33140].

> Warning: Including the same auto trait multiple times is allowed, and each
> instance is considered a unique type. As such, `Trait + Send` is a distinct
> type than `Trait + Send + Send`. See [issue 47010].

Due to the opaqueness of which concrete type the value is of, trait objects are
[dynamically sized types]. Like all
<abbr title="dynamically sized types">DSTs</abbr>, trait objects are used
behind some type of pointer; for example `&SomeTrait` or `Box<SomeTrait>`. Each
instance of a pointer to a trait object includes:

 - a pointer to an instance of a type `T` that implements `SomeTrait`
 - a _virtual method table_, often just called a _vtable_, which contains, for
   each method of `SomeTrait` that `T` implements, a pointer to `T`'s
   implementation (i.e. a function pointer).

The purpose of trait objects is to permit "late binding" of methods. Calling a
method on a trait object results in virtual dispatch at runtime: that is, a
function pointer is loaded from the trait object vtable and invoked indirectly.
The actual implementation for each vtable entry can vary on an object-by-object
basis.

An example of a trait object:

```rust
trait Printable {
    fn stringify(&self) -> String;
}

impl Printable for i32 {
    fn stringify(&self) -> String { self.to_string() }
}

fn print(a: Box<Printable>) {
    println!("{}", a.stringify());
}

fn main() {
    print(Box::new(10) as Box<Printable>);
}
```

In this example, the trait `Printable` occurs as a trait object in both the
type signature of `print`, and the cast expression in `main`.

### Trait Object Lifetime Bounds

Since a trait object can contain references, the lifetimes of those references
need to be expressed as part of the trait object. The assumed lifetime of
references held by a trait object is called its *default object lifetime bound*.
These were defined in [RFC 599] and amended in [RFC 1156].

For traits that themselves have no lifetime parameters:
* If there is a unique bound from the containing type then that is the default.
* If there is more than one bound from the containing type then an explicit
  bound must be specified.
* Otherwise the default bound is `'static`.

```rust,ignore
// For the following trait...
trait Foo { }

// These two are the same as Box<T> has no lifetime bound on T
Box<Foo>
Box<Foo + 'static>

// ...and so are these:
impl Foo {}
impl Foo + 'static {}

// ...so are these, because &'a T requires T: 'a
&'a Foo
&'a (Foo + 'a)

// std::cell::Ref<'a, T> also requires T: 'a, so these are the same
std::cell::Ref<'a, Foo>
std::cell::Ref<'a, Foo + 'a>

// This is an error:
struct TwoBounds<'a, 'b, T: ?Sized + 'a + 'b>
TwoBounds<'a, 'b, Foo> // Error: the lifetime bound for this object type cannot
                       // be deduced from context

```

The `+ 'static` and `+ 'a` refer to the default bounds of those kinds of trait
objects, and also to how you can directly override them. Note that the innermost
object sets the bound, so `&'a Box<Foo>` is still `&'a Box<Foo + 'static>`.

For traits that have a single lifetime _bound_ of their own then, instead of
infering 'static as the default bound, the bound on the trait is used instead

```rust,ignore
// For the following trait...
trait Bar<'a>: 'a { }

// ...these two are the same:
Box<Bar<'a>>
Box<Bar<'a> + 'a>

// ...and so are these:
impl<'a> Foo<'a> {}
impl<'a> Foo<'a> + 'a {}

// This is still an error:
struct TwoBounds<'a, 'b, T: ?Sized + 'a + 'b>
TwoBounds<'a, 'b, Foo<'c>>
```

## Type parameters

Within the body of an item that has type parameter declarations, the names of
its type parameters are types:

```rust
fn to_vec<A: Clone>(xs: &[A]) -> Vec<A> {
    if xs.is_empty() {
        return vec![];
    }
    let first: A = xs[0].clone();
    let mut rest: Vec<A> = to_vec(&xs[1..]);
    rest.insert(0, first);
    rest
}
```

Here, `first` has type `A`, referring to `to_vec`'s `A` type parameter; and
`rest` has type `Vec<A>`, a vector with element type `A`.

## Self types

The special type `Self` has a meaning within traits and implementations: it
refers to the implementing type. For example, in:

```rust
pub trait From<T> {
    fn from(T) -> Self;
}

impl From<i32> for String {
    fn from(x: i32) -> Self {
        x.to_string()
    }
}
```

The notation `Self` in the impl refers to the implementing type: `String`. In
another example:

```rust
trait Printable {
    fn make_string(&self) -> String;
}

impl Printable for String {
    fn make_string(&self) -> String {
        (*self).clone()
    }
}
```

> Note: The notation `&self` is a shorthand for `self: &Self`.

[Fn]: ../std/ops/trait.Fn.html
[FnMut]: ../std/ops/trait.FnMut.html
[FnOnce]: ../std/ops/trait.FnOnce.html
[Copy]: special-types-and-traits.html#copy
[Clone]: special-types-and-traits.html#clone
[Send]: special-types-and-traits.html#send
[Sync]: special-types-and-traits.html#sync
[derive]: attributes.html#derive
[`Vec<T>`]: ../std/vec/struct.Vec.html
[dynamically sized type]: dynamically-sized-types.html
[dynamically sized types]: dynamically-sized-types.html
[RFC 599]: https://github.com/rust-lang/rfcs/blob/master/text/0599-default-object-bound.md
[RFC 1156]: https://github.com/rust-lang/rfcs/blob/master/text/1156-adjust-default-object-bounds.md
[struct expression]: expressions/struct-expr.html
[closure expression]: expressions/closure-expr.html
[auto traits]: special-types-and-traits.html#auto-traits
[object safe]: items/traits.html#object-safety
[issue 47010]: https://github.com/rust-lang/rust/issues/47010
[issue 33140]: https://github.com/rust-lang/rust/issues/33140
