# The `unsafe` keyword

r[unsafe]

r[unsafe.intro]
The `unsafe` keyword can occur in several different contexts:
unsafe functions (`unsafe fn`), unsafe blocks (`unsafe {}`), unsafe traits (`unsafe trait`), unsafe trait implementations (`unsafe impl`), unsafe external blocks (`unsafe extern`), and unsafe attributes (`#[unsafe(attr)]`).
It plays several different roles, depending on where it is used and whether the `unsafe_op_in_unsafe_fn` lint is enabled:
- it is used to mark code that *defines* extra safety conditions (`unsafe fn`, `unsafe trait`)
- it is used to mark code that needs to *satisfy* extra safety conditions (`unsafe {}`, `unsafe impl`, `unsafe fn` without [`unsafe_op_in_unsafe_fn`], `unsafe extern`, `#[unsafe(attr)]`)

The following discusses each of these cases.
See the [keyword documentation][keyword] for some illustrative examples.

## Unsafe functions (`unsafe fn`)

r[unsafe.fn]

r[unsafe.fn.intro]
Unsafe functions are functions that are not safe in all contexts and/or for all possible inputs.
We say they have *extra safety conditions*, which are requirements that must be upheld by all callers and that the compiler does not check.
For example, [`get_unchecked`] has the extra safety condition that the index must be in-bounds.
The unsafe function should come with documentation explaining what those extra safety conditions are.

r[unsafe.fn.safety]
Such a function must be prefixed with the keyword `unsafe` and can only be called from inside an `unsafe` block, or inside `unsafe fn` without the [`unsafe_op_in_unsafe_fn`] lint.

## Unsafe blocks (`unsafe {}`)

r[unsafe.block]

r[unsafe.block.intro]
A block of code can be prefixed with the `unsafe` keyword, to permit calling `unsafe` functions or dereferencing raw pointers.

r[unsafe.block.fn-body]
By default, the body of an unsafe function is also considered to be an unsafe block;
this can be changed by enabling the [`unsafe_op_in_unsafe_fn`] lint.

By putting operations into an unsafe block, the programmer states that they have taken care of satisfying the extra safety conditions of all operations inside that block.

Unsafe blocks are the logical dual to unsafe functions:
where unsafe functions define a proof obligation that callers must uphold, unsafe blocks state that all relevant proof obligations of functions or operations called inside the block have been discharged.
There are many ways to discharge proof obligations;
for example, there could be run-time checks or data structure invariants that guarantee that certain properties are definitely true, or the unsafe block could be inside an `unsafe fn`, in which case the block can use the proof obligations of that function to discharge the proof obligations arising inside the block.

Unsafe blocks are used to wrap foreign libraries, make direct use of hardware or implement features not directly present in the language.
For example, Rust provides the language features necessary to implement memory-safe concurrency in the language but the implementation of threads and message passing in the standard library uses unsafe blocks.

Rust's type system is a conservative approximation of the dynamic safety requirements, so in some cases there is a performance cost to using safe code.
For example, a doubly-linked list is not a tree structure and can only be represented with reference-counted pointers in safe code.
By using `unsafe` blocks to represent the reverse links as raw pointers, it can be implemented without reference counting.
(See ["Learn Rust With Entirely Too Many Linked Lists"](https://rust-unofficial.github.io/too-many-lists/) for a more in-depth exploration of this particular example.)

## Unsafe traits (`unsafe trait`)

r[unsafe.trait]

r[unsafe.trait.intro]
An unsafe trait is a trait that comes with extra safety conditions that must be upheld by *implementations* of the trait.
The unsafe trait should come with documentation explaining what those extra safety conditions are.

r[unsafe.trait.safety]
Such a trait must be prefixed with the keyword `unsafe` and can only be implemented by `unsafe impl` blocks.

## Unsafe trait implementations (`unsafe impl`)

r[unsafe.impl]

When implementing an unsafe trait, the implementation needs to be prefixed with the `unsafe` keyword.
By writing `unsafe impl`, the programmer states that they have taken care of satisfying the extra safety conditions required by the trait.

Unsafe trait implementations are the logical dual to unsafe traits: where unsafe traits define a proof obligation that implementations must uphold, unsafe implementations state that all relevant proof obligations have been discharged.

[keyword]: ../std/keyword.unsafe.html
[`get_unchecked`]: slice::get_unchecked
[`unsafe_op_in_unsafe_fn`]: ../rustc/lints/listing/allowed-by-default.html#unsafe-op-in-unsafe-fn

## Unsafe external blocks (`unsafe extern`)

r[unsafe.extern]

The programmer who declares an [external block] must assure that the signatures of the items contained within are correct. Failing to do so may lead to undefined behavior.  That this obligation has been met is indicated by writing `unsafe extern`.

r[unsafe.extern.edition2024]
> **Edition differences**: Prior to edition 2024, `extern` blocks were allowed without being qualified as `unsafe`.

[external block]: items/external-blocks.md

## Unsafe attributes (`#[unsafe(attr)]`)

r[unsafe.attribute]

An [unsafe attribute] is one that has extra safety conditions that must be upheld when using the attribute. The compiler cannot check whether these conditions have been upheld.  To assert that they have been, these attributes must be wrapped in `unsafe(..)`, e.g. `#[unsafe(no_mangle)]`.

[unsafe attribute]: attributes.md
