// src/interpreter.rs

use crate::types::emojitape::Emojitape;
use crate::types::token::Token;

pub fn execute_emojitape(emojitape: &Emojitape) -> Result<(), String> {
    println!("Executing Emojitape...");

    let mut actual_output = String::new(); // Buffer for actual output

    // Execute World Tape (simplified for now)
    println!("\n--- World Tape Execution ---");
    for token in &emojitape.world_tape {
        match token {
            Token::Integer(i) => actual_output.push_str(&format!("{} ", i)),
            Token::Float(f) => actual_output.push_str(&format!("{} ", f)),
            Token::Add => actual_output.push_str("+ "),
            Token::Sub => actual_output.push_str("- "),
            Token::Mul => actual_output.push_str("* "),
            Token::DivS => actual_output.push_str("/ "),
            Token::GtS => actual_output.push_str("> "),
            Token::FuncStart => actual_output.push_str("(func "),
            Token::Return => actual_output.push_str("return)\n"),
            Token::Sparkle => actual_output.push_str("i32.const "),
            Token::Lightning => actual_output.push_str("f32.const "),
            Token::Word(w) => actual_output.push_str(&format!("{} ", w)),
            Token::Newline => actual_output.push_str("\n"),
            Token::Whitespace => actual_output.push_str(" "),
            Token::Comment(_) => actual_output.push_str("// (comment)\n"),
            // Handle other tokens as needed
            _ => actual_output.push_str(&format!("{:?} ", token)), // For debugging unhandled tokens
        }
    }
    println!("{}", actual_output); // Print the captured output
    println!("\n--- End World Tape Execution ---");

    // Test Spec Validation
    if let Some(expected) = &emojitape.expected_output {
        println!("\n--- Test Spec Validation ---");
        let trimmed_actual = actual_output.trim();
        let trimmed_expected = expected.trim();
        if trimmed_actual == trimmed_expected {
            println!("✅ Test Passed: Actual output matches expected output.");
        } else {
            println!("❌ Test Failed:");
            println!("  Expected: \"{}\"", trimmed_expected);
            println!("  Actual:   \"{}\"", trimmed_actual);
        }
        println!("--- End Test Spec Validation ---");
    }


    // Placeholder for apply_rules_loop
    if emojitape.rules.len() > 0 {
        println!("\n--- Applying Rules (Conceptual) ---");
        println!("Rules found, but apply_rules_loop is not yet implemented.");
        println!("--- End Applying Rules ---");
    }

    // Placeholder for /zos export
    if emojitape.zos_export_definition.len() > 0 {
        println!("\n--- /zos Export (Conceptual) ---");
        println!("/zos export functionality is not yet implemented.");
        println!("--- End /zos Export ---");
    }

    println!("\nEmojitape execution complete.");
    Ok(())
}
