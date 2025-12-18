//! Very basic diagnostics output support.

use std::fmt;

/// Handler for errors and warnings.
pub struct Diagnostics {
    /// Whether or not warnings should be errors (set by SPEC_DENY_WARNINGS
    /// environment variable).
    pub deny_warnings: bool,
    /// Number of messages generated.
    pub count: u32,
}

impl Diagnostics {
    pub fn new() -> Diagnostics {
        let deny_warnings = std::env::var("SPEC_DENY_WARNINGS").as_deref() == Ok("1");
        Diagnostics {
            deny_warnings,
            count: 0,
        }
    }

    /// Displays a warning or error (depending on whether warnings are denied).
    ///
    /// Usually you want the [`warn_or_err!`] macro.
    pub fn warn_or_err(&mut self, args: fmt::Arguments<'_>) {
        if self.deny_warnings {
            eprintln!("error: {args}");
        } else {
            eprintln!("warning: {args}");
        }
        self.count += 1;
    }
}

/// Displays a warning or error (depending on whether warnings are denied).
#[macro_export]
macro_rules! warn_or_err {
    ($diag:expr, $($arg:tt)*) => {
        $diag.warn_or_err(format_args!($($arg)*));
    };
}

/// Displays a message for an internal error, and immediately exits.
#[macro_export]
macro_rules! bug {
    ($($arg:tt)*) => {
        eprintln!("mdbook-spec internal error: {}", format_args!($($arg)*));
        std::process::exit(1);
    };
}
