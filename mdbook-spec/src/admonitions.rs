//! Support for admonitions using markdown blockquotes.

use crate::{Diagnostics, warn_or_err};
use mdbook::book::Chapter;
use regex::{Captures, Regex};
use std::sync::LazyLock;

/// The Regex for the syntax for blockquotes that have a specific CSS class,
/// like `> [!WARNING]`.
static ADMONITION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^ *> \[!(?<admon>[^]]+)\]\n(?<blockquote>(?: *>.*\n)+)").unwrap()
});

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
pub fn admonitions(chapter: &Chapter, diag: &mut Diagnostics) -> String {
    ADMONITION_RE
        .replace_all(&chapter.content, |caps: &Captures<'_>| {
            let lower = caps["admon"].to_lowercase();
            let term = to_initial_case(&caps["admon"]);
            let blockquote = &caps["blockquote"];
            let initial_spaces = blockquote.chars().position(|ch| ch != ' ').unwrap_or(0);
            let space = &blockquote[..initial_spaces];
            if lower.starts_with("edition-") {
                let edition = &lower[8..];
                return format!("{space}<div class=\"alert alert-edition\">\n\
                    \n\
                    {space}> <p class=\"alert-title\">\
                        <span class=\"alert-title-edition\">{edition}</span> Edition differences</p>\n\
                    {space} >\n\
                    {blockquote}\n\
                    \n\
                    {space}</div>\n");
            }

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

fn to_initial_case(s: &str) -> String {
    let mut chars = s.chars();
    let first = chars.next().expect("not empty").to_uppercase();
    let rest = chars.as_str().to_lowercase();
    format!("{first}{rest}")
}
