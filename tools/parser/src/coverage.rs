//! Support for recording and rendering coverage of the grammar.

use grammar::{Character, Expression, ExpressionKind, Grammar};

#[derive(Default)]
pub struct Coverage {
    /// Count of the repetitions for each expression.
    ///
    /// The index is the expression ID. The value is the number of times a
    /// particular number of repetitions was found for that expression (index
    /// N means it matched N repetitions of the given number of times).
    pub match_count: Vec<Vec<u32>>,

    /// Count of how often an expression failed to match its input.
    ///
    /// The index is the expression ID, the count is the number of times it failed.
    pub no_match_count: Vec<u32>,

    /// Count of how often the expression caused a `ParseError`.
    ///
    /// The index is the expression ID, the count is the number of times it caused an error.
    pub parse_error: Vec<u32>,
}

impl Coverage {
    /// Marks a node as being matched.
    pub fn cov_match(&mut self, id: u32, count: u32) {
        if self.match_count.len() < (id + 1) as usize {
            self.match_count.resize((id + 1) as usize, Vec::new());
        }
        let ns = self.match_count.get_mut(id as usize).unwrap();
        if ns.len() < (count + 1) as usize {
            ns.resize((count + 1) as usize, 0);
        }
        *ns.get_mut(count as usize).unwrap() += 1;
    }

    /// Marks a node that failed to match its input.
    pub fn cov_no_match(&mut self, id: u32) {
        if self.no_match_count.len() < (id + 1) as usize {
            self.no_match_count.resize((id + 1) as usize, 0);
        }
        *self.no_match_count.get_mut(id as usize).unwrap() += 1;
    }

    /// Marks a node that caused a `ParseError`.
    pub fn cov_parse_error(&mut self, id: u32) {
        if self.parse_error.len() < (id + 1) as usize {
            self.parse_error.resize((id + 1) as usize, 0);
        }
        *self.parse_error.get_mut(id as usize).unwrap() += 1;
    }

    /// Merge one `Coverage` into this one.
    pub fn merge(&mut self, other: Coverage) {
        if self.match_count.len() < other.match_count.len() {
            self.match_count.resize(other.match_count.len(), Vec::new());
        }
        for (id, counts) in other.match_count.into_iter().enumerate() {
            let this = self.match_count.get_mut(id).unwrap();
            if this.len() < counts.len() {
                this.resize(counts.len(), 0);
            }
            for (count, value) in counts.into_iter().enumerate() {
                this[count] += value;
            }
        }

        if self.no_match_count.len() < other.no_match_count.len() {
            self.no_match_count.resize(other.no_match_count.len(), 0);
        }
        for (id, value) in other.no_match_count.into_iter().enumerate() {
            self.no_match_count[id] += value;
        }

        if self.parse_error.len() < other.parse_error.len() {
            self.parse_error.resize(other.parse_error.len(), 0);
        }
        for (id, value) in other.parse_error.into_iter().enumerate() {
            self.parse_error[id] += value;
        }
    }

    /// Saves the coverage data to a file called `coverage.html`.
    pub fn save(&self, grammar: &Grammar) {
        let mut html = String::new();
        let mut span_stack = Vec::new();
        self.render_html(&mut html, &mut span_stack, grammar);
        std::fs::write("coverage.html", html).expect("failed to write coverage.html");
    }

