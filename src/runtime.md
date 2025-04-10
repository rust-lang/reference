r[runtime]
# The Rust runtime

This section documents features that define some aspects of the Rust runtime.

r[runtime.global_allocator]
## The `global_allocator` attribute

The *`global_allocator` attribute* is used on a [static item] implementing the
[`GlobalAlloc`] trait to set the global allocator.

r[runtime.windows_subsystem]
## The `windows_subsystem` attribute

r[runtime.windows_subsystem.intro]
The *`windows_subsystem` attribute* may be applied at the crate level to set
the [subsystem] when linking on a Windows target.

r[runtime.windows_subsystem.syntax]
It uses the [MetaNameValueStr] syntax to specify the subsystem with a value of either
`console` or `windows`.

r[runtime.windows_subsystem.ignored]
This attribute is ignored on non-Windows targets, and for non-`bin` [crate types].

r[runtime.windows_subsystem.console]
The "console" subsystem is the default. If a console process is run from an
existing console then it will be attached to that console, otherwise a new
console window will be created.

r[runtime.windows_subsystem.windows]
The "windows" subsystem is commonly used by GUI applications that do not want to
display a console window on startup. It will run detached from any existing console.

```rust
#![windows_subsystem = "windows"]
```

[`GlobalAlloc`]: alloc::alloc::GlobalAlloc
[crate types]: linkage.md
[static item]: items/static-items.md
[subsystem]: https://msdn.microsoft.com/en-us/library/fcc1zstk.aspx

<script>
(function() {
    var fragments = {
        "#the-panic_handler-attribute": "panic.html#the-panic_handler-attribute",
    };
    var target = fragments[window.location.hash];
    if (target) {
        var url = window.location.toString();
        var base = url.substring(0, url.lastIndexOf('/'));
        window.location.replace(base + "/" + target);
    }
})();
</script>
