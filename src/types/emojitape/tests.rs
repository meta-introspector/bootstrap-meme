use crate::types::emojitape::Emojitape;
use crate::types::token::Token;

#[test]
fn test_render() {
    let emojitape = Emojitape {
        prelude: vec![Token::True, Token::False],
        wasm_compiler_prelude: vec![Token::Compiler],
        rules: vec![],
        world_tape: vec![Token::Integer(42)],
        generated_wat_block: vec![Token::Box],
        clues_keys: vec![],
        zos_export_definition: vec![],
        zos_export_implementation: vec![],
        self_reproducing_footer: vec![],
    };

    let rendered = emojitape.render();
    assert!(rendered.contains("✅"));
    assert!(rendered.contains("❌"));
    assert!(rendered.contains("⚗️"));
    assert!(rendered.contains("42"));
    assert!(rendered.contains("📦"));
}
