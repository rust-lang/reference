# Panic runtimes

XXX fill me in... 

`rustc` option: `-C panic=<foo>`

`panic=abort`
`panic=unwind` (default)

With `panic=abort`:
* Cleanup code (`Drop`) can't be called
* Panics can't be caught with `catch_unwind`

Cargo unifies panic runtimes
