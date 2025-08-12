use crate::types::emojitape::types::Emojitape;
use crate::types::token::Token;
use crate::types::rule::Rule;
use std::collections::HashMap;
use crate::types::token::emojis::drop_token;
use crate::types::token::emojis::true_token;
use crate::types::token::emojis::false_token;
use crate::types::token::emojis::and_token;
use crate::types::token::emojis::or_token;
use crate::types::token::emojis::not_token;
use crate::types::token::emojis::implies_token;
use crate::types::token::emojis::iff_token;
use crate::types::token::emojis::s_token;
use crate::types::token::emojis::k_token;
use crate::types::token::emojis::i_token;
use crate::types::token::emojis::sparkle_token;
use crate::types::token::emojis::lightning_token;
use crate::types::token::emojis::equals_token;
use crate::types::token::emojis::not_equals_token;
use crate::types::token::emojis::identical_token;
use crate::types::token::emojis::box_token;
use crate::types::token::emojis::call_token;
use crate::types::token::emojis::local_get_token;
use crate::types::token::emojis::local_set_token;
use crate::types::token::emojis::emit_wat_block_token;
use crate::types::token::emojis::add_token;
use crate::types::token::emojis::sub_token;
use crate::types::token::emojis::mul_token;
use crate::types::token::emojis::div_s_token;
use crate::types::token::emojis::gt_s_token;
use crate::types::token::emojis::newline_token;
use crate::types::token::emojis::i32_const_token;
use crate::types::token::emojis::whitespace_token;
use crate::types::token::emojis::comment_token;
use crate::types::token::emojis::unhandled_token;

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
            Token::Integer(i) => {
                i32_const_token::execute_i32_const(i, &mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Add => {
                add_token::execute_add(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Sub => {
                sub_token::execute_sub(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Mul => {
                mul_token::execute_mul(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::DivS => {
                div_s_token::execute_div_s(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::GtS => {
                gt_s_token::execute_gt_s(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Newline => {
                newline_token::execute_newline(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Whitespace => {
                whitespace_token::execute_whitespace(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::S => {
                s_token::execute_s(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::K => {
                k_token::execute_k(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::I => {
                i_token::execute_i(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::And => {
                and_token::execute_and(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Or => {
                or_token::execute_or(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Not => {
                not_token::execute_not(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Implies => {
                implies_token::execute_implies(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Iff => {
                iff_token::execute_iff(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Equals => {
                equals_token::execute_equals(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::NotEquals => {
                not_equals_token::execute_not_equals(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Identical => {
                identical_token::execute_identical(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::LocalGet => {
                local_get_token::execute_local_get(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::LocalSet => {
                local_set_token::execute_local_set(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Call => {
                call_token::execute_call(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Drop => {
                drop_token::execute_drop(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::True => {
                true_token::execute_true(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::False => {
                false_token::execute_false(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Sparkle => {
                sparkle_token::execute_sparkle(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Lightning => {
                lightning_token::execute_lightning(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Comment(s) => {
                comment_token::execute_comment(&s, &mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::Box => {
                box_token::execute_box(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            Token::EmitWatBlock => {
                emit_wat_block_token::execute_emit_wat_block(&mut stack, &mut locals, &mut tokens_iter)?;
            },
            _ => {
                unhandled_token::execute_unhandled_token(token, &mut stack, &mut locals, &mut tokens_iter)?;
            }
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
            println!("    Actual:   \"{trimmed_actual}\"");
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
        println!("Rules applied. Final tape: {{current_tape:#?}}");
        println!("--- End Applying Rules ---");
    }

    // Placeholder for /zos export
    if !emojitape.zos_export_definition.is_empty() {
        println!("\n--- /zos Export ---");
        // For now, let's just print the entire emojitape object to a file
        let output_filename = "exported_emojitape.txt"; // Placeholder filename
        match std::fs::write(output_filename, format!(r#"{{emojitape:#?}}"#)) {
            Ok(_) => println!("Emojitape exported to {{output_filename}}"),
            Err(e) => println!("Error exporting emojitape: {{e}}"),
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