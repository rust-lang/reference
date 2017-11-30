# Unions

> **<sup>Syntax</sup>**  
> _Union_ :  
> &nbsp;&nbsp; `union` [IDENTIFIER]&nbsp;[_Generics_]<sup>?</sup> [_WhereClause_]<sup>?</sup>
>   `{`[_StructFields_] `}`

A union declaration uses the same syntax as a struct declaration, except with
`union` in place of `struct`.

```rust
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}
```

The key property of unions is that all fields of a union share common storage.
As a result writes to one field of a union can overwrite its other fields, and
size of a union is determined by the size of its largest field.

A value of a union type can be created using the same syntax that is used for
struct types, except that it must specify exactly one field:

```rust
# union MyUnion { f1: u32, f2: f32 }
#
let u = MyUnion { f1: 1 };
```

The expression above creates a value of type `MyUnion` with active field `f1`.
Active field of a union can be accessed using the same syntax as struct fields:

```rust,ignore
let f = u.f1;
```

Inactive fields can be accessed as well (using the same syntax) if they are
sufficiently layout compatible with the current value kept by the union.
Reading incompatible fields results in undefined behavior. However, the active
field is not generally known statically, so all reads of union fields have to
be placed in `unsafe` blocks.

```rust
# union MyUnion { f1: u32, f2: f32 }
# let u = MyUnion { f1: 1 };
#
unsafe {
    let f = u.f1;
}
```

Writes to `Copy` union fields do not require reads for running destructors, so
these writes don't have to be placed in `unsafe` blocks

```rust
# union MyUnion { f1: u32, f2: f32 }
# let mut u = MyUnion { f1: 1 };
#
u.f1 = 2;
```

Commonly, code using unions will provide safe wrappers around unsafe union
field accesses.

Another way to access union fields is to use pattern matching. Pattern matching
on union fields uses the same syntax as struct patterns, except that the
pattern must specify exactly one field. Since pattern matching accesses
potentially inactive fields it has to be placed in `unsafe` blocks as well.

```rust
# union MyUnion { f1: u32, f2: f32 }
#
fn f(u: MyUnion) {
    unsafe {
        match u {
            MyUnion { f1: 10 } => { println!("ten"); }
            MyUnion { f2 } => { println!("{}", f2); }
        }
    }
}
```

Pattern matching may match a union as a field of a larger structure. In
particular, when using a Rust union to implement a C tagged union via FFI, this
allows matching on the tag and the corresponding field simultaneously:

```rust
#[repr(u32)]
enum Tag { I, F }

#[repr(C)]
union U {
    i: i32,
    f: f32,
}

#[repr(C)]
struct Value {
    tag: Tag,
    u: U,
}

fn is_zero(v: Value) -> bool {
    unsafe {
        match v {
            Value { tag: I, u: U { i: 0 } } => true,
            Value { tag: F, u: U { f: 0.0 } } => true,
            _ => false,
        }
    }
}
```

Since union fields share common storage, gaining write access to one field of a
union can give write access to all its remaining fields. Borrow checking rules
have to be adjusted to account for this fact. As a result, if one field of a
union is borrowed, all its remaining fields are borrowed as well for the same
lifetime.

```rust,ignore
// ERROR: cannot borrow `u` (via `u.f2`) as mutable more than once at a time
fn test() {
    let mut u = MyUnion { f1: 1 };
    unsafe {
        let b1 = &mut u.f1;
                      ---- first mutable borrow occurs here (via `u.f1`)
        let b2 = &mut u.f2;
                      ^^^^ second mutable borrow occurs here (via `u.f2`)
        *b1 = 5;
    }
    - first borrow ends here
    assert_eq!(unsafe { u.f1 }, 5);
}
```

As you could see, in many aspects (except for layouts, safety and ownership)
unions behave exactly like structs, largely as a consequence of inheriting
their syntactic shape from structs. This is also true for many unmentioned
aspects of Rust language (such as privacy, name resolution, type inference,
generics, trait implementations, inherent implementations, coherence, pattern
checking, etc etc etc).

More detailed specification for unions, including unstable bits, can be found
in [RFC 1897 "Unions v1.2"](https://github.com/rust-lang/rfcs/pull/1897).

[IDENTIFIER]: identifiers.html
[_Generics_]: items.html#type-parameters
[_WhereClause_]: items.html#type-parameters
[_StructFields_]: items/structs.html
