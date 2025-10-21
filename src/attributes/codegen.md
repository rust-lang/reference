r[attributes.codegen]
# Code generation attributes

The following [attributes] are used for controlling code generation.

<!-- template:attributes -->
r[attributes.codegen.inline]
### The `inline` attribute

r[attributes.codegen.inline.intro]
The *`inline` [attribute]* suggests whether a copy of the attributed function's code should be placed in the caller rather than generating a call to the function.

> [!EXAMPLE]
> ```rust
> #[inline]
> pub fn example1() {}
>
> #[inline(always)]
> pub fn example2() {}
>
> #[inline(never)]
> pub fn example3() {}
> ```

> [!NOTE]
> `rustc` automatically inlines functions when doing so seems worthwhile. Use this attribute carefully as poor decisions about what to inline can slow down programs.

r[attributes.codegen.inline.syntax]
The syntax for the `inline` attribute is:

```grammar,attributes
@root InlineAttribute ->
      `inline` `(` `always` `)`
    | `inline` `(` `never` `)`
    | `inline`
```

r[attributes.codegen.inline.allowed-positions]
The `inline` attribute may only be applied to functions with [bodies] --- [closures], [async blocks], [free functions], [associated functions] in an [inherent impl] or [trait impl], and associated functions in a [trait definition] when those functions have a [default definition] .

> [!NOTE]
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

> [!NOTE]
> Though the attribute can be applied to [closures] and [async blocks], the usefulness of this is limited as we do not yet support attributes on expressions.
>
> ```rust
> // We allow attributes on statements.
> #[inline] || (); // OK
> #[inline] async {}; // OK
> ```
>
> ```rust,compile_fail,E0658
> // We don't yet allow attributes on expressions.
> let f = #[inline] || (); // ERROR
> ```

r[attributes.codegen.inline.duplicates]
Only the first use of `inline` on a function has effect.

> [!NOTE]
> `rustc` lints against any use following the first. This may become an error in the future.

r[attributes.codegen.inline.modes]
The `inline` attribute supports these modes:

- `#[inline]` *suggests* performing inline expansion.
- `#[inline(always)]` *suggests* that inline expansion should always be performed.
- `#[inline(never)]` *suggests* that inline expansion should never be performed.

> [!NOTE]
> In every form the attribute is a hint. The compiler may ignore it.

r[attributes.codegen.inline.trait]
When `inline` is applied to a function in a [trait], it applies only to the code of the [default definition].

r[attributes.codegen.inline.async]
When `inline` is applied to an [async function] or [async closure], it applies only to the code of the generated `poll` function.

