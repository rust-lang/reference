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

r[memory.byte.contents]
Each byte may have one of the following values:

r[memory.byte.init]
* An initialized byte containing a `u8` value and optional [provenance][type.pointer.provenance],

r[memory.byte.uninit]
* An uninitialized byte.

> [!NOTE]
> Uninitialized bytes do not have a value and do not have a pointer fragment.

> [!NOTE]
> The above list is not yet guaranteed to be exhaustive.

## Value Encoding

r[memory.encoding]

r[memory.encoding.intro]
Each type in Rust has 0 or more values, which can have operations performed on them. Values are represented in memory by encoding them

> [!NOTE]
> `0u8`, `1337i16`, and `Foo{bar: "baz"}` are all values

r[memory.encoding.op]
Each type defines a pair of properties which, together, define the representation of values of the type. The *encode* operation takes a value of the type and converts it into a sequence of bytes equal in length to the size of the type, and the *decode* operation takes such a sequence of bytes and optionally converts it into a value. Encoding occurs when a value is written to memory, and decoding occurs when a value is read from memory.

> [!NOTE]
> Only certain byte sequences may decode into a value of a given type. For example, a byte sequence consisting of all zeroes does not decode to a value of a reference type.

r[memory.encoding.representation]
A sequence of bytes is said to represent a value of a type, if the decode operation for that type produces that value from that sequence of bytes. The representation of a type is the partial relation between byte sequences and values those sequences represent.

> [!NOTE]
> Representation is related to, but is not the same property as, the layout of the type.

r[memory.encoding.symmetric]
The result of encoding a given value of a type is a sequence of bytes that represents that value.

> [!NOTE]
> This means that a value can be copied into memory and copied out and the result is the same value.
> The reverse is not necessarily true, a sequence of bytes read as a value then written to another location (called a typed copy) will not necessarily yield the same sequence of bytes. For example, a typed copy of a struct type will leave the padding bytes of that struct uninitialized.

r[memory.encoding.decode]
If a value of type `T` is decoded from a sequence of bytes that does not represent any value, the behavior is undefined.

> [!NOTE]
> For example, it is undefined behavior to read a `0x02` byte as `bool`.
