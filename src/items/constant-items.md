r[items.const]
# Constant items

r[items.const.syntax]
```grammar,items
ConstantItem ->
    `const` ( IDENTIFIER | `_` ) `:` Type ( `=` Expression )? `;`
```

r[items.const.intro]
A *constant item* is an optionally named _[constant value]_ which is not associated
with a specific memory location in the program.

r[items.const.behavior]
Constants are essentially inlined wherever they are used, meaning that they are copied directly into the relevant
context when used. This includes usage of constants from external crates, and
non-[`Copy`] types. References to the same constant are not necessarily
guaranteed to refer to the same memory address.

r[items.const.namespace]
The constant declaration defines the constant value in the [value namespace] of the module or block where it is located.

r[items.const.static]
Constants must be explicitly typed. The type must have a `'static` lifetime: any
references in the initializer must have `'static` lifetimes. References
in the type of a constant default to `'static` lifetime; see [static lifetime
elision].

r[items.const.static-temporary]
A reference to a constant will have `'static` lifetime if the constant value is eligible for
[promotion]; otherwise, a temporary will be created.

```rust
const BIT1: u32 = 1 << 0;
const BIT2: u32 = 1 << 1;

const BITS: [u32; 2] = [BIT1, BIT2];
const STRING: &'static str = "bitstring";

struct BitsNStrings<'a> {
    mybits: [u32; 2],
    mystring: &'a str,
}

const BITS_N_STRINGS: BitsNStrings<'static> = BitsNStrings {
    mybits: BITS,
    mystring: STRING,
};
```

r[items.const.no-mut-refs]
The final value of a `const` item, after the initializer is evaluated to a value that has the declared type of the constant, cannot contain any mutable references except as described below.

```rust
# #![allow(static_mut_refs)]
static mut S: u8 = 0;
const _: &u8 = unsafe { &mut S }; // OK.
//                      ^^^^^^
// Allowed since this is coerced to `&S`.
```

```rust
# use core::sync::atomic::AtomicU8;
static S: AtomicU8 = AtomicU8::new(0);
const _: &AtomicU8 = &S; // OK.
//                   ^^
// Allowed even though the shared reference is to an interior
// mutable value.
```

```rust,compile_fail,E0080
# #![allow(static_mut_refs)]
static mut S: u8 = 0;
const _: &mut u8 = unsafe { &mut S }; // ERROR.
//                          ^^^^^^
// Not allowed as the mutable reference appears in the final value.
```

> [!NOTE]
> Constant initializers can be thought of, in most cases, as being inlined wherever the constant appears. If a constant whose value contains a mutable reference to a mutable static were to appear twice, and this were to be allowed, that would create two mutable references, each having `'static` lifetime, to the same place. This could produce undefined behavior.
>
> Constants that contain mutable references to temporaries whose scopes have been extended to the end of the program have that same problem and an additional one.
>
> ```rust,compile_fail,E0764
> const _: &mut u8 = &mut 0; // ERROR.
> //                 ^^^^^^
> // Not allowed as the mutable reference appears in the final value and
> // because the constant expression contains a mutable borrow of an
> // expression whose temporary scope would be extended to the end of
> // the program.
> ```
>
> Here, the value `0` is a temporary whose scope is extended to the end of the program (see [destructors.scope.lifetime-extension.static]). Such temporaries cannot be mutably borrowed in constant expressions (see [const-eval.const-expr.borrows]).
>
> To allow this, we'd have to decide whether each use of the constant creates a new `u8` value or whether each use shares the same lifetime-extended temporary. The latter choice, though closer to how `rustc` thinks about this today, would break the conceptual model that, in most cases, the constant initializer can be thought of as being inlined wherever the constant is used. Since we haven't decided, and due to the other problem mentioned, this is not allowed.

```rust,compile_fail,E0080
# #![allow(static_mut_refs)]
static mut S: u8 = 0;
const _: &dyn Send = &unsafe { &mut S }; // ERROR.
//                             ^^^^^^
// Not allowed as the mutable reference appears in the final value,
// even though type erasure occurs.
```

Mutable references where the referent is a value of a [zero-sized type] are allowed.

```rust
# #![allow(static_mut_refs)]
static mut S: () = ();
const _: &mut () = unsafe { &mut S }; // OK.
//            ^^ This is a zero-sized type.
```

```rust
# #![allow(static_mut_refs)]
static mut S: [u8; 0] = [0; 0];
const _: &mut [u8; 0] = unsafe { &mut S }; // OK.
//            ^^^^^^^ This is a zero-sized type.
```

> [!NOTE]
> This is allowed as, for a value of a zero-sized type, no bytes can actually be mutated. We must accept this as `&mut []` is [promoted].

Values of [union type] are not considered to contain any references; for this purpose, a value of union type is treated as a sequence of untyped bytes.

