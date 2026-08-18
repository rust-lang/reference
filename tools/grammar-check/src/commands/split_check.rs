//! Experimental subcommand to identify token-splitting locations.
//!
//! This tries to find where multi-character tokens might be candidates to be
//! split into smaller tokens. This has a fairly high false-positive rate, so
//! it can take some manual effort to analyze.
//!
//! The current analysis of places where tokens are split are exhaustively
//! listed in https://github.com/rust-lang/rust/issues/152398. That issue also
//! highlights situations where rustc fails to split tokens (since it has to
//! do it manually). This highlights a situation where it will be difficult to
//! align the reference grammar with rustc, particularly when doing
//! permutation tests.

use clap::ArgMatches;
use diagnostics::Diagnostics;
use grammar::{Expression, ExpressionKind, Grammar};
use std::collections::{HashMap, HashSet};

// Multi-character tokens that may need to be split
const MULTI_CHAR_TOKENS: &[&str] = &[
    "...", "..=", "<<=", ">>=", "!=", "%=", "&&", "&=", "*=", "+=", "-=", "->", "..", "/=", "::",
    "<-", "<<", "<=", "==", "=>", ">=", ">>", "^=", "|=", "||",
];

pub fn split_check(_matches: &ArgMatches) {
    let mut diag = Diagnostics::new();
    let grammar = grammar::load_grammar(&mut diag);

    println!("Checking grammar for potential token splitting locations...\n");

    // Map to store all locations where token splitting may be necessary
    // Key: token, Value: list of (production_name, context)
    let mut split_locations: HashMap<&str, Vec<(String, String)>> = HashMap::new();

    // Check each production
    for (prod_name, production) in &grammar.productions {
        let mut locations_in_prod = Vec::new();
        let mut visited = HashSet::new();
        find_split_locations(
            &grammar,
            &production.expression,
            &mut locations_in_prod,
            prod_name,
            &mut visited,
        );

        for (token, context) in locations_in_prod {
            split_locations
                .entry(token)
                .or_insert_with(Vec::new)
                .push((prod_name.clone(), context));
        }
    }

    // Print results grouped by token
    if split_locations.is_empty() {
        println!("No potential token splitting locations found.");
    } else {
        for token in MULTI_CHAR_TOKENS {
            if let Some(locations) = split_locations.get(token) {
                println!("Token: `{}`", token);
                println!("  Locations: {}", locations.len());
                for (prod_name, context) in locations {
                    println!("    - {}: {}", prod_name, context);
                }
                println!();
            }
        }
    }
}

