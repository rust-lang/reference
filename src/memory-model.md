r[memory]
# Memory model

> [!WARNING]
> The memory model of Rust is incomplete and not fully decided.

r[memory.bytes]
## Bytes

r[memory.bytes.intro]
The most basic unit of memory in Rust is a byte.

> [!NOTE]
> While bytes are typically lowered to hardware bytes, Rust uses an "abstract" notion of bytes that can make distinctions which are absent in hardware, such as being uninitialized, or storing part of a pointer. Those distinctions can affect whether your program has undefined behavior, so they still have tangible impact on how compiled Rust programs behave.

r[memory.bytes.contents]
Each byte may have one of the following values:

r[memory.bytes.init]
* An initialized byte containing a `u8` value and optional [provenance][std::ptr#provenance],

r[memory.bytes.uninit]
* An uninitialized byte.

> [!NOTE]
> The above list is not yet guaranteed to be exhaustive.
