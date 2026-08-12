//! Subcommand that compares lexer tokenization between tools.
//!
//! To compare against the Reference grammar, this needs to do some
//! normalization because different tools have different ideas of exactly what
//! is a token, or the exact span of bytes of a token.
//!
//! Unfortunately this does a poor job of handling when both the Reference and
//! the tool fails to parse some input. Ideally it should compare the exact
//! error (or the span of the error), but that would be extremely difficult.

use crate::CommonOptions;
use crate::tools::{pm2, rustc};
use crate::{Message, Tool, display_line};
use clap::ArgMatches;
use diagnostics::Diagnostics;
use grammar::Grammar;
use parser::Edition;
use parser::ParseError;
use parser::coverage::Coverage;
use parser::lexer::Tokens;
use std::cell::RefCell;
use std::ops::Range;
use std::panic::AssertUnwindSafe;
use std::sync::{Arc, Mutex};
use std::time::Instant;

const DEFAULT_COMPARE_TOOLS: [Tool; 2] = [Tool::RustcParse, Tool::ProcMacro2];

thread_local! {
    static PANIC_OUTPUT: RefCell<Option<String>> = const { RefCell::new(None) };
}

pub fn compare_parallel(matches: &ArgMatches) {
    let start = Instant::now();
    let (opts, receiver) = CommonOptions::new(matches, &DEFAULT_COMPARE_TOOLS);
    if opts.tools.iter().any(|t| *t == Tool::Reference) {
        panic!("can't compare reference to itself");
    }
    if let Some(t) = opts
        .tools
        .iter()
        .find(|t| !DEFAULT_COMPARE_TOOLS.contains(t))
    {
        panic!("tool {t} is not supported for comparison");
    }

    std::panic::set_hook(Box::new(|info| {
        let payload = info.payload();
        let msg = if let Some(s) = payload.downcast_ref::<&str>() {
            s
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.as_str()
        } else {
            "Box<dyn Any>"
        };
        let location = info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown".to_string());
        let thread = std::thread::current();
        let name = thread.name().unwrap_or("<unnamed>");
        let output = format!("thread '{name}' panicked at {location}:\n{msg}");

        // We print it here as well to ensure it is seen if the thread dies unexpectedly.
        // eprintln!("{}", output);

        PANIC_OUTPUT.with(|c| {
            *c.borrow_mut() = Some(output);
        });
    }));

    let mut diag = Diagnostics::new();
    let grammar = Arc::new(grammar::load_grammar_with_frontmatter(&mut diag));
    let coverage = Arc::new(Mutex::new(Coverage::default()));

    // Spawn threads to run the tests.
    let sender = opts.channel.clone();
    let mut thread_count = opts.thread_count;
    let opts = Arc::new(Mutex::new(opts));
    for _ in 0..thread_count {
        let opts_c = opts.clone();
        let grammar = grammar.clone();
        let coverage = coverage.clone();
        std::thread::spawn(move || {
            compare_loop(opts_c, grammar, coverage);
        });
    }
    ctrlc::set_handler(move || {
        sender.send(Message::CtrlC).unwrap();
    })
    .unwrap();
    // Receive results from the threads.
    loop {
        match receiver.recv().unwrap() {
            Message::ThreadComplete => {
                thread_count -= 1;
                if thread_count == 0 {
                    break;
                }
            }
            Message::CtrlC => {
                break;
            }
        }
    }

    if opts.lock().unwrap().coverage {
        coverage.lock().unwrap().save(&grammar);
    }
    print_final_summary(&opts, start);
}

