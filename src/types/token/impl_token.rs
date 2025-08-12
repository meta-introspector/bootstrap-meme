use super::Token;
use std::fmt;
use crate::types::token::emojis; // Import the emojis module
use crate::types::token::executable::ExecutableToken; // Import the trait
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use crate::types::token::emojis::add_token;
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(i) => write!(f, "{i}"),
	    Token::Drop => write!(f, "Drop"),
            Token::Float(fl) => write!(f, "{fl}"),
            Token::Word(s) => write!(f, "{s}"),
            Token::Comment(s) => write!(f, "💬{s}"), // Comments are special
            Token::Other(s) => write!(f, "{s}"),
            Token::True => write!(f, "{}", emojis::true_token::EMOJI),
            Token::False => write!(f, "{}", emojis::false_token::EMOJI),
            Token::FuncStart => write!(f, "{}", emojis::func_start_token::EMOJI),
            Token::Forall => write!(f, "{}", emojis::forall_token::EMOJI),
            Token::Exists => write!(f, "{}", emojis::exists_token::EMOJI),
            Token::UpArrow => write!(f, "{}", emojis::up_arrow_token::EMOJI),
            Token::And => write!(f, "{}", emojis::and_token::EMOJI),
            Token::Or => write!(f, "{}", emojis::or_token::EMOJI),
            Token::Not => write!(f, "{}", emojis::not_token::EMOJI),
            Token::Implies => write!(f, "{}", emojis::implies_token::EMOJI),
            Token::Iff => write!(f, "{}", emojis::iff_token::EMOJI),
            Token::S => write!(f, "{}", emojis::s_token::EMOJI),
            Token::K => write!(f, "{}", emojis::k_token::EMOJI),
            Token::I => write!(f, "{}", emojis::i_token::EMOJI),
            Token::Sparkle => write!(f, "{}", emojis::sparkle_token::EMOJI),
            Token::I32Const(_) => write!(f, "{}", emojis::i32_const_token::EMOJI),
            Token::Lightning => write!(f, "{}", emojis::lightning_token::EMOJI),
            Token::F32Const(_) => write!(f, "{}", emojis::f32_const_token::EMOJI),
            Token::B => write!(f, "{}", emojis::b_token::EMOJI),
            Token::C => write!(f, "{}", emojis::c_token::EMOJI),
            Token::W => write!(f, "{}", emojis::w_token::EMOJI),
            Token::Y => write!(f, "{}", emojis::y_token::EMOJI),
            Token::Z => write!(f, "{}", emojis::z_token::EMOJI),
            Token::Omega => write!(f, "{}", emojis::omega_token::EMOJI),
            Token::Lambda => write!(f, "{}", emojis::lambda_token::EMOJI),
            Token::Top => write!(f, "{}", emojis::top_token::EMOJI),
            Token::Bottom => write!(f, "{}", emojis::bottom_token::EMOJI),
            Token::MapsTo => write!(f, "{}", emojis::maps_to_token::EMOJI),
            Token::Compose => write!(f, "{}", emojis::compose_token::EMOJI),
            Token::Equals => write!(f, "{}", emojis::equals_token::EMOJI),
            Token::NotEquals => write!(f, "{}", emojis::not_equals_token::EMOJI),
            Token::Identical => write!(f, "{}", emojis::identical_token::EMOJI),
            Token::Proves => write!(f, "{}", emojis::proves_token::EMOJI),
            Token::Entails => write!(f, "{}", emojis::entails_token::EMOJI),
            Token::Compiler => write!(f, "{}", emojis::compiler_token::EMOJI),
            Token::Optimizer => write!(f, "{}", emojis::optimizer_token::EMOJI),
            Token::Box => write!(f, "{}", emojis::box_token::EMOJI),
            Token::CheckTrap => write!(f, "{}", emojis::check_trap_token::EMOJI),
            Token::Return => write!(f, "{}", emojis::return_token::EMOJI),
            Token::Call => write!(f, "{}", emojis::call_token::EMOJI),
            Token::LocalGet => write!(f, "{}", emojis::local_get_token::EMOJI),
            Token::LocalSet => write!(f, "{}", emojis::local_set_token::EMOJI),
            Token::SpawnToken => write!(f, "{}", emojis::spawn_token_token::EMOJI),
            Token::EmitWatBlock => write!(f, "{}", emojis::emit_wat_block_token::EMOJI),
            Token::RuleEntry => write!(f, "{}", emojis::rule_entry_token::EMOJI),
            Token::ApplyRulesLoop => write!(f, "{}", emojis::apply_rules_loop_token::EMOJI),
            Token::Add => write!(f, "{}", emojis::add_token::EMOJI),
            Token::Sub => write!(f, "{}", emojis::sub_token::EMOJI),
            Token::Mul => write!(f, "{}", emojis::mul_token::EMOJI),
            Token::DivS => write!(f, "{}", emojis::div_s_token::EMOJI),
            Token::GtS => write!(f, "{}", emojis::gt_s_token::EMOJI),
            Token::ZosExport => write!(f, "{}", emojis::zos_export_token::EMOJI),
            Token::ZosReady => write!(f, "{}", emojis::zos_ready_token::EMOJI),
            Token::Newline => write!(f, "{}", emojis::newline_token::EMOJI),
            Token::Whitespace => write!(f, " "), // Whitespace is a space
        }
    }
}

