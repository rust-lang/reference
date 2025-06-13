//! Support for rendering the grammar.

use crate::{Diagnostics, warn_or_err};
use mdbook::book::{Book, BookItem, Chapter};
use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::path::PathBuf;
use std::sync::LazyLock;

mod parser;
mod render_markdown;
mod render_railroad;

#[derive(Debug, Default)]
pub struct Grammar {
    pub productions: HashMap<String, Production>,
    /// The order that the production names were discovered.
    pub name_order: Vec<String>,
}

#[derive(Debug)]
pub struct Production {
    name: String,
    /// Category is from the markdown lang string, and defines how it is
    /// grouped and organized on the summary page.
    category: String,
    expression: Expression,
    /// The path to the chapter where this is defined.
    path: PathBuf,
    is_root: bool,
}

#[derive(Clone, Debug)]
struct Expression {
    kind: ExpressionKind,
    /// Suffix is the `_foo_` part that is shown as a subscript.
    suffix: Option<String>,
    /// A footnote is a markdown footnote link.
    footnote: Option<String>,
}

#[derive(Clone, Debug)]
enum ExpressionKind {
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
    /// ``[`A`-`Z` `_` LF]``
    Charset(Vec<Characters>),
    /// ``~[` ` LF]``
    NegExpression(Box<Expression>),
    /// `U+0060`
    Unicode(String),
}

#[derive(Clone, Debug)]
enum Characters {
    /// `LF`
    Named(String),
    /// `` `_` ``
    Terminal(String),
    /// `` `A`-`Z` ``
    Range(char, char),
}

#[derive(Debug)]
pub struct RenderCtx {
    md_link_map: HashMap<String, String>,
    rr_link_map: HashMap<String, String>,
    for_summary: bool,
}

impl Grammar {
    fn visit_nt(&self, callback: &mut dyn FnMut(&str)) {
        for p in self.productions.values() {
            p.expression.visit_nt(callback);
        }
    }
}

impl Expression {
    fn new_kind(kind: ExpressionKind) -> Self {
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
            | ExpressionKind::NegExpression(e) => {
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

    fn is_break(&self) -> bool {
        matches!(self.kind, ExpressionKind::Break(_))
    }
}

static GRAMMAR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?ms)^```grammar,([^\n]+)\n(.*?)^```").unwrap());
static NAMES_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^(?:@root )?([A-Za-z0-9_]+)(?: \([^)]+\))? ->").unwrap());

