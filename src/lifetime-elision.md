# Lifetime elision

Rust has rules that allow lifetimes to be elided in various places where the
compiler can infer a sensible default choice.

## Lifetime elision in functions

In order to make common patterns more ergonomic, lifetime arguments can be
*elided* in [function item], [function pointer] and [closure trait] signatures.
The following rules are used to infer lifetime parameters for elided lifetimes.
It is an error to elide lifetime parameters that cannot be inferred. The
placeholder lifetime, `'_`, can also be used to have a lifetime inferred in the
same way. For lifetimes in paths, using `'_` is preferred. Trait object
lifetimes follow different rules discussed
[below](#default-trait-object-lifetimes).

* Each elided lifetime in the parameters becomes a distinct lifetime parameter.
* If there is exactly one lifetime used in the parameters (elided or not), that
  lifetime is assigned to *all* elided output lifetimes.

In method signatures there is another rule

* If the receiver has type `&Self`  or `&mut Self`, then the lifetime of that
  reference to `Self` is assigned to all elided output lifetime parameters.

Examples:

```rust,ignore
fn print(s: &str);                                      // elided
fn print(s: &'_ str);                                   // also elided
fn print<'a>(s: &'a str);                               // expanded

fn debug(lvl: usize, s: &str);                          // elided
fn debug<'a>(lvl: usize, s: &'a str);                   // expanded

fn substr(s: &str, until: usize) -> &str;               // elided
fn substr<'a>(s: &'a str, until: usize) -> &'a str;     // expanded

fn get_str() -> &str;                                   // ILLEGAL

fn frob(s: &str, t: &str) -> &str;                      // ILLEGAL

fn get_mut(&mut self) -> &mut T;                        // elided
fn get_mut<'a>(&'a mut self) -> &'a mut T;              // expanded

fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command;                  // elided
fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // expanded

fn new(buf: &mut [u8]) -> BufWriter<'_>;                // elided - preferred
fn new(buf: &mut [u8]) -> BufWriter;                    // elided
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>;         // expanded

type FunPtr = fn(&str) -> &str;                         // elided
type FunPtr = for<'a> fn(&'a str) -> &'a str;           // expanded

type FunTrait = dyn Fn(&str) -> &str;                   // elided
type FunTrait = dyn for<'a> Fn(&'a str) -> &'a str;     // expanded
```

## Default trait object lifetimes

The assumed lifetime of references held by a [trait object] is called its
_default object lifetime bound_. These were defined in [RFC 599] and amended in
[RFC 1156].

These default object lifetime bounds are used instead of the lifetime parameter
elision rules defined above when the lifetime bound is omitted entirely. If
`'_` is used as the lifetime bound then the bound follows the usual elision
rules.

If the trait object is used as a type argument of a generic type then the
containing type is first used to try to infer a bound.

* If there is a unique bound from the containing type then that is the default
* If there is more than one bound from the containing type then an explicit
  bound must be specified

If neither of those rules apply, then the bounds on the trait are used:

* If the trait is defined with a single lifetime _bound_ then that bound is
  used.
* If `'static` is used for any lifetime bound then `'static` is used.
* If the trait has no lifetime bounds, then the lifetime is inferred in
  expressions and is `'static` outside of expressions.

```rust,ignore
// For the following trait...
trait Foo { }

// These two are the same as Box<T> has no lifetime bound on T
Box<dyn Foo>
Box<dyn Foo + 'static>

// ...and so are these:
impl dyn Foo {}
impl dyn Foo + 'static {}

// ...so are these, because &'a T requires T: 'a
&'a dyn Foo
&'a (dyn Foo + 'a)

// std::cell::Ref<'a, T> also requires T: 'a, so these are the same
std::cell::Ref<'a, dyn Foo>
std::cell::Ref<'a, dyn Foo + 'a>

// This is an error:
struct TwoBounds<'a, 'b, T: ?Sized + 'a + 'b>
TwoBounds<'a, 'b, dyn Foo> // Error: the lifetime bound for this object type
                           // cannot be deduced from context
```

Note that the innermost object sets the bound, so `&'a Box<dyn Foo>` is still
`&'a Box<dyn Foo + 'static>`.

```rust,ignore
// For the following trait...
trait Bar<'a>: 'a { }

// ...these two are the same:
Box<dyn Bar<'a>>
Box<dyn Bar<'a> + 'a>

// ...and so are these:
impl<'a> dyn Foo<'a> {}
impl<'a> dyn Foo<'a> + 'a {}

// This is still an error:
struct TwoBounds<'a, 'b, T: ?Sized + 'a + 'b>
TwoBounds<'a, 'b, dyn Foo<'c>>
```

## `'static` lifetime elision

Both [constant] and [static] declarations of reference types have *implicit*
`'static` lifetimes unless an explicit lifetime is specified. As such, the
constant declarations involving `'static` above may be written without the
lifetimes.

```rust
// STRING: &'static str
const STRING: &str = "bitstring";

struct BitsNStrings<'a> {
    mybits: [u32; 2],
    mystring: &'a str,
}

// BITS_N_STRINGS: BitsNStrings<'static>
const BITS_N_STRINGS: BitsNStrings<'_> = BitsNStrings {
    mybits: [1, 2],
    mystring: STRING,
};
```

Note that if the `static` or `const` items include function or closure
references, which themselves include references, the compiler will first try
the standard elision rules. If it is unable to resolve the lifetimes by its
usual rules, then it will error. By way of example:

```rust,ignore
// Resolved as `fn<'a>(&'a str) -> &'a str`.
const RESOLVED_SINGLE: fn(&str) -> &str = ..

// Resolved as `Fn<'a, 'b, 'c>(&'a Foo, &'b Bar, &'c Baz) -> usize`.
const RESOLVED_MULTIPLE: &dyn Fn(&Foo, &Bar, &Baz) -> usize = ..

// There is insufficient information to bound the return reference lifetime
// relative to the argument lifetimes, so this is an error.
const RESOLVED_STATIC: &dyn Fn(&Foo, &Bar) -> &Baz = ..
```

[closure trait]: types/closure.html
[constant]: items/constant-items.html
[function item]: types/function-item.html
[function pointer]: types/function-pointer.html
[implementation]: items/implementations.html
[RFC 599]: https://github.com/rust-lang/rfcs/blob/master/text/0599-default-object-bound.md
[RFC 1156]: https://github.com/rust-lang/rfcs/blob/master/text/1156-adjust-default-object-bounds.md
[static]: items/static-items.html
[trait object]: types/trait-object.html
[type aliases]: items/type-aliases.html
