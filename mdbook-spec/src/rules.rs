//! Handling for rule identifiers.

use crate::test_links::RuleToTests;
use crate::Spec;
use mdbook::book::Book;
use mdbook::BookItem;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::collections::{BTreeMap, HashSet};
use std::fmt::Write;
use std::path::PathBuf;

/// The Regex for rules like `r[foo]`.
static RULE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^r\[([^]]+)]$").unwrap());

/// The set of rules defined in the reference.
#[derive(Default)]
pub struct Rules {
    /// A mapping from a rule identifier to a tuple of `(source_path, path)`.
    ///
    /// `source_path` is the path to the markdown source file relative to the
    /// `SUMMARY.md`.
    ///
    /// `path` is the same as `source_path`, except filenames like `README.md`
    /// are translated to `index.md`. Which to use depends on if you are
    /// trying to access the source files (`source_path`), or creating links
    /// in the output (`path`).
    pub def_paths: BTreeMap<String, (PathBuf, PathBuf)>,
    /// Set of rule name prefixes that have more specific rules within.
    ///
    /// For example, `asm.ts-args` is an interior prefix of `asm.ts-args.syntax`.
    pub interior_prefixes: HashSet<String>,
}

impl Spec {
    /// Collects all rule definitions in the book.
    pub fn collect_rules(&self, book: &Book) -> Rules {
        let mut rules = Rules::default();
        for item in book.iter() {
            let BookItem::Chapter(ch) = item else {
                continue;
            };
            if ch.is_draft_chapter() {
                continue;
            }
            RULE_RE
                .captures_iter(&ch.content)
                .for_each(|caps: Captures<'_>| {
                    let rule_id = &caps[1];
                    let source_path = ch.source_path.clone().unwrap_or_default();
                    let path = ch.path.clone().unwrap_or_default();
                    if let Some((old, _)) = rules
                        .def_paths
                        .insert(rule_id.to_string(), (source_path.clone(), path.clone()))
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
                    let mut parts: Vec<_> = rule_id.split('.').collect();
                    while !parts.is_empty() {
                        parts.pop();
                        let prefix = parts.join(".");
                        rules.interior_prefixes.insert(prefix);
                    }
                });
        }

        rules
    }

    /// Converts lines that start with `r[…]` into a "rule" which has special
    /// styling and can be linked to.
    pub fn render_rule_definitions(&self, content: &str, tests: &RuleToTests) -> String {
        RULE_RE
            .replace_all(content, |caps: &Captures<'_>| {
                let rule_id = &caps[1];
                let mut test_html = String::new();
                if let Some(tests) = tests.get(rule_id) {
                    test_html = format!(
                        "<span class=\"popup-container\">\n\
                            &nbsp;&nbsp;&nbsp;&nbsp;<a href=\"javascript:void(0)\" onclick=\"spec_toggle_tests('{rule_id}');\">\
                            Tests</a>\n\
                            <div id=\"tests-{rule_id}\" class=\"tests-popup popup-hidden\">\n\
                            Tests with this rule:
                            <ul>");
                    for test in tests {
                        writeln!(
                            test_html,
                            "<li><a href=\"https://github.com/rust-lang/rust/blob/{git_ref}/{test_path}\">{test_path}</a></li>",
                            test_path = test.path,
                            git_ref = self.git_ref
                        )
                        .unwrap();
                    }

                    test_html.push_str("</ul></div></span>");
                }
                format!(
                    "<div class=\"rule\" id=\"r-{rule_id}\">\
                     <a class=\"rule-link\" href=\"#r-{rule_id}\">[{rule_id}]</a>\
                     {test_html}\
                     </div>\n"
                )
            })
            .to_string()
    }
}
