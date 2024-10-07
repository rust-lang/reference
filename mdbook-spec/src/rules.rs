//! Handling for rule identifiers.

use crate::Spec;
use mdbook::book::Chapter;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// The Regex for rules like `r[foo]`.
static RULE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^r\[([^]]+)]$").unwrap());

impl Spec {
    /// Converts lines that start with `r[…]` into a "rule" which has special
    /// styling and can be linked to.
    pub fn rule_definitions(
        &self,
        chapter: &Chapter,
        found_rules: &mut BTreeMap<String, (PathBuf, PathBuf)>,
    ) -> String {
        let source_path = chapter.source_path.clone().unwrap_or_default();
        let path = chapter.path.clone().unwrap_or_default();
        RULE_RE
            .replace_all(&chapter.content, |caps: &Captures<'_>| {
                let rule_id = &caps[1];
                if let Some((old, _)) =
                    found_rules.insert(rule_id.to_string(), (source_path.clone(), path.clone()))
                {
                    let message = format!(
                        "rule `{rule_id}` defined multiple times\n\
                        First location: {old:?}\n\
                        Second location: {source_path:?}"
                    );
                    if self.deny_warnings {
                        panic!("error: {message}");
                    } else {
                        eprintln!("warning: {message}");
                    }
                }
                format!(
                    "<div class=\"rule\" id=\"r-{rule_id}\">\
                     <a class=\"rule-link\" href=\"#r-{rule_id}\">[{rule_id}]</a>\
                     </div>\n"
                )
            })
            .to_string()
    }
}
