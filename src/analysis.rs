// src/analysis.rs
use std::collections::HashMap;
//use crate::buffer::SystemBuffer; // Import SystemBuffer from canonical buffer module
use system_e_schema_core::buffer::{SystemBuffer}; // Import from canonical buffer module

pub fn perform_vocabulary_analysis(buffer: &SystemBuffer) -> HashMap<String, u32> {
    println!("    -> [Executing Module: Vocabulary Analysis]");
    let mut term_counts: HashMap<String, u32> = HashMap::new();

    for file in &buffer.files {
        for token in file.content.split_whitespace() {
            let clean_token = token.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
            if !clean_token.is_empty() {
                *term_counts.entry(clean_token).or_insert(0) += 1;
            }
        }
    }
    term_counts // Return the HashMap directly
}
