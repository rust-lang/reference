# As-yet-undocumented Features

Several accepted, stabilized, and implemented RFCs lack documentation in this
reference, The Book, _Rust by Example_, or some combination of those three.
Until we have written reference documentation for these features, we provide
links to other sources of information about them. Therefore, expect this list
to shrink!

- [`libstd` facade]
- [Trait reform] – some partial documentation exists (the use of `Self`), but
  not for everything: e.g. coherence and orphan rules.
- [Flexible target specification] - Some---but not all---flags are documented
  in [Conditional compilation]
- [`dllimport`] - one element mentioned but not explained at [FFI attributes]
- [define `crt_link`]
- `impl Trait`

[`libstd` facade]: https://github.com/rust-lang/rfcs/pull/40
[Trait reform]: https://github.com/rust-lang/rfcs/pull/48
[Attributes]: attributes.html
[Flexible target specification]: https://github.com/rust-lang/rfcs/pull/131
[Conditional compilation]: conditional-compilation.html
[`dllimport`]: https://github.com/rust-lang/rfcs/pull/1717
[FFI attributes]: attributes.html
[define `crt_link`]: https://github.com/rust-lang/rfcs/pull/1721
