# 00_univalent_embedding_scheme.md

## 1. Overview

This document defines the process for creating a univalent numerical embedding for each term in the `embedding_space_schema.json`. The process translates the harmonic analysis of the schema's vocabulary into a unique vector for each term.

The final embedding is a 2-dimensional vector composed of a Reciprocal Harmonic Value and a Gödel Number.

## 2. The Reciprocal Harmonic Value

This value captures the term's fundamental frequency or resonance within the system, derived from its harmonic partition.

- **Process:** For each term, identify the prime number (`p`) of its harmonic group (e.g., 2, 3, 5, ...). The reciprocal value is `1/p`.
- **Unity Group:** For terms in the "Unity" group (frequency of 1), we define `p=1`, so their reciprocal value is `1`.

## 3. The Gödel Number

This is a unique integer that formally encodes a term's identity, its frequency, and its harmonic nature into a single number.

- **Process:** The Gödel Number (`G`) for each term is constructed using prime factorization:

  `G = 2^id * 3^count * 5^p`

- **Components:**
  - `2, 3, 5`: The first three prime numbers used as fixed bases.
  - `id`: A unique integer index (from 1 to 55) assigned to each unique term based on alphabetical order.
  - `count`: The frequency of the term's occurrence in the schema document.
  - `p`: The prime of the term's harmonic partition.

## 4. The Final Embedding Vector

The univalent embedding for each term is a 2D vector:

`Embedding = (Reciprocal_Harmonic_Value, Gödel_Number)`

### Example: The term `"lam"`

1.  **Harmonic Analysis:**
    - Frequency `count = 65`.
    - Smallest prime factor from our list is 5, so it belongs to the "Harmonic of 5". Thus, `p = 5`.

2.  **Reciprocal Harmonic Value:**
    - `1/p = 1/5 = 0.2`

3.  **Gödel Number:**
    - Alphabetical index of `"lam"` (hypothetically): `id = 30`.
    - `G = 2^30 * 3^65 * 5^5`

4.  **Embedding for `"lam"`:**
    - `(0.2, 2^30 * 3^65 * 5^5)`
