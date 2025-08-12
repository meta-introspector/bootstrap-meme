use crate::parser::declarations::types::{Declaration, DeclarationType};
use syn::{spanned::Spanned, ItemTrait};

pub fn trait_to_declaration(
    item_trait: &ItemTrait,
    lines: &[&str],
    file_path: Option<String>,
) -> Declaration {
    let name = item_trait.ident.to_string();
    let (start, end) = get_span_lines(&item_trait.span());
    let content = extract_content(lines, start, end);

    Declaration {
        name,
        declaration_type: DeclarationType::Trait,
        content,
        line_start: start,
        line_end: end,
        file_path,
    }
}

fn get_span_lines(span: &proc_macro2::Span) -> (usize, usize) {
    (span.start().line, span.end().line)
}

fn extract_content(lines: &[&str], start: usize, end: usize) -> String {
    if start == 0 || start > lines.len() || end > lines.len() {
        return String::new();
    }

    lines[(start - 1)..end].join("\n")
}
