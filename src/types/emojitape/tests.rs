use crate::types::emojitape::Emojitape;
use crate::types::token::Token;

#[test]
fn test_render() {
    let emojitape = Emojitape {
        prelude: vec![Token::True, Token::False],
        wasm_compiler_prelude: vec![Token::Compiler, Token::Optimizer],
        rules: vec![], // Rules are more complex, will test separately if needed
        world_tape: vec![Token::Integer(42), Token::Word("hello".to_string())],
        generated_wat_block: vec![Token::EmitWatBlock, Token::Other("some_wat".to_string())],
        clues_keys: vec![Token::Sparkle],
        zos_export_definition: vec![Token::ZosExport],
        zos_export_implementation: vec![Token::ZosReady],
        self_reproducing_footer: vec![Token::Omega],
        expected_output: None,
    };

    let rendered = emojitape.render();

    // Assertions for prelude
    assert!(rendered.contains("💬--- PRELUDE\n"));
    assert!(rendered.contains("✅"));
    assert!(rendered.contains("❌"));

    // Assertions for WASM Compiler Prelude
    assert!(rendered.contains("💬--- WASM COMPILER PRELUDE\n"));
    assert!(rendered.contains("⚗️"));
    assert!(rendered.contains("🪄"));

    // Assertions for World Tape
    assert!(rendered.contains("💬--- WORLD TAPE\n"));
    assert!(rendered.contains("42"));
    assert!(rendered.contains("hello"));

    // Assertions for Generated WAT Block (this will also test EmitWatBlock and Other)
    assert!(rendered.contains("💬--- GENERATED WAT BLOCK\n"));
    assert!(rendered.contains("🧱"));
    assert!(rendered.contains("some_wat"));

    // Assertions for Clues & Keys
    assert!(rendered.contains("💬--- CLUES & KEYS\n"));
    assert!(rendered.contains("✨"));

    // Assertions for ZOS Export Definition
    assert!(rendered.contains("💬--- /zos export definition\n"));
    assert!(rendered.contains("/zos export"));

    // Assertions for ZOS Export Implementation
    assert!(rendered.contains("💬--- /zos export implementation\n"));
    assert!(rendered.contains("/zos ready"));

    // Assertions for Self-Reproducing Footer
    assert!(rendered.contains("💬--- SELF-REPRODUCING FOOTER\n"));
    assert!(rendered.contains("Ω"));
}