//! Support for admonitions using markdown blockquotes.
//!
//! To add support for a new admonition:
//!
//! 1. Modify the [`admonitions`] function below to include an icon.
//! 2. Modify `theme/reference.css` to set the color for the different themes.
//!    Look at one of the other admonitions as a guide.
//! 3. Update `src/introduction.md` and describe what this new block is for
//!    with an example.
//! 4. Update `docs/authoring.md` to show an example of your new admonition.

use crate::{Diagnostics, warn_or_err};
use mdbook::book::Chapter;
use regex::{Captures, Regex};
use std::sync::LazyLock;

/// The Regex for the syntax for blockquotes that have a specific CSS class,
/// like `> [!WARNING]`.
static ADMONITION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^ *> \[!(?<admon>[^]]+)\]\n(?<blockquote>(?: *>.*\n)+)").unwrap()
});

// This icon is from GitHub, MIT License, see https://github.com/primer/octicons
const ICON_NOTE: &str = r#"<path d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8Zm8-6.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13ZM6.5 7.75A.75.75 0 0 1 7.25 7h1a.75.75 0 0 1 .75.75v2.75h.25a.75.75 0 0 1 0 1.5h-2a.75.75 0 0 1 0-1.5h.25v-2h-.25a.75.75 0 0 1-.75-.75ZM8 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"></path>"#;

// This icon is from GitHub, MIT License, see https://github.com/primer/octicons
const ICON_WARNING: &str = r#"<path d="M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0 1 14.082 15H1.918a1.75 1.75 0 0 1-1.543-2.575Zm1.763.707a.25.25 0 0 0-.44 0L1.698 13.132a.25.25 0 0 0 .22.368h12.164a.25.25 0 0 0 .22-.368Zm.53 3.996v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path>"#;

// This icon is from GitHub, MIT License, see https://github.com/primer/octicons
const ICON_EXAMPLE: &str = r#"<path d="M8 0a8 8 0 1 1 0 16A8 8 0 0 1 8 0ZM1.5 8a6.5 6.5 0 1 0 13 0 6.5 6.5 0 0 0-13 0Zm4.879-2.773 4.264 2.559a.25.25 0 0 1 0 .428l-4.264 2.559A.25.25 0 0 1 6 10.559V5.442a.25.25 0 0 1 .379-.215Z"></path>"#;

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

            let format_div = |class, content| {
                format!(
                    "{space}<div class=\"alert alert-{class}\">\n\
                    \n\
                    {space}> <p class=\"alert-title\">\
                        {content}</p>\n\
                    {space} >\n\
                    {blockquote}\n\
                    \n\
                    {space}</div>\n",
                )
            };

            if lower.starts_with("edition-") {
                let edition = &lower[8..];
                return format_div(
                    "edition",
                    format!(
                        "<span class=\"alert-title-edition\">{edition}</span> Edition differences"
                    ),
                );
            }

            let svg = match lower.as_str() {
                "note" => ICON_NOTE,
                "warning" => ICON_WARNING,
                "example" => ICON_EXAMPLE,
                _ => {
                    warn_or_err!(
                        diag,
                        "admonition `{lower}` in {:?} is incorrect or not yet supported",
                        chapter.path.as_ref().unwrap()
                    );
                    ""
                }
            };
            format_div(
                &lower,
                format!(
                    "<svg viewBox=\"0 0 16 16\" width=\"18\" height=\"18\">\
                        {svg}\
                    </svg>{term}"
                ),
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
