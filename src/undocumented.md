# As-yet-undocumented Features

Several accepted, stabilized, and implemented RFCs lack documentation in this
reference, The Book, _Rust by Example_, or some combination of those three.
Until we have written reference documentation for these features, we provide
links to other sources of information about them. Therefore, expect this list
to shrink!

- [`libstd` facade]
- [Trait reform] – some partial documentation exists (the use of `Self`), but
  not for everything: e.g. coherence and orphan rules.
- [Attributes on `match` arms] – the underlying idea is documented in the
  [Attributes] section, but the applicability to internal items is never
  specified.
- [Flexible target specification] - Some---but not all---flags are documented
  in [Conditional compilation]
- [Require parentheses for chained comparisons]
- [Integer overflow not `unsafe`] - documented with a reference to the RFC, but
  requires further details
- [`dllimport`] - one element mentioned but not explained at [FFI attributes]
- [define `crt_link`]
- [define `unaligned_access`]

[`libstd` facade]: https://github.com/rust-lang/rfcs/pull/40
[Trait reform]: https://github.com/rust-lang/rfcs/pull/48
[Attributes on `match` arms]: https://github.com/rust-lang/rfcs/pull/49
[Flexible target specification]: https://github.com/rust-lang/rfcs/pull/131
[Conditional compilation]: attributes.html#conditional-compilation
[Unambiguous function call syntax]: https://github.com/rust-lang/rfcs/pull/132
[Require parentheses for chained comparisons]: https://github.com/rust-lang/rfcs/pull/558
[Integer overflow not `unsafe`]: https://github.com/rust-lang/rfcs/pull/560
[`dllimport`]: https://github.com/rust-lang/rfcs/pull/1717
[FFI attributes]: attributes.html#ffi-attributes
[define `crt_link`]: https://github.com/rust-lang/rfcs/pull/1721
[define `unaligned_access`]: https://github.com/rust-lang/rfcs/pull/1725
