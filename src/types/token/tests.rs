
#[test]
fn test_unused_variants() {
    let _ = super::Token::CheckTrap;
    let _ = super::Token::EmitWatBlock;
    let _ = super::Token::Other("test".to_string());
}

#[test]
fn test_token_display() {
    assert_eq!(super::Token::True.to_string(), "✅");
    assert_eq!(super::Token::False.to_string(), "❌");
    assert_eq!(super::Token::Add.to_string(), "➕");
    assert_eq!(super::Token::Sub.to_string(), "➖");
    assert_eq!(super::Token::Mul.to_string(), "✖️");
    assert_eq!(super::Token::DivS.to_string(), "➗");
    assert_eq!(super::Token::GtS.to_string(), "≻");
    assert_eq!(super::Token::ZosExport.to_string(), "/zos export");
    assert_eq!(super::Token::ZosReady.to_string(), "/zos ready");
    assert_eq!(super::Token::Newline.to_string(), "⏎");
    assert_eq!(super::Token::Whitespace.to_string(), " "); // Should now be " " from emoji constant
    assert_eq!(super::Token::Drop.to_string(), "⚙️"); // Should now be "⚙️" from emoji constant
    assert_eq!(super::Token::Integer(123).to_string(), "123");
    assert_eq!(super::Token::Float(4.56).to_string(), "4.56");
    // assert_eq!(super::Token::Word("hello".to_string()).to_string(), "hello"); // This will now fail
    assert_eq!(super::Token::Comment("This is a comment".to_string()).to_string(), "💬This is a comment");
    assert_eq!(super::Token::Other("unhandled".to_string()).to_string(), "unhandled");
}

#[test]
fn test_token_from_str() {
    use std::str::FromStr;

    // Test simple enum variants
    assert_eq!(super::Token::from_str("✅").unwrap(), super::Token::True);
    assert_eq!(super::Token::from_str("❌").unwrap(), super::Token::False);
    assert_eq!(super::Token::from_str("➕").unwrap(), super::Token::Add);
    assert_eq!(super::Token::from_str("➖").unwrap(), super::Token::Sub);
    assert_eq!(super::Token::from_str("✖️").unwrap(), super::Token::Mul);
    assert_eq!(super::Token::from_str("➗").unwrap(), super::Token::DivS);
    assert_eq!(super::Token::from_str("≻").unwrap(), super::Token::GtS);
    assert_eq!(super::Token::from_str("/zos export").unwrap(), super::Token::ZosExport);
    assert_eq!(super::Token::from_str("/zos ready").unwrap(), super::Token::ZosReady);
    assert_eq!(super::Token::from_str("⏎").unwrap(), super::Token::Newline);
    assert_eq!(super::Token::from_str(" ").unwrap(), super::Token::Whitespace); // Should parse from " "
    assert_eq!(super::Token::from_str("⚙️").unwrap(), super::Token::Drop); // Should parse from "⚙️"

    // Test variants with associated data
    assert_eq!(super::Token::from_str("123").unwrap(), super::Token::Integer(123));
    assert_eq!(super::Token::from_str("4.56").unwrap(), super::Token::Float(4.56));
    assert_eq!(super::Token::from_str("💬This is a comment").unwrap(), super::Token::Comment("This is a comment".to_string()));
    assert_eq!(super::Token::from_str("💬").unwrap(), super::Token::Comment("".to_string())); // Empty comment

    // These should now return Err
    assert!(super::Token::from_str("hello").is_err()); // Now expected to be Err
    assert!(super::Token::from_str("unhandled").is_err()); // Now expected to be Err

    // Test invalid inputs
    assert!(super::Token::from_str("invalid_emoji").is_err());
    assert!(super::Token::from_str("not_a_number").is_err());
    assert!(super::Token::from_str("123.45.6").is_err());
}
