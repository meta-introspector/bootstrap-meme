// RUST_CODE_BLOCK_UNPARSABLE:pub mod emojitape;
pub mod rule;
pub mod token;

#[cfg(test)]
mod tests {
    use super::token::Token;

    #[test]
    fn test_unused_variants() {
        let _ = Token::CheckTrap;
        let _ = Token::EmitWatBlock;
        let _ = Token::Other("test".to_string());
    }
}