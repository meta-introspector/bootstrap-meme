// src/interpreter.rs

use crate::types::emojitape::Emojitape;
use crate::types::token::Token;

pub fn execute_emojitape(emojitape: &Emojitape) -> Result<(), String> {
    println!("Executing Emojitape...");

    let mut actual_output = String::new(); // Buffer for actual output

    // Execute World Tape
    println!("\n--- World Tape Execution ---");
    let mut stack: Vec<i32> = Vec::new();
    for token in &emojitape.world_tape {
        match token {
            Token::Integer(i) => stack.push(*i),
            Token::Add => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a + b);
                } else {
                    return Err("Not enough operands for Add operation.".to_string());
                }
            },
            Token::Sub => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a - b);
                } else {
                    return Err("Not enough operands for Sub operation.".to_string());
                }
            },
            Token::Mul => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a * b);
                } else {
                    return Err("Not enough operands for Mul operation.".to_string());
                }
            },
            Token::DivS => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if b == 0 {
                        return Err("Division by zero.".to_string());
                    }
                    stack.push(a / b);
                } else {
                    return Err("Not enough operands for DivS operation.".to_string());
                }
            },
            Token::GtS => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a > b { 1 } else { 0 }); // Push 1 for true, 0 for false
                } else {
                    return Err("Not enough operands for GtS operation.".to_string());
                }
            },
            Token::Newline => {}, // Ignore newlines in execution
            Token::Whitespace => {}, // Ignore whitespace in execution
            // Handle other tokens as needed
            _ => {
                return Err(format!("Unhandled token in World Tape: {token:?}"));
            } // Placeholder for other tokens
        }
    }

    // The final result is the top of the stack
    if let Some(result) = stack.last() {
        actual_output = result.to_string();
    }
    
    println!("{actual_output}"); // Print the captured output
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
            println!("  Expected: \"{trimmed_expected}\"");
            println!("  Actual:   \"{trimmed_actual}\"");
        }
        println!("--- End Test Spec Validation ---");
    }


    // Placeholder for apply_rules_loop
    if !emojitape.rules.is_empty() {
        println!("\n--- Applying Rules (Conceptual) ---");
        println!("Rules found, but apply_rules_loop is not yet implemented.");
        println!("--- End Applying Rules ---");
    }

    // Placeholder for /zos export
    if !emojitape.zos_export_definition.is_empty() {
        println!("\n--- /zos Export (Conceptual) ---");
        println!("/zos export functionality is not yet implemented.");
        println!("--- End /zos Export ---");
    }

    println!("\nEmojitape execution complete.");
    Ok(())
}