fn find_split_locations<'a>(
    grammar: &'a Grammar,
    expr: &'a Expression,
    locations: &mut Vec<(&'a str, String)>,
    current_production: &str,
    visited: &mut HashSet<String>,
) {
    match &expr.kind {
        ExpressionKind::Grouped(e) => {
            find_split_locations(grammar, e, locations, current_production, visited);
        }
        ExpressionKind::Alt(es) => {
            for e in es {
                find_split_locations(grammar, e, locations, current_production, visited);
            }
        }
        ExpressionKind::Sequence(es) => {
            // Check for adjacent elements that might require token splitting
            for (i, e) in es.iter().enumerate() {
                find_split_locations(grammar, e, locations, current_production, visited);

                // Check if this element could combine with following elements
                // Skip non-token-producing elements (Break, Comment) when looking for the next element
                if !matches!(
                    e.kind,
                    ExpressionKind::Break(_) | ExpressionKind::Comment(_)
                ) {
                    // Find the next token-producing element
                    for j in (i + 1)..es.len() {
                        let next = &es[j];
                        if !matches!(
                            next.kind,
                            ExpressionKind::Break(_) | ExpressionKind::Comment(_)
                        ) {
                            check_adjacent_for_splits(
                                grammar,
                                e,
                                next,
                                locations,
                                current_production,
                            );
                            break; // Only check the immediate next token-producing element
                        }
                    }
                }
            }
        }
        ExpressionKind::Optional(e)
        | ExpressionKind::NegativeLookahead(e)
        | ExpressionKind::NegExpression(e)
        | ExpressionKind::Cut(e) => {
            find_split_locations(grammar, e, locations, current_production, visited);
        }
        ExpressionKind::Repeat(e) | ExpressionKind::RepeatPlus(e) => {
            find_split_locations(grammar, e, locations, current_production, visited);
            // Check if repeating this element could create a multi-char token
            check_repeat_for_splits(grammar, e, locations, current_production, "repeat");
        }
        ExpressionKind::RepeatRange { expr: e, .. } | ExpressionKind::RepeatRangeNamed(e, _) => {
            find_split_locations(grammar, e, locations, current_production, visited);
            check_repeat_for_splits(grammar, e, locations, current_production, "repeat range");
        }
        ExpressionKind::Nt(_nt) => {
            // Don't recurse into nonterminals - we only want to find direct uses
            // and adjacent elements within the current production level.
            // The main loop in split_check already visits each production.
        }
        ExpressionKind::Terminal(term) => {
            // Check if this terminal is a multi-char token
            for &multi_token in MULTI_CHAR_TOKENS {
                if term == multi_token {
                    locations.push((
                        multi_token,
                        format!("direct use of terminal `{}`", multi_token),
                    ));
                }
            }
        }
        ExpressionKind::Prose(_)
        | ExpressionKind::Break(_)
        | ExpressionKind::Comment(_)
        | ExpressionKind::Charset(_)
        | ExpressionKind::CharacterRange(..)
        | ExpressionKind::Unicode(_) => {
            // These don't contribute to token splitting
        }
    }
}

fn describe_expression(expr: &Expression) -> String {
    match &expr.kind {
        ExpressionKind::Nt(nt) => nt.clone(),
        ExpressionKind::Terminal(t) => format!("terminal `{}`", t),
        ExpressionKind::Optional(e) => format!("optional {}", describe_expression(e)),
        ExpressionKind::Grouped(e) => format!("grouped {}", describe_expression(e)),
        ExpressionKind::Repeat(e) => format!("{} repeated", describe_expression(e)),
        ExpressionKind::RepeatPlus(e) => format!("{} repeated (+)", describe_expression(e)),
        ExpressionKind::Alt(_) => "alternative".to_string(),
        ExpressionKind::Sequence(_) => "sequence".to_string(),
        ExpressionKind::Prose(p) => format!("<{}>", p),
        _ => "expression".to_string(),
    }
}

fn check_adjacent_for_splits<'a>(
    grammar: &'a Grammar,
    left: &'a Expression,
    right: &'a Expression,
    locations: &mut Vec<(&'a str, String)>,
    _current_production: &str,
) {
    // Get the possible ending tokens from the left expression
    let left_endings = get_possible_endings(grammar, left);
    // Get the possible starting tokens from the right expression
    let right_starts = get_possible_starts(grammar, right);

    // Get descriptions of the left and right elements
    let left_desc = describe_expression(left);
    let right_desc = describe_expression(right);

    // Check if any combination could form a multi-char token
    for left_end in &left_endings {
        for right_start in &right_starts {
            let combined = format!("{}{}", left_end, right_start);
            for &multi_token in MULTI_CHAR_TOKENS {
                if combined == multi_token {
                    // Exact match - the two elements combine to form the token
                    locations.push((
                        multi_token,
                        format!(
                            "{} ends with `{}` and can be immediately followed by {} which can start with `{}`, forming `{}`",
                            left_desc, left_end, right_desc, right_start, multi_token
                        ),
                    ));
                } else if combined.starts_with(multi_token) {
                    // Combined is longer and starts with the token (e.g., "+=" in "+==" for token "+=")
                    locations.push((
                        multi_token,
                        format!(
                            "{} ends with `{}` followed by {} starting with `{}` could form `{}`",
                            left_desc, left_end, right_desc, right_start, multi_token
                        ),
                    ));
                } else if multi_token.starts_with(&combined) {
                    // Token is longer than combined (e.g., "+" and "=" is partial for "+=")
                    // This shouldn't happen since combined should be complete, but keep for completeness
                    locations.push((
                        multi_token,
                        format!(
                            "{} ends with `{}` followed by {} starting with `{}` (partial match for `{}`)",
                            left_desc, left_end, right_desc, right_start, multi_token
                        ),
                    ));
                }
            }
        }
    }
}

