#![feature(rustc_private)]

extern crate rustc_interface;
extern crate rustc_span;

use clap::{Command, arg};
use diagnostics::Diagnostics;
use indicatif::{ProgressBar, ProgressStyle};
use parser::Edition;
use std::cmp::min;
use std::fmt::Display;
use std::io::{IsTerminal, Read};
use std::ops::Range;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::Duration;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use walkdir::WalkDir;

mod permute;
mod test_cases;
mod commands {
    pub mod lex_compare;
    pub mod print_grammar;
    pub mod split_check;
    pub mod tokenize;
    pub mod tree;
}
mod tools {
    pub mod pm2;
    pub mod rustc;
    pub mod rustc_lexer;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tool {
    Reference,
    RustcParse,
    ProcMacro2,
    RustcLexer,
}

impl FromStr for Tool {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "reference" => Ok(Tool::Reference),
            "rustc_parse" => Ok(Tool::RustcParse),
            "proc-macro2" => Ok(Tool::ProcMacro2),
            "rustc_lexer" => Ok(Tool::RustcLexer),
            _ => Err(format!("invalid tool: {s}")),
        }
    }
}

impl Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Tool::Reference => write!(f, "reference"),
            Tool::RustcParse => write!(f, "rustc_parse"),
            Tool::ProcMacro2 => write!(f, "proc-macro2"),
            Tool::RustcLexer => write!(f, "rustc_lexer"),
        }
    }
}

enum Message {
    ThreadComplete,
    CtrlC,
}

struct CommonOptions {
    strings: Vec<(String, String)>,
    paths: Vec<PathBuf>,
    permute_iter: Option<Mutex<Box<dyn Iterator<Item = String> + Send>>>,
    tools: Arc<Vec<Tool>>,
    edition: Option<Edition>,
    coverage: bool,
    test_count: u32,
    thread_count: u32,
    errors: Vec<String>,
    progress: ProgressBar,
    channel: Sender<Message>,
    use_spinner: bool,
}