> [!NOTE]
> For more details, see [Rust issue #129347](https://github.com/rust-lang/rust/issues/129347).

r[attributes.codegen.inline.externally-exported]
The `inline` attribute is ignored if the function is externally exported with [`no_mangle`] or [`export_name`].

<!-- template:attributes -->
r[attributes.codegen.cold]
### The `cold` attribute

r[attributes.codegen.cold.intro]
The *`cold` [attribute]* suggests that the attributed function is unlikely to be called which may help the compiler produce better code.

> [!EXAMPLE]
> ```rust
> #[cold]
> pub fn example() {}
> ```

r[attributes.codegen.cold.syntax]
The `cold` attribute uses the [MetaWord] syntax.

r[attributes.codegen.cold.allowed-positions]
The `cold` attribute may only be applied to functions with [bodies] --- [closures], [async blocks], [free functions], [associated functions] in an [inherent impl] or [trait impl], and associated functions in a [trait definition] when those functions have a [default definition] .

> [!NOTE]
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

> [!NOTE]
> Though the attribute can be applied to [closures] and [async blocks], the usefulness of this is limited as we do not yet support attributes on expressions.

<!-- TODO: rustc currently seems to allow cold on a trait function without a body, but it appears to be ignored. I think that may be a bug, and it should at least warn if not reject (like inline does). -->

r[attributes.codegen.cold.duplicates]
Only the first use of `cold` on a function has effect.

> [!NOTE]
> `rustc` lints against any use following the first. This may become an error in the future.

r[attributes.codegen.cold.trait]
When `cold` is applied to a function in a [trait], it applies only to the code of the [default definition].

r[attributes.codegen.naked]
## The `naked` attribute

r[attributes.codegen.naked.intro]
The *`naked` [attribute]* prevents the compiler from emitting a function prologue and epilogue for the attributed function.

r[attributes.codegen.naked.body]
The [function body] must consist of exactly one [`naked_asm!`] macro invocation.

r[attributes.codegen.naked.prologue-epilogue]
No function prologue or epilogue is generated for the attributed function. The assembly code in the `naked_asm!` block constitutes the full body of a naked function.

r[attributes.codegen.naked.unsafe-attribute]
The `naked` attribute is an [unsafe attribute]. Annotating a function with `#[unsafe(naked)]` comes with the safety obligation that the body must respect the function's calling convention, uphold its signature, and either return or diverge (i.e., not fall through past the end of the assembly code).

r[attributes.codegen.naked.call-stack]
The assembly code may assume that the call stack and register state are valid on entry as per the signature and calling convention of the function.

r[attributes.codegen.naked.no-duplication]
The assembly code may not be duplicated by the compiler except when monomorphizing polymorphic functions.

> [!NOTE]
> Guaranteeing when the assembly code may or may not be duplicated is important for naked functions that define symbols.

r[attributes.codegen.naked.unused-variables]
The [`unused_variables`] lint is suppressed within naked functions.

r[attributes.codegen.naked.inline]
The [`inline`](#the-inline-attribute) attribute cannot by applied to a naked function.

r[attributes.codegen.naked.track_caller]
The [`track_caller`](#the-track_caller-attribute) attribute cannot be applied to a naked function.

r[attributes.codegen.naked.testing]
The [testing attributes](testing.md) cannot be applied to a naked function.

<!-- template:attributes -->
r[attributes.codegen.no_builtins]
## The `no_builtins` attribute

r[attributes.codegen.no_builtins.intro]
The *`no_builtins` [attribute]* disables optimization of certain code patterns related to calls to library functions that are assumed to exist.

<!-- TODO: This needs expanding, see <https://github.com/rust-lang/reference/issues/542>. -->

> [!EXAMPLE]
> ```rust
> #![no_builtins]
> ```

r[attributes.codegen.no_builtins.syntax]
The `no_builtins` attribute uses the [MetaWord] syntax.

r[attributes.codegen.no_builtins.allowed-positions]
The `no_builtins` attribute can only be applied to the crate root.

r[attributes.codegen.no_builtins.duplicates]
Only the first use of the `no_builtins` attribute has effect.

> [!NOTE]
> `rustc` lints against any use following the first.

r[attributes.codegen.target_feature]
## The `target_feature` attribute

r[attributes.codegen.target_feature.intro]
The *`target_feature` [attribute]* may be applied to a function to
enable code generation of that function for specific platform architecture
features. It uses the [MetaListNameValueStr] syntax with a single key of
`enable` whose value is a string of comma-separated feature names to enable.

```rust
# #[cfg(target_feature = "avx2")]
#[target_feature(enable = "avx2")]
fn foo_avx2() {}
```

r[attributes.codegen.target_feature.arch]
Each [target architecture] has a set of features that may be enabled. It is an
error to specify a feature for a target architecture that the crate is not
being compiled for.

r[attributes.codegen.target_feature.closures]
Closures defined within a `target_feature`-annotated function inherit the
attribute from the enclosing function.

r[attributes.codegen.target_feature.target-ub]
It is [undefined behavior] to call a function that is compiled with a feature
that is not supported on the current platform the code is running on, *except*
if the platform explicitly documents this to be safe.

r[attributes.codegen.target_feature.safety-restrictions]
The following restrictions apply unless otherwise specified by the platform rules below:

- Safe `#[target_feature]` functions (and closures that inherit the attribute) can only be safely called within a caller that enables all the `target_feature`s that the callee enables.
  This restriction does not apply in an `unsafe` context.
- Safe `#[target_feature]` functions (and closures that inherit the attribute) can only be coerced to *safe* function pointers in contexts that enable all the `target_feature`s that the coercee enables.
  This restriction does not apply to `unsafe` function pointers.

Implicitly enabled features are included in this rule. For example an `sse2` function can call ones marked with `sse`.

```rust
# #[cfg(target_feature = "sse2")] {
#[target_feature(enable = "sse")]
fn foo_sse() {}

fn bar() {
    // Calling `foo_sse` here is unsafe, as we must ensure that SSE is
    // available first, even if `sse` is enabled by default on the target
    // platform or manually enabled as compiler flags.
    unsafe {
        foo_sse();
    }
}

#[target_feature(enable = "sse")]
fn bar_sse() {
    // Calling `foo_sse` here is safe.
    foo_sse();
    || foo_sse();
}

#[target_feature(enable = "sse2")]
fn bar_sse2() {
    // Calling `foo_sse` here is safe because `sse2` implies `sse`.
    foo_sse();
}
# }
```

r[attributes.codegen.target_feature.fn-traits]
A function with a `#[target_feature]` attribute *never* implements the `Fn` family of traits, although closures inheriting features from the enclosing function do.

r[attributes.codegen.target_feature.allowed-positions]
The `#[target_feature]` attribute is not allowed on the following places:

- [the `main` function][crate.main]
- a [`panic_handler` function][panic.panic_handler]
- safe trait methods
- safe default functions in traits

r[attributes.codegen.target_feature.inline]
Functions marked with `target_feature` are not inlined into a context that
does not support the given features. The `#[inline(always)]` attribute may not
be used with a `target_feature` attribute.

r[attributes.codegen.target_feature.availability]
### Available features

The following is a list of the available feature names.

r[attributes.codegen.target_feature.x86]
#### `x86` or `x86_64`

Executing code with unsupported features is undefined behavior on this platform.
Hence on this platform usage of `#[target_feature]` functions follows the
[above restrictions][attributes.codegen.target_feature.safety-restrictions].

Feature     | Implicitly Enables | Description
------------|--------------------|-------------------
`adx`       |          | [ADX] --- Multi-Precision Add-Carry Instruction Extensions
`aes`       | `sse2`   | [AES] --- Advanced Encryption Standard
`avx`       | `sse4.2` | [AVX] --- Advanced Vector Extensions
`avx2`      | `avx`    | [AVX2] --- Advanced Vector Extensions 2
`avx512bf16`        | `avx512bw`           | [AVX512-BF16] --- Advanced Vector Extensions 512-bit - Bfloat16 Extensions
`avx512bitalg`      | `avx512bw`           | [AVX512-BITALG] --- Advanced Vector Extensions 512-bit - Bit Algorithms
`avx512bw`          | `avx512f`            | [AVX512-BW] --- Advanced Vector Extensions 512-bit - Byte and Word Instructions
`avx512cd`          | `avx512f`            | [AVX512-CD] --- Advanced Vector Extensions 512-bit - Conflict Detection Instructions
`avx512dq`          | `avx512f`            | [AVX512-DQ] --- Advanced Vector Extensions 512-bit - Doubleword and Quadword Instructions
`avx512f`           | `avx2`, `fma`, `f16c`| [AVX512-F] --- Advanced Vector Extensions 512-bit - Foundation
`avx512fp16`        | `avx512bw`           | [AVX512-FP16] --- Advanced Vector Extensions 512-bit - Float16 Extensions
`avx512ifma`        | `avx512f`            | [AVX512-IFMA] --- Advanced Vector Extensions 512-bit - Integer Fused Multiply Add
`avx512vbmi`        | `avx512bw`           | [AVX512-VBMI] --- Advanced Vector Extensions 512-bit - Vector Byte Manipulation Instructions
`avx512vbmi2`       | `avx512bw`           | [AVX512-VBMI2] --- Advanced Vector Extensions 512-bit - Vector Byte Manipulation Instructions 2
`avx512vl`          | `avx512f`            | [AVX512-VL] --- Advanced Vector Extensions 512-bit - Vector Length Extensions
`avx512vnni`        | `avx512f`            | [AVX512-VNNI] --- Advanced Vector Extensions 512-bit - Vector Neural Network Instructions
`avx512vp2intersect`| `avx512f`            | [AVX512-VP2INTERSECT] --- Advanced Vector Extensions 512-bit - Vector Pair Intersection to a Pair of Mask Registers
`avx512vpopcntdq`   | `avx512f`            | [AVX512-VPOPCNTDQ] --- Advanced Vector Extensions 512-bit - Vector Population Count Instruction
`avxifma`           | `avx2`               | [AVX-IFMA] --- Advanced Vector Extensions - Integer Fused Multiply Add
`avxneconvert`      | `avx2`               | [AVX-NE-CONVERT] --- Advanced Vector Extensions - No-Exception Floating-Point conversion Instructions
`avxvnni`           | `avx2`               | [AVX-VNNI] --- Advanced Vector Extensions - Vector Neural Network Instructions
`avxvnniint16`      | `avx2`               | [AVX-VNNI-INT16] --- Advanced Vector Extensions - Vector Neural Network Instructions with 16-bit Integers
`avxvnniint8`       | `avx2`               | [AVX-VNNI-INT8] --- Advanced Vector Extensions - Vector Neural Network Instructions with 8-bit Integers
`bmi1`      |          | [BMI1] --- Bit Manipulation Instruction Sets
`bmi2`      |          | [BMI2] --- Bit Manipulation Instruction Sets 2
`cmpxchg16b`|          | [`cmpxchg16b`] --- Compares and exchange 16 bytes (128 bits) of data atomically
`f16c`      | `avx`    | [F16C] --- 16-bit floating point conversion instructions
`fma`       | `avx`    | [FMA3] --- Three-operand fused multiply-add
`fxsr`      |          | [`fxsave`] and [`fxrstor`] --- Save and restore x87 FPU, MMX Technology, and SSE State
`gfni`      | `sse2`   | [GFNI] --- Galois Field New Instructions
`kl`        | `sse2`   | [KEYLOCKER] --- Intel Key Locker Instructions
`lzcnt`     |          | [`lzcnt`] --- Leading zeros count
`movbe`     |          | [`movbe`] --- Move data after swapping bytes
`pclmulqdq` | `sse2`   | [`pclmulqdq`] --- Packed carry-less multiplication quadword
`popcnt`    |          | [`popcnt`] --- Count of bits set to 1
`rdrand`    |          | [`rdrand`] --- Read random number
`rdseed`    |          | [`rdseed`] --- Read random seed
`sha`       | `sse2`   | [SHA] --- Secure Hash Algorithm
`sha512`    | `avx2`   | [SHA512] --- Secure Hash Algorithm with 512-bit digest
`sm3`       | `avx`    | [SM3] --- ShangMi 3 Hash Algorithm
`sm4`       | `avx2`   | [SM4] --- ShangMi 4 Cipher Algorithm
`sse`       |          | [SSE] --- Streaming <abbr title="Single Instruction Multiple Data">SIMD</abbr> Extensions
`sse2`      | `sse`    | [SSE2] --- Streaming SIMD Extensions 2
`sse3`      | `sse2`   | [SSE3] --- Streaming SIMD Extensions 3
`sse4.1`    | `ssse3`  | [SSE4.1] --- Streaming SIMD Extensions 4.1
`sse4.2`    | `sse4.1` | [SSE4.2] --- Streaming SIMD Extensions 4.2
`sse4a`     | `sse3`   | [SSE4a] --- Streaming SIMD Extensions 4a
`ssse3`     | `sse3`   | [SSSE3] --- Supplemental Streaming SIMD Extensions 3
`tbm`       |          | [TBM] --- Trailing Bit Manipulation
`vaes`      | `avx2`, `aes`     | [VAES] --- Vector AES Instructions
`vpclmulqdq`| `avx`, `pclmulqdq`| [VPCLMULQDQ] --- Vector Carry-less multiplication of Quadwords
`widekl`    | `kl`     | [KEYLOCKER_WIDE] --- Intel Wide Keylocker Instructions
`xsave`     |          | [`xsave`] --- Save processor extended states
`xsavec`    |          | [`xsavec`] --- Save processor extended states with compaction
`xsaveopt`  |          | [`xsaveopt`] --- Save processor extended states optimized
`xsaves`    |          | [`xsaves`] --- Save processor extended states supervisor

<!-- Keep links near each table to make it easier to move and update. -->

[ADX]: https://en.wikipedia.org/wiki/Intel_ADX
[AES]: https://en.wikipedia.org/wiki/AES_instruction_set
[AVX]: https://en.wikipedia.org/wiki/Advanced_Vector_Extensions
[AVX2]: https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX2
[AVX512-BF16]: https://en.wikipedia.org/wiki/AVX-512#BF16
[AVX512-BITALG]: https://en.wikipedia.org/wiki/AVX-512#VPOPCNTDQ_and_BITALG
[AVX512-BW]: https://en.wikipedia.org/wiki/AVX-512#BW,_DQ_and_VBMI
[AVX512-CD]: https://en.wikipedia.org/wiki/AVX-512#Conflict_detection
[AVX512-DQ]: https://en.wikipedia.org/wiki/AVX-512#BW,_DQ_and_VBMI
[AVX512-F]: https://en.wikipedia.org/wiki/AVX-512
[AVX512-FP16]: https://en.wikipedia.org/wiki/AVX-512#FP16
[AVX512-IFMA]: https://en.wikipedia.org/wiki/AVX-512#IFMA
[AVX512-VBMI]: https://en.wikipedia.org/wiki/AVX-512#BW,_DQ_and_VBMI
[AVX512-VBMI2]: https://en.wikipedia.org/wiki/AVX-512#VBMI2
[AVX512-VL]: https://en.wikipedia.org/wiki/AVX-512
[AVX512-VNNI]: https://en.wikipedia.org/wiki/AVX-512#VNNI
[AVX512-VP2INTERSECT]: https://en.wikipedia.org/wiki/AVX-512#VP2INTERSECT
[AVX512-VPOPCNTDQ]:https://en.wikipedia.org/wiki/AVX-512#VPOPCNTDQ_and_BITALG
[AVX-IFMA]: https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX-VNNI,_AVX-IFMA
[AVX-NE-CONVERT]: https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX-VNNI,_AVX-IFMA
[AVX-VNNI]: https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX-VNNI,_AVX-IFMA
[AVX-VNNI-INT16]: https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX-VNNI,_AVX-IFMA
[AVX-VNNI-INT8]: https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX-VNNI,_AVX-IFMA
[BMI1]: https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets
[BMI2]: https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#BMI2
[`cmpxchg16b`]: https://www.felixcloutier.com/x86/cmpxchg8b:cmpxchg16b
[F16C]: https://en.wikipedia.org/wiki/F16C
[FMA3]: https://en.wikipedia.org/wiki/FMA_instruction_set
[`fxsave`]: https://www.felixcloutier.com/x86/fxsave
[`fxrstor`]: https://www.felixcloutier.com/x86/fxrstor
[GFNI]: https://en.wikipedia.org/wiki/AVX-512#GFNI
[KEYLOCKER]: https://en.wikipedia.org/wiki/List_of_x86_cryptographic_instructions#Intel_Key_Locker_instructions
[KEYLOCKER_WIDE]: https://en.wikipedia.org/wiki/List_of_x86_cryptographic_instructions#Intel_Key_Locker_instructions
[`lzcnt`]: https://www.felixcloutier.com/x86/lzcnt
[`movbe`]: https://www.felixcloutier.com/x86/movbe
[`pclmulqdq`]: https://www.felixcloutier.com/x86/pclmulqdq
[`popcnt`]: https://www.felixcloutier.com/x86/popcnt
[`rdrand`]: https://en.wikipedia.org/wiki/RdRand
[`rdseed`]: https://en.wikipedia.org/wiki/RdRand
[SHA]: https://en.wikipedia.org/wiki/Intel_SHA_extensions
[SHA512]: https://en.wikipedia.org/wiki/Intel_SHA_extensions
[SM3]: https://en.wikipedia.org/wiki/List_of_x86_cryptographic_instructions#Intel_SHA_and_SM3_instructions
[SM4]: https://en.wikipedia.org/wiki/List_of_x86_cryptographic_instructions#Intel_SHA_and_SM3_instructions
[SSE]: https://en.wikipedia.org/wiki/Streaming_SIMD_Extensions
[SSE2]: https://en.wikipedia.org/wiki/SSE2
[SSE3]: https://en.wikipedia.org/wiki/SSE3
[SSE4.1]: https://en.wikipedia.org/wiki/SSE4#SSE4.1
[SSE4.2]: https://en.wikipedia.org/wiki/SSE4#SSE4.2
[SSE4a]: https://en.wikipedia.org/wiki/SSE4#SSE4a
[SSSE3]: https://en.wikipedia.org/wiki/SSSE3
[TBM]: https://en.wikipedia.org/wiki/X86_Bit_manipulation_instruction_set#TBM_(Trailing_Bit_Manipulation)
[VAES]: https://en.wikipedia.org/wiki/AVX-512#VAES
[VPCLMULQDQ]: https://en.wikipedia.org/wiki/AVX-512#VPCLMULQDQ
[`xsave`]: https://www.felixcloutier.com/x86/xsave
[`xsavec`]: https://www.felixcloutier.com/x86/xsavec
[`xsaveopt`]: https://www.felixcloutier.com/x86/xsaveopt
[`xsaves`]: https://www.felixcloutier.com/x86/xsaves

r[attributes.codegen.target_feature.aarch64]
#### `aarch64`

On this platform the usage of `#[target_feature]` functions follows the
[above restrictions][attributes.codegen.target_feature.safety-restrictions].

Further documentation on these features can be found in the [ARM Architecture
Reference Manual], or elsewhere on [developer.arm.com].

[ARM Architecture Reference Manual]: https://developer.arm.com/documentation/ddi0487/latest
[developer.arm.com]: https://developer.arm.com

> [!NOTE]
> The following pairs of features should both be marked as enabled or disabled together if used:
> - `paca` and `pacg`, which LLVM currently implements as one feature.

Feature        | Implicitly Enables | Feature Name
-------        | ------------------ | ------------
`aes`          | `neon`             | FEAT_AES & FEAT_PMULL --- Advanced <abbr title="Single Instruction Multiple Data">SIMD</abbr> AES & PMULL instructions
`bf16`         |                    | FEAT_BF16 --- BFloat16 instructions
`bti`          |                    | FEAT_BTI --- Branch Target Identification
`crc`          |                    | FEAT_CRC --- CRC32 checksum instructions
`dit`          |                    | FEAT_DIT  --- Data Independent Timing instructions
`dotprod`      | `neon`             | FEAT_DotProd --- Advanced SIMD Int8 dot product instructions
`dpb`          |                    | FEAT_DPB --- Data cache clean to point of persistence
`dpb2`         | `dpb`              | FEAT_DPB2 --- Data cache clean to point of deep persistence
`f32mm`        | `sve`              | FEAT_F32MM --- SVE single-precision FP matrix multiply instruction
`f64mm`        | `sve`              | FEAT_F64MM --- SVE double-precision FP matrix multiply instruction
`fcma`         | `neon`             | FEAT_FCMA --- Floating point complex number support
`fhm`          | `fp16`             | FEAT_FHM --- Half-precision FP FMLAL instructions
`flagm`        |                    | FEAT_FLAGM --- Conditional flag manipulation
`fp16`         | `neon`             | FEAT_FP16 --- Half-precision FP data processing
`frintts`      |                    | FEAT_FRINTTS --- Floating-point to int helper instructions
`i8mm`         |                    | FEAT_I8MM --- Int8 Matrix Multiplication
`jsconv`       | `neon`             | FEAT_JSCVT --- JavaScript conversion instruction
`lor`          |                    | FEAT_LOR --- Limited Ordering Regions extension
`lse`          |                    | FEAT_LSE --- Large System Extensions
`mte`          |                    | FEAT_MTE & FEAT_MTE2 --- Memory Tagging Extension
`neon`         |                    | FEAT_AdvSimd & FEAT_FP --- Floating Point and Advanced SIMD extension
`paca`         |                    | FEAT_PAUTH --- Pointer Authentication (address authentication)
`pacg`         |                    | FEAT_PAUTH --- Pointer Authentication (generic authentication)
`pan`          |                    | FEAT_PAN --- Privileged Access-Never extension
`pmuv3`        |                    | FEAT_PMUv3 --- Performance Monitors extension (v3)
`rand`         |                    | FEAT_RNG --- Random Number Generator
`ras`          |                    | FEAT_RAS & FEAT_RASv1p1 --- Reliability, Availability and Serviceability extension
`rcpc`         |                    | FEAT_LRCPC --- Release consistent Processor Consistent
`rcpc2`        | `rcpc`             | FEAT_LRCPC2 --- RcPc with immediate offsets
`rdm`          | `neon`             | FEAT_RDM --- Rounding Double Multiply accumulate
`sb`           |                    | FEAT_SB --- Speculation Barrier
`sha2`         | `neon`             | FEAT_SHA1 & FEAT_SHA256 --- Advanced SIMD SHA instructions
`sha3`         | `sha2`             | FEAT_SHA512 & FEAT_SHA3 --- Advanced SIMD SHA instructions
`sm4`          | `neon`             | FEAT_SM3 & FEAT_SM4 --- Advanced SIMD SM3/4 instructions
`spe`          |                    | FEAT_SPE --- Statistical Profiling Extension
`ssbs`         |                    | FEAT_SSBS & FEAT_SSBS2 --- Speculative Store Bypass Safe
`sve`          | `neon`             | FEAT_SVE --- Scalable Vector Extension
`sve2`         | `sve`              | FEAT_SVE2 --- Scalable Vector Extension 2
`sve2-aes`     | `sve2`, `aes`      | FEAT_SVE_AES & FEAT_SVE_PMULL128 --- SVE AES instructions
`sve2-bitperm` | `sve2`             | FEAT_SVE2_BitPerm --- SVE Bit Permute
`sve2-sha3`    | `sve2`, `sha3`     | FEAT_SVE2_SHA3 --- SVE SHA3 instructions
`sve2-sm4`     | `sve2`, `sm4`      | FEAT_SVE2_SM4 --- SVE SM4 instructions
`tme`          |                    | FEAT_TME --- Transactional Memory Extension
`vh`           |                    | FEAT_VHE --- Virtualization Host Extensions

r[attributes.codegen.target_feature.loongarch]
#### `loongarch`

On this platform the usage of `#[target_feature]` functions follows the
[above restrictions][attributes.codegen.target_feature.safety-restrictions].

Feature     | Implicitly Enables  | Description
------------|---------------------|-------------------
`f`         |                     | [F][la-f] --- Single-precision float-point instructions
`d`         | `f`                 | [D][la-d] --- Double-precision float-point instructions
`frecipe`   |                     | [FRECIPE][la-frecipe] --- Reciprocal approximation instructions
`lasx`      | `lsx`               | [LASX][la-lasx] --- 256-bit vector instructions
`lbt`       |                     | [LBT][la-lbt] --- Binary translation instructions
`lsx`       | `d`                 | [LSX][la-lsx] --- 128-bit vector instructions
`lvz`       |                     | [LVZ][la-lvz] --- Virtualization instructions

<!-- Keep links near each table to make it easier to move and update. -->

[la-f]: https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-fp_sp
[la-d]: https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-fp_dp
[la-frecipe]: https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-frecipe
[la-lasx]: https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-lasx
[la-lbt]: https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-lbt_x86
[la-lsx]: https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-lsx
[la-lvz]: https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-lvz

r[attributes.codegen.target_feature.riscv]
#### `riscv32` or `riscv64`

On this platform the usage of `#[target_feature]` functions follows the
[above restrictions][attributes.codegen.target_feature.safety-restrictions].

Further documentation on these features can be found in their respective
specification. Many specifications are described in the [RISC-V ISA Manual],
[version 20250508], or in another manual hosted on the [RISC-V GitHub Account].

[RISC-V ISA Manual]: https://github.com/riscv/riscv-isa-manual
[version 20250508]: https://github.com/riscv/riscv-isa-manual/tree/20250508
[RISC-V GitHub Account]: https://github.com/riscv

Feature     | Implicitly Enables  | Description
------------|---------------------|-------------------
`a`         |                     | [A][rv-a] --- Atomic instructions
`c`         |                     | [C][rv-c] --- Compressed instructions
`m`         |                     | [M][rv-m] --- Integer Multiplication and Division instructions
`zba`       |                     | [Zba][rv-zba] --- Address Generation instructions
`zbb`       |                     | [Zbb][rv-zbb] --- Basic bit-manipulation
`zbc`       | `zbkc`              | [Zbc][rv-zbc] --- Carry-less multiplication
`zbkb`      |                     | [Zbkb][rv-zbkb] --- Bit Manipulation Instructions for Cryptography
`zbkc`      |                     | [Zbkc][rv-zbkc] --- Carry-less multiplication for Cryptography
`zbkx`      |                     | [Zbkx][rv-zbkx] --- Crossbar permutations
`zbs`       |                     | [Zbs][rv-zbs] --- Single-bit instructions
`zk`        | `zkn`, `zkr`, `zks`, `zkt`, `zbkb`, `zbkc`, `zkbx` | [Zk][rv-zk] --- Scalar Cryptography
`zkn`       | `zknd`, `zkne`, `zknh`, `zbkb`, `zbkc`, `zkbx`     | [Zkn][rv-zkn] --- NIST Algorithm suite extension
`zknd`      |                                                    | [Zknd][rv-zknd] --- NIST Suite: AES Decryption
`zkne`      |                                                    | [Zkne][rv-zkne] --- NIST Suite: AES Encryption
`zknh`      |                                                    | [Zknh][rv-zknh] --- NIST Suite: Hash Function Instructions
`zkr`       |                                                    | [Zkr][rv-zkr] --- Entropy Source Extension
`zks`       | `zksed`, `zksh`, `zbkb`, `zbkc`, `zkbx`            | [Zks][rv-zks] --- ShangMi Algorithm Suite
`zksed`     |                                                    | [Zksed][rv-zksed] --- ShangMi Suite: SM4 Block Cipher Instructions
`zksh`      |                                                    | [Zksh][rv-zksh] --- ShangMi Suite: SM3 Hash Function Instructions
`zkt`       |                                                    | [Zkt][rv-zkt] --- Data Independent Execution Latency Subset

<!-- Keep links near each table to make it easier to move and update. -->

[rv-a]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/a-st-ext.adoc
[rv-c]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/c-st-ext.adoc
[rv-m]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/m-st-ext.adoc
[rv-zba]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc
[rv-zbb]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc
[rv-zbc]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc
[rv-zbkb]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc
[rv-zbkc]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc
[rv-zbkx]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc
[rv-zbs]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc
[rv-zk]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc
[rv-zkn]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc
[rv-zkne]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc
[rv-zknd]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc
[rv-zknh]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc
[rv-zkr]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc
[rv-zks]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc
[rv-zksed]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc
[rv-zksh]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc
[rv-zkt]: https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc

r[attributes.codegen.target_feature.wasm]
#### `wasm32` or `wasm64`

Safe `#[target_feature]` functions may always be used in safe contexts on Wasm
platforms. It is impossible to cause undefined behavior via the
`#[target_feature]` attribute because attempting to use instructions
unsupported by the Wasm engine will fail at load time without the risk of being
interpreted in a way different from what the compiler expected.

Feature               | Implicitly Enables  | Description
----------------------|---------------------|-------------------
`bulk-memory`         |                     | [WebAssembly bulk memory operations proposal][bulk-memory]
`extended-const`      |                     | [WebAssembly extended const expressions proposal][extended-const]
`mutable-globals`     |                     | [WebAssembly mutable global proposal][mutable-globals]
`nontrapping-fptoint` |                     | [WebAssembly non-trapping float-to-int conversion proposal][nontrapping-fptoint]
`relaxed-simd`        | `simd128`           | [WebAssembly relaxed simd proposal][relaxed-simd]
`sign-ext`            |                     | [WebAssembly sign extension operators Proposal][sign-ext]
`simd128`             |                     | [WebAssembly simd proposal][simd128]
`multivalue`          |                     | [WebAssembly multivalue proposal][multivalue]
`reference-types`     |                     | [WebAssembly reference-types proposal][reference-types]
`tail-call`           |                     | [WebAssembly tail-call proposal][tail-call]

[bulk-memory]: https://github.com/WebAssembly/bulk-memory-operations
[extended-const]: https://github.com/WebAssembly/extended-const
[mutable-globals]: https://github.com/WebAssembly/mutable-global
[nontrapping-fptoint]: https://github.com/WebAssembly/nontrapping-float-to-int-conversions
[relaxed-simd]: https://github.com/WebAssembly/relaxed-simd
[sign-ext]: https://github.com/WebAssembly/sign-extension-ops
[simd128]: https://github.com/webassembly/simd
[reference-types]: https://github.com/webassembly/reference-types
[tail-call]: https://github.com/webassembly/tail-call
[multivalue]: https://github.com/webassembly/multi-value

r[attributes.codegen.target_feature.info]
### Additional information

r[attributes.codegen.target_feature.remark-cfg]
See the [`target_feature` conditional compilation option] for selectively
enabling or disabling compilation of code based on compile-time settings. Note
that this option is not affected by the `target_feature` attribute, and is
only driven by the features enabled for the entire crate.

r[attributes.codegen.target_feature.remark-rt]
See the [`is_x86_feature_detected`] or [`is_aarch64_feature_detected`] macros
in the standard library for runtime feature detection on these platforms.

> [!NOTE]
> `rustc` has a default set of features enabled for each target and CPU. The CPU may be chosen with the [`-C target-cpu`] flag. Individual features may be enabled or disabled for an entire crate with the [`-C target-feature`] flag.

<!-- template:attributes -->
r[attributes.codegen.track_caller]
## The `track_caller` attribute

r[attributes.codegen.track_caller.intro]
The *`track_caller` [attribute][attributes]* is used on functions to indicate that the caller should be tracked for the purpose of using [`Location`] to determine the caller.

> [!EXAMPLE]
> ```rust
> #[track_caller]
> fn f() {
>     println!("{}", std::panic::Location::caller());
> }
> ```

r[attributes.codegen.track_caller.syntax]
The `track_caller` attribute uses the [MetaWord] syntax.

r[attributes.codegen.track_caller.allowed-positions]
The `track_caller` attribute may only be applied to:

- [Free functions][items.fn]
- [Inherent associated functions][items.associated.fn]
- [Trait impl functions][items.impl.trait]
- [Trait definition functions][items.traits]
- [External block functions][items.extern.fn]
- [Closures][expr.closure]

All functions must have the [`"Rust"` ABI][rust-abi].

It may not be applied to the [the `main` function][crate.main].

r[attributes.codegen.track_caller.duplicates]
Only the first use of `track_caller` on an item has effect.

> [!NOTE]
> `rustc` lints against any use following the first.

r[attributes.codegen.track_caller.traits]
When applied to functions and methods in trait declarations, the `track_caller` attribute applies to all implementations. If the trait provides a default implementation with the attribute, then the attribute also applies to override implementations.

r[attributes.codegen.track_caller.extern]
When applied to a function in an `extern` block, the `track_caller` attribute must also be applied to any linked implementations, otherwise undefined behavior results. When applied to a function which is made available to an `extern` block, the declaration in the `extern` block must also have the attribute, otherwise undefined behavior results.

r[attributes.codegen.track_caller.behavior]
Applying the `track_caller` attribute to a function `f` allows code within `f` to get a hint of the [`Location`] of the *topmost* tracked call that led to `f`'s invocation. At the point of observation, an implementation behaves as if it walks up the stack from `f`'s frame to find the nearest frame of an *unattributed* function `outer`, and it returns the [`Location`] of the tracked call in `outer`.

> [!NOTE]
> `core` provides [`core::panic::Location::caller`] for observing caller locations. It wraps the [`core::intrinsics::caller_location`] intrinsic implemented by `rustc`.

> [!NOTE]
> Because the resulting `Location` is a hint, an implementation may halt its walk up the stack early. See [Limitations](#track_caller-limitations) for important caveats.

> [!EXAMPLE]
> When `f` is called directly by `calls_f`, code in `f` observes its callsite within `calls_f`:
>
> ```rust
> # #[track_caller]
> # fn f() {
> #     println!("{}", std::panic::Location::caller());
> # }
> fn calls_f() {
>     f(); // <-- f() prints this location
> }
> ```
>
> When `f` is called by another attributed function `g` which is in turn called by `calls_g`, code in both `f` and `g` observes `g`'s callsite within `calls_g`:
>
> ```rust
> # #[track_caller]
> # fn f() {
> #     println!("{}", std::panic::Location::caller());
> # }
> #[track_caller]
> fn g() {
>     println!("{}", std::panic::Location::caller());
>     f();
> }
>
> fn calls_g() {
>     g(); // <-- g() prints this location twice, once itself and once from f()
> }
> ```
>
> When `g` is called by another attributed function `h` which is in turn called by `calls_h`, all code in `f`, `g`, and `h` observes `h`'s callsite within `calls_h`:
>
> ```rust
> # #[track_caller]
> # fn f() {
> #     println!("{}", std::panic::Location::caller());
> # }
> # #[track_caller]
> # fn g() {
> #     println!("{}", std::panic::Location::caller());
> #     f();
> # }
> #[track_caller]
> fn h() {
>     println!("{}", std::panic::Location::caller());
>     g();
> }
>
> fn calls_h() {
>     h(); // <-- prints this location three times, once itself, once from g(), once from f()
> }
> ```
>
> And so on.

r[attributes.codegen.track_caller.limits]
### `track_caller` limitations

r[attributes.codegen.track_caller.hint]
This information is a hint and implementations are not required to preserve it.

r[attributes.codegen.track_caller.decay]
In particular, coercing a function with `#[track_caller]` to a function pointer creates a shim which appears to observers to have been called at the attributed function's definition site, losing actual caller information across virtual calls. A common example of this coercion is the creation of a trait object whose methods are attributed.

> [!NOTE]
> The aforementioned shim for function pointers is necessary because `rustc` implements `track_caller` in a codegen context by appending an implicit parameter to the function ABI, but this would be unsound for an indirect call because the parameter is not a part of the function's type and a given function pointer type may or may not refer to a function with the attribute. The creation of a shim hides the implicit parameter from callers of the function pointer, preserving soundness.

<!-- template:attributes -->
r[attributes.codegen.instruction_set]
## The `instruction_set` attribute

r[attributes.codegen.instruction_set.intro]
The *`instruction_set` [attribute]* specifies the instruction set that a function will use during code generation. This allows mixing more than one instruction set in a single program.

> [!EXAMPLE]
> <!-- ignore: arm-only -->
> ```rust,ignore
> #[instruction_set(arm::a32)]
> fn arm_code() {}
>
> #[instruction_set(arm::t32)]
> fn thumb_code() {}
> ```

r[attributes.codegen.instruction_set.syntax]
The `instruction_set` attribute uses the [MetaListPaths] syntax to specify a single path consisting of the architecture family name and instruction set name.

r[attributes.codegen.instruction_set.allowed-positions]
The `instruction_set` attribute may only be applied to functions with [bodies] --- [closures], [async blocks], [free functions], [associated functions] in an [inherent impl] or [trait impl], and associated functions in a [trait definition] when those functions have a [default definition] .

> [!NOTE]
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

> [!NOTE]
> Though the attribute can be applied to [closures] and [async blocks], the usefulness of this is limited as we do not yet support attributes on expressions.

r[attributes.codegen.instruction_set.duplicates]
The `instruction_set` attribute may be used only once on a function.

r[attributes.codegen.instruction_set.target-limits]
The `instruction_set` attribute may only be used with a target that supports the given value.

r[attributes.codegen.instruction_set.inline-asm]
When the `instruction_set` attribute is used, any inline assembly in the function must use the specified instruction set instead of the target default.

r[attributes.codegen.instruction_set.arm]
### `instruction_set` on ARM

When targeting the `ARMv4T` and `ARMv5te` architectures, the supported values for `instruction_set` are:

- `arm::a32` --- Generate the function as A32 "ARM" code.
- `arm::t32` --- Generate the function as T32 "Thumb" code.

If the address of the function is taken as a function pointer, the low bit of the address will depend on the selected instruction set:

- For `arm::a32` ("ARM"), it will be 0.
- For `arm::t32` ("Thumb"), it will be 1.

[`-C target-cpu`]: ../../rustc/codegen-options/index.html#target-cpu
[`-C target-feature`]: ../../rustc/codegen-options/index.html#target-feature
[`export_name`]: abi.export_name
[`is_aarch64_feature_detected`]: ../../std/arch/macro.is_aarch64_feature_detected.html
[`is_x86_feature_detected`]: ../../std/arch/macro.is_x86_feature_detected.html
[`Location`]: core::panic::Location
[`naked_asm!`]: ../inline-assembly.md
[`no_mangle`]: abi.no_mangle
[`target_feature` conditional compilation option]: ../conditional-compilation.md#target_feature
[`unused_variables`]: ../../rustc/lints/listing/warn-by-default.html#unused-variables
[associated functions]: items.associated.fn
[async blocks]: expr.block.async
[async closure]: expr.closure.async
[async function]: items.fn.async
[attribute]: ../attributes.md
[attributes]: ../attributes.md
[bodies]: items.fn.body
[closures]: expr.closure
[default definition]: items.traits.associated-item-decls
[free functions]: items.fn
[function body]: ../items/functions.md#function-body
[functions]: ../items/functions.md
[inherent impl]: items.impl.inherent
[rust-abi]: ../items/external-blocks.md#abi
[target architecture]: ../conditional-compilation.md#target_arch
[trait]: items.traits
[trait definition]: items.traits
[trait impl]: items.impl.trait
[undefined behavior]: ../behavior-considered-undefined.md
[unsafe attribute]: ../attributes.md#r-attributes.safety
