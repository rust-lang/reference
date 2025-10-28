r[macro.decl]
# Macros by example

r[macro.decl.syntax]
```grammar,macros
MacroRulesDefinition ->
    `macro_rules` `!` IDENTIFIER MacroRulesDef

MacroRulesDef ->
      `(` MacroRules `)` `;`
    | `[` MacroRules `]` `;`
    | `{` MacroRules `}`

MacroRules ->
    MacroRule ( `;` MacroRule )* `;`?

MacroRule ->
    MacroMatcher `=>` MacroTranscriber

MacroMatcher ->
      `(` MacroMatch* `)`
    | `[` MacroMatch* `]`
    | `{` MacroMatch* `}`

MacroMatch ->
      Token _except `$` and [delimiters][lex.token.delim]_
    | MacroMatcher
    | `$` ( IDENTIFIER_OR_KEYWORD _except `crate`_ | RAW_IDENTIFIER ) `:` MacroFragSpec
    | `$` `(` MacroMatch+ `)` MacroRepSep? MacroRepOp

MacroFragSpec ->
      `block` | `expr` | `expr_2021` | `ident` | `item` | `lifetime` | `literal`
    | `meta` | `pat` | `pat_param` | `path` | `stmt` | `tt` | `ty` | `vis`

MacroRepSep -> Token _except [delimiters][lex.token.delim] and [MacroRepOp]_

MacroRepOp -> `*` | `+` | `?`

MacroTranscriber -> DelimTokenTree
```

r[macro.decl.intro]
`macro_rules` allows users to define syntax extension in a declarative way.  We
call such extensions "macros by example" or simply "macros".

Each macro by example has a name, and one or more _rules_. Each rule has two
parts: a _matcher_, describing the syntax that it matches, and a _transcriber_,
describing the syntax that will replace a successfully matched invocation. Both
the matcher and the transcriber must be surrounded by delimiters. Macros can
expand to expressions, statements, items (including traits, impls, and foreign
items), types, or patterns.

r[macro.decl.transcription]
## Transcribing

r[macro.decl.transcription.intro]
When a macro is invoked, the macro expander looks up macro invocations by name,
and tries each macro rule in turn. It transcribes the first successful match; if
this results in an error, then future matches are not tried.

r[macro.decl.transcription.lookahead]
When matching, no lookahead is performed; if the compiler cannot unambiguously determine how to
parse the macro invocation one token at a time, then it is an error. In the
following example, the compiler does not look ahead past the identifier to see
if the following token is a `)`, even though that would allow it to parse the
invocation unambiguously:

```rust,compile_fail
macro_rules! ambiguity {
    ($($i:ident)* $j:ident) => { };
}

ambiguity!(error); // Error: local ambiguity
```

r[macro.decl.transcription.syntax]
In both the matcher and the transcriber, the `$` token is used to invoke special
behaviours from the macro engine (described below in [Metavariables] and
[Repetitions]). Tokens that aren't part of such an invocation are matched and
transcribed literally, with one exception. The exception is that the outer
delimiters for the matcher will match any pair of delimiters. Thus, for
instance, the matcher `(())` will match `{()}` but not `{{}}`. The character
`$` cannot be matched or transcribed literally.

r[macro.decl.transcription.fragment]
### Forwarding a matched fragment

When forwarding a matched fragment to another macro-by-example, matchers in
the second macro will see an opaque AST of the fragment type. The second macro
can't use literal tokens to match the fragments in the matcher, only a
fragment specifier of the same type. The `ident`, `lifetime`, and `tt`
fragment types are an exception, and *can* be matched by literal tokens. The
following illustrates this restriction:

```rust,compile_fail
macro_rules! foo {
    ($l:expr) => { bar!($l); }
// ERROR:               ^^ no rules expected this token in macro call
}

macro_rules! bar {
    (3) => {}
}

foo!(3);
```

The following illustrates how tokens can be directly matched after matching a
`tt` fragment:

```rust
// compiles OK
macro_rules! foo {
    ($l:tt) => { bar!($l); }
}

macro_rules! bar {
    (3) => {}
}

foo!(3);
```

r[macro.decl.meta]
## Metavariables

r[macro.decl.meta.intro]
In the matcher, `$` _name_ `:` _fragment-specifier_ matches a Rust syntax
fragment of the kind specified and binds it to the metavariable `$`_name_.

