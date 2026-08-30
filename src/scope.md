# Scope

## Scope of the Reference

The scope of the Rust Reference is to fully describe the syntax and semantics of the Rust language. In this context, the language encompasses:

- Valid syntax and forms.
- Meaning and interpretation of the syntax.
- Semantic properties of the runtime behavior.
- Built-in attributes.
- Built-in types.
- *Language items* from the standard library, which directly interface with the language (such as operators desugaring to trait calls).
- All [editions] of Rust.

## Out of scope

The Reference does not cover:

- Contents of the standard library, unless required to describe other language aspects.
    - The standard library is described in [the standard library API documentation][std].
- Introductions or guides to the language. Background familiarity with the language is assumed. A separate [book] is available to help acquire such background familiarity.
- Recommendations on how to write Rust code.
    - Exceptional cases may be included in a [note] or [warning].
- Specifics of tooling provided by the Rust organization, such as `rustc` or Cargo.
    - `rustc` has its own [book][rustc book]. Cargo has a [book][cargo book] that contains a [reference][cargo reference].
    - In exceptional circumstances, notes may refer to these tools when they have significant relevance.
- Specifics of which targets exist, or properties of specific targets. See [target-specific behavior] for more details.
- Unstable features only available on the [nightly channel].
    - For documentation of unstable features, see the [Unstable Book].
    - Occasionally the Reference may refer to an unstable feature because it is important to understand the model of stable code.
- Future changes, with the following exceptions:
    - Language constructs that are designed to specifically allow for future extensions. For example, some syntax is reserved for future use, or some values such as config predicates may be documented as *open*, meaning they may be extended in the future.
    - Notes for [incomplete or undecided behavior] may refer to the future intentions.
    - Notes about deviation from [intended behavior] may mention that the behavior may change.
- Version history or past behavior.
    - Changes in Rust releases are recorded in the [Release Notes]. Prior releases of the Reference are available as described in [Rust releases].
- Historical documentation.
    - This includes documentation such as [RFCs], design documents, reports, blog posts, issues, pull requests, etc.
- Rationale for most aspects of the language.
    - Rationale may be included in a note in exceptional situations, particularly when a feature is not obvious or has especially useful relevance.
- `rustc` lints.
    - Lints are documented in the [rustc book][lints].
    - Lints that have a specific relation to some part of the language may be referred to by name. For example, the `expect` attribute can issue the `unfulfilled_lint_expectations` lint. However, the exact behavior of when the lint is issued is usually not documented in the Reference.
    - Notes may refer to lints when the lint has exceptional relevance to some aspect of the language.
- Method of translation into executable code, unless required (for example, as it relates to the translation of [inline assembly]).
- Allowed or disallowed optimizations, unless specifically afforded by the language.
- Limits on compiler inputs (such as maximum source file size), unless the language explicitly defines specific limits (for example, see [limits] or the [number of `#` symbols in raw strings][lex.token.literal.str-raw.intro]).

## Completeness

Portions of the Reference may be incomplete or undecided. Work is ongoing to complete these sections. New features and changes to the language must be completely documented, unless they involve sections of the language that were previously undocumented.

*Complete* means that the syntax and semantics are sufficiently documented for someone reasonably skilled in the art to fully understand them. However, because the majority of the Reference is written in English prose rather than a rigorous formalism, ambiguities or misunderstandings can arise. If you encounter something unclear, we welcome your feedback through our [issue tracker]. Separate work is ongoing to provide more formal definitions of the language in the future.

## Correctness

The Reference is intended to be correct in all aspects. If errors or ambiguities are discovered, the Reference is updated to correct them.