    fn get_coverage_status(&self, id: u32, kind: &ExpressionKind) -> CoverageStatus {
        let match_count = self.match_count.get(id as usize);
        let no_match = self.no_match_count.get(id as usize).copied().unwrap_or(0);
        let parse_error = self.parse_error.get(id as usize).copied().unwrap_or(0);

        let has_matches = match_count
            .map(|counts| counts.iter().any(|&c| c > 0))
            .unwrap_or(false);
        let has_no_match = no_match > 0;

        // Special case logic for specific expression kinds
        match kind {
            ExpressionKind::Optional(_) => {
                // Green means match_count contains both 0 and 1
                if let Some(counts) = match_count {
                    let has_zero = counts.get(0).copied().unwrap_or(0) > 0;
                    let has_one = counts.get(1).copied().unwrap_or(0) > 0;
                    if has_zero && has_one {
                        return CoverageStatus::Green;
                    } else if has_zero || has_one {
                        return CoverageStatus::Yellow;
                    }
                }
                CoverageStatus::Red
            }
            ExpressionKind::Repeat(_) => {
                // Green means match_count contains 0, 1, and more than 1
                if let Some(counts) = match_count {
                    let has_zero = counts.get(0).copied().unwrap_or(0) > 0;
                    let has_one = counts.get(1).copied().unwrap_or(0) > 0;
                    let has_more = counts.iter().skip(2).any(|&c| c > 0);
                    if has_zero && has_one && has_more {
                        return CoverageStatus::Green;
                    } else if has_zero || has_one || has_more {
                        return CoverageStatus::Yellow;
                    }
                }
                CoverageStatus::Red
            }
            ExpressionKind::RepeatPlus(_) => {
                // Green means match_count contains 1 and more than 1 and no_match_count is not zero
                if let Some(counts) = match_count {
                    let has_one = counts.get(1).copied().unwrap_or(0) > 0;
                    let has_more = counts.iter().skip(2).any(|&c| c > 0);
                    if has_one && has_more && has_no_match {
                        return CoverageStatus::Green;
                    } else if (has_one || has_more) || has_no_match {
                        return CoverageStatus::Yellow;
                    }
                }
                CoverageStatus::Red
            }
            ExpressionKind::RepeatRange { min, max, .. } => {
                // Green means match_count contains the minimum and maximum values
                if let Some(counts) = match_count {
                    let min_val = min.unwrap_or(0) as usize;
                    let has_min = counts.get(min_val).copied().unwrap_or(0) > 0;

                    let has_max = if let Some(max_val) = max {
                        counts.get(*max_val as usize).copied().unwrap_or(0) > 0
                    } else {
                        // If max is None, consider it green if there are matches for any count over min
                        counts.iter().skip(min_val + 1).any(|&c| c > 0)
                    };

                    if has_min && has_max {
                        return CoverageStatus::Green;
                    } else if has_min || has_max {
                        return CoverageStatus::Yellow;
                    }
                }
                CoverageStatus::Red
            }
            ExpressionKind::Cut(_) => {
                // Green means match_count contains a nonzero value and parse_error contains a nonzero value
                if has_matches && parse_error > 0 {
                    return CoverageStatus::Green;
                } else if has_matches || parse_error > 0 {
                    return CoverageStatus::Yellow;
                }
                CoverageStatus::Red
            }
            _ => {
                // Default logic for other expression kinds
                if !has_matches && no_match == 0 && parse_error == 0 {
                    CoverageStatus::Red
                } else if has_matches && !has_no_match {
                    CoverageStatus::Yellow
                } else if has_matches && has_no_match {
                    CoverageStatus::Green
                } else {
                    CoverageStatus::Red
                }
            }
        }
    }

    fn render_html(&self, output: &mut String, span_stack: &mut Vec<String>, grammar: &Grammar) {
        output.push_str(HTML_HEADER);

        // Group productions by category, preserving first-appearance order.
        let mut category_order: Vec<String> = Vec::new();
        let mut categories: std::collections::HashMap<String, Vec<&str>> =
            std::collections::HashMap::new();
        for name in &grammar.name_order {
            if let Some(prod) = grammar.productions.get(name) {
                let cat = &prod.category;
                if !categories.contains_key(cat) {
                    category_order.push(cat.clone());
                    categories.insert(cat.clone(), Vec::new());
                }
                categories.get_mut(cat).unwrap().push(name.as_str());
            }
        }

        for category in &category_order {
            output.push_str(&format!(
                "<div class=\"category\"><h2 class=\"category-name\">{}</h2>\n",
                html_escape(category)
            ));
            if let Some(names) = categories.get(category) {
                for name in names {
                    if let Some(prod) = grammar.productions.get(*name) {
                        self.render_production(prod, output, span_stack);
                    }
                }
            }
            output.push_str("</div>\n");
        }

        output.push_str(HTML_FOOTER);
    }

    fn render_production(
        &self,
        prod: &grammar::Production,
        output: &mut String,
        span_stack: &mut Vec<String>,
    ) {
        output.push_str("<div class=\"production\">");
        output.push_str(&format!(
            "<span class=\"production-name\">{}</span>",
            html_escape(&prod.name)
        ));
        output.push_str(" → ");
        self.render_expression(&prod.expression, output, span_stack);
        output.push_str("</div>\n");
    }

