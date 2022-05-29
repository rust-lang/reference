# Panic runtimes

XXX fill me in... 

From the Book:

> By default, when a panic occurs, the program starts unwinding, which means
> Rust walks back up the stack and cleans up the data from each function it
> encounters. However, this walking back and cleanup is a lot of work. Rust,
> therefore, allows you to choose the alternative of immediately aborting,
> which ends the program without cleaning up. Memory that the program was using
> will then need to be cleaned up by the operating system.

`rustc` option: `-C panic=<foo>`

`panic=abort`
`panic=unwind` (default)

With `panic=abort`:
* Cleanup code (`Drop`) can't be called
* Panics can't be caught with `catch_unwind`

Cargo unifies panic runtimes
