use crate::parser::declarations::types::{Declaration, DeclarationType};
use syn::{spanned::Spanned, ItemImpl};

pub fn impl_to_declaration(
    item_impl: &ItemImpl,
    lines: &[&str],
    file_path: Option<String>,
) -> Declaration {
    let name = format!("impl_{}", item_impl.self_ty.span().start().line);
    let (start, end) = get_span_lines(&item_impl.span());
    let content = extract_content(lines, start, end);

    Declaration {
        name,
        declaration_type: DeclarationType::Impl,
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