    fn render_expression(
        &self,
        expr: &Expression,
        output: &mut String,
        span_stack: &mut Vec<String>,
    ) {
        if let ExpressionKind::Break(indent) = &expr.kind {
            for _ in 0..span_stack.len() {
                output.push_str("</span>");
            }
            output.push_str("<br>\n");
            for span in span_stack {
                output.push_str(span);
            }
            for _ in 0..*indent {
                output.push_str("&nbsp;");
            }
            return;
        }

        if let ExpressionKind::Comment(s) = &expr.kind {
            output.push_str(&format!(
                "<span class=\"comment\">// {}</span>",
                html_escape(s)
            ));
            return;
        }

        let status = self.get_coverage_status(expr.id, &expr.kind);
        let bg_color = status.color();
        let has_error = self.parse_error.get(expr.id as usize).copied().unwrap_or(0) > 0;

        let tooltip = self.generate_tooltip(expr.id);

        let span = format!(
            "<span class=\"expr\" style=\"background-color: {};\" \
             onmouseover=\"showTooltip(event, '{}')\" \
             onmouseout=\"hideTooltip()\">",
            bg_color,
            html_escape(&tooltip).replace("'", "&apos;")
        );
        output.push_str(&span);
        span_stack.push(span);

        if has_error {
            output.push_str("<span class=\"error-marker\">●</span>");
        }

        self.render_expression_kind(&expr.kind, output, span_stack);

        if let Some(suffix) = &expr.suffix {
            output.push_str(&format!("<sub>{}</sub>", html_escape(suffix)));
        }

        output.push_str("</span>");
        span_stack.pop();
    }

    fn render_expression_kind(
        &self,
        kind: &ExpressionKind,
        output: &mut String,
        span_stack: &mut Vec<String>,
    ) {
        match kind {
            ExpressionKind::Grouped(e) => {
                output.push_str("( ");
                self.render_expression(e, output, span_stack);
                output.push_str(" )");
            }
            ExpressionKind::Alt(es) => {
                let mut iter = es.iter().peekable();
                while let Some(e) = iter.next() {
                    self.render_expression(e, output, span_stack);
                    if iter.peek().is_some() {
                        if !e.last_expr().is_break() {
                            output.push(' ');
                        }
                        output.push_str("| ");
                    }
                }
            }
            ExpressionKind::Sequence(es) => {
                let mut iter = es.iter().peekable();
                while let Some(e) = iter.next() {
                    self.render_expression(e, output, span_stack);
                    if iter.peek().is_some() && !e.last_expr().is_break() {
                        output.push(' ');
                    }
                }
            }
            ExpressionKind::Optional(e) => {
                self.render_expression(e, output, span_stack);
                output.push_str("<sup>?</sup>");
            }
            ExpressionKind::NegativeLookahead(e) => {
                output.push('!');
                self.render_expression(e, output, span_stack);
            }
            ExpressionKind::Repeat(e) => {
                self.render_expression(e, output, span_stack);
                output.push_str("<sup>*</sup>");
            }
            ExpressionKind::RepeatPlus(e) => {
                self.render_expression(e, output, span_stack);
                output.push_str("<sup>+</sup>");
            }
            ExpressionKind::RepeatRange {
                expr,
                name,
                min,
                max,
                limit,
            } => {
                self.render_expression(expr, output, span_stack);
                output.push_str("<sup>");
                if let Some(n) = name {
                    output.push_str(&html_escape(n));
                    output.push(':');
                }
                if let Some(m) = min {
                    output.push_str(&m.to_string());
                }
                output.push_str(&format!("{}", limit));
                if let Some(m) = max {
                    output.push_str(&m.to_string());
                }
                output.push_str("</sup>");
            }
            ExpressionKind::RepeatRangeNamed(e, name) => {
                self.render_expression(e, output, span_stack);
                output.push_str(&format!("<sup>{}</sup>", html_escape(name)));
            }
            ExpressionKind::Nt(nt) => {
                output.push_str(&format!(
                    "<span class=\"nonterminal\">{}</span>",
                    html_escape(nt)
                ));
            }
            ExpressionKind::Terminal(t) => {
                output.push_str(&format!(
                    "<span class=\"terminal\">`{}`</span>",
                    html_escape(t)
                ));
            }
            ExpressionKind::Prose(s) => {
                output.push_str(&format!(
                    "<span class=\"prose\">&lt;{}&gt;</span>",
                    html_escape(s)
                ));
            }
            ExpressionKind::Break(_) | ExpressionKind::Comment(_) => {
                // These are handled in render_expression to avoid coverage spans
                unreachable!("Break and Comment should be handled in render_expression")
            }
            ExpressionKind::Charset(set) => {
                output.push('[');
                for (i, chars) in set.iter().enumerate() {
                    if i > 0 {
                        output.push(' ');
                    }
                    self.render_expression(chars, output, span_stack);
                }
                output.push(']');
            }
            ExpressionKind::CharacterRange(a, b) => {
                // TODO: It would be nice if this showed more info about the
                // actual range that was covered.
                let render_ch = |ch: &Character| -> String {
                    match ch {
                        Character::Char(c) => format!("`{}`", html_escape(&c.to_string())),
                        Character::Unicode((_, s)) => format!("U+{}", html_escape(s)),
                    }
                };
                output.push_str(&render_ch(a));
                output.push('-');
                output.push_str(&render_ch(b));
            }
            ExpressionKind::NegExpression(e) => {
                output.push('~');
                self.render_expression(e, output, span_stack);
            }
            ExpressionKind::Cut(e) => {
                output.push_str("^ ");
                self.render_expression(e, output, span_stack);
            }
            ExpressionKind::Unicode((_, s)) => {
                output.push_str(&format!("U+{}", html_escape(s)));
            }
        }
    }

