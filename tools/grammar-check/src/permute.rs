//! Permutation-based tests.
//!
//! This attempts to generate exhaustive coverage of the grammar by using
//! permutations of all of the possible inputs to the grammar. This includes
//! both valid and invalid inputs (particularly those that are truncated).
//!
//! It generates representative inputs for some of the expressions. For
//! example, a repetition generates an output that includes 0, 1, or 2
//! repetitions of the expression. Or something like "Identifier" just does a
//! few representative values like "a", "ab", and "abb" (with the assumption
//! that the Identifier grammar is already correct).
//!
//! This uses a state machine and is driven using the `Iterator` API to fetch
//! each new input to test.
//!
//! This is incomplete, and I'm not entirely happy with the design. This
//! misses some inputs, particularly invalid ones with unexpected inputs. I
//! intended to spend more time reading
//! https://www.fuzzingbook.org/html/Grammars.html to think of better
//! strategies to stress the parser.
//!
//! However, a coverage-based fuzzer wouldn't necessarily give all the input
//! that I would want because the point of this tool is to compare against
//! rustc. The fuzzer would be fuzzing the Reference grammar, not the rustc
//! parser. We want to get full coverage of *both* those parsers. A fuzzer
//! based on just the Reference coverage wouldn't ensure that the Reference
//! isn't missing something.
//!
//! Known issues:
//!
//! - Permute didn't find that pm2 doesn't error on `prefix'x'` because it was
//!   only generating `prefix'`. Any ideas on how to generate tests that
//!   exercise this?

use grammar::{Expression, ExpressionKind, Grammar, RangeLimit};
use std::collections::HashMap;

pub struct PermutationIterator<'g> {
    pub grammar: &'g Grammar,
    name_context: HashMap<String, usize>,
    state: IteratorState<'g>,
}

enum IteratorState<'g> {
    Terminal {
        value: String,
        done: bool,
    },
    Seq {
        exprs: Vec<&'g Expression>,
        /// Current active length; counts down from exprs.len() to 1 to emit truncated sequences.
        truncated_len: usize,
        iterators: Vec<PermutationIterator<'g>>,
        current_values: Vec<String>,
        initialized: bool,
        exhausted: bool,
    },
    SeqWithNamedRanges {
        exprs: Vec<&'g Expression>,
        named_range_indices: Vec<(usize, String, usize, usize)>, // (index, name, min, max)
        current_named_values: HashMap<String, usize>,
        iterators: Vec<PermutationIterator<'g>>,
        current_values: Vec<String>,
        exhausted: bool,
    },
    Alt {
        iterators: Vec<PermutationIterator<'g>>,
        current_index: usize,
    },
    Optional {
        iterator: Box<PermutationIterator<'g>>,
        emitted_empty: bool,
    },
    Repeat {
        expr: &'g Expression,
        include_empty: bool,
        current_stage: usize, // 0 = empty (if include_empty), 1 = single, 2 = double
        iterator: Option<Box<PermutationIterator<'g>>>,
    },
    RepeatRange {
        expr: &'g Expression,
        max: usize,
        current_count: usize,
        iterator: Option<Box<PermutationIterator<'g>>>,
        pending_repeat: Option<(String, usize)>, // (value, times_left_to_emit)
    },
}