If there are errors, ambiguities, or conflicts with the `rustc` implementation, we must first determine the [intended behavior](#intended-behavior). Whether something is an error can sometimes be unclear; in those situations, the Reference editors consult:

- The behavior in `rustc`.
- Historical documentation and communication.
- Teams in the Rust organization.

## Intended behavior

At times, the actual behavior of the official `rustc` implementation may diverge from the documented or intended behavior. Generally, the Reference documents the intended behavior, even if it differs from the actual behavior in `rustc`. Informational notes may be included to highlight significant discrepancies. These notes may refer to *future-incompatible warnings*, which are lints used by `rustc` to alert users about planned changes.

However, documenting every bug or implementation quirk in `rustc` is out of scope. Only discrepancies that are significant, long-standing, or particularly likely to affect users are mentioned. Minor bugs, temporary implementation issues, or behaviors expected to be fixed in an upcoming release are typically not documented.

## Unspecified behavior

*Unspecified behavior* is behavior that is documented as not explicitly defined, but still covers a well-formed program. This should be a relatively rare concept, as the intent is for programmers to rely on expected behavior in as many areas as possible. Unspecified behavior may change between releases or compilation settings, and may become specified in a future release.

Behavior is typically left unspecified when the language intentionally allows implementation flexibility for optimization or platform-specific concerns, provided the variation does not affect program correctness. Examples include the exact layout of certain types using [the `Rust` representation] or how and when optimization hints such as inlining occur.

Unspecified behavior differs from [undefined behavior](#undefined-behavior) in that all possible outcomes are valid and do not compromise program safety. Programs should not rely on specific unspecified behaviors, as they may vary between compiler versions, optimization levels, or platforms.

Unspecified behavior differs from [incomplete documentation](#completeness) in that the Reference specifically identifies the behavior as intentionally unspecified, whereas incomplete documentation is intended to be specified but has not yet been written.

## Undefined behavior

[*Undefined behavior*][undefined] is compile-time or run-time behavior that is not specified. See the corresponding chapter for a complete description. Other chapters may mention undefined behavior where relevant, but should also link back to the [undefined behavior][undefined] chapter.

## Target-specific behavior

The Reference does not document which targets exist or the properties of specific targets. The Reference may refer to *platforms* or *target properties* where required. Examples of defined target properties include:

- Conditional-compilation keys like [`target_os`] are specified to exist, but not what their values must be.
- The [`windows_subsystem` attribute] specifies that it only works on Windows platforms.
- [Inline assembly] and the [`target_feature` attribute] specify the supported architectures.

For a list of targets supported by `rustc`, see [the rustc book][rustc-platforms].

## Normative and informational content

The Reference contains both normative and informational content. *Normative content* defines the official requirements and specifications of the Rust language: the rules that determine what constitutes valid Rust code and its behavior. *Informational content* provides context, examples, and clarifications that aid understanding but do not define requirements.

Normative content consists of [rules], [grammar productions], and anything else explicitly listed as normative.

Informational content consists of [notes], [examples], [warnings], introductions (rule labels ending in `.intro`), footnotes, and appendices (unless otherwise noted).

[`target_feature` attribute]: attributes.codegen.target_feature
[`target_os`]: cfg.target_os
[`windows_subsystem` attribute]: runtime.windows_subsystem
[book]: ../book/index.html
[cargo book]: ../cargo/index.html
[cargo reference]: ../cargo/reference/index.html
[editions]: conventions.md#editions
[examples]: conventions.md#examples
[grammar productions]: notation.md
[incomplete or undecided behavior]: #completeness
[Inline assembly]: asm
[intended behavior]: #intended-behavior
[issue tracker]: https://github.com/rust-lang/reference/issues
[limits]: attributes.limits
[lints]: ../rustc/lints/index.html
[nightly channel]: ../book/appendix-07-nightly-rust.html
[note]: conventions.md#notes
[notes]: conventions.md#notes
[Release Notes]: https://doc.rust-lang.org/releases.html
[RFCs]: https://rust-lang.github.io/rfcs/
[rules]: conventions.md#rules
[Rust releases]: introduction.md#rust-releases
[rustc book]: ../rustc/index.html
[rustc-platforms]: ../rustc/platform-support.html
[std]: ../std/index.html
[target-specific behavior]: #target-specific-behavior
[the `Rust` representation]: layout.repr.rust
[Unstable Book]: https://doc.rust-lang.org/nightly/unstable-book/
[warning]: conventions.md#warnings
[warnings]: conventions.md#warnings
