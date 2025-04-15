r[items.union]
# Unions

r[items.union.syntax]
```grammar,items
Union ->
    `union` IDENTIFIER GenericParams? WhereClause? `{` StructFields? `}`
```

r[items.union.intro]
A union declaration uses the same syntax as a struct declaration, except with
`union` in place of `struct`.

r[items.union.namespace]
A union declaration defines the given name in the [type namespace] of the module or block where it is located.

```rust
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}
```

r[items.union.common-storage]
The key property of unions is that all fields of a union share common storage.
As a result, writes to one field of a union can overwrite its other fields, and
size of a union is determined by the size of its largest field.

r[items.union.field-restrictions]
Union field types are restricted to the following subset of types:

r[items.union.field-copy]
- `Copy` types

r[items.union.field-references]
- References (`&T` and `&mut T` for arbitrary `T`)

r[items.union.field-manually-drop]
- `ManuallyDrop<T>` (for arbitrary `T`)

r[items.union.field-tuple]
- Tuples and arrays containing only allowed union field types

r[items.union.drop]
This restriction ensures, in particular, that union fields never need to be
dropped. Like for structs and enums, it is possible to `impl Drop` for a union
to manually define what happens when it gets dropped.

r[items.union.fieldless]
Unions without any fields are not accepted by the compiler, but can be accepted by macros.

r[items.union.init]
## Initialization of a union

r[items.union.init.intro]
A value of a union type can be created using the same syntax that is used for
struct types, except that it must specify exactly one field:

```rust
# union MyUnion { f1: u32, f2: f32 }
#
let u = MyUnion { f1: 1 };
```

r[items.union.init.result]
The expression above creates a value of type `MyUnion` and initializes the
storage using field `f1`. The union can be accessed using the same syntax as
struct fields:

```rust
# union MyUnion { f1: u32, f2: f32 }
#
# let u = MyUnion { f1: 1 };
let f = unsafe { u.f1 };
```

r[items.union.fields]
## Reading and writing union fields

r[items.union.fields.intro]
Unions have no notion of an "active field". Instead, every union access just
interprets the storage as the type of the field used for the access.

r[items.union.fields.read]
Reading a union field reads the bits of the union at the field's type.

r[items.union.fields.offset]
Fields might have a non-zero offset (except when [the C representation] is used); in that case the
bits starting at the offset of the fields are read

r[items.union.fields.validity]
It is the programmer's responsibility to make sure that the data is valid at the field's type. Failing
to do so results in [undefined behavior]. For example, reading the value `3`
from a field of the [boolean type] is undefined behavior. Effectively,
writing to and then reading from a union with [the C representation] is
analogous to a [`transmute`] from the type used for writing to the type used for
reading.

r[items.union.fields.read-safety]
Consequently, all reads of union fields have to be placed in `unsafe` blocks:

```rust
# union MyUnion { f1: u32, f2: f32 }
# let u = MyUnion { f1: 1 };
#
unsafe {
    let f = u.f1;
}
```

Commonly, code using unions will provide safe wrappers around unsafe union
field accesses.

r[items.union.fields.write-safety]
In contrast, writes to union fields are safe, since they just overwrite
arbitrary data, but cannot cause undefined behavior. (Note that union field
types can never have drop glue, so a union field write will never implicitly
drop anything.)

r[items.union.pattern]
## Pattern matching on unions

r[items.union.pattern.intro]
Another way to access union fields is to use pattern matching.

r[items.union.pattern.one-field]
Pattern matching on union fields uses the same syntax as struct patterns, except that the pattern
must specify exactly one field.

r[items.union.pattern.safety]
Since pattern matching is like reading the union with a particular field, it has to be placed in `unsafe` blocks as well.

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

r[items.union.pattern.subpattern]
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
            Value { tag: Tag::I, u: U { i: 0 } } => true,
            Value { tag: Tag::F, u: U { f: num } } if num == 0.0 => true,
            _ => false,
        }
    }
}
```

r[items.union.ref]
## References to union fields

r[items.union.ref.intro]
Since union fields share common storage, gaining write access to one field of a
union can give write access to all its remaining fields.

r[items.union.ref.borrow]
Borrow checking rules have to be adjusted to account for this fact. As a result, if one field of a
union is borrowed, all its remaining fields are borrowed as well for the same
lifetime.

```rust,compile_fail
# union MyUnion { f1: u32, f2: f32 }
// ERROR: cannot borrow `u` (via `u.f2`) as mutable more than once at a time
fn test() {
    let mut u = MyUnion { f1: 1 };
    unsafe {
        let b1 = &mut u.f1;
//                    ---- first mutable borrow occurs here (via `u.f1`)
        let b2 = &mut u.f2;
//                    ^^^^ second mutable borrow occurs here (via `u.f2`)
        *b1 = 5;
    }
//  - first borrow ends here
    assert_eq!(unsafe { u.f1 }, 5);
}
```

r[items.union.ref.usage]
As you could see, in many aspects (except for layouts, safety, and ownership)
unions behave exactly like structs, largely as a consequence of inheriting
their syntactic shape from structs. This is also true for many unmentioned
aspects of Rust language (such as privacy, name resolution, type inference,
generics, trait implementations, inherent implementations, coherence, pattern
checking, etc etc etc).

[`transmute`]: std::mem::transmute
[boolean type]: ../types/boolean.md
[the C representation]: ../type-layout.md#reprc-unions
[type namespace]: ../names/namespaces.md
[undefined behavior]: ../behavior-considered-undefined.md
