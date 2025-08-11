# System `e` Schema Core

## Objective

This project implements the core logic for **System `e`**, a framework for generating **Calculated Embeddings** from formal schemas. 

Unlike traditional learned embeddings (e.g., BERT, Word2Vec), which are statistical representations derived from massive datasets, a Calculated Embedding is generated deterministically through a formal, transparent, and mathematically-grounded process.

## Methodology

The process creates a unique, high-dimensional numerical representation for each term found in a given schema (e.g., `embedding_space_schema.json`).

1.  **Harmonic Analysis:** The vocabulary of the schema is extracted, and each term's frequency is analyzed for its prime harmonic signature. This yields a **Reciprocal Harmonic Value (`1/p`)**, representing the term's fundamental resonance in the system.

2.  **Gödel Numbering:** A unique, large integer (`G`) is constructed for each term, encoding its alphabetical identity, its frequency, and its harmonic prime. This number serves as a formal, unique identifier.

3.  **Univalent Multivector:** The Reciprocal Harmonic and the Gödel Number are combined into a 2-component vector. This vector is treated as a **multivector**, grounding it in the principles of geometric algebra.

4.  **Calculated Embedding Matrix:** The Gödel number component (`G`), a 1746-bit integer, is reshaped into a `4x7` matrix of `u64` integers. This matrix is the final **Calculated Embedding** for the term.

## Implementation

The reference implementation for these structures and transformations is defined in `01_rust_embedding_design.rs`.

## Project Structure
The core logic of this project is organized into the following modules within the `src/` directory:
*   `main.rs`: The entry point of the application, orchestrating the OODA loop.
*   `ingestion.rs`: Handles the observation phase, responsible for ingesting project files into the `SystemBuffer`.
*   `analysis.rs`: Contains the `perform_vocabulary_analysis` function, part of the orientation phase.
*   `ooda_loop.rs`: Implements the main OODA (Observe, Orient, Decide, Act) loop, managing the flow of the application and user interaction.
*   `interpreter.rs`: (Existing, not directly used in current OODA flow, but part of the project)

## Next Steps

The next phase of this project is to build the Rust application that reads the `embedding_space_schema.json`, applies this methodology, and outputs the final Calculated Embedding matrix for each term.

## Standard Operating Procedures (SOPs)
This project adheres to a structured development process. Key Standard Operating Procedures (SOPs) are documented to ensure consistency, quality, and continuous improvement.
*   [OODA Loop SOP for Model Evolution](docs/sops/ooda_sop.md) - Outlines the Observe, Orient, Decide, Act framework for rapid decision-making and continuous improvement.
*   [Meta-Meme Hero's Journey SOP](docs/sops/sop_meta_meme_heros_journey.md) - Outlines the conceptual journey of a meta-meme through the System `e` ecosystem.

## Key Memories

*   **Project Goal:** To build a "Model Builder Quiz" system that explores the intersection of AI, Rust, and semantic web technologies, focusing on "quine-like self-replication" and "semantic embedding" to enable a system that can "understand, generate, and evolve its own code and knowledge representations."
*   **Core Technologies:** Rust, `sophia_rs` (for RDF graph manipulation), `clap` (for CLI), `syn` (for AST parsing), `tokio` (for async operations).
*   **Key Concepts:**
    *   **Quine-like Self-Replication:** The system aims to generate and evolve its own code, similar to a quine (a program that outputs its own source code). This is detailed in `02_rust_quine_engine_plan.md` and `sop_quine_bootstrap.md`.
    *   **Semantic Embedding:** Mapping code, data, and concepts into a unified semantic space. This involves "Univalent Embedding Scheme" (`00_univalent_embedding_scheme.md`) and the use of "Clifford Algebra" and "multivectors" (`02_meta_meme_clifford_embedding.md`, `05_new_concept_bivectors.md`).
    *   **Ontology and Vocabulary Analysis:** Building a semantic representation of the codebase's "vocabulary" and relationships (`00_vocabulary_analysis.md`).
    *   **Gödel Numbers & Numerical Theory of Everything (NToE):** Concepts related to arithmetizing code and concepts for self-reference and meta-mathematical properties (`00_univalent_embedding_scheme.md`).
    *   **Ngram Harmonics:** A technique for initial analysis of patterns in code/text (`03_first_analysis_ngram_harmonics.md`).
*   **Architectural Components:**
    *   **CLI (`main.rs`):** Provides `Bootstrap`, `Run`, and `Analyze` commands.
    *   **Interpreter (`src/interpreter.rs`):** Core component for executing programs (likely generated quine code).
    *   **Embedding Design (`01_rust_embedding_design.rs`):** Details how Rust code elements are embedded.
    *   **Main Engine Design (`04_main_engine_design.rs`):** Overall architecture integrating various components.
*   **Design Principles:** "One declaration per file" is a recurring principle for code organization (`01_rust_embedding_design.rs`).
*   **SOPs:** `sop_quine_bootstrap.md` outlines the procedure for the quine's initial self-bootstrapping.