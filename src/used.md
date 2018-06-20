## #[used]

The `#[used]` attribute can only be applied to `static` variables. This attribute forces the
compiler to keep the variable in the output object file (.o, .rlib, etc.) even if the variable is
not *used*, or referenced, by any other item in the crate. Without this attribute the compiler is
free to remove variable if it's *unused*, or dead, when optimizations are enabled.

Below is an example that shows under what conditions the compiler keeps a `static` variable in the
output object file.

``` rust
// foo.rs

#![feature(used)]

// kept because of #[used]
#[used]
static FOO: u32 = 0;

// removed because it's unused
#[allow(dead_code)]
static BAR: u32 = 0;

// kept because it's referenced by a *public*, reachable function
pub static BAZ: u32 = 0;

pub static QUUX: u32 = 0;

pub fn quux() -> &'static u32 {
    &QUUX
}

// removed because it's referenced by a private, unused (dead) function
static CORGE: u32 = 0;

#[allow(dead_code)]
fn corge() -> &'static u32 {
    &CORGE
}
```

``` console
$ # with optimizations
$ rustc -O --emit=obj --crate-type=rlib foo.rs

$ nm -C foo.o
0000000000000000 R foo::BAZ
0000000000000000 r foo::FOO
0000000000000000 R foo::QUUX
0000000000000000 T foo::quux

$ # without optimizations
$ rustc --emit=obj --crate-type=rlib foo.rs

$ nm -C foo.o
0000000000000000 r foo::BAR
0000000000000000 R foo::BAZ
0000000000000000 r foo::FOO
0000000000000000 R foo::QUUX
0000000000000000 T foo::quux
0000000000000000 r foo::CORGE
```
