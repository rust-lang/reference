//! Handling for linking tests in rust's testsuite to rule identifiers.

use crate::{Rules, Spec};
use mdbook::book::{Book, BookItem};
use std::collections::HashMap;
use std::fmt::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Mapping of rule identifier to the tests that include that identifier.
pub type RuleToTests = HashMap<String, Vec<Test>>;
/// A test in rustc's test suite.
pub struct Test {
    pub path: String,
}

const TABLE_START: &str = "
<table>
<tr>
  <th></th>
  <th>Rules</th>
  <th>Tests</th>
  <th>Uncovered Rules</th>
  <th>Coverage</th>
</tr>
";

/// Generates an HTML table summarizing the coverage of the testsuite.
pub fn make_summary_table(book: &Book, tests: &RuleToTests, rules: &Rules) -> String {
    let ch_to_rules = invert_rule_map(rules);

    let mut table = String::from(TABLE_START);
    let mut total_rules = 0;
    let mut total_tests = 0;
    let mut total_uncovered = 0;

    for (item_index, item) in book.iter().enumerate() {
        let BookItem::Chapter(ch) = item else {
            continue;
        };
        let Some(ch_path) = &ch.path else {
            continue;
        };
        let level = ch
            .number
            .as_ref()
            .map(|ch| ch.len() - 1)
            .unwrap_or_default() as u32;
        // Note: This path assumes that the summary chapter is in the root of
        // the book. If instead it is in a subdirectory, then this needs to
        // include relative `../` as needed.
        let html_path = ch_path
            .with_extension("html")
            .to_str()
            .unwrap()
            .replace('\\', "/");
        let number = ch
            .number
            .as_ref()
            .map(|n| n.to_string())
            .unwrap_or_default();
        let mut num_rules = 0;
        let mut num_tests_str = String::from("");
        let mut uncovered_str = String::from("");
        let mut coverage_str = String::from("");
        if let Some(rules) = ch_to_rules.get(ch_path) {
            num_rules = rules.len();
            total_rules += num_rules;
            let num_tests = rules
                .iter()
                .map(|rule| tests.get(rule).map(|ts| ts.len()).unwrap_or_default())
                .sum::<usize>();
            total_tests += num_tests;
            num_tests_str = num_tests.to_string();
            let uncovered_rules: Vec<_> = rules
                .iter()
                .filter(|rule| !tests.contains_key(rule.as_str()))
                .collect();
            let uncovered = uncovered_rules.len();
            total_uncovered += uncovered;
            coverage_str = fmt_pct(uncovered, num_rules);
            if uncovered == 0 {
                uncovered_str = String::from("0");
            } else {
                uncovered_str = format!(
                    "<div class=\"popup-container\">\n\
                        <a href=\"javascript:void(0)\" onclick=\"spec_toggle_uncovered({item_index});\">\
                        {uncovered}</a>\n\
                        <div id=\"uncovered-{item_index}\" class=\"uncovered-rules-popup popup-hidden\">\n\
                        Uncovered rules
                        <ul>");
                for uncovered_rule in uncovered_rules {
                    writeln!(
                        uncovered_str,
                        "<li><a href=\"{html_path}#r-{uncovered_rule}\">{uncovered_rule}</a></li>"
                    )
                    .unwrap();
                }
                uncovered_str.push_str("</ul></div></div>");
            }
        }
        let indent = "&nbsp;".repeat(level as usize * 6);

        writeln!(
            table,
            "<tr>\n\
                <td><a href=\"{html_path}\">{indent}{number} {name}</a></td>\n\
                <td>{num_rules}</td>\n\
                <td>{num_tests_str}</td>\n\
                <td>{uncovered_str}</td>\n\
                <td>{coverage_str}</td>\n\
            </tr>",
            name = ch.name,
        )
        .unwrap();
    }

    let total_coverage = fmt_pct(total_uncovered, total_rules);
    writeln!(
        table,
        "<tr>\n\
            <td><b>Total:</b></td>\n\
            <td>{total_rules}</td>\n\
            <td>{total_tests}</td>\n\
            <td>{total_uncovered}</td>\n\
            <td>{total_coverage}</td>\n\
        </tr>"
    )
    .unwrap();
    table.push_str("</table>\n");
    table
}

/// Formats a float as a percentage string.
fn fmt_pct(uncovered: usize, total: usize) -> String {
    let pct = ((total - uncovered) as f32 / total as f32) * 100.0;
    // Round up to tenths of a percent.
    let x = (pct * 10.0).ceil() / 10.0;
    format!("{x:.1}%")
}

/// Inverts the rule map so that it is chapter path to set of rules in that
/// chapter.
fn invert_rule_map(rules: &Rules) -> HashMap<PathBuf, Vec<String>> {
    let mut map: HashMap<PathBuf, Vec<String>> = HashMap::new();
    for (rule, (_, path)) in &rules.def_paths {
        map.entry(path.clone()).or_default().push(rule.clone());
    }
    for value in map.values_mut() {
        value.sort();
    }
    map
}

impl Spec {
    /// Scans all tests in rust-lang/rust, and creates a mapping of a rule
    /// identifier to the set of tests that include that identifier.
    pub fn collect_tests(&self, rules: &Rules) -> RuleToTests {
        let mut map = HashMap::new();
        let Some(rust_root) = &self.rust_root else {
            return map;
        };
        for entry in WalkDir::new(rust_root.join("tests")) {
            let entry = entry.unwrap();
            let path = entry.path();
            let relative = path.strip_prefix(rust_root).unwrap_or_else(|_| {
                panic!("expected root {rust_root:?} to be a prefix of {path:?}")
            });
            if path.extension().unwrap_or_default() == "rs" {
                let contents = std::fs::read_to_string(path).unwrap();
                for line in contents.lines() {
                    if let Some(id) = line.strip_prefix("//@ reference: ") {
                        if rules.interior_prefixes.contains(id) {
                            let instead: Vec<_> = rules
                                .def_paths
                                .keys()
                                .filter(|key| key.starts_with(&format!("{id}.")))
                                .collect();
                            eprintln!(
                                "info: Interior prefix rule {id} found in {path:?}\n    \
                                 Tests should not be annotated with prefixed rule names.\n    \
                                 Use the rules from {instead:?} instead."
                            );
                        } else if !rules.def_paths.contains_key(id) {
                            eprintln!(
                                "info: Orphaned rule identifier {id} found in {path:?}\n    \
                                 Please update the test to use an existing rule name."
                            );
                        }
                        let test = Test {
                            path: relative.to_str().unwrap().replace('\\', "/"),
                        };
                        map.entry(id.to_string()).or_default().push(test);
                    }
                }
            }
        }
        for tests in map.values_mut() {
            tests.sort_by(|a, b| a.path.cmp(&b.path));
        }
        map
    }
}
