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

## Next Steps

The next phase of this project is to build the Rust application that reads the `embedding_space_schema.json`, applies this methodology, and outputs the final Calculated Embedding matrix for each term.
