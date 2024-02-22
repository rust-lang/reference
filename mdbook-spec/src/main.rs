use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use regex::{Captures, Regex};
use semver::{Version, VersionReq};
use std::collections::BTreeMap;
use std::io;
use std::path::PathBuf;
use std::process;

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            // Supports all renderers.
            return;
        }
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

    let preprocessor = Spec::new();

    if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
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

struct Spec {
    deny_warnings: bool,
    rule_re: Regex,
    admonition_re: Regex,
}

impl Spec {
    pub fn new() -> Spec {
        Spec {
            deny_warnings: std::env::var("SPEC_DENY_WARNINGS").as_deref() == Ok("1"),
            rule_re: Regex::new(r"(?m)^r\[([^]]+)]$").unwrap(),
            admonition_re: Regex::new(
                r"(?m)^ *> \[!(?<admon>[^]]+)\]\n(?<blockquote>(?: *> .*\n)+)",
            )
            .unwrap(),
        }
    }

    /// Converts lines that start with `r[…]` into a "rule" which has special
    /// styling and can be linked to.
    fn rule_definitions(
        &self,
        chapter: &Chapter,
        found_rules: &mut BTreeMap<String, (PathBuf, PathBuf)>,
    ) -> String {
        let source_path = chapter.source_path.clone().unwrap_or_default();
        let path = chapter.path.clone().unwrap_or_default();
        self.rule_re
            .replace_all(&chapter.content, |caps: &Captures| {
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
                    "<div class=\"rule\" id=\"{rule_id}\">\
                     <a class=\"rule-link\" href=\"#{rule_id}\">[{rule_id}]</a>\
                     </div>\n"
                )
            })
            .to_string()
    }

    /// Generates link references to all rules on all pages, so you can easily
    /// refer to rules anywhere in the book.
    fn auto_link_references(
        &self,
        chapter: &Chapter,
        found_rules: &BTreeMap<String, (PathBuf, PathBuf)>,
    ) -> String {
        let current_path = chapter.path.as_ref().unwrap().parent().unwrap();
        let definitions: String = found_rules
            .iter()
            .map(|(rule_id, (_, path))| {
                let relative = pathdiff::diff_paths(path, current_path).unwrap();
                format!("[{rule_id}]: {}#{rule_id}\n", relative.display())
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
    /// ```
    /// > [!WARNING]
    /// > ...
    /// ```
    ///
    /// This will add a `<div class="warning">` around the blockquote so that
    /// it can be styled differently. Any text between the brackets that can
    /// be a CSS class is valid. The actual styling needs to be added in a CSS
    /// file.
    fn admonitions(&self, chapter: &Chapter) -> String {
        self.admonition_re
            .replace_all(&chapter.content, |caps: &Captures| {
                let lower = caps["admon"].to_lowercase();
                format!(
                    "<div class=\"{lower}\">\n\n{}\n\n</div>\n",
                    &caps["blockquote"]
                )
            })
            .to_string()
    }
}

impl Preprocessor for Spec {
    fn name(&self) -> &str {
        "nop-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut found_rules = BTreeMap::new();
        for section in &mut book.sections {
            let BookItem::Chapter(ch) = section else {
                continue;
            };
            if ch.is_draft_chapter() {
                continue;
            }
            ch.content = self.rule_definitions(&ch, &mut found_rules);
            ch.content = self.admonitions(&ch);
        }
        for section in &mut book.sections {
            let BookItem::Chapter(ch) = section else {
                continue;
            };
            if ch.is_draft_chapter() {
                continue;
            }
            ch.content = self.auto_link_references(&ch, &found_rules);
        }

        Ok(book)
    }
}
