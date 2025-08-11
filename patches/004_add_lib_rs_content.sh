#!/bin/bash
# This script sets the content of src/lib.rs.

# Ensure the src directory exists
mkdir -p src

# Write the content to src/lib.rs
cat <<EOF > src/lib.rs
pub mod buffer;
pub use buffer::{FileRecord, SystemBuffer};
EOF
