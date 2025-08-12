use crate::types::token::Token;
use syn::{visit::{self, Visit}, ItemFn, ExprMacro};

pub struct RustToEmojiVisitor<'a> {
    tokens: &'a mut Vec<Token>,
}

impl<'a> RustToEmojiVisitor<'a> {
    pub fn new(tokens: &'a mut Vec<Token>) -> Self {
        RustToEmojiVisitor { tokens }
    }
}

impl<'a> visit::Visit<'a> for RustToEmojiVisitor<'a> {
    fn visit_item_fn(&mut self, i: &'a syn::ItemFn) {
        self.tokens.push(Token::Comment(format!("FN_START:{}", i.sig.ident)));
        visit::walk_item_fn(self, i);
        self.tokens.push(Token::Comment("FN_END".to_string()));
    }

    fn visit_expr_macro(&mut self, i: &'a syn::ExprMacro) {
        if let Some(path_segment) = i.mac.path.segments.last() {
            if path_segment.ident == "println" {
                self.tokens.push(Token::Comment("PRINTLN_MACRO".to_string()));
            }
        }
        visit::walk_expr_macro(self, i);
    }

    // TODO: Implement other visit methods for different Rust constructs
}