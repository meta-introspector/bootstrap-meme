use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

// An index record for a single file loaded into the SystemBuffer.
#[derive(Debug)]
pub struct FileRecord {
    pub file_path: PathBuf,
    pub content: String,
}

/// The main, structured memory buffer for the entire system.
#[derive(Debug)]
pub struct SystemBuffer {
    pub files: Vec<FileRecord>,
}

fn main() {
    println!("--- Starting System `e` Quine Bootstrap ---");

    let mut buffer = SystemBuffer {
        files: Vec::new(),
    };

    // 1. Ingest all source files from the current directory.
    println!("\n[PHASE 1] Ingesting project files...");
    let walker = WalkDir::new(".").into_iter();
    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.into_path();
            if !path.to_string_lossy().contains("target/") {
                if let Ok(content) = fs::read_to_string(&path) {
                    buffer.files.push(FileRecord {
                        file_path: path,
                        content,
                    });
                }
            }
        }
    }
    println!("Total files loaded into buffer: {}", buffer.files.len());

    // 2. Load and execute modules.
    println!("\n[PHASE 2] Loading and executing modules...");
    let module_dir = PathBuf::from("modules");
    if module_dir.is_dir() {
        for entry in fs::read_dir(module_dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_file() && path.extension().map_or(false, |e| e == "md") {
                println!("  -> Found executable module: {:?}", path);
                let content = fs::read_to_string(&path).unwrap();
                
                // In a real system, we would parse, compile, and dynamically link this code.
                // For this simulation, we will just check for the function's existence.
                if content.contains("fn perform_vocabulary_analysis") {
                    // Conceptually, we run the function here.
                    let report = perform_vocabulary_analysis(&buffer);
                    println!("    -> [Module Report]: {}", report);
                }
            }
        }
    }

    println!("\n--- Quine bootstrap complete. System state loaded and analyzed. ---");
}


// This is the function that would be conceptually extracted from the markdown file.
// In a real implementation, this would be handled by a build script or dynamic compilation.
pub fn perform_vocabulary_analysis(buffer: &SystemBuffer) -> String {
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
    format!("Vocabulary analysis complete. Found {} unique terms.", term_counts.len())
}