impl<'g> PermutationIterator<'g> {
    pub fn new(grammar: &'g Grammar, expression: &'g Expression) -> PermutationIterator<'g> {
        Self::new_with_context(grammar, expression, HashMap::new())
    }

    fn new_with_context(
        grammar: &'g Grammar,
        expression: &'g Expression,
        name_context: HashMap<String, usize>,
    ) -> PermutationIterator<'g> {
        let state = match &expression.kind {
            ExpressionKind::Alt(exprs) => {
                let iterators: Vec<_> = exprs
                    .iter()
                    .map(|e| Self::new_with_context(grammar, e, name_context.clone()))
                    .collect();
                IteratorState::Alt {
                    iterators,
                    current_index: 0,
                }
            }
            ExpressionKind::Grouped(expr) => {
                return Self::new_with_context(grammar, expr, name_context);
            }
            ExpressionKind::Sequence(exprs) => {
                if exprs.is_empty() {
                    IteratorState::Terminal {
                        value: String::new(),
                        done: false,
                    }
                } else {
                    let filtered_exprs: Vec<&Expression> = exprs
                        .iter()
                        .filter(|e| {
                            !matches!(
                                e.kind,
                                ExpressionKind::Break(_) | ExpressionKind::Comment(_)
                            )
                        })
                        .collect();

                    // Check if any expressions are named repeat ranges
                    let mut named_range_indices = Vec::new();
                    for (idx, expr) in filtered_exprs.iter().enumerate() {
                        if let ExpressionKind::RepeatRange {
                            name: Some(name),
                            min,
                            max,
                            limit,
                            ..
                        } = &expr.kind
                        {
                            let min_count = min.unwrap_or(0) as usize;
                            let max_count = match max {
                                Some(m) => match limit {
                                    RangeLimit::HalfOpen => *m as usize,
                                    RangeLimit::Closed => (*m + 1) as usize,
                                },
                                None => min_count + 3,
                            };
                            named_range_indices.push((idx, name.clone(), min_count, max_count));
                        }
                    }

                    if named_range_indices.is_empty() {
                        // No named ranges, use regular Seq
                        let n = filtered_exprs.len();

                        IteratorState::Seq {
                            exprs: filtered_exprs,
                            truncated_len: n,
                            iterators: Vec::new(),
                            current_values: Vec::new(),
                            initialized: false,
                            exhausted: false,
                        }
                    } else {
                        // Has named ranges, use special handling
                        let current_named_values: HashMap<String, usize> = named_range_indices
                            .iter()
                            .map(|(_, name, min, _)| (name.clone(), *min))
                            .collect();

                        IteratorState::SeqWithNamedRanges {
                            exprs: filtered_exprs,
                            named_range_indices,
                            current_named_values,
                            iterators: Vec::new(),
                            current_values: Vec::new(),
                            exhausted: false,
                        }
                    }
                }
            }
            ExpressionKind::Optional(expr) => {
                let iterator =
                    Box::new(Self::new_with_context(grammar, expr, name_context.clone()));
                IteratorState::Optional {
                    iterator,
                    emitted_empty: false,
                }
            }
            ExpressionKind::NegativeLookahead(expr) => {
                let iterator =
                    Box::new(Self::new_with_context(grammar, expr, name_context.clone()));
                IteratorState::Optional {
                    iterator,
                    emitted_empty: false,
                }
            }
            ExpressionKind::Repeat(expr) | ExpressionKind::RepeatPlus(expr) => {
                IteratorState::Repeat {
                    expr,
                    include_empty: true,
                    current_stage: 0,
                    iterator: None,
                }
            }
            ExpressionKind::RepeatRange {
                expr,
                name,
                min,
                max,
                limit,
            } => {
                // If this has a name and it's in the context, use that specific count
                if let Some(name) = name {
                    if let Some(&count) = name_context.get(name) {
                        // Use the specified count from context
                        IteratorState::RepeatRange {
                            expr,
                            max: count + 1,
                            current_count: count,
                            iterator: None,
                            pending_repeat: None,
                        }
                    } else {
                        // Name not in context yet, this shouldn't happen in SeqWithNamedRanges
                        // but handle it anyway
                        let min_count = min.unwrap_or(0) as usize;
                        let max_count = match max {
                            Some(m) => match limit {
                                RangeLimit::HalfOpen => *m as usize,
                                RangeLimit::Closed => (*m + 1) as usize,
                            },
                            None => min_count + 3,
                        };
                        let start_count = if min_count == 0 { 0 } else { min_count };
                        IteratorState::RepeatRange {
                            expr,
                            max: max_count,
                            current_count: start_count,
                            iterator: None,
                            pending_repeat: None,
                        }
                    }
                } else {
                    // No name, normal behavior
                    let min_count = min.unwrap_or(0) as usize;
                    let max_count = match max {
                        Some(m) => match limit {
                            RangeLimit::HalfOpen => *m as usize,
                            RangeLimit::Closed => (*m + 1) as usize,
                        },
                        None => min_count + 3,
                    };
                    let start_count = if min_count == 0 { 0 } else { min_count };
                    IteratorState::RepeatRange {
                        expr,
                        max: max_count,
                        current_count: start_count,
                        iterator: None,
                        pending_repeat: None,
                    }
                }
            }
            ExpressionKind::RepeatRangeNamed(expr, name) => {
                // Look up the count from the context
                let count = name_context.get(name).copied().unwrap_or(1);
                IteratorState::RepeatRange {
                    expr,
                    max: count + 1,
                    current_count: count,
                    iterator: None,
                    pending_repeat: None,
                }
            }
            ExpressionKind::Nt(name) => {
                let prod = grammar.productions.get(name).unwrap();
                return Self::new_with_context(grammar, &prod.expression, name_context);
            }
            ExpressionKind::Terminal(s) => IteratorState::Terminal {
                value: s.clone(),
                done: false,
            },
            ExpressionKind::Prose(prose) => match prose.as_str() {
                "`XID_Start` defined by Unicode" => IteratorState::Terminal {
                    value: "a".to_string(),
                    done: false,
                },
                "`XID_Continue` defined by Unicode" => IteratorState::Terminal {
                    value: "b".to_string(),
                    done: false,
                },
                _ => panic!("prose {prose} not supported"),
            },
            ExpressionKind::Break(_) => unreachable!(),
            ExpressionKind::Comment(_) => unreachable!(),
            ExpressionKind::Charset(chars) => {
                let iterators: Vec<_> = chars
                    .iter()
                    .map(|e| Self::new_with_context(grammar, e, name_context.clone()))
                    .collect();
                IteratorState::Alt {
                    iterators,
                    current_index: 0,
                }
            }
            ExpressionKind::CharacterRange(start, end) => {
                // Behave like Alt of start and end characters
                let mut iterators = Vec::new();
                let start_ch = start.get_ch();
                let end_ch = end.get_ch();
                iterators.push(PermutationIterator {
                    grammar,
                    name_context: name_context.clone(),
                    state: IteratorState::Terminal {
                        value: start_ch.to_string(),
                        done: false,
                    },
                });
                iterators.push(PermutationIterator {
                    grammar,
                    name_context: name_context.clone(),
                    state: IteratorState::Terminal {
                        value: end_ch.to_string(),
                        done: false,
                    },
                });
                IteratorState::Alt {
                    iterators,
                    current_index: 0,
                }
            }
            ExpressionKind::NegExpression(_expr) => IteratorState::Terminal {
                value: String::from("a"), // TODO: Comment here why this choice.
                done: false,
            },
            ExpressionKind::Cut(expr) => {
                return Self::new_with_context(grammar, expr, name_context);
            }
            ExpressionKind::Unicode((ch, _)) => IteratorState::Terminal {
                value: ch.to_string(),
                done: false,
            },
        };
        PermutationIterator {
            grammar,
            name_context,
            state,
        }
    }
}

