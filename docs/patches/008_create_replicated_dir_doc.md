# Patch 008: Create Replicated Directory

## Purpose
This patch adds the initial logic to `src/replication.rs` for creating a new directory where the replicated program will reside. It includes logic to remove the directory if it already exists, ensuring a clean slate for each replication.

## Specific Code Changes
- Adds `let new_dir_name = "replicated_program_output";`, `let new_dir_path = PathBuf::from(new_dir_name);`, `if new_dir_path.exists() { fs::remove_dir_all(&new_dir_path)?; }`, and `fs::create_dir(&new_dir_path)?;` to the `replicate_self` function in `src/replication.rs`.
- Adds a `println!` statement to confirm directory creation.

## Verification
- `cargo check` passes successfully after applying this patch, with expected warnings for unused imports.
