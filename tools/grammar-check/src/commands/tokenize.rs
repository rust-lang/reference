//! A subcommand that converts input to human-readable sequence of tokens.

use crate::tools::{pm2, rustc, rustc_lexer};
use crate::{CommonOptions, Tool, display_line};
use clap::ArgMatches;
use diagnostics::Diagnostics;
use parser::Edition;
use parser::coverage::Coverage;
use parser::lexer::Tokens;
use std::ops::Range;

pub fn tokenize(matches: &ArgMatches) {
    let (mut opts, _) = CommonOptions::new(matches, &[Tool::Reference]);
    opts.progress.finish_and_clear();
    for tool in &*opts.tools.clone() {
        while let Some((name, src)) = opts.next() {
            println!("------------------------------------------------------------");
            println!("tool `{tool}` token results for `{name}`:");
            tokenize_src(&src, *tool, opts.edition());
            println!("------------------------------------------------------------");
        }
    }
}

fn tokenize_src(src: &str, tool: Tool, edition: Edition) {
    let tokens = match tool {
        Tool::Reference => {
            let mut diag = Diagnostics::new();
            let grammar = grammar::load_grammar_with_frontmatter(&mut diag);
            let mut coverage = Coverage::default();
            let tokens = parser::lexer::tokenize(&grammar, &mut coverage, src);
            if let Ok(Tokens {
                shebang: Some(shebang),
                ..
            }) = &tokens
            {
                println!("Shebang in range: {:?}", shebang);
            }
            if let Ok(Tokens {
                frontmatter: Some(frontmatter),
                ..
            }) = &tokens
            {
                println!("Frontmatter in range: {:?}", frontmatter);
            }
            tokens.map(|ts| ts.tokens)
        }
        Tool::RustcParse => rustc::tokenize(src, edition),
        Tool::ProcMacro2 => pm2::tokenize(src),
        Tool::RustcLexer => rustc_lexer::tokenize(src),
    };
    let tokens = match tokens {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!(
                "error: {}\n\
                {}",
                e.message,
                display_line(
                    src,
                    &Range {
                        start: e.byte_offset,
                        end: e.byte_offset + 1
                    }
                )
            );
            return;
        }
    };
    for token in tokens {
        println!("{:?}: {}", &src[token.range], token.name);
    }
}