impl<'g> Iterator for PermutationIterator<'g> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // Capture grammar reference before mutably borrowing state
        let grammar = self.grammar;

        match &mut self.state {
            IteratorState::Terminal { value, done } => {
                if *done {
                    None
                } else {
                    *done = true;
                    Some(value.clone())
                }
            }
            IteratorState::Alt {
                iterators,
                current_index,
            } => {
                while *current_index < iterators.len() {
                    if let Some(val) = iterators[*current_index].next() {
                        return Some(val);
                    }
                    *current_index += 1;
                }
                None
            }
            IteratorState::Optional {
                iterator,
                emitted_empty,
            } => {
                if !*emitted_empty {
                    *emitted_empty = true;
                    return Some(String::new());
                }
                iterator.next()
            }
            IteratorState::Repeat {
                expr,
                include_empty,
                current_stage,
                iterator,
            } => {
                // Stage 0: emit empty string (only for Repeat, not RepeatPlus)
                if *current_stage == 0 && *include_empty {
                    *current_stage = 1;
                    return Some(String::new());
                }

                // Stage 1: emit single permutations
                if *current_stage == 1 {
                    if iterator.is_none() {
                        *iterator = Some(Box::new(Self::new_with_context(
                            grammar,
                            expr,
                            self.name_context.clone(),
                        )));
                    }

                    if let Some(iter) = iterator {
                        if let Some(result) = iter.next() {
                            return Some(result);
                        }
                    }

                    // Stage 1 complete, move to stage 2
                    *current_stage = 2;
                    *iterator = Some(Box::new(Self::new_with_context(
                        grammar,
                        expr,
                        self.name_context.clone(),
                    )));
                }

                // Stage 2: emit double permutations (each element repeated twice)
                if *current_stage == 2 {
                    // Get next single value and prepare to emit it twice
                    if let Some(iter) = iterator {
                        if let Some(val) = iter.next() {
                            let doubled = format!("{val}{val}");
                            return Some(doubled);
                        }
                    }
                }

                None
            }
            IteratorState::RepeatRange {
                expr,
                max,
                current_count,
                iterator,
                pending_repeat,
            } => {
                // If we're at count 0 (min was 0), emit empty string
                if *current_count == 0 {
                    *current_count = 1;
                    if *current_count >= *max {
                        return None;
                    }
                    return Some(String::new());
                }

                loop {
                    // If we haven't reached max count yet
                    if *current_count >= *max {
                        return None;
                    }

                    // Check if we have a pending repeat to emit
                    if let Some((val, times_left)) = pending_repeat {
                        if *times_left > 1 {
                            *times_left -= 1;
                            return Some(val.clone());
                        } else {
                            // Emit last repetition and clear pending
                            let result = val.clone();
                            *pending_repeat = None;
                            return Some(result);
                        }
                    }

                    // Initialize iterator for current count if needed
                    if iterator.is_none() {
                        *iterator = Some(Box::new(Self::new_with_context(
                            grammar,
                            expr,
                            self.name_context.clone(),
                        )));
                    }

                    // Try to get next value from iterator
                    if let Some(iter) = iterator {
                        if let Some(val) = iter.next() {
                            let result = val.repeat(*current_count);
                            return Some(result);
                        }
                    }

                    // Current count exhausted, move to next
                    *current_count += 1;
                    *iterator = None;
                }
            }
            IteratorState::Seq {
                exprs,
                truncated_len,
                iterators,
                current_values,
                initialized,
                exhausted,
            } => {
                if *exhausted {
                    return None;
                }

                loop {
                    // (Re)initialize iterators for the current truncated_len.
                    if !*initialized {
                        let tlen = *truncated_len;
                        *iterators = exprs[..tlen]
                            .iter()
                            .map(|e| Self::new_with_context(grammar, e, self.name_context.clone()))
                            .collect();
                        *current_values = vec![String::new(); tlen];

                        // Get first value from each iterator.
                        let mut ok = true;
                        for (i, iter) in iterators.iter_mut().enumerate() {
                            if let Some(val) = iter.next() {
                                current_values[i] = val;
                            } else {
                                ok = false;
                                break;
                            }
                        }

                        if ok {
                            *initialized = true;
                            return Some(current_values.concat());
                        } else {
                            // Empty iterator at this length; try shorter.
                            if *truncated_len > 1 {
                                *truncated_len -= 1;
                                continue;
                            } else {
                                *exhausted = true;
                                return None;
                            }
                        }
                    }

                    // Try to advance the rightmost iterator.
                    let mut pos = iterators.len() - 1;
                    let mut advanced = false;
                    loop {
                        if let Some(val) = iterators[pos].next() {
                            current_values[pos] = val;
                            advanced = true;
                            break;
                        } else {
                            // This iterator is exhausted; reset it and move left.
                            if pos == 0 {
                                // All iterators for this truncated_len are exhausted.
                                break;
                            }
                            iterators[pos] = Self::new_with_context(
                                grammar,
                                &exprs[pos],
                                self.name_context.clone(),
                            );
                            if let Some(val) = iterators[pos].next() {
                                current_values[pos] = val;
                            }
                            pos -= 1;
                        }
                    }

                    if advanced {
                        return Some(current_values.concat());
                    }

                    // Current length exhausted; move to the next shorter truncation.
                    if *truncated_len > 1 {
                        *truncated_len -= 1;
                        *initialized = false;
                    } else {
                        *exhausted = true;
                        return None;
                    }
                }
            }
            IteratorState::SeqWithNamedRanges {
                exprs,
                named_range_indices,
                current_named_values,
                iterators,
                current_values,
                exhausted,
            } => {
                if *exhausted {
                    return None;
                }

                loop {
                    // Initialize iterators if needed
                    if iterators.is_empty() {
                        for expr in exprs.iter() {
                            iterators.push(Self::new_with_context(
                                grammar,
                                expr,
                                current_named_values.clone(),
                            ));
                        }
                        *current_values = iterators.iter().map(|_| String::new()).collect();

                        // Get first value from each iterator
                        for (i, iter) in iterators.iter_mut().enumerate() {
                            if let Some(val) = iter.next() {
                                current_values[i] = val;
                            } else {
                                // Empty iterator, try next name values
                                break;
                            }
                        }

                        if current_values.iter().all(|v| !v.is_empty()) {
                            return Some(current_values.concat());
                        }
                    }

                    // Try to advance rightmost iterator
                    let mut pos = iterators.len().checked_sub(1)?;
                    loop {
                        if let Some(val) = iterators[pos].next() {
                            current_values[pos] = val;
                            return Some(current_values.concat());
                        } else {
                            // This iterator exhausted
                            if pos == 0 {
                                // All iterators for this name combo exhausted
                                // Try to increment named values (only min and max, not values in between)
                                let mut incremented = false;
                                for (_idx, name, min, max) in named_range_indices.iter().rev() {
                                    let current_val =
                                        current_named_values.get(name).copied().unwrap_or(*min);
                                    // Only generate for min and max values
                                    if current_val == *min && *min + 1 < *max {
                                        // Jump from min to max-1 (which is the actual max value since max is exclusive)
                                        current_named_values.insert(name.clone(), *max - 1);
                                        incremented = true;
                                        break;
                                    } else {
                                        // Reset to min
                                        current_named_values.insert(name.clone(), *min);
                                    }
                                }

                                if !incremented {
                                    *exhausted = true;
                                    return None;
                                }

                                // Reset all iterators with new named values
                                iterators.clear();
                                current_values.clear();
                                break; // Go back to initialization
                            }

                            // Reset this iterator and move left
                            iterators[pos] = Self::new_with_context(
                                grammar,
                                &exprs[pos],
                                current_named_values.clone(),
                            );
                            if let Some(val) = iterators[pos].next() {
                                current_values[pos] = val;
                            }
                            pos -= 1;
                        }
                    }
                }
            }
        }
    }
}

