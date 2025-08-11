## SOP: N00b Tutorial - Applying a Single Cocci Patch

### 1. Purpose
This Standard Operating Procedure (SOP) serves as a tutorial for new contributors ("n00bs") to understand the iterative process of applying a single `coccinelleforrust` semantic patch within the "Cocci-Driven Development" methodology. It demonstrates the steps involved in transforming code, verifying changes, and documenting the process.

### 2. Scope
This tutorial focuses on the application of a single `.cocci` patch to a Rust source file within the `model-builder-quiz` project, following the guidelines established in the "SOP: Iterative Program Reconstruction via Semantic Patches (Cocci-Driven Development)" (`docs/sops/cocci_driven_development_sop.md`).

### 3. Prerequisites
*   Familiarity with basic Git commands.
*   `coccinelleforrust` (cfr) installed and accessible.
*   A Rust project initialized and ready for code modifications.

### 4. Procedure
This tutorial will walk through the steps of adding a simple `log` function to `src/main.rs` using a `cocci` patch.

#### Step 1: Define Transformation (Patch)
The first step is to define the specific code transformation you want to apply. For semantic changes, this involves creating a `.cocci` file.

1.  **Create the `.cocci` file:**
    *   **Action:** Create a new `.cocci` file, for example, `replicated_program_cocci/patches/013_add_log_function.cocci`.
    *   **Content:** The `.cocci` file will contain a semantic patch rule to add the `log` function.
        ```cocci
        @add_log_function@
        @@
        +fn log(message: &str) {
        +    println!("LOG: {}", message);
        +}
        ```
    *   **Tool:** This action is typically performed using a `write_file` command or a shell script that writes the content.

#### Step 2: Apply Transformation (Semantic Patch)
Once the `.cocci` file is defined, apply it to the target Rust source file using the `coccinelleforrust` tool.

1.  **Execute `cfr`:**
    *   **Action:** Run the `coccinelleforrust` (cfr) tool, specifying the `.cocci` file and the target Rust file. The `--apply` flag ensures the changes are made directly to the file.
    *   **Command:**
        ```bash
        ./vendor/coccinelleforrust/target/release/cfr -c replicated_program_cocci/patches/013_add_log_function.cocci replicated_program_cocci/src/main.rs --apply
        ```
    *   **Tool:** This action is performed using `run_shell_command`.

#### Step 3: Verify Transformation
After applying the patch, it's crucial to verify that the changes were applied correctly and that the codebase remains functional.

1.  **Compile the code:**
    *   **Action:** Run `cargo check` to ensure the code still compiles without errors.
    *   **Command:**
        ```bash
        cargo check -p replicated_program_cocci
        ```
    *   **Tool:** This action is performed using `run_shell_command`.
2.  **Manual Review (Optional but Recommended):**
    *   **Action:** Read the modified Rust file (`replicated_program_cocci/src/main.rs`) to visually confirm that the `log` function has been added correctly.
    *   **Tool:** This action is performed using `read_file`.

#### Step 4: Document and Commit
Every applied patch and its associated changes must be documented and committed to maintain an auditable history of the program's evolution.

1.  **Create Patch Documentation:**
    *   **Action:** Create a new Markdown file (e.g., `replicated_program_cocci/docs/patches/013_add_log_function_doc.md`) that describes the patch.
    *   **Content:** This documentation should detail the purpose of the patch, the specific code changes it introduces, and the verification steps.
    *   **Tool:** This action is typically performed using a `write_file` command.
2.  **Stage Changes:**
    *   **Action:** Stage all modified and new files (the `.cocci` file, the modified Rust file, and the documentation file).
    *   **Command:**
        ```bash
        git -C replicated_program_cocci add .
        ```
    *   **Tool:** This action is performed using `run_shell_command`.
3.  **Commit Changes:**
    *   **Action:** Commit the staged changes with a clear and descriptive message that summarizes the patch.
    *   **Command:**
        ```bash
        git -C replicated_program_cocci commit -m "feat: Apply patch 013 (adds log function)"
        ```
    *   **Tool:** This action is performed using `run_shell_command`.

### 5. Expected Outcome
Upon successful completion of this tutorial, the `src/main.rs` file will have a new `log` function, and all changes will be properly documented and committed, demonstrating a single iteration of the "Cocci-Driven Development" process.
