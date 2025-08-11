## SOP: Documenting Self-Replication and Cocci-Driven Development

### 1. Purpose
This Standard Operating Procedure (SOP) outlines the process and methodology used to implement and document the self-replication capability of the `model-builder-quiz` program. It details the "Cocci-Driven Development" approach, where program construction and evolution are achieved through a series of auditable, atomic transformations.

### 2. Scope
This SOP applies to the documentation of the `replicate_self` function within `src/replication.rs` and the overall process of building and verifying the self-replicating program using a patch-based, iterative development methodology.

### 3. Methodology: Cocci-Driven Development
The program's construction adheres to the "Cocci-Driven Development" SOP (`docs/sops/cocci_driven_development_sop.md`), which emphasizes:
*   **Atomic Transformations:** Each change is a single, logical step.
*   **Patch-Based Application:** Transformations are applied via numbered scripts (shell scripts for initial file content, `.cocci` files for semantic refactoring).
*   **Iterative Process:** Each patch is applied, verified, documented, and committed.
*   **Auditable History:** Every step is recorded in Git, providing a clear lineage of the program's evolution.

### 4. Self-Replication Implementation Details
The core self-replication logic is encapsulated within the `replicate_self` function in `src/replication.rs`. This function performs the following steps:

#### 4.1. Directory Creation
*   **Patch:** `replicated_program_cocci/patches/008_create_replicated_dir.sh`
*   **Description:** Creates a new output directory (`replicated_program_output`) for the replicated program. It ensures a clean slate by removing any existing directory with the same name.

#### 4.2. Git Repository Initialization
*   **Patch:** `replicated_program_cocci/patches/009_init_git_in_replicated_dir.sh`
*   **Description:** Initializes a new Git repository within the `replicated_program_output` directory, preparing it for version control of the replicated codebase.

#### 4.3. Source File Copying
*   **Patches:**
    *   `replicated_program_cocci/patches/010_copy_source_files.sh`
    *   `replicated_program_cocci/patches/011_add_recursive_copy.sh`
*   **Description:** Copies all relevant source files and directories from the current program's location into the `replicated_program_output` directory. This process includes recursive copying of subdirectories and intelligent skipping of Git-related files, build artifacts (`target`), and the output directory itself to prevent infinite loops.

#### 4.4. Initial Commit of Replicated Program
*   **Patch:** `replicated_program_cocci/patches/012_commit_replicated_files.sh`
*   **Description:** Stages all copied files within the `replicated_program_output` Git repository and performs an initial commit, effectively creating a version-controlled snapshot of the replicated program.

### 5. Verification of Self-Replication
The self-replication capability was verified by:
1.  Building the `replicated_program_cocci` project (`cargo build -p replicated_program_cocci`).
2.  Running the executable (`cargo run -p replicated_program_cocci`).
3.  Observing the console output confirming directory creation, Git initialization, file copying, and commit.
4.  Inspecting the `replicated_program_output` directory to confirm the presence of copied files and a valid Git history.

### 6. Future Work / Quine-like Behavior
To fully demonstrate the quine-like behavior, the next steps would involve:
*   Navigating into the `replicated_program_output` directory.
*   Building and running the program within that directory.
*   Verifying that it successfully creates *another* replicated directory, demonstrating recursive self-replication.

### 7. Related Documentation
*   SOP: Cocci-Driven Development (`docs/sops/cocci_driven_development_sop.md`)
*   Individual Patch Documentation (`docs/patches/001_main_rs_initial_doc.md`, etc.)
