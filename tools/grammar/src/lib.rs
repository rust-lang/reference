//! Support for loading the grammar.

use diagnostics::{Diagnostics, warn_or_err};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use walkdir::WalkDir;

mod parser;

#[derive(Debug, Default)]
pub struct Grammar {
    pub productions: HashMap<String, Production>,
    /// The order that the production names were discovered.
    pub name_order: Vec<String>,
}

#[derive(Debug)]
pub struct Production {
    pub name: String,
    /// Comments and breaks that precede the production name.
    pub comments: Vec<Expression>,
    /// Category is from the markdown lang string, and defines how it is
    /// grouped and organized on the summary page.
    pub category: String,
    pub expression: Expression,
    /// The path to the chapter where this is defined, relative to the book's
    /// `src` directory.
    pub path: PathBuf,
    pub is_root: bool,
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub kind: ExpressionKind,
    /// Suffix is the `_foo_` part that is shown as a subscript.
    pub suffix: Option<String>,
    /// A footnote is a markdown footnote link.
    pub footnote: Option<String>,
}

#[derive(Clone, Debug)]
pub enum ExpressionKind {
    /// `( A B C )`
    Grouped(Box<Expression>),
    /// `A | B | C`
    Alt(Vec<Expression>),
    /// `A B C`
    Sequence(Vec<Expression>),
    /// `A?`
    Optional(Box<Expression>),
    /// `A*`
    Repeat(Box<Expression>),
    /// `A*?`
    RepeatNonGreedy(Box<Expression>),
    /// `A+`
    RepeatPlus(Box<Expression>),
    /// `A+?`
    RepeatPlusNonGreedy(Box<Expression>),
    /// `A{2..4}`
    RepeatRange(Box<Expression>, Option<u32>, Option<u32>),
    /// `NonTerminal`
    Nt(String),
    /// `` `string` ``
    Terminal(String),
    /// `<english description>`
    Prose(String),
    /// An LF followed by the given number of spaces.
    ///
    /// Used by the renderer to help format and structure the grammar.
    Break(usize),
    /// `// Single line comment.`
    Comment(String),
    /// ``[`A`-`Z` `_` LF]``
    Charset(Vec<Characters>),
    /// ``~[` ` LF]``
    NegExpression(Box<Expression>),
    /// `^ A B C`
    Cut(Box<Expression>),
    /// `U+0060`
    Unicode(String),
}

#[derive(Clone, Debug)]
pub enum Characters {
    /// `LF`
    Named(String),
    /// `` `_` ``
    Terminal(String),
    /// `` `A`-`Z` ``
    Range(char, char),
}

impl Grammar {
    fn visit_nt(&self, callback: &mut dyn FnMut(&str)) {
        for p in self.productions.values() {
            p.expression.visit_nt(callback);
        }
    }
}

impl Expression {
    pub fn new_kind(kind: ExpressionKind) -> Self {
        Self {
            kind,
            suffix: None,
            footnote: None,
        }
    }

    fn visit_nt(&self, callback: &mut dyn FnMut(&str)) {
        match &self.kind {
            ExpressionKind::Grouped(e)
            | ExpressionKind::Optional(e)
            | ExpressionKind::Repeat(e)
            | ExpressionKind::RepeatNonGreedy(e)
            | ExpressionKind::RepeatPlus(e)
            | ExpressionKind::RepeatPlusNonGreedy(e)
            | ExpressionKind::RepeatRange(e, _, _)
            | ExpressionKind::NegExpression(e)
            | ExpressionKind::Cut(e) => {
                e.visit_nt(callback);
            }
            ExpressionKind::Alt(es) | ExpressionKind::Sequence(es) => {
                for e in es {
                    e.visit_nt(callback);
                }
            }
            ExpressionKind::Nt(nt) => {
                callback(&nt);
            }
            ExpressionKind::Terminal(_)
            | ExpressionKind::Prose(_)
            | ExpressionKind::Break(_)
            | ExpressionKind::Comment(_)
            | ExpressionKind::Unicode(_) => {}
            ExpressionKind::Charset(set) => {
                for ch in set {
                    match ch {
                        Characters::Named(s) => callback(s),
                        Characters::Terminal(_) | Characters::Range(_, _) => {}
                    }
                }
            }
        }
    }

