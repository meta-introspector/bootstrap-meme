### SOP: Rust Code Translation and Analysis

**1. Purpose**

This SOP outlines the process for translating Rust code into different representations (like Emojitape) and performing code analysis using the `syn` crate for parsing and AST manipulation.

**2. Scope**

This SOP applies to any task that requires parsing Rust code for translation, analysis, or refactoring.

**3. Procedure**

**Phase 1: Parsing and Declaration Splitting**

**Objective:** To parse Rust source code into a structured format and split it into individual declarations.

**Steps:**

1.  **Parse the source code:** Use `syn::parse_file` to parse the Rust source code into a `syn::File` (AST).
2.  **Split into declarations:** Use a `DeclarationSplitter` (as seen in `declaration_splitter.rs`) to traverse the AST and extract individual declarations (`ItemFn`, `ItemStruct`, `ItemEnum`, etc.).
3.  **Store declaration data:** For each declaration, store the following information in a `Declaration` struct:
    *   `name`: The name of the declaration (e.g., function name, struct name).
    *   `declaration_type`: The type of the declaration (e.g., `Function`, `Struct`).
    *   `content`: The full string content of the declaration.
    *   `line_start`, `line_end`: The starting and ending line numbers of the declaration.
    *   `file_path`: The path to the source file.

**Phase 2: Code Analysis**

**Objective:** To perform various analyses on the parsed code.

**Steps:**

1.  **Generate JSON AST:** Use `syn_serde::json::to_string_pretty` to serialize the `syn::File` AST into a human-readable JSON format. This is useful for debugging and for feeding the AST to other tools.
2.  **Calculate Code Metrics:** Traverse the list of `Declaration`s and calculate metrics such as:
    *   Total lines of code.
    *   Counts of functions, structs, enums, traits, and impls.
    *   Cyclomatic complexity or other complexity scores.
3.  **Vectorize Code:** For each declaration's content, use a `CodeVectorizer` to generate a vector embedding. This can be used for semantic similarity checks and duplicate detection.
4.  **Detect Duplicates:** Use a `DuplicateDetector` to identify duplicate or near-duplicate declarations, both within a single file and across multiple files.

**Phase 3: Code Translation (e.g., to Emojitape)**

**Objective:** To translate the Rust code into a target language or representation.

**Steps:**

1.  **Create a Visitor:** Implement the `syn::visit::Visit` trait to create a custom visitor that will traverse the `syn` AST.
2.  **Define a Token Sequence:** Define a set of `Token`s that represent the target language (e.g., the `Token` enum in `emojitape_interpreter/src/types/token.rs`).
3.  **Implement `visit_*` methods:** In your visitor, implement the `visit_*` methods (e.g., `visit_item_fn`, `visit_expr_macro`) to handle different AST nodes.
4.  **Generate Tokens:** As the visitor traverses the AST, generate the corresponding sequence of `Token`s for the target language.
5.  **Output the Translation:** The sequence of `Token`s is the translated representation of the original Rust code.

**4. Tools**

*   **`syn` crate:** The primary tool for parsing Rust code into an AST.
*   **`syn_serde` crate:** For serializing the `syn` AST to JSON.
*   **Custom `DeclarationSplitter`:** For splitting the AST into individual declarations.
*   **Custom `CodeAnalyzer`:** For performing code analysis.
*   **Custom `syn::visit::Visit` implementation:** For traversing the AST and performing translations.

**5. Expected Outcome**

*   A structured representation of Rust code, split into individual declarations.
*   A comprehensive analysis of the code, including metrics, vector embeddings, and duplicate reports.
*   A translated representation of the code in the target format (e.g., Emojitape).