fn compare_loop(
    opts: Arc<Mutex<CommonOptions>>,
    grammar: Arc<Grammar>,
    final_coverage: Arc<Mutex<Coverage>>,
) {
    let mut coverage = Coverage::default();
    let channel = opts.lock().unwrap().channel.clone();
    let edition = opts.lock().unwrap().edition();
    loop {
        let mut opts_l = opts.lock().unwrap();
        let Some((name, src)) = opts_l.next() else {
            break;
        };
        let tools = opts_l.tools.clone();
        drop(opts_l);
        let lexer_result = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            parser::lexer::tokenize(&grammar, &mut coverage, &src)
        })) {
            Ok(r) => r,
            Err(_) => {
                let panic_msg = PANIC_OUTPUT.with(|c| {
                    c.borrow_mut()
                        .take()
                        .unwrap_or_else(|| "unknown panic".to_string())
                });
                let mut opts_l = opts.lock().unwrap();
                opts_l.errors.push(format!(
                    "test {name} for reference lexer panicked:\n{panic_msg}"
                ));
                opts_l.set_progress_err_msg();
                break;
            }
        };

        for tool in &*tools {
            match std::panic::catch_unwind(|| {
                compare_src(lexer_result.clone(), &name, &src, *tool, edition)
            }) {
                Ok(Ok(())) => {}
                Ok(Err(e)) => {
                    let mut opts_l = opts.lock().unwrap();
                    opts_l.errors.push(e);
                    opts_l.set_progress_err_msg();
                }
                Err(_) => {
                    let panic_msg = PANIC_OUTPUT.with(|c| {
                        c.borrow_mut()
                            .take()
                            .unwrap_or_else(|| "unknown panic".to_string())
                    });
                    let mut opts_l = opts.lock().unwrap();
                    opts_l.errors.push(format!(
                        "test {name} for tool {tool} panicked:\n{panic_msg}"
                    ));
                    opts_l.set_progress_err_msg();
                }
            }
            let opts_l = opts.lock().unwrap();
            opts_l.progress.inc(1);
        }
    }
    final_coverage.lock().unwrap().merge(coverage);
    channel.send(Message::ThreadComplete).unwrap();
}

