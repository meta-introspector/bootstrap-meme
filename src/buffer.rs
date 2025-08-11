// src/buffer.rs
//use std::path::PathBuf;

// An index record for a single file loaded into the SystemBuffer.
// This is the canonical definition for the entire system.
#[derive(Debug, Clone)] // Added Clone for easier passing around
pub struct FileRecord {
    pub content: String,
    // If file_path is needed later, it can be added here.
    // pub file_path: PathBuf,
}

/// The main, structured memory buffer for the entire system.
/// This is the canonical definition for the entire system.
#[derive(Debug, Clone)] // Added Clone
pub struct SystemBuffer {
    pub files: Vec<FileRecord>,
}
