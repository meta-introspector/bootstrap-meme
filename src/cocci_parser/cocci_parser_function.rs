use crate::cocci_parser::cocci_element::CocciElement;

pub fn parse_cocci_content(cocci_content: &str) -> Vec<CocciElement> {
    let mut elements = Vec::new();
    let lines: Vec<&str> = cocci_content.lines().collect();

    for line in lines {
        let trimmed_line = line.trim();

        if trimmed_line == "@@" {
            elements.push(CocciElement::Separator);
        } else if trimmed_line.starts_with("@") && trimmed_line.ends_with("@") {
            elements.push(CocciElement::RuleHeader(trimmed_line.trim_matches('@').to_string()));
        } else if trimmed_line.starts_with("identifier") {
            elements.push(CocciElement::Identifier(trimmed_line.to_string()));
        } else if trimmed_line.starts_with("+") {
            elements.push(CocciElement::AddLine(trimmed_line[1..].trim().to_string()));
        } else if trimmed_line.starts_with("-") {
            elements.push(CocciElement::RemoveLine(trimmed_line[1..].trim().to_string()));
        } else if trimmed_line.starts_with("//") {
            elements.push(CocciElement::Comment(trimmed_line.to_string()));
        } else if !trimmed_line.is_empty() {
            elements.push(CocciElement::ContextLine(trimmed_line.to_string()));
        }
    }
    elements
}
