use crate::cocci_parser::cocci_element::CocciElement;
use crate::cocci_parser::cocci_parser_function::parse_cocci_content;

#[test]
fn test_parse_empty_content() {
    let content = "";
    let elements = parse_cocci_content(content);
    assert!(elements.is_empty());
}

#[test]
fn test_parse_rule_header() {
    let content = "@rule_name@";
    let elements = parse_cocci_content(content);
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0], CocciElement::RuleHeader("rule_name".to_string()));
}

#[test]
fn test_parse_separator() {
    let content = "@@";
    let elements = parse_cocci_content(content);
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0], CocciElement::Separator);
}

#[test]
fn test_parse_add_line() {
    let content = "+    new_line();";
    let elements = parse_cocci_content(content);
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0], CocciElement::AddLine("new_line();".to_string()));
}

#[test]
fn test_parse_remove_line() {
    let content = "-    old_line();";
    let elements = parse_cocci_content(content);
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0], CocciElement::RemoveLine("old_line();".to_string()));
}

#[test]
fn test_parse_comment() {
    let content = "// This is a comment";
    let elements = parse_cocci_content(content);
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0], CocciElement::Comment("// This is a comment".to_string()));
}

#[test]
fn test_parse_context_line() {
    let content = "    some_context();";
    let elements = parse_cocci_content(content);
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0], CocciElement::ContextLine("some_context();".to_string()));
}

#[test]
fn test_parse_mixed_content() {
    let content = r#"
@my_rule@
@@
+ new_code();
- old_code();
// a comment
    context_line();
@another_rule@
"#;
    let elements = parse_cocci_content(content);
    assert_eq!(elements.len(), 7);
    assert_eq!(elements[0], CocciElement::RuleHeader("my_rule".to_string()));
    assert_eq!(elements[1], CocciElement::Separator);
    assert_eq!(elements[2], CocciElement::AddLine("new_code();".to_string()));
    assert_eq!(elements[3], CocciElement::RemoveLine("old_code();".to_string()));
    assert_eq!(elements[4], CocciElement::Comment("// a comment".to_string()));
    assert_eq!(elements[5], CocciElement::ContextLine("context_line();".to_string()));
    assert_eq!(elements[6], CocciElement::RuleHeader("another_rule".to_string()));
}
