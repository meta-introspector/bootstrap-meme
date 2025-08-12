use super::Token;

#[test]
fn test_unused_variants() {
    let _ = Token::CheckTrap;
    let _ = Token::EmitWatBlock;
    let _ = Token::Other("test".to_string());
}