/// Generates all permutations of one, two, or three character long strings.
pub struct ThreeIterator {
    /// Current string length being generated (1, 2, or 3).
    len: u8,
    /// Character indices for each position (values 0..=0x7F).
    indices: [u8; 3],
    /// Set when all lengths are exhausted.
    done: bool,
}

impl ThreeIterator {
    pub fn new() -> ThreeIterator {
        ThreeIterator {
            len: 1,
            indices: [0; 3],
            done: false,
        }
    }
}

impl Iterator for ThreeIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let len = self.len as usize;

        // Build the current string from the active indices.
        let result: String = self.indices[..len]
            .iter()
            .map(|&i| char::from_u32(i as u32).unwrap())
            .collect();

        // Advance indices right-to-left, carrying into higher positions.
        let mut carry = true;
        for i in (0..len).rev() {
            if carry {
                if self.indices[i] < 0x7F {
                    self.indices[i] += 1;
                    carry = false;
                } else {
                    self.indices[i] = 0;
                    // carry remains true; propagate left
                }
            }
        }

        if carry {
            // All positions overflowed — this length is exhausted.
            if self.len < 3 {
                self.len += 1;
                self.indices = [0; 3];
            } else {
                self.done = true;
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn assert_permutations(grammar: &str, expected: &[&str]) {
        let g = Grammar::grammar_from_str(grammar, "cat").unwrap();
        let e = &g.productions.get("P").unwrap().expression;
        let ps: Vec<_> = PermutationIterator::new(&g, e).collect();
        assert_eq!(ps, expected);
    }

    #[test]
    fn seq_and_alt() {
        // Full sequence, then truncations (length 2, then length 1).
        assert_permutations(
            "P -> `A` (`B` | (`C1` | `C2`) | `D`) `E`",
            &["ABE", "AC1E", "AC2E", "ADE", "AB", "AC1", "AC2", "AD", "A"],
        );
    }

    #[test]
    fn optional() {
        // Full sequence, then truncations.
        assert_permutations(
            "P -> `A` (`B` | `C`)? `D`",
            &["AD", "ABD", "ACD", "A", "AB", "AC", "A"],
        );
    }

    #[test]
    fn seq_truncated() {
        // A single sequence with no alternatives: ABC, then AB, then A.
        assert_permutations("P -> `A` `B` `C`", &["ABC", "AB", "A"]);
    }

    #[test]
    fn seq_truncated_with_alts() {
        // Each position has alternatives; verify all combos per length, then shorter lengths.
        assert_permutations(
            "P -> (`A` | `X`) (`B` | `Y`)",
            &["AB", "AY", "XB", "XY", "A", "X"],
        );
    }

    #[test]
    fn repeat() {
        assert_permutations("P -> (`A` | `B`)*", &["", "A", "B", "AA", "BB"]);
    }

    #[test]
    fn repeat_plus() {
        assert_permutations("P -> (`A` | `B`)+", &["", "A", "B", "AA", "BB"]);
    }

    #[test]
    fn repeat_range() {
        assert_permutations("P -> (`A` | `B`){0..}", &["", "A", "B", "AA", "BB"]);

        assert_permutations("P -> (`A` | `B`){1..3}", &["A", "B", "AA", "BB"]);

        assert_permutations("P -> (`A` | `B`){2..=3}", &["AA", "BB", "AAA", "BBB"]);
    }

    #[test]
    fn charset() {
        // Test with Terminal and Range
        assert_permutations("P -> [`A` `X`-`Z`]", &["A", "X", "Z"]);

        // Test with just Range
        assert_permutations("P -> [`a`-`c`]", &["a", "c"]);
    }

    #[test]
    fn named_repeat_range() {
        // Test named repeat ranges are synchronized (only min and max values)
        assert_permutations("P -> `A`{n:1..=5} `B` `C`{n}", &["ABC", "AAAAABCCCCC"]);
    }

    #[test]
    fn negative_lookahead() {
        // NegativeLookahead emits empty string first, then all permutations of the expression.
        assert_permutations("P -> !`A`", &["", "A"]);
        assert_permutations("P -> !(`A` | `B`)", &["", "A", "B"]);
        // In a sequence: empty lookahead plus the rest, then lookahead expr plus the rest,
        // then truncated-length permutations (just the lookahead expression alone).
        assert_permutations("P -> !`X` `Y`", &["Y", "XY", "", "X"]);
    }
}