r[macro.decl.meta.specifier]
Valid fragment specifiers are:

  * `block`: a [BlockExpression]
  * `expr`: an [Expression]
  * `expr_2021`: an [Expression] except [UnderscoreExpression] and [ConstBlockExpression] (see [macro.decl.meta.edition2024])
  * `ident`: an [IDENTIFIER_OR_KEYWORD] except `_`, [RAW_IDENTIFIER], or [`$crate`]
  * `item`: an [Item]
  * `lifetime`: a [LIFETIME_TOKEN]
  * `literal`: matches `-`<sup>?</sup>[LiteralExpression]
  * `meta`: an [Attr], the contents of an attribute
  * `pat`: a [Pattern] (see [macro.decl.meta.edition2021])
  * `pat_param`: a [PatternNoTopAlt]
  * `path`: a [TypePath] style path
  * `stmt`: a [Statement][grammar-Statement] without the trailing semicolon (except for item statements that require semicolons)
  * `tt`: a [TokenTree]&nbsp;(a single [token] or tokens in matching delimiters `()`, `[]`, or `{}`)
  * `ty`: a [Type][grammar-Type]
  * `vis`: a possibly empty [Visibility] qualifier

r[macro.decl.meta.transcription]
In the transcriber, metavariables are referred to simply by `$`_name_, since
the fragment kind is specified in the matcher. Metavariables are replaced with
the syntax element that matched them.
Metavariables can be transcribed more than once or not at all.

r[macro.decl.meta.dollar-crate]
The keyword metavariable [`$crate`] can be used to refer to the current crate.

r[macro.decl.meta.edition2021]
> [!EDITION-2021]
> Starting with the 2021 edition, `pat` fragment-specifiers match top-level or-patterns (that is, they accept [Pattern]).
>
> Before the 2021 edition, they match exactly the same fragments as `pat_param` (that is, they accept [PatternNoTopAlt]).
>
> The relevant edition is the one in effect for the `macro_rules!` definition.

r[macro.decl.meta.edition2024]
> [!EDITION-2024]
> Before the 2024 edition, `expr` fragment specifiers do not match [UnderscoreExpression] or [ConstBlockExpression] at the top level. They are allowed within subexpressions.
>
> The `expr_2021` fragment specifier exists to maintain backwards compatibility with editions before 2024.

r[macro.decl.repetition]
## Repetitions

r[macro.decl.repetition.intro]
In both the matcher and transcriber, repetitions are indicated by placing the
tokens to be repeated inside `$(`…`)`, followed by a repetition operator,
optionally with a separator token between.

r[macro.decl.repetition.separator]
The separator token can be any token
other than a delimiter or one of the repetition operators, but `;` and `,` are
the most common. For instance, `$( $i:ident ),*` represents any number of
identifiers separated by commas. Nested repetitions are permitted.

r[macro.decl.repetition.operators]
The repetition operators are:

- `*` --- indicates any number of repetitions.
- `+` --- indicates any number but at least one.
- `?` --- indicates an optional fragment with zero or one occurrence.

r[macro.decl.repetition.optional-restriction]
Since `?` represents at most one occurrence, it cannot be used with a
separator.

r[macro.decl.repetition.fragment]
The repeated fragment both matches and transcribes to the specified number of
the fragment, separated by the separator token. Metavariables are matched to
every repetition of their corresponding fragment. For instance, the `$( $i:ident
),*` example above matches `$i` to all of the identifiers in the list.

During transcription, additional restrictions apply to repetitions so that the
compiler knows how to expand them properly:

1.  A metavariable must appear in exactly the same number, kind, and nesting
    order of repetitions in the transcriber as it did in the matcher. So for the
    matcher `$( $i:ident ),*`, the transcribers `=> { $i }`,
    `=> { $( $( $i)* )* }`, and `=> { $( $i )+ }` are all illegal, but
    `=> { $( $i );* }` is correct and replaces a comma-separated list of
    identifiers with a semicolon-separated list.
