# `rustc`

The Rust compiler has many options and can accept a wide variety of arguments,
and its behavior can vary depending on the values of several environment
variables.

We document the compiler's command-line options, arguments, and operative
environment variables here.

Some discussions of environment variables exists in the [Linkage](linkage.html)
chapter and the [Operator expressions](expressions/operator-expr.html#overflow)
chapter.

## Lint options

    -W, --warn OPT      Set lint warnings
    -A, --allow OPT     Set lint allowed
    -D, --deny OPT      Set lint denied
    -F, --forbid OPT    Set lint forbidden
        --cap-lints LEVEL
                        Set the most restrictive lint level. More restrictive
                        lints are capped at this level

## Codegen options

`rustc` provides many options for codegen, all accessible as arguments to the
`-C` option.

### Debug info

To produce output with debug info use the `-C debuginfo=val` option, where
`val` may be one of `0`, `1`, or `2`. The default is `0`.
  - `0` means output no debug info
  - `1` means output only line tables
  - `2` means output full debug info with variable and type information

Providing the `-g` option is equivalent to `-C debuginfo=2`. If both `-g` and
`-C debuginfo` are provided, the compiler will complain.

### Optimization

To produce optimized output, use the `-C opt-level=val` option, where `val`
may be one of `0`, `1`, `2`, `3`. The nightly compiler will also accept `s`,
or `z`. The default is `0`.
  - `0-3` direct `rustc` to optimize for execution speed with `0` meaning no
    optimizations, and `3` meaning aggressive optimization.
  - `s` and `z` direct `rustc` to optimize for output size, with `s` meaning
    typical size optimizations and `z` meaning aggressive size optimizations.

Providing the option `-O` is equivalent to `-C opt-level=2`. If both `-O` and
`-C opt-level` are provided, the compiler will complain.

If `s` or `z` are provided on a non-nightly compiler, the compiler will
complain.
