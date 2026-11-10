//! Preprocessor to add chapter indices to chapter pages.

use std::env;
use std::error::Error;

use mdbook_preprocessor::errors::Error as PreprocessorError;
use mdbook_preprocessor::errors::Result as PreprocessorResult;

fn main() -> PreprocessorResult<()> {
    // Skip index 0 which is the path.
    let mut args = env::args().skip(1);
    if let Some(command) = args.next() {
        return match command.as_str() {
            "supports" => Ok(()),
            unknown => {
                let error: Box<dyn Error + Send + Sync> =
                    format!("Unknown_command: {unknown}").into();
                Err(PreprocessorError::from_boxed(error))
            }
        };
    }

    mdbook_chapter_index::process_chapter_indices()
}
