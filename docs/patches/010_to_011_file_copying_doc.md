# Patch 010-011: File Copying Logic

## Purpose
These patches implement the core logic for copying the current program's source files into the newly created replicated directory. This includes handling both files and directories, and recursively copying subdirectory contents.

## Specific Code Changes
- **Patch 010 (`010_copy_source_files.sh`):** Adds the initial loop to iterate through the current directory's entries, handling files and top-level directories, and skipping `.git`, `target`, and the output directory.
- **Patch 011 (`011_add_recursive_copy.sh`):** Replaces the placeholder for recursive directory copying with a call to a new helper function `copy_dir_all`, and adds the `copy_dir_all` function definition to `src/replication.rs`.

## Verification
- `cargo check` passes successfully after applying these patches.
