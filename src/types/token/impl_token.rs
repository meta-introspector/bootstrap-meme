use super::Token;

use crate::types::token::emojis::{true_token, false_token, func_start_token}; // Import the emoji modules

impl Token {
    pub fn to_string_representation(&self) -> String {
        match self {
            Token::True => true_token::EMOJI.to_string(),
            Token::False => false_token::EMOJI.to_string(),
            Token::FuncStart => func_start_token::EMOJI.to_string(),
            Token::Forall => "∀".to_string(),
            Token::Exists => "∃".to_string(),
            Token::UpArrow => "⏫⏫⏫".to_string(),
            Token::And => "(∧)".to_string(),
            Token::Or => "(∨)".to_string(),
            Token::Not => "(¬)".to_string(),
            Token::Implies => "(→)".to_string(),
            Token::Iff => "(↔)".to_string(),
            Token::S => "S".to_string(),
            Token::K => "K".to_string(),
            Token::I => "I".to_string(),
            Token::Sparkle => "✨".to_string(),
            Token::Lightning => "⚡".to_string(),
            Token::B => "B".to_string(),
            Token::C => "C".to_string(),
            Token::W => "W".to_string(),
            Token::Y => "Y".to_string(),
            Token::Z => "Z".to_string(),
            Token::Omega => "Ω".to_string(),
            Token::Lambda => "Λ".to_string(),
            Token::Top => "⊤".to_string(),
            Token::Bottom => "⊥".to_string(),
            Token::MapsTo => "↦".to_string(),
            Token::Compose => "∘".to_string(),
            Token::Equals => "=".to_string(),
            Token::NotEquals => "≠".to_string(),
            Token::Identical => "≡".to_string(),
            Token::Proves => "⊢".to_string(),
            Token::Entails => "⊨️".to_string(),
            Token::Compiler => "⚗️".to_string(),
            Token::Optimizer => "⚙️".to_string(),
            Token::Box => "📦".to_string(),
            Token::CheckTrap => "CheckTrap".to_string(), // Placeholder, as no emoji defined
            Token::Return => "↩️".to_string(),
            Token::Call => "📞".to_string(),
            Token::LocalGet => "📥".to_string(),
            Token::LocalSet => "📤".to_string(),
            Token::SpawnToken => "🌱".to_string(),
            Token::Comment(s) => format!("💬{s}"),
            Token::EmitWatBlock => "EmitWatBlock".to_string(), // Placeholder
            Token::RuleEntry => "(… )".to_string(),
            Token::ApplyRulesLoop => ".apply".to_string(),
            Token::Word(s) => s.clone(),
            Token::Integer(i) => i.to_string(),
            Token::Float(f) => f.to_string(),
            Token::Add => "➕".to_string(),
            Token::Sub => "➖".to_string(),
            Token::Mul => "✖️".to_string(),
            Token::DivS => "➗".to_string(),
            Token::GtS => "≻".to_string(),
            Token::ZosExport => "/zos export".to_string(),
            Token::ZosReady => "/zos ready".to_string(),
            Token::Newline => "\n".to_string(),
            Token::Whitespace => " ".to_string(),
            Token::Other(s) => s.clone(),
        }
    }
}