fn check_repeat_for_splits<'a>(
    grammar: &'a Grammar,
    expr: &'a Expression,
    locations: &mut Vec<(&'a str, String)>,
    _current_production: &str,
    repeat_type: &str,
) {
    // Get the possible endings and starts from the expression
    let endings = get_possible_endings(grammar, expr);
    let starts = get_possible_starts(grammar, expr);

    let expr_desc = describe_expression(expr);

    // Check if repeating could form a multi-char token
    for ending in &endings {
        for start in &starts {
            let combined = format!("{}{}", ending, start);
            for &multi_token in MULTI_CHAR_TOKENS {
                if combined == multi_token {
                    locations.push((
                        multi_token,
                        format!(
                            "{} (in {}) ends with `{}` and can be immediately followed by another {} which can start with `{}`, forming `{}`",
                            expr_desc, repeat_type, ending, expr_desc, start, multi_token
                        ),
                    ));
                } else if combined.starts_with(multi_token) || multi_token.starts_with(&combined) {
                    locations.push((
                        multi_token,
                        format!(
                            "{} (in {}) ending with `{}` followed by start `{}` could form `{}`",
                            expr_desc, repeat_type, ending, start, multi_token
                        ),
                    ));
                }
            }
        }
    }
}

fn get_possible_endings(grammar: &Grammar, expr: &Expression) -> HashSet<String> {
    let mut endings = HashSet::new();
    get_possible_endings_impl(grammar, expr, &mut endings, &mut HashSet::new());
    endings
}

fn get_possible_endings_impl(
    grammar: &Grammar,
    expr: &Expression,
    endings: &mut HashSet<String>,
    visited: &mut HashSet<String>,
) {
    match &expr.kind {
        ExpressionKind::Terminal(term) => {
            // Extract the last character from the terminal
            if let Some(last_ch) = term.chars().last() {
                endings.insert(last_ch.to_string());
            }
        }
        ExpressionKind::Grouped(e)
        | ExpressionKind::Optional(e)
        | ExpressionKind::NegativeLookahead(e)
        | ExpressionKind::Repeat(e)
        | ExpressionKind::RepeatPlus(e)
        | ExpressionKind::RepeatRange { expr: e, .. }
        | ExpressionKind::RepeatRangeNamed(e, _)
        | ExpressionKind::NegExpression(e)
        | ExpressionKind::Cut(e) => {
            get_possible_endings_impl(grammar, e, endings, visited);
        }
        ExpressionKind::Alt(es) => {
            for e in es {
                get_possible_endings_impl(grammar, e, endings, visited);
            }
        }
        ExpressionKind::Sequence(es) => {
            // The ending comes from the last element in the sequence that produces tokens
            // Skip trailing Breaks and Comments
            for e in es.iter().rev() {
                if !matches!(
                    e.kind,
                    ExpressionKind::Break(_) | ExpressionKind::Comment(_)
                ) {
                    get_possible_endings_impl(grammar, e, endings, visited);
                    break;
                }
            }
        }
        ExpressionKind::Nt(nt) => {
            if visited.insert(nt.clone()) {
                if let Some(prod) = grammar.productions.get(nt) {
                    get_possible_endings_impl(grammar, &prod.expression, endings, visited);
                }
            }
        }
        ExpressionKind::Charset(chars) => {
            for ch in chars {
                get_possible_endings_impl(grammar, ch, endings, visited);
            }
        }
        ExpressionKind::CharacterRange(a, b) => {
            // For ranges, we'll just add the boundary characters
            endings.insert(a.get_ch().to_string());
            endings.insert(b.get_ch().to_string());
        }
        ExpressionKind::Unicode((ch, _)) => {
            endings.insert(ch.to_string());
        }
        ExpressionKind::Prose(text) => {
            // Handle "Token" prose - it can be any token
            if text.to_lowercase().contains("token") {
                // Add all characters that could be part of multi-char tokens
                for &token in MULTI_CHAR_TOKENS {
                    for ch in token.chars() {
                        endings.insert(ch.to_string());
                    }
                }
            }
        }
        ExpressionKind::Break(_) | ExpressionKind::Comment(_) => {
            // These don't produce tokens
        }
    }
}

