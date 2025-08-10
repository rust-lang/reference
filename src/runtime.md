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
The *`windows_subsystem` [attribute][attributes]* sets the [subsystem] when linking on a Windows target.

> [!EXAMPLE]
> ```rust
> #![windows_subsystem = "windows"]
> ```

r[runtime.windows_subsystem.syntax]
The `windows_subsystem` attribute uses the [MetaNameValueStr] syntax. Accepted values are `"console"` and `"windows"`.

r[runtime.windows_subsystem.allowed-positions]
The `windows_subsystem` attribute may only be applied to the crate root.

r[runtime.windows_subsystem.duplicates]
Only the first use of `windows_subsystem` is honored.

> [!NOTE]
> `rustc` currently lints against uses following the first. This may become a hard error in the future.

r[runtime.windows_subsystem.ignored]
The `windows_subsystem` attribute is ignored on non-Windows targets and non-`bin` [crate types].

r[runtime.windows_subsystem.console]
The `"console"` subsystem is the default. If a console process is run from an existing console then it will be attached to that console; otherwise a new console window will be created.

r[runtime.windows_subsystem.windows]
The `"windows"` subsystem will run detached from any existing console.

> [!NOTE]
> The `"windows"` subsystem is commonly used by GUI applications that do not want to display a console window on startup.

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