impl CommonOptions {
    fn new(
        matches: &clap::ArgMatches,
        default_tools: &[Tool],
    ) -> (CommonOptions, Receiver<Message>) {
        fn map_case(case: &String) -> Vec<(String, String)> {
            match case.as_ref() {
                "all" => test_cases::LEX_CASES
                    .iter()
                    .flat_map(|(name, cases)| {
                        cases.iter().map(|c| (name.to_string(), c.to_string()))
                    })
                    .collect(),
                case_pattern => {
                    let cs: Vec<_> = test_cases::LEX_CASES
                        .iter()
                        .filter(|(name, _)| {
                            let name_parts: Vec<_> = name.split("::").collect();
                            let pattern_parts: Vec<_> = case_pattern.split("::").collect();
                            if pattern_parts.len() > name_parts.len() {
                                return false;
                            }
                            name_parts
                                .iter()
                                .zip(pattern_parts.iter())
                                .all(|(n, p)| n == p)
                        })
                        .flat_map(|(name, cases)| {
                            cases.iter().map(|c| (name.to_string(), c.to_string()))
                        })
                        .collect();
                    if cs.is_empty() {
                        eprintln!(
                            "error: case pattern `{case_pattern}` did not match any test cases"
                        );
                        std::process::exit(1);
                    }
                    cs
                }
            }
        }
        fn map_path(path: &String) -> Result<Vec<PathBuf>, walkdir::Error> {
            WalkDir::new(path)
                .into_iter()
                .collect::<Result<Vec<_>, _>>()
                .map(|entries| {
                    entries
                        .into_iter()
                        .filter(|e| e.file_type().is_file())
                        .filter(|e| e.path().extension().map(|ext| ext == "rs").unwrap_or(false))
                        .map(|e| e.into_path())
                        .collect()
                })
        }
        let mut strings: Vec<_> = matches
            .get_many("string")
            .map(|ss| {
                ss.map(|s: &String| ("CLI string".to_string(), s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let cases: Vec<_> = matches
            .get_many("case")
            .map(|ps| ps.flat_map(map_case).collect())
            .unwrap_or_default();
        strings.extend(cases);
        if matches.get_flag("stdin") {
            let mut buffer = String::new();
            if std::io::stdin().is_terminal() {
                println!("Enter source text:");
            }
            std::io::stdin().read_to_string(&mut buffer).unwrap();
            strings.push(("stdin".to_string(), buffer));
        }
        let paths: Vec<_> = matches
            .get_many("path")
            .map(|ps| {
                ps.map(map_path)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap_or_else(|e| {
                        eprintln!("error: failed to read path: {}", e);
                        std::process::exit(1);
                    })
                    .into_iter()
                    .flatten()
                    .collect()
            })
            .unwrap_or_default();

        // Handle --permute flag to generate test cases from grammar productions.
        let permute_iter = matches.get_one::<String>("permute").map(|permute_name| {
            if permute_name == "three" {
                return Mutex::new(Box::new(permute::ThreeIterator::new())
                    as Box<dyn Iterator<Item = String> + Send>);
            }
            let mut diag = Diagnostics::new();
            let grammar = grammar::load_grammar(&mut diag);

            // Leak the grammar to get a 'static reference for the iterator
            let grammar_ref: &'static grammar::Grammar = Box::leak(Box::new(grammar));
            let production = &grammar_ref
                .productions
                .get(permute_name)
                .unwrap_or_else(|| panic!("production `{permute_name}` not found"))
                .expression;

            Mutex::new(
                Box::new(permute::PermutationIterator::new(grammar_ref, production))
                    as Box<dyn Iterator<Item = String> + Send>,
            )
        });

        let use_spinner = permute_iter.is_some();

        if strings.is_empty() && paths.is_empty() && permute_iter.is_none() {
            strings.extend(map_case(&"all".to_string()));
        }
        let tools: Vec<_> = matches
            .get_many("tool")
            .map(|ts| ts.cloned().collect())
            .unwrap_or_else(|| default_tools.to_vec());
        let tools = Arc::new(tools);
        let edition = matches
            .get_one::<String>("edition")
            .map(|e| e.parse::<Edition>().unwrap());
        for tool in &*tools {
            match (tool, edition) {
                (Tool::RustcParse, _) => {}
                (Tool::Reference, Some(_)) => panic!("reference does not yet support editions"),
                (Tool::ProcMacro2, Some(_)) => panic!("proc-macro2 does not support editions"),
                (Tool::RustcLexer, Some(_)) => panic!("rustc_lexer is edition agnostic"),
                (_, None) => {}
            }
        }
        let coverage = matches.get_flag("coverage");
        // When using permute, we don't know the total count upfront.
        let test_count = if use_spinner {
            0
        } else {
            ((strings.len() + paths.len()) as u32) * tools.len() as u32
        };
        let available_parallelism = std::thread::available_parallelism().unwrap().get() as u32;
        let thread_count = if use_spinner {
            available_parallelism
        } else {
            min(test_count.max(1), available_parallelism)
        };
        let progress = if use_spinner {
            let p = ProgressBar::new_spinner();
            p.enable_steady_tick(Duration::from_millis(100));
            p
        } else {
            let p = ProgressBar::new(test_count as u64);
            p.enable_steady_tick(Duration::from_millis(200));
            p
        };
        progress.set_message("0");
        let (channel, receiver) = channel();
        let opts = CommonOptions {
            strings,
            paths,
            permute_iter,
            tools,
            edition,
            coverage,
            test_count,
            thread_count,
            errors: Vec::new(),
            progress,
            channel,
            use_spinner,
        };
        opts.set_progress_style();
        (opts, receiver)
    }

    fn next(&mut self) -> Option<(String, String)> {
        if let Some((name, src)) = self.strings.pop() {
            return Some((name, src));
        }
        if let Some(path) = self.paths.pop() {
            // TODO: Switch path to a string, not needed as PathBuf anymore.
            let contents = std::fs::read_to_string(&path).unwrap();
            let display = format!("{}", path.display());
            return Some((display, contents));
        }
        if let Some(ref iter) = self.permute_iter {
            if let Ok(mut iter) = iter.lock() {
                if let Some(content) = iter.next() {
                    // println!("{:?}", content);
                    return Some(("permutation".to_string(), content));
                }
            }
        }
        None
    }

    fn set_progress_style(&self) {
        let color = if self.errors.is_empty() {
            "green"
        } else {
            "red"
        };
        let tick_chars = "🌑🌒🌓🌔🌕🌖🌗🌘";
        if self.use_spinner {
            self.progress.set_style(
                ProgressStyle::with_template(&format!(
                    "{{spinner:.green}} [{{elapsed_precise}}] {{pos}} tests — {{msg:.{color}}} failures"
                ))
                .unwrap()
                .tick_chars(tick_chars),
            );
        } else {
            self.progress.set_style(ProgressStyle::with_template(&format!("{{spinner:.green}} [{{elapsed_precise}}] [{{wide_bar:.blue}}] {{pos}}/{{len}} — {{msg:.{color}}} failures")).unwrap()
                .progress_chars("█▉▊▋▌▍▎▏  ")
                .tick_chars(tick_chars));
        }
    }

    fn set_progress_err_msg(&self) {
        self.progress.set_message(format!("{}", self.errors.len()));
        self.set_progress_style();
    }

    fn edition(&self) -> Edition {
        self.edition.unwrap_or(Edition::Edition2024)
    }
}

fn common_args() -> Vec<clap::Arg> {
    vec![
        arg!(--case <CASE> ... "internal test cases to compare"),
        arg!(--string <STRING> ... "source string to tokenize"),
        arg!(--path <PATH> ... "path of rust files to compare"),
        arg!(--permute <NAME> "grammar production to generate permutations for"),
        arg!(--tool <TOOLS> ... "tool to compare").value_parser(clap::value_parser!(Tool)),
        arg!(--edition <EDITION> "edition to use"),
        arg!(--coverage "record coverage data"),
        arg!(--stdin "read input from stdin"),
    ]
}

fn main() {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_env_var("GRAMMAR_LOG")
        .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_tree::HierarchicalLayer::new(2)
                .with_writer(std::io::stderr)
                .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stderr())),
        )
        .init();

    let matches = Command::new("grammar-check")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("lex-compare")
                .about("Compare tokenization between implementations")
                .args(common_args()),
        )
        .subcommand(
            Command::new("tokenize")
                .about("Convert source to tokens")
                .args(common_args()),
        )
        .subcommand(
            Command::new("tree")
                .about("Convert source to a tree")
                .arg(
                    arg!(--production <NAME> "the production name to parse").default_value("Crate"),
                )
                .args(common_args()),
        )
        .subcommand(
            Command::new("split-check")
                .about("Check for potential token splitting locations in the grammar"),
        )
        .subcommand(
            Command::new("print-grammar")
                .about("Print the grammar to stdout")
                .arg(arg!(--debug "Print using Debug format")),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("lex-compare", sub_matches)) => {
            commands::lex_compare::compare_parallel(sub_matches);
        }
        Some(("tokenize", sub_matches)) => {
            commands::tokenize::tokenize(sub_matches);
        }
        Some(("tree", sub_matches)) => {
            commands::tree::tree(sub_matches);
        }
        Some(("split-check", sub_matches)) => {
            commands::split_check::split_check(sub_matches);
        }
        Some(("print-grammar", sub_matches)) => {
            commands::print_grammar::print_grammar(sub_matches);
        }
        _ => unreachable!(),
    }
}

/// Helper to translate a byte index to a `(line, line_no, col_no)` (1-based).
fn translate_position(input: &str, index: usize) -> (&str, usize, usize) {
    if input.is_empty() {
        return ("", 0, 0);
    }
    let index = index.min(input.len());

    let mut line_start = 0;
    let mut line_number = 0;
    for line in input.lines() {
        let line_end = line_start + line.len();
        if index >= line_start && index <= line_end {
            let column_number = index - line_start + 1;
            return (line, line_number + 1, column_number);
        }
        line_start = line_end + 1;
        line_number += 1;
    }
    ("", line_number + 1, 0)
}

fn display_line(src: &str, range: &Range<usize>) -> String {
    let (line, line_no, col_no) = translate_position(src, range.start);
    let line = line.replace('\r', "␍");
    let prefix = format!("{line_no}: ");
    let indent = col_no.saturating_sub(1);
    let len = (range.end - range.start).min(line.len().saturating_sub(indent));
    let underline = format!("{}{}", " ".repeat(prefix.len() + indent), "━".repeat(len));
    format!("{prefix}{line}\n{underline}\n")
}
