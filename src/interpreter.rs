// src/interpreter.rs

use crate::types::emojitape::Emojitape;
use crate::types::token::Token;
use crate::types::rule::Rule;
use std::collections::HashMap;

pub fn execute_emojitape(emojitape: &Emojitape) -> Result<(), String> {
    println!("Executing Emojitape...");

    let mut actual_output = String::new(); // Buffer for actual output

    // Execute World Tape
    println!("\n--- World Tape Execution ---");
    let mut stack: Vec<i32> = Vec::new();
    let mut locals: HashMap<i32, i32> = HashMap::new();
    let mut tokens_iter = emojitape.world_tape.iter().peekable();

    while let Some(token) = tokens_iter.next() {
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
            Token::S => {
                // S f g x = f x (g x)
                // Requires at least 3 operands on the stack: x, g, f
                if stack.len() >= 3 {
                    let x = stack.pop().unwrap();
                    let g = stack.pop().unwrap();
                    let f = stack.pop().unwrap();
                    // This is a simplified interpretation. In a real combinator system,
                    // f, g, and x would be functions or values. Here, we're just
                    // manipulating their integer representations on the stack.
                    // A more complex implementation would involve a way to represent and apply functions.
                    // For now, let's just push them back in the order of application.
                    stack.push(f);
                    stack.push(x);
                    stack.push(g);
                    stack.push(x);
                } else {
                    return Err("Not enough operands for S combinator.".to_string());
                }
            },
            Token::K => {
                // K x y = x
                // Requires at least 2 operands on the stack: y, x
                if stack.len() >= 2 {
                    let _y = stack.pop().unwrap(); // Consume y
                    let x = stack.pop().unwrap();
                    stack.push(x);
                } else {
                    return Err("Not enough operands for K combinator.".to_string());
                }
            },
            Token::I => {
                // I x = x
                // Requires at least 1 operand on the stack: x
                if !stack.is_empty() {
                    let x = stack.pop().unwrap();
                    stack.push(x);
                } else {
                    return Err("Not enough operands for I combinator.".to_string());
                }
            },
            Token::And => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a != 0 && b != 0 { 1 } else { 0 });
                } else {
                    return Err("Not enough operands for And operation.".to_string());
                }
            },
            Token::Or => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a != 0 || b != 0 { 1 } else { 0 });
                } else {
                    return Err("Not enough operands for Or operation.".to_string());
                }
            },
            Token::Not => {
                if !stack.is_empty() {
                    let a = stack.pop().unwrap();
                    stack.push(if a == 0 { 1 } else { 0 });
                } else {
                    return Err("Not enough operands for Not operation.".to_string());
                }
            },
            Token::Implies => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a == 0 || b != 0 { 1 } else { 0 }); // NOT a OR b
                } else {
                    return Err("Not enough operands for Implies operation.".to_string());
                }
            },
            Token::Iff => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if (a != 0 && b != 0) || (a == 0 && b == 0) { 1 } else { 0 }); // (a AND b) OR (NOT a AND NOT b)
                } else {
                    return Err("Not enough operands for Iff operation.".to_string());
                }
            },
            Token::Equals => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a == b { 1 } else { 0 });
                } else {
                    return Err("Not enough operands for Equals operation.".to_string());
                }
            },
            Token::NotEquals => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a != b { 1 } else { 0 });
                } else {
                    return Err("Not enough operands for NotEquals operation.".to_string());
                }
            },
            Token::Identical => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a == b { 1 } else { 0 }); // For now, identical is same as equals
                } else {
                    return Err("Not enough operands for Identical operation.".to_string());
                }
            },
            Token::LocalGet => {
                if !stack.is_empty() {
                    let index = stack.pop().unwrap();
                    if let Some(&value) = locals.get(&index) {
                        stack.push(value);
                    } else {
                        return Err(format!("Local variable at index {index} not found."));
                    }
                } else {
                    return Err("Not enough operands for LocalGet operation.".to_string());
                }
            },
            Token::LocalSet => {
                if stack.len() >= 2 {
                    let index = stack.pop().unwrap();
                    let value = stack.pop().unwrap();
                    locals.insert(index, value);
                } else {
                    return Err("Not enough operands for LocalSet operation.".to_string());
                }
            },
            Token::Call => {
                if !stack.is_empty() {
                    let func_id = stack.pop().unwrap();
                    match func_id {
                        0 => { // Example: print top of stack
                            if let Some(value) = stack.pop() {
                                println!("Call (func_id 0): {value}");
                            } else {
                                return Err("Stack empty for print function.".to_string());
                            }
                        },
                        _ => return Err(format!("Unknown function ID: {func_id}")),
                    }
                } else {
                    return Err("Not enough operands for Call operation.".to_string());
                }
            },
            Token::Drop => {
                if !stack.is_empty() {
                    stack.pop();
                } else {
                    return Err("Not enough operands for Drop operation.".to_string());
                }
            },
            Token::True => {
                stack.push(1);
            },
            Token::False => {
                stack.push(0);
            },
            Token::Sparkle => {
                // Expect next token to be an Integer
                if let Some(next_token) = tokens_iter.next() {
                    if let Token::Integer(i) = next_token {
                        stack.push(*i);
                    } else {
                        return Err(format!("Expected Integer after Sparkle, got {next_token:?}"));
                    }
                } else {
                    return Err("Expected Integer after Sparkle, but found end of tape.".to_string());
                }
            },
            Token::Lightning => {
                // Expect next token to be a Float
                if let Some(next_token) = tokens_iter.next() {
                    if let Token::Float(f) = next_token {
                        stack.push(*f as i32); // Convert float to int for stack
                    } else {
                        return Err(format!("Expected Float after Lightning, got {next_token:?}"));
                    }
                } else {
                    return Err("Expected Float after Lightning, but found end of tape.".to_string());
                }
            },
            Token::Comment(_) => {}, // Ignore comments in execution
            Token::Box => {
                println!("Box (📦) token encountered. This signifies a WAT block or WASM binary emission.");
            },
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
        println!("\n--- Applying Rules ---");
        let mut current_tape = emojitape.world_tape.clone();
        let mut changed = true;
        while changed {
            changed = false;
            let mut next_tape = Vec::new();
            for rule in &emojitape.rules {
                let new_tape = match_and_replace(&current_tape, rule);
                if new_tape != current_tape {
                    changed = true;
                    current_tape = new_tape;
                    break; // Restart iteration over rules if a change occurred
                }
            }
            // If no rule applied in a full pass, stop
            if !changed {
                next_tape = current_tape.clone();
            }
            current_tape = next_tape;
        }
        // Update world_tape with the result of rule application
        // This might need to be handled differently depending on the semantics of rules
        // For now, let's just print the result
        println!("Rules applied. Final tape: {current_tape:?}");
        println!("--- End Applying Rules ---");
    }

    // Placeholder for /zos export
    if !emojitape.zos_export_definition.is_empty() {
        println!("\n--- /zos Export ---");
        // For now, let's just print the entire emojitape object to a file
        let output_filename = "exported_emojitape.txt"; // Placeholder filename
        match std::fs::write(output_filename, format!(r#"{emojitape:#?}"#)) {
            Ok(_) => println!("Emojitape exported to {output_filename}"),
            Err(e) => println!("Error exporting emojitape: {e}"),
        }
        println!("--- End /zos Export ---");
    }

    println!("\nEmojitape execution complete.");
    Ok(())
}

fn match_and_replace(target: &[Token], rule: &Rule) -> Vec<Token> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < target.len() {
        if i + rule.pattern.len() <= target.len() && target[i..i + rule.pattern.len()] == rule.pattern[..] {
            // Match found, append replacement
            result.extend_from_slice(&rule.replacement);
            i += rule.pattern.len(); // Advance past the matched pattern
        } else {
            // No match, append current token
            result.push(target[i].clone());
            i += 1;
        }
    }
    result
}
