// src/types.rs

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
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
    Comment, // 💬
    EmitWatBlock, // 📦 (re-used for emit_wat_block)
    RuleEntry, // (... )
    ApplyRulesLoop, // ️.apply

    // World Tape
    Word(&'a str), // Generic word token
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
    Other(&'a str), // Catch-all for unparsed characters
}

#[derive(Debug, PartialEq, Clone)]
pub struct Rule<'a> {
    pub name: Option<&'a str>,
    pub pattern: Vec<Token<'a>>,
    pub replacement: Vec<Token<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Emojitape<'a> {
    pub prelude: Vec<Token<'a>>,
    pub wasm_compiler_prelude: Vec<Token<'a>>,
    pub rules: Vec<Rule<'a>>,
    pub world_tape: Vec<Token<'a>>,
    pub generated_wat_block: Vec<Token<'a>>,
    pub clues_keys: Vec<Token<'a>>,
    pub zos_export_definition: Vec<Token<'a>>,
    pub zos_export_implementation: Vec<Token<'a>>,
    pub self_reproducing_footer: Vec<Token<'a>>,
}
