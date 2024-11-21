# Memory model

r[memory]

The Memory Model of Rust is incomplete and not fully decided. The following is some of the detail worked out so far.

## Bytes

r[memory.byte]

r[memory.byte.intro]
The most basic unit of memory in Rust is a byte. All values in Rust are computed from 0 or more bytes read from an allocation.

> [!NOTE]
> While bytes in Rust are typically lowered to hardware bytes, they may contain additional values,
> such as being uninitialized, or storing part of a pointer.

r[memory.byte.init]
Each byte may be initialized, and contain a value of type `u8`, as well as an optional pointer fragment. When present, the pointer fragment carries [provenance][type.pointer.provenance] information.

r[memory.byte.uninit]
Each byte may be uninitialized.

> [!NOTE]
> Uninitialized bytes do not have a value and do not have a pointer fragment.

## Value Encoding

r[memory.encoding]

r[memory.encoding.intro]
Each type in Rust has 0 or more values, which can have operations performed on them

> [!NOTE]
> `0u8`, `1337i16`, and `Foo{bar: "baz"}` are all values

r[memory.encoding.op]
Each value of a type can be encoded into a sequence of bytes, and decoded from a sequence of bytes, which has a length equal to the size of the type.
The operation to encode or decode a value is determined by the representation of the type.

> [!NOTE]
> Representation is related to, but is not the same property as, the layout of the type.

r[memory.encoding.decode]
If a value of type `T` is decoded from a sequence of bytes that does not correspond to a defined value, the behavior is undefined. If a value of type `T` is decoded from a sequence of bytes that contain pointer fragments, which are not used to represent the value, the pointer fragments are ignored.
