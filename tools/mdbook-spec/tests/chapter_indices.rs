//! Usage tests for the chapter indices code.

use mdbook_preprocessor::book::{Book, BookItem, Chapter};
use mdbook_preprocessor::config::Config;
use mdbook_spec::chapter_indices::ChapterIndices;

macro_rules! make_config {
    ( $( $key:literal => $value:literal ),* $(,)? ) => {{
        let mut config = Config::default();
        config.set("preprocessor.chapter-index", "{}")
            .expect("Able to set preprocessor.chapter-index");
        $(
            config.set($key, $value).expect(concat!("Able to set ", $key));
        )*
        config
    }}
}

macro_rules! make_chapter {
    ($name:literal, $content:literal, $parents:tt) => {
        make_chapter!($name, $content, path = concat!("/path/to/book/", $name, ".md"), $parents)
    };
    ($name:literal, $content:literal, path = $path:expr, [ $( $parent:literal ),* ]) => {
        Chapter::new(
            $name,
            $content.to_owned(),
            $path,
            vec![
                $( $parent.to_owned(), )*
            ]
        )
    };
}

fn check_preprocessor(config: &Config, original: Vec<Chapter>, expected: Vec<Chapter>) {
    let updated_book = Book::new_with_items(
        original
            .into_iter()
            .map(|mut ch| {
                ChapterIndices::process_chapter(config, &mut ch);
                BookItem::Chapter(ch)
            })
            .collect(),
    );
    let expected_book = Book::new_with_items(expected.into_iter().map(BookItem::Chapter).collect());
    assert_eq!(expected_book, updated_book, "Books should match");
}

#[test]
fn no_change() {
    let config = make_config!();
    let chapters = vec![make_chapter!("MAIN_PAGE", "# Main page", [])];
    // The chapter doesn't have a <!-- mdbook-spec:add-chapter-index -->
    // directive, so there shouldn't be any changes to the output.
    check_preprocessor(&config, chapters.clone(), chapters);
}

#[test]
fn add_index() {
    // Set up config (default):
    let config = make_config!();
    // Set up input (one chapter with an index and two nested chapters):
    let mut examples = make_chapter!(
        "Examples",
        "# Examples\n\n<!-- mdbook-spec:add-chapter-index -->",
        []
    );
    examples.sub_items = vec![
        BookItem::Chapter(make_chapter!("Foo", "# Foo", ["Examples"])),
        BookItem::Chapter(make_chapter!("Bar", "# Bar", ["Examples"])),
    ];
    // Set up expected output (the main chapter now has an index):
    let mut expected = examples.clone();
    expected.content = concat!(
        "# Examples\n\n",
        "Chapter contents:\n",
        "* [Foo](Foo.md)\n",
        "* [Bar](Bar.md)\n"
    )
    .to_owned();
    // Run the checks:
    check_preprocessor(&config, vec![examples], vec![expected]);
}

#[test]
fn skip_draft() {
    // Set up config (default):
    let config = make_config!();
    // Set up input (one chapter with an index and nested chapters, including a draft):
    let mut examples = make_chapter!(
        "Examples",
        "# Examples\n\n<!-- mdbook-spec:add-chapter-index -->",
        []
    );
    examples.sub_items = vec![
        BookItem::Chapter(make_chapter!("Foo", "# Foo", ["Examples"])),
        BookItem::Chapter(make_chapter!("Bar", "# Bar", ["Examples"])),
        BookItem::Chapter(Chapter::new_draft("Baz", vec!["Examples".to_owned()])),
    ];
    // Set up expected output (the main chapter now has an index, which excludes the draft):
    let mut expected = examples.clone();
    expected.content = concat!(
        "# Examples\n\n",
        "Chapter contents:\n",
        "* [Foo](Foo.md)\n",
        "* [Bar](Bar.md)\n"
    )
    .to_owned();
    // Run the checks:
    check_preprocessor(&config, vec![examples], vec![expected]);
}

#[test]
fn only_drafts() {
    // Set up config (default):
    let config = make_config!();
    // Set up input (one chapter with an index and nested chapters, all of which are drafts):
    let mut examples = make_chapter!(
        "Examples",
        "# Examples\n\n<!-- mdbook-spec:add-chapter-index -->",
        []
    );
    examples.sub_items = vec![
        BookItem::Chapter(Chapter::new_draft("Foo", vec!["Examples".to_owned()])),
        BookItem::Chapter(Chapter::new_draft("Bar", vec!["Examples".to_owned()])),
        BookItem::Chapter(Chapter::new_draft("Baz", vec!["Examples".to_owned()])),
    ];
    // Set up expected output (the main chapter has no index because it only contains drafts):
    let mut expected = examples.clone();
    expected.content = "# Examples\n\n".to_owned();
    // Run the checks:
    check_preprocessor(&config, vec![examples], vec![expected]);
}

