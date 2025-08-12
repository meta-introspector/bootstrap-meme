#[derive(PartialEq, Debug)]
pub enum CocciElement {
    RuleHeader(String),
    Separator,
    Identifier(String),
    AddLine(String),
    RemoveLine(String),
    Comment(String),
    ContextLine(String),
}
