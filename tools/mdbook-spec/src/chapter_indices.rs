//! Preprocessor logic to add chapter indices to chapter pages.

use std::path::{Path, PathBuf};

use mdbook_preprocessor::book::{BookItem, Chapter};
use mdbook_preprocessor::config::Config;

use pathdiff::diff_paths;

/// Marker to look for and replace in chapter pages.
const ADD_INDEX_MARKER: &str = "<!-- mdbook-chapter-index:add-index -->";

/// A page in a chapter, which chapter it is in should be tracked separately.
struct ChapterPage {
    /// Chapter name, to display, including number if applicable.
    name: String,
    /// Chapter path, to link.
    path: PathBuf,
}

/// Overall hierarchy of chapters.
enum ChapterHierarchy {
    /// This chapter has no sub-chapters.
    Leaf(ChapterPage),
    /// This chapter has sub-chapters.
    Nested(ChapterPage, Vec<ChapterHierarchy>),
}

impl ChapterHierarchy {
    /// Convert an mdbook chapter into its hierarchy, ignoring draft chapters.
    fn from_mdbook_chapter(chapter: &Chapter) -> Option<Self> {
        let Some(ref path) = chapter.path else {
            // This is a draft chapter.
            return None;
        };
        let current_page = ChapterPage {
            name: chapter.to_string(),
            path: path.clone(),
        };
        let inner: Vec<ChapterHierarchy> = chapter
            .sub_items
            .iter()
            .filter_map(|item| match item {
                BookItem::Chapter(inner_chapter) => Some(inner_chapter),
                _ => None,
            })
            .filter_map(ChapterHierarchy::from_mdbook_chapter)
            .collect();
        if inner.is_empty() {
            Some(ChapterHierarchy::Leaf(current_page))
        } else {
            Some(ChapterHierarchy::Nested(current_page, inner))
        }
    }

    /// Get the markdown to represent the current hierarchy.
    ///
    /// The markdown list is nested four spaces for each level of `nesting`.
    ///
    /// Links to chapters are created relative to the provided `root`.
    fn to_markdown_links(&self, nesting: usize, root: &Path) -> String {
        let current_indent = "    ".repeat(nesting);
        match self {
            ChapterHierarchy::Leaf(current_page) => {
                let relative_path =
                    diff_paths(&current_page.path, root).expect("diff_paths() should succeed");
                format!(
                    "{current_indent}* [{display}]({link})\n",
                    display = current_page.name,
                    link = relative_path.display()
                )
            }
            ChapterHierarchy::Nested(current_page, inner) => {
                let relative_path =
                    diff_paths(&current_page.path, root).expect("diff_paths() should succeed");
                let inner_display: String = inner
                    .iter()
                    .map(|hierarchy| hierarchy.to_markdown_links(nesting + 1, root))
                    .collect();
                format!(
                    "{current_indent}* [{display}]({link}):\n{inner_display}",
                    display = current_page.name,
                    link = relative_path.display()
                )
            }
        }
    }
}

/// Check the configuration for a custom header to be configured for the chapter
/// index.
///
/// The `headings` configuration can be:
/// - A string to use for all headings.
/// - A table mapping chapter names to the string to use.
///
/// Anything else is ignored and results in the default.
fn get_configured_header(config: &Config, chapter_name: &String) -> Option<String> {
    let value: toml::Value = config.get("preprocessor.chapter-index.headings").ok()??;
    match value {
        toml::Value::String(fixed_header) => Some(fixed_header),
        toml::Value::Table(custom_headers) => match custom_headers.get(chapter_name)? {
            toml::Value::String(chapter_header) => Some(chapter_header.to_owned()),
            _ => None,
        },
        _ => None,
    }
}

/// Implementation of the preprocessor.
#[derive(Default, Copy, Clone, Debug)]
#[non_exhaustive]
pub struct ChapterIndices;

impl ChapterIndices {
    /// Handle a single chapter in the book, updating its content if requested.
    pub fn process_chapter(config: &Config, chapter: &mut Chapter) {
        let Some(ref root_path) = chapter.path else {
            // Draft chapter.
            return;
        };
        let root_directory = root_path
            .parent()
            .expect("Root path should have a parent directory");
        if !chapter.content.contains(ADD_INDEX_MARKER) {
            return;
        }
        let mut index: String = chapter
            .sub_items
            .iter()
            .filter_map(|item| match item {
                BookItem::Chapter(inner_chapter) => Some(inner_chapter),
                _ => None,
            })
            .filter_map(ChapterHierarchy::from_mdbook_chapter)
            .map(|hierarchy| hierarchy.to_markdown_links(0, root_directory))
            .collect();
        if !index.is_empty() {
            // Support for custom headings for different pages.
            let index_header = match get_configured_header(config, &chapter.name) {
                Some(header) => header,
                None => "Chapter contents".to_owned(),
            };
            index = format!("{index_header}:\n{index}");
        }
        chapter.content = chapter.content.replace(ADD_INDEX_MARKER, &index);
    }
}
