use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub declaration_type: DeclarationType,
    pub content: String,
    pub line_start: usize,
    pub line_end: usize,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeclarationType {
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Module,
    Use,
    Const,
    Static,
    Type,
    Macro,
}
