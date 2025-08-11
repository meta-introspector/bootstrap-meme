// src/types.rs

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Prelude
    True, // ✅
    False, // ❌
    FuncStart, // ➡️
    Forall, // ∀
    Exists, // ∃
    UpArrow, // ⏫⏫⏫
    And, // (∧)
    Or, // (∨)
    Not, // (¬)
    Implies, // (→)
    Iff, // (↔)
    S, // S
    K, // K
    I, // I
    Sparkle, // ✨ (i32_const)
    Lightning, // ⚡ (f32_const)
    B, // B
    C, // C
    W, // W
    Y, // Y
    Z, // Z
    Omega, // Ω
    Lambda, // Λ
    Top, // ⊤
    Bottom, // ⊥
    MapsTo, // ↦
    Compose, // ∘
    Equals, // =
    NotEquals, // ≠
    Identical, // ≡
    Proves, // ⊢
    Entails, // ⊨️

    // WASM Compiler Prelude (conceptual)
    Compiler, // ⚗️
    Optimizer, // ⚙️
    Box, // 📦
    CheckTrap, // ️

    // Rules (emoji -> token mapping)
    Return, // ↩️
    Call, // 📞
    LocalGet, // 📥
    LocalSet, // 📤
    SpawnToken, // 🌱
    Comment(String), // 💬
    EmitWatBlock, // 📦 (re-used for emit_wat_block)
    RuleEntry, // (... )
    ApplyRulesLoop, // ️.apply

    // World Tape
    Word(String), // Generic word token
    Integer(i32), // For i32.const
    Float(f32), // For f32.const
    Add, // ➕
    Sub, // ➖
    Mul, // ✖️
    DivS, // ➗
    GtS, // ≻
    ZosExport, // /zos export
    ZosReady, // /zos ready
    Newline, // For parsing newlines
    Whitespace, // For parsing whitespace
    Other(String), // Catch-all for unparsed characters
}

#[derive(Debug, PartialEq, Clone)]
pub struct Rule {
    pub name: Option<String>,
    pub pattern: Vec<Token>,
    pub replacement: Vec<Token>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Emojitape {
    pub prelude: Vec<Token>,
    pub wasm_compiler_prelude: Vec<Token>,
    pub rules: Vec<Rule>,
    pub world_tape: Vec<Token>,
    pub generated_wat_block: Vec<Token>,
    pub clues_keys: Vec<Token>,
    pub zos_export_definition: Vec<Token>,
    pub zos_export_implementation: Vec<Token>,
    pub self_reproducing_footer: Vec<Token>,
}