2.  Each repetition in the transcriber must contain at least one metavariable to
    decide how many times to expand it. If multiple metavariables appear in the
    same repetition, they must be bound to the same number of fragments. For
    instance, `( $( $i:ident ),* ; $( $j:ident ),* ) => (( $( ($i,$j) ),* ))` must
    bind the same number of `$i` fragments as `$j` fragments. This means that
    invoking the macro with `(a, b, c; d, e, f)` is legal and expands to
    `((a,d), (b,e), (c,f))`, but `(a, b, c; d, e)` is illegal because it does
    not have the same number. This requirement applies to every layer of nested
    repetitions.

r[macro.decl.scope]
## Scoping, exporting, and importing

r[macro.decl.scope.intro]
For historical reasons, the scoping of macros by example does not work entirely
like items. Macros have two forms of scope: textual scope, and path-based scope.
Textual scope is based on the order that things appear in source files, or even
across multiple files, and is the default scoping. It is explained further below.
Path-based scope works exactly the same way that item scoping does. The scoping,
exporting, and importing of macros is controlled largely by attributes.

r[macro.decl.scope.unqualified]
When a macro is invoked by an unqualified identifier (not part of a multi-part
path), it is first looked up in textual scoping. If this does not yield any
results, then it is looked up in path-based scoping. If the macro's name is
qualified with a path, then it is only looked up in path-based scoping.

<!-- ignore: requires external crates -->
```rust,ignore
use lazy_static::lazy_static; // Path-based import.

macro_rules! lazy_static { // Textual definition.
    (lazy) => {};
}

lazy_static!{lazy} // Textual lookup finds our macro first.
self::lazy_static!{} // Path-based lookup ignores our macro, finds imported one.
```

r[macro.decl.scope.textual]
### Textual scope

r[macro.decl.scope.textual.intro]
Textual scope is based largely on the order that things appear in source files,
and works similarly to the scope of local variables declared with `let` except
it also applies at the module level. When `macro_rules!` is used to define a
macro, the macro enters the scope after the definition (note that it can still
be used recursively, since names are looked up from the invocation site), up
until its surrounding scope, typically a module, is closed. This can enter child
modules and even span across multiple files:

<!-- ignore: requires external modules -->
```rust,ignore
//// src/lib.rs
mod has_macro {
    // m!{} // Error: m is not in scope.

    macro_rules! m {
        () => {};
    }
    m!{} // OK: appears after declaration of m.

    mod uses_macro;
}

// m!{} // Error: m is not in scope.

//// src/has_macro/uses_macro.rs

m!{} // OK: appears after declaration of m in src/lib.rs
```

r[macro.decl.scope.textual.shadow]
It is not an error to define a macro multiple times; the most recent declaration
will shadow the previous one unless it has gone out of scope.

```rust
macro_rules! m {
    (1) => {};
}

m!(1);

mod inner {
    m!(1);

    macro_rules! m {
        (2) => {};
    }
    // m!(1); // Error: no rule matches '1'
    m!(2);

    macro_rules! m {
        (3) => {};
    }
    m!(3);
}

m!(1);
```

Macros can be declared and used locally inside functions as well, and work
similarly:

```rust
fn foo() {
    // m!(); // Error: m is not in scope.
    macro_rules! m {
        () => {};
    }
    m!();
}

// m!(); // Error: m is not in scope.
```

<!-- template:attributes -->
r[macro.decl.scope.macro_use]
### The `macro_use` attribute

r[macro.decl.scope.macro_use.intro]
The *`macro_use` [attribute][attributes]* has two purposes: it may be used on modules to extend the scope of macros defined within them, and it may be used on [`extern crate`][items.extern-crate] to import macros from another crate into the [`macro_use` prelude].

> [!EXAMPLE]
> ```rust
> #[macro_use]
> mod inner {
>     macro_rules! m {
>         () => {};
>     }
> }
> m!();
> ```
>
> ```rust,ignore
> #[macro_use]
> extern crate log;
> ```

r[macro.decl.scope.macro_use.syntax]
When used on modules, the `macro_use` attribute uses the [MetaWord] syntax.

When used on `extern crate`, it uses the [MetaWord] and [MetaListIdents] syntaxes. For more on how these syntaxes may be used, see [macro.decl.scope.macro_use.prelude].

r[macro.decl.scope.macro_use.allowed-positions]
The `macro_use` attribute may be applied to modules or `extern crate`.

> [!NOTE]
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

r[macro.decl.scope.macro_use.extern-crate-self]
The `macro_use` attribute may not be used on [`extern crate self`].