/// Loads the [`Grammar`] from the book.
pub fn load_grammar(book: &Book, diag: &mut Diagnostics) -> Grammar {
    let mut grammar = Grammar::default();
    for item in book.iter() {
        let BookItem::Chapter(ch) = item else {
            continue;
        };
        if ch.is_draft_chapter() {
            continue;
        }
        let path = ch.path.as_ref().unwrap().to_owned();
        for cap in GRAMMAR_RE.captures_iter(&ch.content) {
            let category = &cap[1];
            let input = &cap[2];
            if let Err(e) = parser::parse_grammar(input, &mut grammar, category, &path) {
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

/// Replaces the text grammar in the given chapter with the rendered version.
pub fn insert_grammar(grammar: &Grammar, chapter: &Chapter, diag: &mut Diagnostics) -> String {
    let link_map = make_relative_link_map(grammar, chapter);

    let mut content = GRAMMAR_RE
        .replace_all(&chapter.content, |cap: &Captures<'_>| {
            let names: Vec<_> = NAMES_RE
                .captures_iter(&cap[2])
                .map(|cap| cap.get(1).unwrap().as_str())
                .collect();
            let for_lexer = &cap[1] == "lexer";
            render_names(grammar, &names, &link_map, for_lexer, chapter, diag)
        })
        .to_string();

    // Make all production names easily linkable.
    let is_summary = is_summary(chapter);
    for (name, path) in &link_map {
        let id = render_markdown::markdown_id(name, is_summary);
        if is_summary {
            // On the summary page, link to the production on the summary page.
            writeln!(content, "[{name}]: #{id}").unwrap();
        } else {
            // This includes two variants, one for convenience (like
            // `[ArrayExpression]`), and one with the `grammar-` prefix to
            // disambiguate links that have the same name as a rule (rules
            // take precedence).
            writeln!(
                content,
                "[{name}]: {path}#{id}\n\
                 [grammar-{name}]: {path}#{id}"
            )
            .unwrap();
        }
    }
    content
}

/// Creates a map of production name -> relative link path.
fn make_relative_link_map(grammar: &Grammar, chapter: &Chapter) -> HashMap<String, String> {
    let current_path = chapter.path.as_ref().unwrap().parent().unwrap();
    grammar
        .productions
        .values()
        .map(|p| {
            let relative = pathdiff::diff_paths(&p.path, current_path).unwrap();
            // Adjust paths for Windows.
            let relative = relative.display().to_string().replace('\\', "/");
            (p.name.clone(), relative)
        })
        .collect()
}

/// Helper to take a list of production names and to render all of those to a
/// mixture of markdown and HTML.
fn render_names(
    grammar: &Grammar,
    names: &[&str],
    link_map: &HashMap<String, String>,
    for_lexer: bool,
    chapter: &Chapter,
    diag: &mut Diagnostics,
) -> String {
    let for_summary = is_summary(chapter);
    let mut output = String::new();
    output.push_str(
        "<div class=\"grammar-container\">\n\
         \n",
    );
    if for_lexer {
        output.push_str("**<sup>Lexer</sup>**\n");
    } else {
        output.push_str("**<sup>Syntax</sup>**\n");
    }
    output.push_str("<br>\n");

    // Convert the link map to add the id.
    let update_link_map = |get_id: fn(&str, bool) -> String| -> HashMap<String, String> {
        link_map
            .iter()
            .map(|(name, path)| {
                let id = get_id(name, for_summary);
                let path = if for_summary {
                    format!("#{id}")
                } else {
                    format!("{path}#{id}")
                };
                (name.clone(), path)
            })
            .collect()
    };

    let render_ctx = RenderCtx {
        md_link_map: update_link_map(render_markdown::markdown_id),
        rr_link_map: update_link_map(render_railroad::railroad_id),
        for_summary,
    };

    if let Err(e) = grammar.render_markdown(&render_ctx, &names, &mut output) {
        warn_or_err!(
            diag,
            "grammar failed in chapter {:?}: {e}",
            chapter.source_path.as_ref().unwrap()
        );
    }

    output.push_str(
        "\n\
         <button class=\"grammar-toggle-railroad\" type=\"button\" \
            title=\"Toggle railroad display\" \
            onclick=\"toggle_railroad()\">\
            Show Railroad\
         </button>\n\
         </div>\n\
         <div class=\"grammar-railroad grammar-hidden\">\n\
         \n",
    );

    if let Err(e) = grammar.render_railroad(&render_ctx, &names, &mut output) {
        warn_or_err!(
            diag,
            "grammar failed in chapter {:?}: {e}",
            chapter.source_path.as_ref().unwrap()
        );
    }

    output.push_str("</div>\n");

    output
}

pub fn is_summary(chapter: &Chapter) -> bool {
    chapter.name == "Grammar summary"
}

/// Inserts the summary of all grammar rules into the grammar summary chapter.
pub fn insert_summary(grammar: &Grammar, chapter: &Chapter, diag: &mut Diagnostics) -> String {
    let link_map = make_relative_link_map(grammar, chapter);
    let mut seen = HashSet::new();
    let categories: Vec<_> = grammar
        .name_order
        .iter()
        .map(|name| &grammar.productions[name].category)
        .filter(|cat| seen.insert(*cat))
        .collect();
    let mut grammar_summary = String::new();
    for category in categories {
        let mut chars = category.chars();
        let cap = chars.next().unwrap().to_uppercase().collect::<String>() + chars.as_str();
        write!(grammar_summary, "\n## {cap} summary\n\n").unwrap();
        let names: Vec<_> = grammar
            .name_order
            .iter()
            .filter(|name| grammar.productions[*name].category == *category)
            .map(|s| s.as_str())
            .collect();
        let for_lexer = category == "lexer";
        let s = render_names(grammar, &names, &link_map, for_lexer, chapter, diag);
        grammar_summary.push_str(&s);
    }

    chapter
        .content
        .replace("{{ grammar-summary }}", &grammar_summary)
}
