# Glossary

### Abstract syntax tree

An ‘abstract syntax tree’, or ‘AST’, is an intermediate representation of
the structure of the program when the compiler is compiling it.

### Alignment

The alignment of a value specifies what addresses values are preferred to
start at. Always a power of two. References to a value must be aligned.
[More][alignment].

### Arity

Arity refers to the number of arguments a function or operator takes.
For some examples, `f(2, 3)` and `g(4, 6)` have arity 2, while `h(8, 2, 6)`
has arity 3. The `!` operator has arity 1.

### Array

An array, sometimes also called a fixed-size array or an inline array, is a value
describing a collection of elements, each selected by an index that can be computed
at run time by the program. It occupies a contiguous region of memory.

### Associated item

An associated item is an item that is associated with another item. Associated
items are defined in [implementations] and declared in [traits]. Only
functions, constants, and type aliases can be associated. Contrast to a [free
item].

### Bound

Bounds are constraints on a type or trait. For example, if a bound
is placed on the argument a function takes, types passed to that function
must abide by that constraint.

### Combinator

Combinators are higher-order functions that apply only functions and
earlier defined combinators to provide a result from its arguments.
They can be used to manage control flow in a modular fashion.

### Dispatch

Dispatch is the mechanism to determine which specific version of code is actually
run when it involves polymorphism. Two major forms of dispatch are static dispatch and
dynamic dispatch. While Rust favors static dispatch, it also supports dynamic dispatch
through a mechanism called ‘trait objects’.

### Dynamically sized type

A dynamically sized type (DST) is a type without a statically known size or alignment.

### Expression

An expression is a combination of values, constants, variables, operators
and functions that evaluate to a single value, with or without side-effects.

For example, `2 + (3 * 4)` is an expression that returns the value 14.

### Free item

An [item] that is not a member of an [implementation], such as a *free
function* or a *free const*. Contrast to an [associated item].

### Inherent implementation

An [implementation] that applies to a nominal type, not to a trait-type pair.
[More][inherent implementation].

### Inherent method

A [method] defined in an [inherent implementation], not in a trait
implementation.

### Initialized

A variable is initialized if it has been assigned a value and hasn't since been
moved from. All other memory locations are assumed to be uninitialized. Only
unsafe Rust can create such a memory without initializing it.

### Nominal types

Types that can be referred to by a path directly. Specifically [enums],
[structs], [unions], and [trait objects].

### Object safe traits

[Traits] that can be used as [trait objects]. Only traits that follow specific
[rules][object safety] are object safe.

### Prelude

Prelude, or The Rust Prelude, is a small collection of items - mostly traits - that are
imported into every module of every crate. The traits in the prelude are pervasive.

### Scrutinee

A scrutinee is the expression that is matched on in `match` expressions and
similar pattern matching constructs. For example, in `match x { A => 1, B => 2 }`,
the expression `x` is the scrutinee.

### Size

The size of a value has two definitions.

The first is that it is how much memory must be allocated to store that value.

The second is that it is the offset in bytes between successive elements in an
array with that item type.

It is a multiple of the alignment, including zero. The size can change
depending on compiler version (as new optimizations are made) and target
platform (similar to how `usize` varies per-platform).

[More][alignment].

### Slice

A slice is dynamically-sized view into a contiguous sequence, written as `[T]`.

It is often seen in its borrowed forms, either mutable or shared. The shared
slice type is `&[T]`, while the mutable slice type is `&mut [T]`, where `T` represents
the element type.

### Statement

A statement is the smallest standalone element of a programming language
that commands a computer to perform an action.

### String literal

A string literal is a string stored directly in the final binary, and so will be
valid for the `'static` duration.

Its type is `'static` duration borrowed string slice, `&'static str`.

### String slice

A string slice is the most primitive string type in Rust, written as `str`. It is
often seen in its borrowed forms, either mutable or shared. The shared
string slice type is `&str`, while the mutable string slice type is `&mut str`.

Strings slices are always valid UTF-8.

### Trait

A trait is a language item that is used for describing the functionalities a type must provide.
It allows a type to make certain promises about its behavior.

Generic functions and generic structs can use traits to constrain, or bound, the types they accept.

### Undefined behavior

Compile-time or run-time behavior that is not specified. This may result in,
but is not limited to: process termination or corruption; improper, incorrect,
or unintended computation; or platform-specific results.
[More][undefined-behavior].

[alignment]: type-layout.md#size-and-alignment
[associated item]: #associated-item
[enums]: items/enumerations.md
[free item]: #free-item
[implementation]: items/implementations.md
[implementations]: items/implementations.md
[inherent implementation]: items/implementations.md#inherent-implementations
[item]: items.md
[method]: items/associated-items.md#methods
[object safety]: items/traits.md#object-safety
[structs]: items/structs.md
[trait objects]: types/trait-object.md
[traits]: items/traits.md
[undefined-behavior]: behavior-considered-undefined.md
[unions]: items/unions.md
