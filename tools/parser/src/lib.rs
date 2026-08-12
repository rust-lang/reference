//! Rust parser based on the Reference grammar.

use std::ops::Range;
use std::str::FromStr;

pub mod coverage;
pub mod lexer;
mod parser;
pub mod tree;

#[derive(Clone, Debug)]
pub struct ParseError {
    pub byte_offset: usize,
    pub message: String,
}

impl ParseError {
    pub fn display(&self, src: &str) -> String {
        let s = &src[self.byte_offset..];
        match s.char_indices().nth(100) {
            Some((i, _)) => format!("{} at `{}…`", self.message, &s[..i]),
            None => format!("{} at `{s}`", self.message),
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Eq)]
pub enum Edition {
    Edition2015,
    Edition2018,
    Edition2021,
    Edition2024,
}

impl FromStr for Edition {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "2015" => Ok(Edition::Edition2015),
            "2018" => Ok(Edition::Edition2018),
            "2021" => Ok(Edition::Edition2021),
            "2024" => Ok(Edition::Edition2024),
            _ => Err(()),
        }
    }
}

/// A parsed section of source corresponding to some grammar expression.
#[derive(Clone, Debug, Default)]
pub struct Node {
    pub name: String,
    /// Range in bytes of the original source that this node covers.
    pub range: Range<usize>,
    pub children: Nodes,
}

impl Node {
    pub fn new(name: String, range: Range<usize>) -> Node {
        Node {
            name,
            range,
            children: Nodes::default(),
        }
    }

    /// Returns a new `Node` with the given children.
    fn with_children(name: String, start: usize, children: Nodes) -> Node {
        let range = if children.0.is_empty() {
            Range { start, end: start }
        } else {
            Range {
                start: children.0.first().unwrap().range.start,
                end: children.0.last().unwrap().range.end,
            }
        };
        Node {
            name,
            range,
            children,
        }
    }

    /// Length in bytes of this node.
    fn byte_len(&self) -> usize {
        self.range.end - self.range.start
    }
}

/// Abstraction over a sequence of nodes.
#[derive(Clone, Debug, Default)]
pub struct Nodes(pub Vec<Node>);

impl Nodes {
    fn new(name: String, range: Range<usize>) -> Nodes {
        let node = Node {
            name,
            range,
            children: Nodes::default(),
        };
        Nodes(vec![node])
    }

    /// Converts this `Nodes` to one with a single `Node`.
    fn wrap(self, name: String, start: usize) -> Nodes {
        Nodes(vec![Node::with_children(name.to_string(), start, self)])
    }

    fn extend(&mut self, other: Nodes) {
        self.0.extend(other.0)
    }

    fn byte_len(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0.last().unwrap().range.end - self.0.first().unwrap().range.start
        }
    }
}
