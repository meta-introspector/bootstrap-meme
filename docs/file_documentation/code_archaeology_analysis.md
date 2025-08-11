# code_archaeology.md - Conceptual Multivector Analysis

## Summary
This SOP outlines "Code Archaeology and Documentation," a process to understand and document existing code to prevent duplication and promote reuse. It emphasizes searching before writing, using `ragit` for semantic location, and avoiding broad searches. The procedure details defining targets, starting with `Cargo.toml`, using focused structural searches, documenting findings in dedicated Markdown files, updating a central index (`docs/index.md`), and employing the "KitKat" meta-program for managing frustrating tasks.

## Key Concepts and Simulated Multivector Embeddings

| Concept                       | Simulated 8D Multivector (Conceptual) |
| :---------------------------- | :------------------------------------ |
| SOP: Code Archaeology and Documentation | [0.9, 0.8, 0.7, 0.9, 0.6, 0.8, 0.9, 0.7] |
| Objective                     | [0.8, 0.7, 0.6, 0.9, 0.7, 0.8, 0.9, 0.8] |
| Understand and document existing code functionality | [0.7, 0.6, 0.5, 0.4, 0.8, 0.7, 0.6, 0.5] |
| Avoid duplication             | [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2] |
| Promote reuse                 | [0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1] |
| Guiding Principles            | [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2] |
| Search Before You Write       | [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2] |
| Find the Vibe                 | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |
| `ragit` and `vibe` systems    | [0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1] |
| Procedure                     | [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2] |
| Step 1: Define Your Target    | [0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1] |
| Step 2: Start with the Manifest (`Cargo.toml`) | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |
| Step 3: Use Focused Search    | [0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1] |
| Prioritize structural search  | [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2] |
| Step 4: Document Your Findings | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |
| Dedicated Markdown file       | [0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0, 0.0] |
| `docs/rust_code/`             | [0.5, 0.4, 0.3, 0.2, 0.1, 0.0, 0.0, 0.0] |
| Step 5: Update the Central Index (`docs/index.md`) | [0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1] |
| Step 6: The "KitKat" Meta-Program | [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2] |
| Pause, Reset, Re-plan, Commit, Resume | [0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1] |

## Relationships between Concepts (Conceptual Geometric Operations)

*   The **Objective** of **SOP: Code Archaeology and Documentation** *is to* **Understand and document existing code functionality** to **Avoid duplication** and **Promote reuse**. (Conceptual "purpose")
    *   `Objective` of `SOP` -> (`Understand and document` ^ `Avoid duplication` ^ `Promote reuse`)
*   The **Guiding Principles** (`Search Before You Write`, `Find the Vibe`, `Avoid Duplication`) *inform* the **Procedure**. (Conceptual "principles guiding action")
    *   `Guiding Principles` -> `Procedure`
*   The **Procedure** *consists of* sequential steps: **Define Your Target**, **Start with the Manifest (`Cargo.toml`)**, **Use Focused Search** (prioritizing **structural search** and `ragit` querying), **Document Your Findings** (in **Dedicated Markdown file**s stored in `docs/rust_code/`), and **Update the Central Index (`docs/index.md`)**. (Conceptual "workflow steps")
    *   `Procedure` = `Define Target` -> `Start with Manifest` -> `Use Focused Search` -> `Document Findings` -> `Update Central Index`
*   **The "KitKat" Meta-Program** *is a mechanism within the Procedure* to manage frustrating tasks by initiating a cycle of **Pause, Reset, Re-plan, Commit, Resume**. (Conceptual "self-management mechanism")
    *   `"KitKat" Meta-Program` is part of `Procedure` and involves (`Pause` ^ `Reset` ^ `Re-plan` ^ `Commit` ^ `Resume`)