```rust
# #![allow(static_mut_refs)]
union U { f: &'static mut u8 }
static mut S: u8 = 0;
const _: U = unsafe { U { f: &mut S }}; // OK.
//                    ^^^^^^^^^^^^^^^
// This is treated as a sequence of untyped bytes.
```

Mutable references contained within a [mutable static] may be referenced in the final value of a constant.

```rust
# #![allow(static_mut_refs)]
static mut S: &mut u8 = unsafe { static mut I: u8 = 0; &mut I };
const _: &&mut u8 = unsafe { &S }; // OK.
//        ^^^^^^^
// This mutable reference comes from a `static mut`.
```

> [!NOTE]
> This is allowed as it's separately not allowed to read from a mutable static during constant evaluation. See [const-eval.const-expr.path-static].

Mutable references contained within an [external static] may be referenced in the final value of a constant.

```rust
# #![allow(static_mut_refs)]
unsafe extern "C" { static S: &'static mut u8; }
const _: &&mut u8 = unsafe { &S }; // OK.
//        ^^^^^^^
// This mutable references comes from an extern static.
```

> [!NOTE]
> This is allowed as it's separately not allowed to read from an external static during constant evaluation. See [const-eval.const-expr.path-static].

> [!NOTE]
> As described above, we accept, in the final value of constant items, shared references to static items whose values have interior mutability.
>
> ```rust
> # use core::sync::atomic::AtomicU8;
> static S: AtomicU8 = AtomicU8::new(0);
> const _: &AtomicU8 = &S; // OK.
> ```
>
> However, we disallow similar code when the interior mutable value is created in the initializer.
>
> ```rust,compile_fail,E0492
> # use core::sync::atomic::AtomicU8;
> const _: &AtomicU8 = &AtomicU8::new(0); // ERROR.
> ```
>
> Here, the `AtomicU8` is a temporary whose scope is extended to the end of the program (see [destructors.scope.lifetime-extension.static]). Such temporaries with interior mutability cannot be borrowed in constant expressions (see [const-eval.const-expr.borrows]).
>
> To allow this, we'd have to decide whether each use of the constant creates a new `AtomicU8` or whether each use shares the same lifetime-extended temporary. The latter choice, though closer to how `rustc` thinks about this today, would break the conceptual model that, in most cases, the constant initializer can be thought of as being inlined wherever the constant is used. Since we haven't decided, this is not allowed.

r[items.const.expr-omission]
The constant expression may only be omitted in a [trait definition].

r[items.const.destructor]
## Constants with destructors

Constants can contain destructors. Destructors are run when the value goes out
of scope.

```rust
struct TypeWithDestructor(i32);

impl Drop for TypeWithDestructor {
    fn drop(&mut self) {
        println!("Dropped. Held {}.", self.0);
    }
}

const ZERO_WITH_DESTRUCTOR: TypeWithDestructor = TypeWithDestructor(0);

fn create_and_drop_zero_with_destructor() {
    let x = ZERO_WITH_DESTRUCTOR;
    // x gets dropped at end of function, calling drop.
    // prints "Dropped. Held 0.".
}
```

r[items.const.unnamed]
## Unnamed constant

r[items.const.unnamed.intro]
Unlike an [associated constant], a [free] constant may be unnamed by using
an underscore instead of the name. For example:

```rust
const _: () =  { struct _SameNameTwice; };

// OK although it is the same name as above:
const _: () =  { struct _SameNameTwice; };
```

r[items.const.unnamed.repetition]
As with [underscore imports], macros may safely emit the same unnamed constant in
the same scope more than once. For example, the following should not produce an error:

```rust
macro_rules! m {
    ($item: item) => { $item $item }
}

m!(const _: () = (););
// This expands to:
// const _: () = ();
// const _: () = ();
```

r[items.const.eval]
## Evaluation

[Free][free] constants are always [evaluated][const_eval] at compile-time to surface
panics. This happens even within an unused function:

```rust,compile_fail
// Compile-time panic
const PANIC: () = std::unimplemented!();

fn unused_generic_function<T>() {
    // A failing compile-time assertion
    const _: () = assert!(usize::BITS == 0);
}
```

[const_eval]: ../const_eval.md
[associated constant]: ../items/associated-items.md#associated-constants
[constant value]: ../const_eval.md#constant-expressions
[external static]: items.extern.static
[free]: ../glossary.md#free-item
[static lifetime elision]: ../lifetime-elision.md#const-and-static-elision
[trait definition]: traits.md
[underscore imports]: use-declarations.md#underscore-imports
[`Copy`]: ../special-types-and-traits.md#copy
[value namespace]: ../names/namespaces.md
[mutable static]: items.static.mut
[promoted]: destructors.scope.const-promotion
[promotion]: destructors.scope.const-promotion
[union type]: type.union
[zero-sized type]: layout.properties.size
