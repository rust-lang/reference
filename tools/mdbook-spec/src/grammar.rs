//! Support for rendering the grammar.

use diagnostics::{Diagnostics, warn_or_err};
use grammar::{GRAMMAR_RE, Grammar};
use mdbook_preprocessor::book::Chapter;
use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::sync::LazyLock;

mod render_markdown;
mod render_railroad;

static NAMES_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^(?:@root )?([A-Za-z0-9_]+)(?: \([^)]+\))? ->").unwrap());

#[derive(Debug)]
pub struct RenderCtx {
    md_link_map: HashMap<String, String>,
    rr_link_map: HashMap<String, String>,
    for_summary: bool,
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

    if let Err(e) = render_markdown::render_markdown(grammar, &render_ctx, &names, &mut output) {
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

    if let Err(e) = render_railroad::render_railroad(grammar, &render_ctx, &names, &mut output) {
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