    fn generate_tooltip(&self, id: u32) -> String {
        let mut tooltip = String::new();

        tooltip.push_str(&format!("ID: {}\\n", id));

        if let Some(counts) = self.match_count.get(id as usize) {
            if counts.iter().any(|&c| c > 0) {
                tooltip.push_str("Match counts:\\n");
                for (n, &count) in counts.iter().enumerate() {
                    if count > 0 {
                        let bar = "█".repeat((count.min(50) / 5).max(1) as usize);
                        tooltip.push_str(&format!("  {}: {} {}\\n", n, count, bar));
                    }
                }
            }
        }

        let no_match = self.no_match_count.get(id as usize).copied().unwrap_or(0);
        if no_match > 0 {
            tooltip.push_str(&format!("No match: {}\\n", no_match));
        }

        let parse_error = self.parse_error.get(id as usize).copied().unwrap_or(0);
        if parse_error > 0 {
            tooltip.push_str(&format!("Parse errors: {}\\n", parse_error));
        }

        if tooltip.ends_with("\\n") {
            tooltip.truncate(tooltip.len() - 2);
        }

        tooltip
    }
}

/// An indication of how well a node was covered.
#[derive(Debug, Clone, Copy)]
enum CoverageStatus {
    /// Indicates the node was not covered at all.
    Red,
    /// Indicates the node was only partially covered.
    Yellow,
    /// Indicates the node was completely covered.
    Green,
}

impl CoverageStatus {
    fn color(&self) -> &'static str {
        match self {
            CoverageStatus::Red => "#ffcccc",
            CoverageStatus::Yellow => "#ffffcc",
            CoverageStatus::Green => "#ccffcc",
        }
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

const HTML_HEADER: &str = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Grammar Coverage Report</title>
    <style>
        body {
            font-family: monospace;
            padding: 20px;
            background-color: #f5f5f5;
        }
        .production {
            margin: 20px 0;
            padding: 10px;
            background-color: white;
            border: 1px solid #ddd;
            border-radius: 4px;
        }
        .production-name {
            font-weight: bold;
            color: #0066cc;
        }
        .expr {
            display: inline-block;
            border-radius: 6px;
            position: relative;
        }
        .nonterminal {
            color: #0066cc;
        }
        .terminal {
            color: #009900;
        }
        .prose {
            color: #666;
            font-style: italic;
        }
        .comment {
            color: #999;
        }
        .error-marker {
            color: #9370db;
            font-size: 18px;
            vertical-align: super;
            margin-left: 2px;
        }
        #tooltip {
            position: fixed;
            background-color: #333;
            color: white;
            padding: 10px;
            border-radius: 4px;
            font-size: 12px;
            white-space: pre;
            z-index: 1000;
            display: none;
            max-width: 400px;
            pointer-events: none;
        }
        h1 {
            color: #333;
        }
        .category {
            margin: 30px 0;
            border: 2px solid #aaa;
            border-radius: 6px;
            padding: 10px 16px;
            background-color: #fafafa;
        }
        .category-name {
            margin: 0 0 12px 0;
            font-size: 1.1em;
            color: #444;
            border-bottom: 1px solid #ccc;
            padding-bottom: 6px;
        }
    </style>
</head>
<body>
    <h1>Grammar Coverage Report</h1>
    <div id="tooltip"></div>
    <div class="grammar">
"#;

const HTML_FOOTER: &str = r#"    </div>
    <script>
        function showTooltip(event, text) {
            event.stopPropagation();
            const tooltip = document.getElementById('tooltip');
            tooltip.textContent = text;
            tooltip.style.display = 'block';
            tooltip.style.left = (event.clientX + 10) + 'px';
            tooltip.style.top = (event.clientY + 10) + 'px';
        }
        
        function hideTooltip(event) {
            if (event) {
                event.stopPropagation();
            }
            const tooltip = document.getElementById('tooltip');
            tooltip.style.display = 'none';
        }
    </script>
</body>
</html>
"#;
