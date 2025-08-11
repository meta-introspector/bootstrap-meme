#!/bin/bash
# This script sets the initial content of src/main.rs

# Ensure the target directory exists
mkdir -p src

# Write the content to src/main.rs
cat <<EOF > src/main.rs
mod ingestion;
mod analysis;
mod ooda_loop;
mod embedding;
mod replication; // New module for self-replication

fn main() {
    // For testing self-replication
    replication::replicate_self().expect("Failed to replicate self");
    // Original OODA loop call
    // ooda_loop::run_ooda_loop();
}
EOF