fn get_possible_starts(grammar: &Grammar, expr: &Expression) -> HashSet<String> {
    let mut starts = HashSet::new();
    get_possible_starts_impl(grammar, expr, &mut starts, &mut HashSet::new());
    starts
}

fn get_possible_starts_impl(
    grammar: &Grammar,
    expr: &Expression,
    starts: &mut HashSet<String>,
    visited: &mut HashSet<String>,
) {
    match &expr.kind {
        ExpressionKind::Terminal(term) => {
            // Extract the first character from the terminal
            if let Some(first_ch) = term.chars().next() {
                starts.insert(first_ch.to_string());
            }
        }
        ExpressionKind::Grouped(e)
        | ExpressionKind::NegativeLookahead(e)
        | ExpressionKind::Repeat(e)
        | ExpressionKind::RepeatPlus(e)
        | ExpressionKind::RepeatRange { expr: e, .. }
        | ExpressionKind::RepeatRangeNamed(e, _)
        | ExpressionKind::NegExpression(e)
        | ExpressionKind::Cut(e) => {
            get_possible_starts_impl(grammar, e, starts, visited);
        }
        ExpressionKind::Optional(e) => {
            get_possible_starts_impl(grammar, e, starts, visited);
            // Optional also means the next element could be the start
        }
        ExpressionKind::Alt(es) => {
            for e in es {
                get_possible_starts_impl(grammar, e, starts, visited);
            }
        }
        ExpressionKind::Sequence(es) => {
            // The start comes from the first element in the sequence that produces tokens
            // Skip leading Breaks and Comments
            for e in es.iter() {
                if !matches!(
                    e.kind,
                    ExpressionKind::Break(_) | ExpressionKind::Comment(_)
                ) {
                    get_possible_starts_impl(grammar, e, starts, visited);
                    break;
                }
            }
        }
        ExpressionKind::Nt(nt) => {
            if visited.insert(nt.clone()) {
                if let Some(prod) = grammar.productions.get(nt) {
                    get_possible_starts_impl(grammar, &prod.expression, starts, visited);
                }
            }
        }
        ExpressionKind::Charset(chars) => {
            for ch in chars {
                get_possible_starts_impl(grammar, ch, starts, visited);
            }
        }
        ExpressionKind::CharacterRange(a, b) => {
            starts.insert(a.get_ch().to_string());
            starts.insert(b.get_ch().to_string());
        }
        ExpressionKind::Unicode((ch, _)) => {
            starts.insert(ch.to_string());
        }
        ExpressionKind::Prose(text) => {
            // Handle "Token" prose - it can be any token
            if text.to_lowercase().contains("token") {
                // Add all characters that could be part of multi-char tokens
                for &token in MULTI_CHAR_TOKENS {
                    for ch in token.chars() {
                        starts.insert(ch.to_string());
                    }
                }
            }
        }
        ExpressionKind::Break(_) | ExpressionKind::Comment(_) => {
            // These don't produce tokens
        }
    }
}
