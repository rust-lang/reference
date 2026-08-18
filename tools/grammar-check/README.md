# Reference grammar checker

This is a CLI tool for validating the Reference grammar against other parsers (called *tools*).

## Commands

There are several different subcommands:

- `grammar-check lex-compare` — Compare tokenization between implementations.
- `grammar-check tokenize` — Convert source to tokens.
- `grammar-check tree` — Convert source to a tree.

Pass `--help` for more information.

It is recommended to run this in the release profile, especially when testing against a large corpus.

```shell
cargo r -r -- lex-compare --path /path/to/rust/tests
```

Some subcommands like `lex-compare` can parse multiple different kinds of sources, like stdin or auto-generated permutations. See the help output for more.

## Tools

This tool supports various parsers which are called *tools*. They are:

- `reference` — The Reference interpreter using the grammar from the Reference.
- `rustc_parse` — The AST parser from `rustc`.
- `rustc_lexer` — The low-level lexer from `rustc`. This generally isn't useful other than doing deeper analysis on rustc.
- `proc-macro2` — The `proc-macro2` crate.

## Coverage analysis

The tool can emit an HTML coverage report of the Reference grammar. Run a command like this:

```shell
cargo r -r -- lex-compare --coverage --permute Token
```

Then open `coverage.html` and look at the token rules to see how well they were covered. Green means it was fully covered, yellow was partially covered, and red is not covered at all. You can mouse-over to get a popup that shows more details about each sub-expression.

Ideally this should have full coverage, but it's not quite there.

## Edition support

There are the beginnings of edition support here, but generally it is incomplete. The Reference grammar itself is not Edition-aware. This will take some significant more work to support properly. Ideally the path-based input could parse the compiletest-based headers to figure out which edition to use for each file.

## AST parsing

The tree-based parsing is incomplete and needs some work. It can parse a simple individual item (like `struct S;`), but otherwise can't parse general Rust source. It needs work on both the parser itself and the Reference grammar itself. Example command:

```shell
cargo r -r -- tree --string 'struct S {x: i32}'
```

Comparison against other parsers is not implemented. A new `tree-compare` subcommand needs to be added. It will need to somehow be able to compare the trees between the Reference and the tool (either by normalizing, or having a large `match` that would compare every expression kind).

