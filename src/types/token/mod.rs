pub mod emojis;
pub mod executable;
pub mod executable_impl;
pub mod impl_token;
pub mod impl_token_display_fromstr;
pub mod tests; // Added to make tests discoverable

use strum_macros::{EnumIter, EnumProperty}; // Removed EnumString

#[derive(Debug, PartialEq, Clone, EnumIter, EnumProperty)] // Removed EnumString
pub enum Token {
    // Prelude
    #[strum(serialize = "✅")]
    True, // ✅
    #[strum(serialize = "❌")]
    False, // ❌
    #[strum(serialize = "➡️")]
    FuncStart, // ➡️
    #[strum(serialize = "∀")]
    Forall, // ∀
    #[strum(serialize = "∃")]
    Exists, // ∃
    #[strum(serialize = "⏫⏫⏫")]
    UpArrow, // ⏫⏫⏫
    #[strum(serialize = "(∧)")]
    And, // (∧)
    #[strum(serialize = "(∨)")]
    Or, // (∨)
    #[strum(serialize = "(¬)")]
    Not, // (¬)
    #[strum(serialize = "(→)")]
    Implies, // (→)
    #[strum(serialize = "(↔)")]
    Iff, // (↔)
    #[strum(serialize = "S")]
    S, // S
    #[strum(serialize = "K")]
    K, // K
    #[strum(serialize = "I")]
    I, // I
    #[strum(serialize = "✨")]
    Sparkle, // ✨ (i32_const)
    I32Const(i32), // For explicit i32.const instruction
    #[strum(serialize = "⚡")]
    Lightning, // ⚡ (f32_const)
    F32Const(f32), // For explicit f32.const instruction
    #[strum(serialize = "B")]
    B, // B
    #[strum(serialize = "C")]
    C, // C
    #[strum(serialize = "W")]
    W, // W
    #[strum(serialize = "Y")]
    Y, // Y
    #[strum(serialize = "Z")]
    Z, // Z
    #[strum(serialize = "Ω")]
    Omega, // Ω
    #[strum(serialize = "Λ")]
    Lambda, // Λ
    #[strum(serialize = "⊤")]
    Top, // ⊤
    #[strum(serialize = "⊥")]
    Bottom, // ⊥
    #[strum(serialize = "↦")]
    MapsTo, // ↦
    #[strum(serialize = "∘")]
    Compose, // ∘
    #[strum(serialize = "=")]
    Equals, // =
    #[strum(serialize = "≠")]
    NotEquals, // ≠
    #[strum(serialize = "≡")]
    Identical, // ≡
    #[strum(serialize = "⊢")]
    Proves, // ⊢
    #[strum(serialize = "⊨️")]
    Entails, // ⊨️

    // WASM Compiler Prelude (conceptual)
    #[strum(serialize = "⚗️")]
    Compiler, // ⚗️
    #[strum(serialize = "🪄")]
    Optimizer, // 🪄
    #[strum(serialize = "📦")]
    Box, // 📦
    #[strum(serialize = "🛡️")]
    CheckTrap, // ️

    // Rules (emoji -> token mapping)
    #[strum(serialize = "🎯")] // Changed from ↩️ to 🎯
    Return, // 🎯
    #[strum(serialize = "🔁")] // Changed from 📞 to 🔁
    Call, // 🔁
    #[strum(serialize = "🧩")] // Changed from 📥 to 🧩
    LocalGet, // 🧩
    #[strum(serialize = "🪞")] // Changed from 📤 to 🪞
    LocalSet, // 🪞
    #[strum(serialize = "🍄")] // Changed from 🌱 to 🍄
    SpawnToken, // 🍄
    #[strum(serialize = "💬")]
    Comment(String), // 💬
    #[strum(serialize = "🧱")]
    EmitWatBlock, // 🧱 (re-used for emit_wat_block)
    #[strum(serialize = "🍃")] // Changed from (... ) to 🍃
    RuleEntry, // 🍃
    #[strum(serialize = "🔁🕳️.apply")] // Changed from ️.apply to 🔁🕳️.apply
    ApplyRulesLoop, // 🔁🕳️.apply

    #[strum(serialize = "⚙️")] // Added strum attribute
    Drop, // ⚙️
    
    // World Tape
    Word(String), // Generic word token
    Integer(i32), // For i32.const
    Float(f32), // For f32.const
    #[strum(serialize = "➕")]
    Add, // ➕
    #[strum(serialize = "➖")]
    Sub, // ➖
    #[strum(serialize = "✖️")]
    Mul, // ✖️
    #[strum(serialize = "➗")]
    DivS, // ➗
    #[strum(serialize = "≻")]
    GtS, // ≻
    #[strum(serialize = "/zos export")]
    ZosExport, // /zos export
    #[strum(serialize = "/zos ready")]
    ZosReady, // /zos ready
    #[strum(serialize = "⏎")] // Newline emoji
    Newline, // For parsing newlines
    Whitespace, // For parsing whitespace
    Other(String), // Catch-all for unparsed characters
}