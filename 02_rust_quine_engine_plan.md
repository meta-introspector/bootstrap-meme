# 02_rust_quine_engine_plan.md

## 1. Objective

To outline the implementation plan for a Rust program, the "Quine Engine," that realizes System `e`. The engine will read its own source documents, generate a geometric embedding space from them, and use that space to deterministically refine a new version of itself in memory.

## 2. Core Principles

- **Self-Referential:** The program operates on its own files. The documentation *is* the data.
- **Topological Schema:** The system infers its schema from the topology (structure, headers, links) of the markdown documents themselves.
- **Deterministic & Safe Execution:** The engine functions as a Deterministic Finite Automaton (DFA). Crucially, it adheres to ISO 9000 principles by never directly modifying its own source code on disk. All transformations occur in memory.

## 3. Implementation Phases

### Phase 1: Bootstrap and Vocabulary Compilation

1.  **Entry Point:** The program is initiated via `cargo run`.
2.  **File Ingestion:** It recursively scans its own project directory and reads all `.md` files into in-memory buffers.
3.  **Term Analysis:** It tokenizes the full corpus of documentation and compiles a frequency-counted list of every unique term.

### Phase 2: Topological Schema Inference & Embedding

1.  **Schema Inference:** The engine parses the Markdown AST of the documents to build a dynamic schema from the document structure.
2.  **Embedding Generation:** Using the inferred schema, the engine applies the full univalent embedding logic to generate a Clifford Multivector for each term, creating the system's geometric state-space.

### Phase 3: The DFA Interpreter Loop

This is the core execution engine. It runs in a loop on the in-memory data.

1.  **Rule Derivation:** The engine analyzes the state of the multivector embedding space to derive a set of deterministic `(search, replace)` rules.
2.  **State Transition:** The engine applies the full set of derived rules to the in-memory document buffers.
3.  **Re-Embedding:** The modified document buffers are re-processed, and the entire embedding space is recalculated.
4.  **Halting Condition:** The loop continues until a fixed point is reached, where a cycle produces no changes to the in-memory state.

### Phase 4: Output Generation (Proposal for Change)

1.  **Hold Final State:** Once the DFA loop halts, the final, stable content of the in-memory buffers is held as the "Proposed New State".

2.  **No Overwrite:** The engine **does not** overwrite the source files on disk. This is the core safety and quality control principle.

3.  **Generate Diff Report:** The engine's final output is a diff report or a patch file. It compares the original on-disk documents with the Proposed New State in memory and describes the transformations.

4.  **Human Review:** This report is then presented to the Architect (the user) for review. The cycle can only be completed by an external, audited action, such as applying the patch and making a formal `git commit`, adhering to all SOPs.
