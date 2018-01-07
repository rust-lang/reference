# Lifetime elision

Rust has rules that allow lifetimes to be elided in various places where the
compiler can infer a sensible default choice.

## Lifetime elision in functions

In order to make common patterns more ergonomic, Rust allows lifetimes to be
*elided* in function signatures.

A *lifetime position* is anywhere you can write a lifetime in a type:

```rust,ignore
&'a T
&'a mut T
T<'a>
```

Lifetime positions can appear as either "input" or "output":

* For `fn` definitions, input refers to the types of the formal arguments
  in the `fn` definition, while output refers to
  result types. So `fn foo(s: &str) -> (&str, &str)` has elided one lifetime in
  input position and two lifetimes in output position.
  Note that the input positions of a `fn` method definition do not
  include the lifetimes that occur in the method's `impl` header
  (nor lifetimes that occur in the trait header, for a default method).

* In the future, it should be possible to elide `impl` headers in the same manner.

Elision rules are as follows:

* Each elided lifetime in input position becomes a distinct lifetime
  parameter.

* If there is exactly one input lifetime position (elided or not), that lifetime
  is assigned to *all* elided output lifetimes.

* If there are multiple input lifetime positions, but one of them is `&self` or
  `&mut self`, the lifetime of `self` is assigned to *all* elided output lifetimes.

* Otherwise, it is an error to elide an output lifetime.

Examples:

```rust,ignore
fn print(s: &str);                                      // elided
fn print<'a>(s: &'a str);                               // expanded

fn debug(lvl: usize, s: &str);                          // elided
fn debug<'a>(lvl: usize, s: &'a str);                   // expanded

fn substr(s: &str, until: usize) -> &str;               // elided
fn substr<'a>(s: &'a str, until: usize) -> &'a str;     // expanded

fn get_str() -> &str;                                   // ILLEGAL

fn frob(s: &str, t: &str) -> &str;                      // ILLEGAL

fn get_mut(&mut self) -> &mut T;                        // elided
fn get_mut<'a>(&'a mut self) -> &'a mut T;              // expanded

fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command                  // elided
fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command // expanded

fn new(buf: &mut [u8]) -> BufWriter;                    // elided
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>          // expanded

```

## Defaults trait object lifetimes

The assumed lifetime of references held by a trait object is called its
*default object lifetime bound*. These were defined in [RFC 599] and amended in
[RFC 1156].

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

## `'static` lifetime elision

Both constant and static declarations of reference types have *implicit*
`'static` lifetimes unless an explicit lifetime is specified. As such, the
constant declarations involving `'static` above may be written without the
lifetimes. Returning to our previous example:

```rust
const BIT1: u32 = 1 << 0;
const BIT2: u32 = 1 << 1;

const BITS: [u32; 2] = [BIT1, BIT2];
const STRING: &str = "bitstring";

struct BitsNStrings<'a> {
    mybits: [u32; 2],
    mystring: &'a str,
}

const BITS_N_STRINGS: BitsNStrings = BitsNStrings {
    mybits: BITS,
    mystring: STRING,
};
```

Note that if the `static` or `const` items include function or closure
references, which themselves include references, the compiler will first try
the standard elision rules ([see discussion in the nomicon][elision-nomicon]).
If it is unable to resolve the lifetimes by its usual rules, it will default to
using the `'static` lifetime. By way of example:

```rust,ignore
// Resolved as `fn<'a>(&'a str) -> &'a str`.
const RESOLVED_SINGLE: fn(&str) -> &str = ..

// Resolved as `Fn<'a, 'b, 'c>(&'a Foo, &'b Bar, &'c Baz) -> usize`.
const RESOLVED_MULTIPLE: Fn(&Foo, &Bar, &Baz) -> usize = ..

// There is insufficient information to bound the return reference lifetime
// relative to the argument lifetimes, so the signature is resolved as
// `Fn(&'static Foo, &'static Bar) -> &'static Baz`.
const RESOLVED_STATIC: Fn(&Foo, &Bar) -> &Baz = ..
```

[RFC 599]: https://github.com/rust-lang/rfcs/blob/master/text/0599-default-object-bound.md
[RFC 1156]: https://github.com/rust-lang/rfcs/blob/master/text/1156-adjust-default-object-bounds.md
[elision-nomicon]: ../nomicon/lifetime-elision.html
