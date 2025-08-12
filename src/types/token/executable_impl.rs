use super::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use super::executable::ExecutableToken;
use super::impl_token::*;

impl ExecutableToken for Token {
    fn execute(
        &self,
        stack: &mut Vec<i32>,
        locals: &mut HashMap<i32, i32>,
        tokens_iter: &mut Peekable<Iter<Token>>,
    ) -> Result<(), String> {
        match self {
            Token::Integer(i) => integer::execute_integer(i, stack, locals, tokens_iter),
            Token::I32Const(i) => i32_const::execute_i32_const(i, stack, locals, tokens_iter),
            Token::Add => add::execute_add(stack, locals, tokens_iter),
            Token::Sub => sub::execute_sub(stack, locals, tokens_iter),
            Token::Mul => mul::execute_mul(stack, locals, tokens_iter),
            Token::DivS => div_s::execute_div_s(stack, locals, tokens_iter),
            Token::GtS => gt_s::execute_gt_s(stack, locals, tokens_iter),
            Token::Newline => newline::execute_newline(stack, locals, tokens_iter),
            Token::Whitespace => whitespace::execute_whitespace(stack, locals, tokens_iter),
            Token::S => s::execute_s(stack, locals, tokens_iter),
            Token::K => k::execute_k(stack, locals, tokens_iter),
            Token::I => i::execute_i(stack, locals, tokens_iter),
            Token::And => and::execute_and(stack, locals, tokens_iter),
            Token::Or => or::execute_or(stack, locals, tokens_iter),
            Token::Not => not::execute_not(stack, locals, tokens_iter),
            Token::Implies => implies::execute_implies(stack, locals, tokens_iter),
            Token::Iff => iff::execute_iff(stack, locals, tokens_iter),
            Token::Equals => equals::execute_equals(stack, locals, tokens_iter),
            Token::NotEquals => not_equals::execute_not_equals(stack, locals, tokens_iter),
            Token::Identical => identical::execute_identical(stack, locals, tokens_iter),
            Token::LocalGet => local_get::execute_local_get(stack, locals, tokens_iter),
            Token::LocalSet => local_set::execute_local_set(stack, locals, tokens_iter),
            Token::Call => call::execute_call(stack, locals, tokens_iter),
            Token::Drop => drop::execute_drop(stack, locals, tokens_iter),
            Token::True => r#true::execute_true(stack, locals, tokens_iter),
            Token::False => r#false::execute_false(stack, locals, tokens_iter),
            Token::Sparkle => sparkle::execute_sparkle(stack, locals, tokens_iter),
            Token::Lightning => lightning::execute_lightning(stack, locals, tokens_iter),
            Token::Comment(s) => comment::execute_comment(s, stack, locals, tokens_iter),
            Token::Box => r#box::execute_box(stack, locals, tokens_iter),
            Token::EmitWatBlock => emit_wat_block::execute_emit_wat_block(stack, locals, tokens_iter),
            _ => Err(format!("Unhandled token in World Tape: {self:?}")),
        }
    }
}