r[macro.decl.scope.macro_use.duplicates]
The `macro_use` attribute may be used any number of times on a form.

Multiple instances of `macro_use` in the [MetaListIdents] syntax may be specified. The union of all specified macros will be imported.

> [!NOTE]
> On modules, `rustc` lints against any [MetaWord] `macro_use` attributes following the first.
>
> On `extern crate`, `rustc` lints against any `macro_use` attributes that have no effect due to not importing any macros not already imported by another `macro_use` attribute. If two or more [MetaListIdents] `macro_use` attributes import the same macro, the first is linted against. If any [MetaWord] `macro_use` attributes are present, all [MetaListIdents] `macro_use` attributes are linted against. If two or more [MetaWord] `macro_use` attributes are present, the ones following the first are linted against.

r[macro.decl.scope.macro_use.mod-decl]
When `macro_use` is used on a module, the module's macro scope extends beyond the module's lexical scope.

> [!EXAMPLE]
> ```rust
> #[macro_use]
> mod inner {
>     macro_rules! m {
>         () => {};
>     }
> }
> m!(); // OK
> ```

r[macro.decl.scope.macro_use.prelude]
Specifying `macro_use` on an `extern crate` declaration in the crate root imports exported macros from that crate.

Macros imported this way are imported into the [`macro_use` prelude], not textually, which means that they can be shadowed by any other name. Macros imported by `macro_use` can be used before the import statement.

