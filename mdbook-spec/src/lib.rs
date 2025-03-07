#![deny(rust_2018_idioms, unused_lifetimes)]

use crate::rules::Rules;
use anyhow::{bail, Context, Result};
use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use semver::{Version, VersionReq};
use std::fmt;
use std::io;
use std::ops::Range;
use std::path::PathBuf;

mod rules;
mod std_links;
mod test_links;

/// The Regex for the syntax for blockquotes that have a specific CSS class,
/// like `> [!WARNING]`.
static ADMONITION_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^ *> \[!(?<admon>[^]]+)\]\n(?<blockquote>(?: *>.*\n)+)").unwrap()
});

pub fn handle_preprocessing() -> Result<(), Error> {
    let pre = Spec::new(None)?;
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

/// Handler for errors and warnings.
pub struct Diagnostics {
    /// Whether or not warnings should be errors (set by SPEC_DENY_WARNINGS
    /// environment variable).
    deny_warnings: bool,
    /// Number of messages generated.
    count: u32,
}

impl Diagnostics {
    fn new() -> Diagnostics {
        let deny_warnings = std::env::var("SPEC_DENY_WARNINGS").as_deref() == Ok("1");
        Diagnostics {
            deny_warnings,
            count: 0,
        }
    }

    /// Displays a warning or error (depending on whether warnings are denied).
    ///
    /// Usually you want the [`warn_or_err!`] macro.
    fn warn_or_err(&mut self, args: fmt::Arguments<'_>) {
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

pub struct Spec {
    /// Path to the rust-lang/rust git repository (set by SPEC_RUST_ROOT
    /// environment variable).
    rust_root: Option<PathBuf>,
}

impl Spec {
    /// Creates a new `Spec` preprocessor.
    ///
    /// The `rust_root` parameter specifies an optional path to the root of
    /// the rust git checkout. If `None`, it will use the `SPEC_RUST_ROOT`
    /// environment variable. If the root is not specified, then no tests will
    /// be linked unless `SPEC_DENY_WARNINGS` is set in which case this will
    /// return an error.
    pub fn new(rust_root: Option<PathBuf>) -> Result<Spec> {
        let rust_root = rust_root.or_else(|| std::env::var_os("SPEC_RUST_ROOT").map(PathBuf::from));
        Ok(Spec { rust_root })
    }

    /// Generates link references to all rules on all pages, so you can easily
    /// refer to rules anywhere in the book.
    fn auto_link_references(&self, chapter: &Chapter, rules: &Rules) -> String {
        let current_path = chapter.path.as_ref().unwrap().parent().unwrap();
        let definitions: String = rules
            .def_paths
            .iter()
            .map(|(rule_id, (_, path))| {
                let relative = pathdiff::diff_paths(path, current_path).unwrap();
                // Adjust paths for Windows.
                let relative = relative.display().to_string().replace('\\', "/");
                format!("[{rule_id}]: {}#r-{rule_id}\n", relative)
            })
            .collect();
        format!(
            "{}\n\
            {definitions}",
            chapter.content
        )
    }

    /// Converts blockquotes with special headers into admonitions.
    ///
    /// The blockquote should look something like:
    ///
    /// ```markdown
    /// > [!WARNING]
    /// > ...
    /// ```
    ///
    /// This will add a `<div class="alert alert-warning">` around the
    /// blockquote so that it can be styled differently, and injects an icon.
    /// The actual styling needs to be added in the `reference.css` CSS file.
    fn admonitions(&self, chapter: &Chapter, diag: &mut Diagnostics) -> String {
        ADMONITION_RE
            .replace_all(&chapter.content, |caps: &Captures<'_>| {
                let lower = caps["admon"].to_lowercase();
                let term = to_initial_case(&caps["admon"]);
                let blockquote = &caps["blockquote"];
                let initial_spaces = blockquote.chars().position(|ch| ch != ' ').unwrap_or(0);
                let space = &blockquote[..initial_spaces];
                // These icons are from GitHub, MIT License, see https://github.com/primer/octicons
                let svg = match lower.as_str() {
                    "note" => "<path d=\"M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8Zm8-6.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13ZM6.5 7.75A.75.75 0 0 1 7.25 7h1a.75.75 0 0 1 .75.75v2.75h.25a.75.75 0 0 1 0 1.5h-2a.75.75 0 0 1 0-1.5h.25v-2h-.25a.75.75 0 0 1-.75-.75ZM8 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z\"></path>",
                    "warning" => "<path d=\"M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0 1 14.082 15H1.918a1.75 1.75 0 0 1-1.543-2.575Zm1.763.707a.25.25 0 0 0-.44 0L1.698 13.132a.25.25 0 0 0 .22.368h12.164a.25.25 0 0 0 .22-.368Zm.53 3.996v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z\"></path>",
                    _ => {
                        warn_or_err!(
                            diag,
                            "admonition `{lower}` in {:?} is incorrect or not yet supported",
                            chapter.path.as_ref().unwrap()
                        );
                        ""
                    }
                };
                format!(
                    "{space}<div class=\"alert alert-{lower}\">\n\
                    \n\
                    {space}> <p class=\"alert-title\">\
                        <svg viewBox=\"0 0 16 16\" width=\"18\" height=\"18\">\
                            {svg}\
                        </svg>{term}</p>\n\
                    {space} >\n\
                    {blockquote}\n\
                    \n\
                    {space}</div>\n",
                )
            })
            .to_string()
    }
}

fn to_initial_case(s: &str) -> String {
    let mut chars = s.chars();
    let first = chars.next().expect("not empty").to_uppercase();
    let rest = chars.as_str().to_lowercase();
    format!("{first}{rest}")
}

/// Determines the git ref used for linking to a particular branch/tag in GitHub.
fn git_ref(rust_root: &Option<PathBuf>) -> Result<String> {
    let Some(rust_root) = rust_root else {
        return Ok("master".into());
    };
    let channel = std::fs::read_to_string(rust_root.join("src/ci/channel"))
        .context("failed to read src/ci/channel")?;
    let git_ref = match channel.trim() {
        // nightly/beta are branches, not stable references. Should be ok
        // because we're not expecting those channels to be long-lived.
        "nightly" => "master".into(),
        "beta" => "beta".into(),
        "stable" => {
            let version = std::fs::read_to_string(rust_root.join("src/version"))
                .context("|| failed to read src/version")?;
            version.trim().into()
        }
        ch => bail!("unknown channel {ch}"),
    };
    Ok(git_ref)
}

impl Preprocessor for Spec {
    fn name(&self) -> &str {
        "spec"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut diag = Diagnostics::new();
        if diag.deny_warnings && self.rust_root.is_none() {
            bail!("error: SPEC_RUST_ROOT environment variable must be set");
        }
        let rules = self.collect_rules(&book, &mut diag);
        let tests = self.collect_tests(&rules);
        let summary_table = test_links::make_summary_table(&book, &tests, &rules);
        let git_ref = match git_ref(&self.rust_root) {
            Ok(s) => s,
            Err(e) => {
                warn_or_err!(&mut diag, "{e:?}");
                "master".into()
            }
        };

        book.for_each_mut(|item| {
            let BookItem::Chapter(ch) = item else {
                return;
            };
            if ch.is_draft_chapter() {
                return;
            }
            ch.content = self.admonitions(&ch, &mut diag);
            ch.content = self.auto_link_references(&ch, &rules);
            ch.content = self.render_rule_definitions(&ch.content, &tests, &git_ref);
            if ch.name == "Test summary" {
                ch.content = ch.content.replace("{{summary-table}}", &summary_table);
            }
        });

        // Final pass will resolve everything as a std link (or error if the
        // link is unknown).
        std_links::std_links(&mut book, &mut diag);

        if diag.count > 0 {
            if diag.deny_warnings {
                eprintln!("mdbook-spec exiting due to {} errors", diag.count);
                std::process::exit(1);
            }
            eprintln!("mdbook-spec generated {} warnings", diag.count);
        }

        Ok(book)
    }
}

fn line_from_range<'a>(contents: &'a str, range: &Range<usize>) -> &'a str {
    assert!(range.start < contents.len());

    let mut start_index = 0;
    for line in contents.lines() {
        let end_index = start_index + line.len();
        if range.start >= start_index && range.start <= end_index {
            return line;
        }
        start_index = end_index + 1;
    }
    panic!("did not find line {range:?} in contents");
}
