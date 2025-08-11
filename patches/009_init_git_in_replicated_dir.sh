#!/bin/bash
# This script adds logic to initialize a Git repository in the replicated program directory.

# Write the content to src/replication.rs
cat <<EOF > src/replication.rs
use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};

pub fn replicate_self() -> Result<(), Box<dyn std::error::Error>> {
    println!("Replicating self...");

    let new_dir_name = "replicated_program_output";
    let new_dir_path = PathBuf::from(new_dir_name);

    if new_dir_path.exists() {
        fs::remove_dir_all(&new_dir_path)?;
    }
    fs::create_dir(&new_dir_path)?;
    println!("Created new directory: {:?}", new_dir_path);

    // Initialize Git repository
    Command::new("git")
        .arg("init")
        .arg(&new_dir_path)
        .output()?;
    println!("Initialized Git repository in: {:?}", new_dir_path);

    // TODO: Add actual replication logic here
    Ok(())
}
EOF