#[test]
fn nested_chapters() {
    // Set up config (default):
    let config = make_config!();
    // Set up input (one chapter with an index and three nested chapters, one of
    // which has nested chapters of its own):
    let mut numbers = make_chapter!("Numbers", "# Numbers", ["Examples"]);
    numbers.sub_items = vec![
        BookItem::Chapter(make_chapter!("One", "# One", ["Examples", "Numbers"])),
        BookItem::Chapter(make_chapter!("Two", "# Two", ["Examples", "Numbers"])),
    ];
    let mut examples = make_chapter!(
        "Examples",
        "# Examples\n\n<!-- mdbook-spec:add-chapter-index -->",
        []
    );
    examples.sub_items = vec![
        BookItem::Chapter(make_chapter!("Foo", "# Foo", ["Examples"])),
        BookItem::Chapter(make_chapter!("Bar", "# Bar", ["Examples"])),
        BookItem::Chapter(numbers),
    ];
    // Set up expected output (the main chapter has an index that properly reflects
    // the hierarchy of nested chapters):
    let mut expected = examples.clone();
    expected.content = concat!(
        "# Examples\n\n",
        "Chapter contents:\n",
        "* [Foo](Foo.md)\n",
        "* [Bar](Bar.md)\n",
        "* [Numbers](Numbers.md):\n",
        "    * [One](One.md)\n",
        "    * [Two](Two.md)\n",
    )
    .to_owned();
    // Run the checks:
    check_preprocessor(&config, vec![examples], vec![expected]);
}

#[test]
fn weird_paths() {
    // Set up config (default):
    let config = make_config!();
    // Set up input (one chapter with an index and two nested chapters, with
    // more complicated paths to the chapter pages):
    let mut examples = make_chapter!(
        "Examples",
        "# Examples\n\n<!-- mdbook-spec:add-chapter-index -->",
        []
    );
    examples.sub_items = vec![
        BookItem::Chapter(make_chapter!(
            "Foo",
            "# Foo",
            path = "/some/absolute/path/Foo.md",
            ["Examples"]
        )),
        BookItem::Chapter(make_chapter!(
            "Bar",
            "# Bar",
            path = "/path/to/next-to-book/Bar.md",
            ["Examples"]
        )),
    ];
    // Set up expected output (the main chapter has an index that properly links
    // to the nested chapters):
    let mut expected = examples.clone();
    expected.content = concat!(
        "# Examples\n\n",
        "Chapter contents:\n",
        "* [Foo](../../../some/absolute/path/Foo.md)\n",
        "* [Bar](../next-to-book/Bar.md)\n",
    )
    .to_owned();
    // Run the checks:
    check_preprocessor(&config, vec![examples], vec![expected]);
}

#[test]
fn custom_header() {
    // Set up config (same custom heading for all indices):
    let config = make_config!("preprocessor.chapter-index.headings" => "Example pages");
    // Set up input (one chapter with an index and two nested chapters):
    let mut examples = make_chapter!(
        "Examples",
        "# Examples\n\n<!-- mdbook-spec:add-chapter-index -->",
        []
    );
    examples.sub_items = vec![
        BookItem::Chapter(make_chapter!("Foo", "# Foo", ["Examples"])),
        BookItem::Chapter(make_chapter!("Bar", "# Bar", ["Examples"])),
    ];
    // Set up expected output (the main chapter has an index that has the configured
    // heading):
    let mut expected = examples.clone();
    expected.content = concat!(
        "# Examples\n\n",
        "Example pages:\n",
        "* [Foo](Foo.md)\n",
        "* [Bar](Bar.md)\n",
    )
    .to_owned();
    // Run the checks:
    check_preprocessor(&config, vec![examples], vec![expected]);
}

#[test]
fn different_headers() {
    // Set up config (different custom headings for different indices):
    let config = make_config!(
        "preprocessor.chapter-index.headings" => "{}",
        "preprocessor.chapter-index.headings.Examples" => "Example pages",
        "preprocessor.chapter-index.headings.Numbers" => "Some numbers",
    );
    // Set up input (two chapter with indices, each with two nested chapters):
    let mut examples = make_chapter!(
        "Examples",
        "# Examples\n\n<!-- mdbook-spec:add-chapter-index -->",
        []
    );
    examples.sub_items = vec![
        BookItem::Chapter(make_chapter!("Foo", "# Foo", ["Examples"])),
        BookItem::Chapter(make_chapter!("Bar", "# Bar", ["Examples"])),
    ];
    let mut numbers = make_chapter!(
        "Numbers",
        "# Numbers\n\n<!-- mdbook-spec:add-chapter-index -->",
        []
    );
    numbers.sub_items = vec![
        BookItem::Chapter(make_chapter!("One", "# One", ["Numbers"])),
        BookItem::Chapter(make_chapter!("Two", "# Two", ["Numbers"])),
    ];
    // Set up expected output (the chapters have indices with their respective
    // configured headings):
    let mut expected_examples = examples.clone();
    expected_examples.content = concat!(
        "# Examples\n\n",
        "Example pages:\n",
        "* [Foo](Foo.md)\n",
        "* [Bar](Bar.md)\n",
    )
    .to_owned();
    let mut expected_numbers = numbers.clone();
    expected_numbers.content = concat!(
        "# Numbers\n\n",
        "Some numbers:\n",
        "* [One](One.md)\n",
        "* [Two](Two.md)\n",
    )
    .to_owned();
    // Run the checks:
    check_preprocessor(
        &config,
        vec![examples, numbers],
        vec![expected_examples, expected_numbers],
    );
}
