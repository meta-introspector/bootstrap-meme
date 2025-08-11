// src/ingestion_refactored.rs
use std::fs;
use walkdir::WalkDir;

// An index record for a single file loaded into the SystemBuffer.
#[derive(Debug)]
pub struct FileRecord {
    pub content: String,
}

/// The main, structured memory buffer for the entire system.
#[derive(Debug)]
pub struct SystemBuffer {
    pub files: Vec<FileRecord>,
}

pub fn ingest_project_files(file_extension_filter: Option<&str>) -> SystemBuffer {
    println!("\n[OODA - OBSERVE] Ingesting project files...\n");
    let mut buffer = SystemBuffer {
        files: Vec::new(),
    };

    let walker = WalkDir::new(".").into_iter();
    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.into_path();
            let path_str = path.to_string_lossy();

            if !path_str.contains("target/") && !path_str.contains(".git/") {
                let should_ingest = if let Some(filter) = file_extension_filter {
                    path.extension().map_or(false, |ext| ext == filter)
                } else {
                    true
                };

                if should_ingest {
                    if let Ok(content) = fs::read_to_string(&path) {
                        buffer.files.push(FileRecord {
                            content,
                        });
                    }
                }
            }
        }
    }
    println!("Total files loaded into buffer: {}", buffer.files.len());
    buffer
}
