## SOP: Iterative Program Reconstruction via Semantic Patches (Cocci-Driven Development)

### 1. Purpose
This Standard Operating Procedure (SOP) defines a rigorous, iterative process for reconstructing or evolving a Rust program entirely through the application of `coccinelleforrust` semantic patches (`.cocci` files). This approach ensures auditable, testable, and modular code transformations, facilitating self-replication and continuous program evolution.

### 2. Scope
This SOP applies to all phases of program development and refactoring within the `model-builder-quiz` project where code is generated, modified, or transformed using `coccinelleforrust`.

### 3. Principles
*   **Atomic Transformations:** Each `.cocci` file should represent a single, logical, and testable code transformation.
*   **Test-Driven Development (Cocci-Style):** Each transformation should be immediately followed by verification (compilation, linting, and if applicable, running tests).
*   **Auditable History:** Every successful `cocci` application will be documented and committed to version control.
*   **Self-Replication as a Core Feature:** The entire process is designed to be self-contained and repeatable, enabling the program to reconstruct itself.

### 4. Prerequisites
*   `coccinelleforrust` (cfr) installed and accessible.
*   A clean Git repository initialized for the target program.
*   Initial `Cargo.toml` and `src/main.rs` (or equivalent) for `cargo init`.

### 5. Procedure
The process is cyclical, repeating for each desired code transformation.

#### Phase 1: Initialization (One-time)
1.  **Create New Project Directory:**
    ```bash
    mkdir new_program_name
    cd new_program_name
    ```
2.  **Initialize Git Repository:**
    ```bash
    git init
    ```
3.  **Initialize Rust Project:**
    ```bash
    cargo init --bin # Or --lib if it's a library
    ```
4.  **Initial Commit:**
    ```bash
    git add .
    git commit -m "feat: Initial program structure (git init, cargo init)"
    ```

#### Phase 2: Iterative Cocci Application
For each desired code change:

1.  **Define Semantic Patch (`.cocci`):**
    *   Create a new `.cocci` file (e.g., `patches/001_add_feature_x.cocci`) that implements a single, atomic transformation.
    *   Ensure the `.cocci` file adheres to `coccinelleforrust` syntax and best practices (e.g., no leading comments that cause parsing errors).
    *   **Self-Correction:** If `cfr` fails, debug the `.cocci` file based on error messages and `coccinelleforrust` examples.

2.  **Apply Semantic Patch:**
    ```bash
    /path/to/cfr -c patches/001_add_feature_x.cocci src/main.rs --apply # Adjust target path as needed
    ```
    *   **Note:** The `--apply` flag modifies the file in place.

3.  **Verify Transformation:**
    *   **Compile:**
        ```bash
        cargo check
        ```
    *   **Lint (if applicable):**
        ```bash
        cargo clippy # Or project-specific linting
        ```
    *   **Test (if applicable):**
        ```bash
        cargo test # If unit/integration tests are relevant to the applied patch
        ```
    *   **Manual Review:** Visually inspect the modified code to ensure the transformation is correct and idiomatic.

4.  **Document and Commit:**
    *   **Create Documentation:** Write a brief Markdown file (e.g., `docs/patches/001_add_feature_x_doc.md`) describing:
        *   The purpose of the patch.
        *   The specific code changes it introduces.
        *   Any assumptions or dependencies.
        *   Verification steps.
    *   **Stage Changes:**
        ```bash
        git add . # Add the modified code and the new documentation
        ```
    *   **Commit Changes:**
        ```bash
        git commit -m "feat: Apply patch 001_add_feature_x (adds module X)" # Use a descriptive message
        ```

### 6. Tools
*   `git`
*   `cargo` (for `init`, `check`, `clippy`, `test`)
*   `coccinelleforrust` (cfr)
*   Text editor (for `.cocci` and documentation files)

### 7. Expected Outcome
*   A Rust program entirely constructed and maintained through a series of auditable semantic patches.
*   A clear, version-controlled history of all code transformations.
*   A self-replicating system capable of reconstructing its own codebase from a minimal initial state.