    pub fn is_break(&self) -> bool {
        matches!(self.kind, ExpressionKind::Break(_))
    }
}

pub static GRAMMAR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?ms)^```grammar,([^\n]+)\n(.*?)^```").unwrap());

/// Loads the [`Grammar`] from the book.
pub fn load_grammar(diag: &mut Diagnostics) -> Grammar {
    let mut grammar = Grammar::default();
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src");
    for entry in WalkDir::new(&base) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let content = std::fs::read_to_string(path).unwrap();
        let relative_path = pathdiff::diff_paths(path, &base).expect("one path must be absolute");
        for cap in GRAMMAR_RE.captures_iter(&content) {
            let category = &cap[1];
            let input = &cap[2];
            if let Err(e) = parser::parse_grammar(input, &mut grammar, category, &relative_path) {
                warn_or_err!(diag, "failed to parse grammar in {path:?}: {e}");
            }
        }
    }

    check_undefined_nt(&grammar, diag);
    check_unexpected_roots(&grammar, diag);
    grammar
}

/// Checks for nonterminals that are used but not defined.
fn check_undefined_nt(grammar: &Grammar, diag: &mut Diagnostics) {
    grammar.visit_nt(&mut |nt| {
        if !grammar.productions.contains_key(nt) {
            warn_or_err!(diag, "non-terminal `{nt}` is used but not defined");
        }
    });
}

/// This checks that all the grammar roots are what we expect.
///
/// This is intended to help catch any unexpected misspellings, orphaned
/// productions, or general mistakes.
fn check_unexpected_roots(grammar: &Grammar, diag: &mut Diagnostics) {
    // `set` starts with every production name.
    let mut set: HashSet<_> = grammar.name_order.iter().map(|s| s.as_str()).collect();
    fn remove(set: &mut HashSet<&str>, grammar: &Grammar, prod: &Production, root_name: &str) {
        prod.expression.visit_nt(&mut |nt| {
            // Leave the root name in the set if we find it recursively.
            if nt == root_name {
                return;
            }
            if !set.remove(nt) {
                return;
            }
            if let Some(nt_prod) = grammar.productions.get(nt) {
                remove(set, grammar, nt_prod, root_name);
            }
        });
    }
    // Walk the productions starting from the root nodes, and remove every
    // non-terminal from `set`. What's left must be the set of roots.
    grammar
        .productions
        .values()
        .filter(|prod| prod.is_root)
        .for_each(|root| {
            remove(&mut set, grammar, root, &root.name);
        });
    let expected: HashSet<_> = grammar
        .productions
        .values()
        .filter_map(|p| p.is_root.then(|| p.name.as_str()))
        .collect();
    if set != expected {
        let new: Vec<_> = set.difference(&expected).collect();
        let removed: Vec<_> = expected.difference(&set).collect();
        if !new.is_empty() {
            warn_or_err!(
                diag,
                "New grammar production detected that is not used in any root-accessible\n\
                 production. If this is expected, mark the production with\n\
                 `@root`. If not, make sure it is spelled correctly and used in\n\
                 another root-accessible production.\n\
                 \n\
                 The new names are: {new:?}\n"
            );
        } else if !removed.is_empty() {
            warn_or_err!(
                diag,
                "Old grammar production root seems to have been removed\n\
                 (it is used in some other production that is root-accessible).\n\
                 If this is expected, remove `@root` from the production.\n\
                 \n\
                 The removed names are: {removed:?}\n"
            );
        } else {
            unreachable!("unexpected");
        }
    }
}
