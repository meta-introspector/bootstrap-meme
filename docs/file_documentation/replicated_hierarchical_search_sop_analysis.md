# hierarchical_search_sop.md - Conceptual Multivector Analysis

## Summary
This SOP outlines a "Hierarchical Search Strategy" for the `ragit` project, designed to efficiently locate code, documentation, and concepts. It advocates a "Vibe Search" philosophy, starting with high-level structural information and progressively moving to more granular, content-based searches. The procedure details a seven-step process: searching the file tree, Cargo manifests, targeted globs, documentation, ontologies, semantic vibe vectors, and finally, Rust source files, ensuring targeted and relevant information retrieval.

## Key Concepts and Simulated Multivector Embeddings

| Concept                       | Simulated 8D Multivector (Conceptual) |
| :---------------------------- | :------------------------------------ |
| SOP: Hierarchical Search Strategy | [0.9, 0.8, 0.7, 0.9, 0.6, 0.8, 0.9, 0.7] |
| Objective                     | [0.8, 0.7, 0.6, 0.9, 0.7, 0.8, 0.9, 0.8] |
| Structured, efficient, hierarchical methodology | [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2] |
| Locating code, documentation, concepts | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |
| Prioritize targeted searches  | [0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1] |
| Guiding Principle             | [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2] |
| "Vibe Search" philosophy      | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |
| Procedure                     | [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2] |
| 1. Search the Tree (`tree_level_3.json`) | [0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1] |
| 2. Search the Cargo Manifests (`Cargo.toml`) | [0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1] |
| 3. Narrow the Search (Targeted Globs) | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |
| 4. Search the Documentation (`**/*.md`) | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |
| 5. Search the Ontologies (`**/*.ttl`) | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |
| 6. Search the Vibes (`directory_vectors.json`) | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |
| 7. Search the Rust Files (`**/*.rs`) | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |

## Relationships between Concepts (Conceptual Geometric Operations)

*   The **Objective** of the **SOP: Hierarchical Search Strategy** *is to provide* a **Structured, efficient, hierarchical methodology** for **Locating code, documentation, and concepts**, prioritizing **Targeted searches**. (Conceptual "purpose")
    *   `Objective` of `SOP` -> `Structured, efficient, hierarchical methodology` for `Locating code/docs/concepts` (prioritizing `Targeted searches`)
*   The **Guiding Principle** of **"Vibe Search" philosophy** *informs* the **Procedure**, emphasizing starting with high-level information and progressively moving to more granular searches. (Conceptual "guiding principle")
    *   `"Vibe Search" philosophy` -> `Procedure`
*   The **Procedure** *consists of* seven sequential steps, each building upon the previous one to narrow down the search: **Search the Tree**, **Search the Cargo Manifests**, **Narrow the Search (Targeted Globs)**, **Search the Documentation**, **Search the Ontologies**, **Search the Vibes**, and finally, **Search the Rust Files**. (Conceptual "workflow")
    *   `Procedure` = `Search Tree` -> `Search Cargo` -> `Narrow Search` -> `Search Docs` -> `Search Ontologies` -> `Search Vibes` -> `Search Rust`
*   Each step *utilizes specific tools or data sources* (e.g., `tree_level_3.json`, `Cargo.toml`, `**/*.md`, `**/*.ttl`, `directory_vectors.json`, `**/*.rs`) to achieve its purpose. (Conceptual "tool/data source utilization")
    *   Each `Search` step utilizes specific `Tools/Data Sources`