fn compare_src(
    lexer_result: Result<Tokens, ParseError>,
    name: &str,
    src: &str,
    tool: Tool,
    edition: Edition,
) -> Result<(), String> {
    let (tool_result, mut lexer_result) = match tool {
        Tool::RustcParse => {
            let lexer_result = lexer_result.and_then(|ts| rustc::normalize(&ts.tokens));
            (rustc::tokenize(src, edition), lexer_result)
        }
        Tool::ProcMacro2 => {
            // Unfortunately proc-macro2 does not handle shebang or
            // frontmatter. In order to handle files with that, this replaces
            // those with whitespace in order to retain the original byte
            // positions.
            if let Err(ParseError { message, .. }) = &lexer_result
                && message.contains("invalid frontmatter")
            {
                return Ok(());
            }
            let mut stripped_src = String::from(src);
            let mut replace = |range: &Range<usize>| {
                let replacement = "\n".repeat(range.end - range.start);
                stripped_src.replace_range(range.clone(), &replacement);
            };
            if let Ok(Tokens {
                shebang: Some(shebang),
                ..
            }) = &lexer_result
            {
                replace(&shebang.range);
            }
            if let Ok(Tokens {
                frontmatter: Some(frontmatter),
                ..
            }) = &lexer_result
            {
                replace(&frontmatter.range);
            }
            let pm2_result = pm2::tokenize(&stripped_src);
            pm2::normalize(pm2_result, lexer_result, src)
        }
        _ => unreachable!(),
    };
    if let Ok(tokens) = &lexer_result
        && let Some(invalid) = tokens
            .iter()
            .find(|token| token.name == "RESERVED_TOKEN" || token.name.starts_with("INVALID_"))
    {
        lexer_result = Err(ParseError {
            byte_offset: invalid.range.start,
            message: format!("invalid token {}", invalid.name),
        });
    }

    match (lexer_result, tool_result) {
        (Ok(lex_tokens), Ok(tool_tokens)) => {
            let mut lex_iter = lex_tokens.iter();
            let mut tool_iter = tool_tokens.iter();
            loop {
                let lex_token = lex_iter.next();
                let tool_token = tool_iter.next();
                match (lex_token, tool_token) {
                    (Some(lex_token), Some(tool_token)) => {
                        let lex_text = &src[lex_token.range.clone()];
                        let tool_text = &src[tool_token.range.clone()];
                        if lex_text != tool_text || lex_token.name != tool_token.name {
                            return Err(format!(
                                "error: token mismatch\n\
                                test: {name}\n\
                                reference token: {:?} {:?} {:?}\n\
                                {}\n\
                                {tool} token: {:?} {:?} {:?}\n\
                                {}",
                                lex_token.name,
                                lex_text,
                                lex_token.range,
                                display_line(src, &lex_token.range),
                                tool_token.name,
                                tool_text,
                                tool_token.range,
                                display_line(src, &tool_token.range),
                            ));
                        }
                    }
                    (None, None) => break,
                    (Some(lex_token), None) => {
                        return Err(format!(
                            "error: reference has more tokens (compared to {tool})\n\
                            test: {name}\n\
                            reference token: {:?} {:?}\n\
                            {}",
                            lex_token.name,
                            &src[lex_token.range.clone()],
                            display_line(src, &lex_token.range),
                        ));
                    }
                    (None, Some(tool_token)) => {
                        return Err(format!(
                            "error: {tool} has more tokens (compared to reference grammar)\n\
                            test: {name}\n\
                            {tool} token: {:?} {:?}\n\
                            {}",
                            tool_token.name,
                            &src[tool_token.range.clone()],
                            display_line(src, &tool_token.range),
                        ));
                    }
                }
            }
            return Ok(());
        }
        (Err(e), Ok(_)) => {
            return Err(format!(
                "error: reference failed, {tool} passed\n\
                test: {name}\n\
                reference error: {}\n\
                {}",
                e.display(src),
                display_line(
                    src,
                    &Range {
                        start: e.byte_offset,
                        end: e.byte_offset + 1
                    }
                )
            ));
        }
        (Ok(_), Err(e)) => {
            return Err(format!(
                "error: {tool} failed, reference passed\n\
                test: {name}\n\
                {tool} error: {}\n\
                {}",
                e.display(src),
                display_line(
                    src,
                    &Range {
                        start: e.byte_offset,
                        end: e.byte_offset + 1
                    }
                )
            ));
        }
        (Err(_), Err(_)) => {
            // Unfortunately getting the error byte offsets to match between
            // the reference lexer and the tools is probably just too much
            // effort. This means that they could be reporting errors for
            // different reasons, but we wouldn't know.
            //
            // There are some substantial challenges here:
            //
            // - Cut errors can supersede previous RESERVED_ tokens, making the offset wildly different.
            // - Recovery is a problem, for tests that have several errors.
            //   Example is /rust/tests/ui/rust-2021/reserved-prefixes.rs.
            //   - rustc ParseError only includes the byte offset of the first error.
            //   - reference has no recovery.
            return Ok(());
        }
    }
}

fn print_final_summary(opts: &Arc<Mutex<CommonOptions>>, start: Instant) {
    let opts_l = opts.lock().unwrap();
    // Get the actual count of tests run from progress position.
    let actual_test_count = opts_l.progress.position() as u32;
    opts_l.progress.finish_and_clear();
    if !opts_l.errors.is_empty() {
        eprintln!("------------------------------------------------------------");
        for error in &opts_l.errors {
            eprintln!(
                "{error}\n\
                 ------------------------------------------------------------"
            );
        }
    }
    let n_errs = opts_l.errors.len() as u32;
    // Use actual test count (from progress) when test_count is 0 (spinner mode).
    let total = if opts_l.test_count == 0 {
        actual_test_count
    } else {
        opts_l.test_count
    };
    eprintln!("passed: {}", total.saturating_sub(n_errs));
    eprintln!("failed: {n_errs}");
    let elapsed = start.elapsed();
    if elapsed.as_secs() < 60 {
        eprintln!("finished in {:.1} seconds", elapsed.as_secs_f64());
    } else {
        eprintln!(
            "finished in {} minutes {} seconds",
            elapsed.as_secs() / 60,
            elapsed.as_secs() % 60
        );
    }
    if !opts_l.errors.is_empty() {
        std::process::exit(1);
    }
}
