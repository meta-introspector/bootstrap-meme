#!/bin/bash
# This script creates empty module files for the replicated program.

# Ensure the src directory exists
mkdir -p src

# Create empty module files
touch src/ingestion.rs
touch src/analysis.rs
touch src/ooda_loop.rs
touch src/embedding.rs
touch src/replication.rs
