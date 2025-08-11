#!/bin/bash
# This script adds logic to commit replicated files to the new Git repository.

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

    // Copy source files
    let current_dir = std::env::current_dir()?;
    for entry in fs::read_dir(&current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(&current_dir)?;
        let dest_path = new_dir_path.join(relative_path);

        if path.is_file() {
            fs::copy(&path, &dest_path)?;
        }
        else if path.is_dir() {
            if relative_path.to_str() == Some(".git") || relative_path.to_str() == Some("target") || relative_path.to_str() == Some("replicated_program_output") {
                continue; // Skip .git, target, and the output directory itself
            }
            // Recursively copy contents of subdirectories
            copy_dir_all(&path, &dest_path)?;
        }
    }
    println!("Copied source files to: {:?}", new_dir_path);

    // Commit copied files
    Command::new("git")
        .arg("-C")
        .arg(&new_dir_path)
        .arg("add")
        .arg(".")
        .output()?;
    Command::new("git")
        .arg("-C")
        .arg(&new_dir_path)
        .arg("commit")
        .arg("-m")
        .arg("feat: Initial commit of replicated program")
        .output()?;
    println!("Committed replicated files in: {:?}", new_dir_path);

    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
EOF