impl ExecutableToken for Token {
    fn execute(
        &self,
        stack: &mut Vec<i32>,
        locals: &mut HashMap<i32, i32>,
        tokens_iter: &mut Peekable<Iter<Token>>,
    ) -> Result<(), String> {
        match self {
            Token::Integer(i) => {
                stack.push(*i);
                Ok(())
            },
            Token::Add => {
                add_token::execute_add(stack, locals, tokens_iter)
            },
            Token::Sub => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a - b);
                    Ok(())
                } else {
                    Err("Not enough operands for Sub operation.".to_string())
                }
            },
            Token::Mul => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a * b);
                    Ok(())
                } else {
                    Err("Not enough operands for Mul operation.".to_string())
                }
            },
            Token::DivS => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if b == 0 {
                        Err("Division by zero.".to_string())
                    } else {
                        stack.push(a / b);
                        Ok(())
                    }
                } else {
                    Err("Not enough operands for DivS operation.".to_string())
                }
            },
            Token::GtS => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a > b { 1 } else { 0 }); // Push 1 for true, 0 for false
                    Ok(())
                } else {
                    Err("Not enough operands for GtS operation.".to_string())
                }
            },
            Token::Newline => Ok(()), // Ignore newlines in execution
            Token::Whitespace => Ok(()), // Ignore whitespace in execution
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
                    Ok(())
                } else {
                    Err("Not enough operands for S combinator.".to_string())
                }
            },
            Token::K => {
                // K x y = x
                // Requires at least 2 operands on the stack: y, x
                if stack.len() >= 2 {
                    let _y = stack.pop().unwrap(); // Consume y
                    let x = stack.pop().unwrap();
                    stack.push(x);
                    Ok(())
                } else {
                    Err("Not enough operands for K combinator.".to_string())
                }
            },
            Token::I => {
                // I x = x
                // Requires at least 1 operand on the stack: x
                if !stack.is_empty() {
                    let x = stack.pop().unwrap();
                    stack.push(x);
                    Ok(())
                } else {
                    Err("Not enough operands for I combinator.".to_string())
                }
            },
            Token::And => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a != 0 && b != 0 { 1 } else { 0 });
                    Ok(())
                } else {
                    Err("Not enough operands for And operation.".to_string())
                }
            },
            Token::Or => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a != 0 || b != 0 { 1 } else { 0 });
                    Ok(())
                } else {
                    Err("Not enough operands for Or operation.".to_string())
                }
            },
            Token::Not => {
                if !stack.is_empty() {
                    let a = stack.pop().unwrap();
                    stack.push(if a == 0 { 1 } else { 0 });
                    Ok(())
                } else {
                    Err("Not enough operands for Not operation.".to_string())
                }
            },
            Token::Implies => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a == 0 || b != 0 { 1 } else { 0 }); // NOT a OR b
                    Ok(())
                } else {
                    Err("Not enough operands for Implies operation.".to_string())
                }
            },
            Token::Iff => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if (a != 0 && b != 0) || (a == 0 && b == 0) { 1 } else { 0 }); // (a AND b) OR (NOT a AND NOT b)
                    Ok(())
                } else {
                    Err("Not enough operands for Iff operation.".to_string())
                }
            },
            Token::Equals => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a == b { 1 } else { 0 });
                    Ok(())
                } else {
                    Err("Not enough operands for Equals operation.".to_string())
                }
            },
            Token::NotEquals => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a != b { 1 } else { 0 });
                    Ok(())
                } else {
                    Err("Not enough operands for NotEquals operation.".to_string())
                }
            },
            Token::Identical => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a == b { 1 } else { 0 }); // For now, identical is same as equals
                    Ok(())
                } else {
                    Err("Not enough operands for Identical operation.".to_string())
                }
            },
            Token::LocalGet => {
                if !stack.is_empty() {
                    let index = stack.pop().unwrap();
                    if let Some(&value) = locals.get(&index) {
                        stack.push(value);
                        Ok(())
                    } else {
                        Err(format!("Local variable at index {index} not found."))
                    }
                } else {
                    Err("Not enough operands for LocalGet operation.".to_string())
                }
            },
            Token::LocalSet => {
                if stack.len() >= 2 {
                    let index = stack.pop().unwrap();
                    let value = stack.pop().unwrap();
                    locals.insert(index, value);
                    Ok(())
                } else {
                    Err("Not enough operands for LocalSet operation.".to_string())
                }
            },
            Token::Call => {
                if !stack.is_empty() {
                    let func_id = stack.pop().unwrap();
                    match func_id {
                        0 => { // Example: print top of stack
                            if let Some(value) = stack.pop() {
                                println!("Call (func_id 0): {value}");
                                Ok(())
                            } else {
                                Err("Stack empty for print function.".to_string())
                            }
                        },
                        _ => Err(format!("Unknown function ID: {func_id}")),
                    }
                } else {
                    Err("Not enough operands for Call operation.".to_string())
                }
            },
            Token::Drop => {
                if !stack.is_empty() {
                    stack.pop();
                    Ok(())
                } else {
                    Err("Not enough operands for Drop operation.".to_string())
                }
            },
            Token::True => {
                stack.push(1);
                Ok(())
            },
            Token::False => {
                stack.push(0);
                Ok(())
            },
            Token::Sparkle => {
                // Expect next token to be an Integer
                if let Some(next_token) = tokens_iter.next() {
                    if let Token::Integer(i) = next_token {
                        stack.push(*i);
                        Ok(())
                    } else {
                        Err(format!("Expected Integer after Sparkle, got {next_token:?}"))
                    }
                } else {
                    Err("Expected Integer after Sparkle, but found end of tape.".to_string())
                }
            },
            Token::Lightning => {
                // Expect next token to be a Float
                if let Some(next_token) = tokens_iter.next() {
                    if let Token::Float(f) = next_token {
                        stack.push(*f as i32); // Convert float to int for stack
                        Ok(())
                    } else {
                        Err(format!("Expected Float after Lightning, got {next_token:?}"))
                    }
                } else {
                    Err("Expected Float after Lightning, but found end of tape.".to_string())
                }
            },
            Token::Comment(_) => Ok(()), // Ignore comments in execution
            Token::Box => {
                println!("Box (📦) token encountered. This signifies a WAT block or WASM binary emission.");
                Ok(())
            },
            Token::EmitWatBlock => {
                let mut wat_content = String::new();
                let mut paren_count = 0;
                // Consume tokens until matching closing parenthesis
                while let Some(next_token) = tokens_iter.next() {
                    if let Token::Other(s) = next_token {
                        if s == "(" {
                            paren_count += 1;
                        } else if s == ")" {
                            if paren_count == 0 {
                                break; // Found matching closing parenthesis
                            } else {
                                paren_count -= 1;
                            }
                        }
                        wat_content.push_str(s);
                    } else {
                        wat_content.push_str(&next_token.to_string());
                    }
                }
                println!("Emitting WAT Block: {}", wat_content);
                Ok(())
            },
            // Handle other tokens as needed
            _ => {
                Err(format!("Unhandled token in World Tape: {self:?}"))
            } // Placeholder for other tokens
        }
    }
}