> [!NOTE]
> `rustc` currently prefers the last macro imported in case of conflict. Don't rely on this. This behavior is unusual, as imports in Rust are generally order-independent. This behavior of `macro_use` may change in the future.
>
> For details, see [Rust issue #148025](https://github.com/rust-lang/rust/issues/148025).

When using the [MetaWord] syntax, all exported macros are imported. When using the [MetaListIdents] syntax, only the specified macros are imported.

> [!EXAMPLE]
> <!-- ignore: requires external crates -->
> ```rust,ignore
> #[macro_use(lazy_static)] // Or `#[macro_use]` to import all macros.
> extern crate lazy_static;
>
> lazy_static!{}
> // self::lazy_static!{} // ERROR: lazy_static is not defined in `self`.
> ```

r[macro.decl.scope.macro_use.export]
Macros to be imported with `macro_use` must be exported with [`macro_export`][macro.decl.scope.macro_export].

<!-- template:attributes -->
r[macro.decl.scope.macro_export]
### The `macro_export` attribute

r[macro.decl.scope.macro_export.intro]
The *`macro_export` [attribute][attributes]* exports the macro from the crate and makes it available in the root of the crate for path-based resolution.

> [!EXAMPLE]
> ```rust
> self::m!();
> //  ^^^^ OK: Path-based lookup finds `m` in the current module.
> m!(); // As above.
>
> mod inner {
>     super::m!();
>     crate::m!();
> }
>
> mod mac {
>     #[macro_export]
>     macro_rules! m {
>         () => {};
>     }
> }
> ```

r[macro.decl.scope.macro_export.syntax]
The `macro_export` attribute uses the [MetaWord] and [MetaListIdents] syntaxes. With the [MetaListIdents] syntax, it accepts a single [`local_inner_macros`][macro.decl.scope.macro_export.local_inner_macros] value.

r[macro.decl.scope.macro_export.allowed-positions]
The `macro_export` attribute may be applied to `macro_rules` definitions.

> [!NOTE]
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

r[macro.decl.scope.macro_export.duplicates]
Only the first use of `macro_export` on a macro has effect.

> [!NOTE]
> `rustc` lints against any use following the first.

r[macro.decl.scope.macro_export.path-based]
By default, macros only have [textual scope][macro.decl.scope.textual] and cannot be resolved by path. When the `macro_export` attribute is used, the macro is made available in the crate root and can be referred to by its path.

> [!EXAMPLE]
> Without `macro_export`, macros only have textual scope, so path-based resolution of the macro fails.
>
> ```rust,compile_fail,E0433
> macro_rules! m {
>     () => {};
> }
> self::m!(); // ERROR
> crate::m!(); // ERROR
> # fn main() {}
> ```
>
> With `macro_export`, path-based resolution works.
>
> ```rust
> #[macro_export]
> macro_rules! m {
>     () => {};
> }
> self::m!(); // OK
> crate::m!(); // OK
> # fn main() {}
> ```

r[macro.decl.scope.macro_export.export]
The `macro_export` attribute causes a macro to be exported from the crate root so that it can be referred to in other crates by path.

> [!EXAMPLE]
> Given the following in a `log` crate:
>
> ```rust
> #[macro_export]
> macro_rules! warn {
>     ($message:expr) => { eprintln!("WARN: {}", $message) };
> }
> ```
>
> From another crate, you can refer to the macro by path:
>
> <!-- ignore: requires external crates -->
> ```rust,ignore
> fn main() {
>     log::warn!("example warning");
> }
> ```

r[macro.decl.scope.macro_export.macro_use]
`macro_export` allows the use of [`macro_use`][macro.decl.scope.macro_use] on an `extern crate` to import the macro into the [`macro_use` prelude].

> [!EXAMPLE]
> Given the following in a `log` crate:
>
> ```rust
> #[macro_export]
> macro_rules! warn {
>     ($message:expr) => { eprintln!("WARN: {}", $message) };
> }
> ```
>
> Using `macro_use` in a dependent crate allows you to use the macro from the prelude:
>
> <!-- ignore: requires external crates -->
> ```rust,ignore
> #[macro_use]
> extern crate log;
>
> pub mod util {
>     pub fn do_thing() {
>         // Resolved via macro prelude.
>         warn!("example warning");
>     }
> }
> ```

r[macro.decl.scope.macro_export.local_inner_macros]
Adding `local_inner_macros` to the `macro_export` attribute causes all single-segment macro invocations in the macro definition to have an implicit `$crate::` prefix.

> [!NOTE]
> This is intended primarily as a tool to migrate code written before [`$crate`] was added to the language to work with Rust 2018's path-based imports of macros. Its use is discouraged in new code.

> [!EXAMPLE]
> ```rust
> #[macro_export(local_inner_macros)]
> macro_rules! helped {
>     () => { helper!() } // Automatically converted to $crate::helper!().
> }
>
> #[macro_export]
> macro_rules! helper {
>     () => { () }
> }
> ```

r[macro.decl.hygiene]
## Hygiene

r[macro.decl.hygiene.intro]
Macros by example have _mixed-site hygiene_. This means that [loop labels], [block labels], and local variables are looked up at the macro definition site while other symbols are looked up at the macro invocation site. For example:

```rust
let x = 1;
fn func() {
    unreachable!("this is never called")
}

macro_rules! check {
    () => {
        assert_eq!(x, 1); // Uses `x` from the definition site.
        func();           // Uses `func` from the invocation site.
    };
}

{
    let x = 2;
    fn func() { /* does not panic */ }
    check!();
}
```

Labels and local variables defined in macro expansion are not shared between invocations, so this code doesn’t compile:

```rust,compile_fail,E0425
macro_rules! m {
    (define) => {
        let x = 1;
    };
    (refer) => {
        dbg!(x);
    };
}

m!(define);
m!(refer);
```

And local variables has higher priority than items:

```rust
// example 1:
let f0 = || 42;
fn f0() -> i32 { 8 }

macro_rules! m0 {
    () => { f0() }
}

assert_eq!(m0!(), 42);

// example 2:
fn f1() -> i32 { 8 }
let f1 = || 42;

macro_rules! m1 {
    () => { f1() }
}

assert_eq!(m1!(), 42);
```

r[macro.decl.hygiene.crate]
A special case is the `$crate` metavariable. It refers to the crate defining the macro, and can be used at the start of the path to look up items or macros which are not in scope at the invocation site.

<!-- ignore: requires external crates -->
```rust,ignore
//// Definitions in the `helper_macro` crate.
#[macro_export]
macro_rules! helped {
    // () => { helper!() } // This might lead to an error due to 'helper' not being in scope.
    () => { $crate::helper!() }
}

#[macro_export]
macro_rules! helper {
    () => { () }
}

//// Usage in another crate.
// Note that `helper_macro::helper` is not imported!
use helper_macro::helped;

fn unit() {
    helped!();
}
```

Note that, because `$crate` refers to the current crate, it must be used with a
fully qualified module path when referring to non-macro items:

```rust
pub mod inner {
    #[macro_export]
    macro_rules! call_foo {
        () => { $crate::inner::foo() };
    }

    pub fn foo() {}
}
```

r[macro.decl.hygiene.vis]
Additionally, even though `$crate` allows a macro to refer to items within its
own crate when expanding, its use has no effect on visibility. An item or macro
referred to must still be visible from the invocation site. In the following
example, any attempt to invoke `call_foo!()` from outside its crate will fail
because `foo()` is not public.

```rust
#[macro_export]
macro_rules! call_foo {
    () => { $crate::foo() };
}

fn foo() {}
```

> [!NOTE]
> Prior to Rust 1.30, `$crate` and [`local_inner_macros`][macro.decl.scope.macro_export.local_inner_macros] were unsupported. They were added alongside [path-based imports of macros][macro.decl.scope.macro_export], to ensure that helper macros did not need to be manually imported by users of a macro-exporting crate. Crates written for earlier versions of Rust that use helper macros need to be modified to use `$crate` or `local_inner_macros` to work well with path-based imports.

r[macro.decl.follow-set]
## Follow-set ambiguity restrictions

r[macro.decl.follow-set.intro]
The parser used by the macro system is reasonably powerful, but it is limited in
order to prevent ambiguity in current or future versions of the language.

r[macro.decl.follow-set.token-restriction]
In particular, in addition to the rule about ambiguous expansions, a nonterminal
matched by a metavariable must be followed by a token which has been decided can
be safely used after that kind of match.

As an example, a macro matcher like `$i:expr [ , ]` could in theory be accepted
in Rust today, since `[,]` cannot be part of a legal expression and therefore
the parse would always be unambiguous. However, because `[` can start trailing
expressions, `[` is not a character which can safely be ruled out as coming
after an expression. If `[,]` were accepted in a later version of Rust, this
matcher would become ambiguous or would misparse, breaking working code.
Matchers like `$i:expr,` or `$i:expr;` would be legal, however, because `,` and
`;` are legal expression separators. The specific rules are:

r[macro.decl.follow-set.token-expr-stmt]
  * `expr` and `stmt` may only be followed by one of: `=>`, `,`, or `;`.

r[macro.decl.follow-set.token-pat_param]
  * `pat_param` may only be followed by one of: `=>`, `,`, `=`, `|`, `if`, or `in`.

r[macro.decl.follow-set.token-pat]
  * `pat` may only be followed by one of: `=>`, `,`, `=`, `if`, or `in`.

r[macro.decl.follow-set.token-path-ty]
  * `path` and `ty` may only be followed by one of: `=>`, `,`, `=`, `|`, `;`,
    `:`, `>`, `>>`, `[`, `{`, `as`, `where`, or a macro variable of `block`
    fragment specifier.

r[macro.decl.follow-set.token-vis]
  * `vis` may only be followed by one of: `,`, an identifier other than a
    non-raw `priv`, any token that can begin a type, or a metavariable with a
    `ident`, `ty`, or `path` fragment specifier.

r[macro.decl.follow-set.token-other]
  * All other fragment specifiers have no restrictions.

r[macro.decl.follow-set.edition2021]
> [!EDITION-2021]
> Before the 2021 edition, `pat` may also be followed by `|`.

r[macro.decl.follow-set.repetition]
When repetitions are involved, then the rules apply to every possible number of
expansions, taking separators into account. This means:

  * If the repetition includes a separator, that separator must be able to
    follow the contents of the repetition.
  * If the repetition can repeat multiple times (`*` or `+`), then the contents
    must be able to follow themselves.
  * The contents of the repetition must be able to follow whatever comes
    before, and whatever comes after must be able to follow the contents of the
    repetition.
  * If the repetition can match zero times (`*` or `?`), then whatever comes
    after must be able to follow whatever comes before.

For more detail, see the [formal specification].

[`extern crate self`]: items.extern-crate.self
[`macro_use` prelude]: names/preludes.md#macro_use-prelude
[block labels]: expr.loop.block-labels
[delimiters]: tokens.md#delimiters
[formal specification]: macro-ambiguity.md
[Hygiene]: #hygiene
[loop labels]: expressions/loop-expr.md#loop-labels
[Metavariables]: #metavariables
[Repetitions]: #repetitions
[token]: tokens.md
[`$crate`]: macro.decl.hygiene.crate
