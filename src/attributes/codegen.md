# Code generation attributes

The following [attributes] are used for controlling code generation.

## Optimization hints

The `cold` and `inline` [attributes] give suggestions to generate code in a
way that may be faster than what it would do without the hint. The attributes
are only hints, and may be ignored.

Both attributes can be used on [functions]. When applied to a function in a
[trait], they apply only to that function when used as a default function for
a trait implementation and not to all trait implementations. The attributes
have no effect on a trait function without a body.

### The `inline` attribute

The *`inline` [attribute]* suggests that a copy of the attributed function
should be placed in the caller, rather than generating code to call the
function where it is defined.

> ***Note***: The `rustc` compiler automatically inlines functions based on
> internal heuristics. Incorrectly inlining functions can make the program
> slower, so this attribute should be used with care.

There are three ways to use the inline attribute:

* `#[inline]` suggests performing an inline expansion.
* `#[inline(always)]` suggests that an inline expansion should always be
  performed.
* `#[inline(never)]` suggests that an inline expansion should never be
  performed.

### The `cold` attribute

The *`cold` [attribute]* suggests that the attributed function is unlikely to
be called.

## The `no_builtins` attribute

The *`no_builtins` [attribute]* may be applied at the crate level to disable
optimizing certain code patterns to invocations of library functions that are
assumed to exist.

## The `target_feature` attribute

The *`target_feature` [attribute]* may be applied to an [unsafe function] to
enable code generation of that function for specific platform architecture
features. It uses the [_MetaListNameValueStr_] syntax with a single key of
`enable` whose value is a string of comma-separated feature names to enable.

```rust
# #[cfg(target_feature = "avx2")]
#[target_feature(enable = "avx2")]
unsafe fn foo_avx2() {}
```

Each [target architecture] has a set of features that may be enabled. It is an
error to specify a feature for a target architecture that the crate is not
being compiled for.

It is [undefined behavior] to call a function that is compiled with a feature
that is not supported on the current platform the code is running on.

Functions marked with `target_feature` are not inlined into a context that
does not support the given features. The `#[inline(always)]` attribute may not
be used with a `target_feature` attribute.

### Available features

The following is a list of the available feature names.

#### `x86` or `x86_64`

Feature     | Implicitly Enables | Description
------------|--------------------|-------------------
`aes`       | `sse2`   | [AES] — Advanced Encryption Standard
`avx`       | `sse4.2` | [AVX] — Advanced Vector Extensions
`avx2`      | `avx`    | [AVX2] — Advanced Vector Extensions 2
`bmi1`      |          | [BMI1] — Bit Manipulation Instruction Sets
`bmi2`      |          | [BMI2] — Bit Manipulation Instruction Sets 2
`fma`       | `avx`    | [FMA3] — Three-operand fused multiply-add
`fxsr`      |          | [`fxsave`] and [`fxrstor`] — Save and restore x87 FPU, MMX Technology, and SSE State
`lzcnt`     |          | [`lzcnt`] — Leading zeros count
`pclmulqdq` | `sse2`   | [`pclmulqdq`] — Packed carry-less multiplication quadword
`popcnt`    |          | [`popcnt`] — Count of bits set to 1
`rdrand`    |          | [`rdrand`] — Read random number
`rdseed`    |          | [`rdseed`] — Read random seed
`sha`       | `sse2`   | [SHA] — Secure Hash Algorithm
`sse`       |          | [SSE] — Streaming <abbr title="Single Instruction Multiple Data">SIMD</abbr> Extensions
`sse2`      | `sse`    | [SSE2] — Streaming SIMD Extensions 2
`sse3`      | `sse2`   | [SSE3] — Streaming SIMD Extensions 3
`sse4.1`    | `sse3`   | [SSE4.1] — Streaming SIMD Extensions 4.1
`sse4.2`    | `sse4.1` | [SSE4.2] — Streaming SIMD Extensions 4.2
`ssse3`     | `sse3`   | [SSSE3] — Supplemental Streaming SIMD Extensions 3
`xsave`     |          | [`xsave`] — Save processor extended states
`xsavec`    |          | [`xsavec`] — Save processor extended states with compaction
`xsaveopt`  |          | [`xsaveopt`] — Save processor extended states optimized
`xsaves`    |          | [`xsaves`] — Save processor extended states supervisor

<!-- Keep links near each table to make it easier to move and update. -->

[AES]: https://en.wikipedia.org/wiki/AES_instruction_set
[AVX]: https://en.wikipedia.org/wiki/Advanced_Vector_Extensions
[AVX2]: https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX2
[BMI1]: https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets
[BMI2]: https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#BMI2
[FMA3]: https://en.wikipedia.org/wiki/FMA_instruction_set
[`fxsave`]: https://www.felixcloutier.com/x86/fxsave
[`fxrstor`]: https://www.felixcloutier.com/x86/fxrstor
[`lzcnt`]: https://www.felixcloutier.com/x86/lzcnt
[`pclmulqdq`]: https://www.felixcloutier.com/x86/pclmulqdq
[`popcnt`]: https://www.felixcloutier.com/x86/popcnt
[`rdrand`]: https://en.wikipedia.org/wiki/RdRand
[`rdseed`]: https://en.wikipedia.org/wiki/RdRand
[SHA]: https://en.wikipedia.org/wiki/Intel_SHA_extensions
[SSE]: https://en.wikipedia.org/wiki/Streaming_SIMD_Extensions
[SSE2]: https://en.wikipedia.org/wiki/SSE2
[SSE3]: https://en.wikipedia.org/wiki/SSE3
[SSE4.1]: https://en.wikipedia.org/wiki/SSE4#SSE4.1
[SSE4.2]: https://en.wikipedia.org/wiki/SSE4#SSE4.2
[SSSE3]: https://en.wikipedia.org/wiki/SSSE3
[`xsave`]: https://www.felixcloutier.com/x86/xsave
[`xsavec`]: https://www.felixcloutier.com/x86/xsavec
[`xsaveopt`]: https://www.felixcloutier.com/x86/xsaveopt
[`xsaves`]: https://www.felixcloutier.com/x86/xsaves

### Additional information

See the [`target_feature` conditional compilation option] for selectively
enabling or disabling compilation of code based on compile-time settings. Note
that this option is not affected by the `target_feature` attribute, and is
only driven by the features enabled for the entire crate.

See the [`is_x86_feature_detected`] macro in the standard library for runtime
feature detection on the x86 platforms.

> Note: `rustc` has a default set of features enabled for each target and CPU.
> The CPU may be chosen with the [`-C target-cpu`] flag. Individual features
> may be enabled or disabled for an entire crate with the
> [`-C target-feature`] flag.

[_MetaListNameValueStr_]: ../attributes.md#meta-item-attribute-syntax
[`-C target-cpu`]: ../../rustc/codegen-options/index.html#target-cpu
[`-C target-feature`]: ../../rustc/codegen-options/index.html#target-feature
[`is_x86_feature_detected`]: ../../std/macro.is_x86_feature_detected.html
[`target_feature` conditional compilation option]: ../conditional-compilation.md#target_feature
[attribute]: ../attributes.md
[attributes]: ../attributes.md
[functions]: ../items/functions.md
[target architecture]: ../conditional-compilation.md#target_arch
[trait]: ../items/traits.md
[undefined behavior]: ../behavior-considered-undefined.md
[unsafe function]: ../unsafe-functions.md
