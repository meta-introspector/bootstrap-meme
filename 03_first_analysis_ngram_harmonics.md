# 03_first_analysis_ngram_harmonics.md

## 1. Principle: The Document as a Jupyter Notebook

System `e` treats its own documentation (`.md` files) as living documents, analogous to Jupyter Notebooks. The Quine Engine can execute analyses and append the results as new "cells" to these documents.

This document describes the algorithm for the first analysis the engine performs on itself.

## 2. The First Analysis: N-Gram Harmonic Analysis

This is the initial, bootstrap analysis phase. Its purpose is to understand the structure of the language used in the system's own documentation at various levels of granularity.

### 2.1. Input

The full, concatenated text content of all `.md` files within the repository.

### 2.2. Algorithm

1.  **Tokenization:** The input text is converted into a single, ordered sequence of tokens (words, punctuation).

2.  **N-Gram Extraction & Counting:** The engine slides a window across the token sequence to extract and count the frequency of all unique n-grams for the following lengths (`n`):
    -   **Pairs:** `n=2`
    -   **Triples:** `n=3`
    -   **Prime N-Grams:** `n=5, 7, 11, 13, 17, 19`

3.  **Power-of-Prime Analysis:** For each n-gram's final frequency count, the engine performs a detailed harmonic analysis. Instead of just finding the smallest prime factor, it calculates the **powers of primes** within the count. 
    -   *Example:* An n-gram with a frequency of `72` (`= 2^3 * 3^2`) would be analyzed as having a `power-of-2` of `3` and a `power-of-3` of `2`.

### 2.3. Output: The Appended Cell

The result of this analysis is formatted as a new Markdown section (a "cell"). This cell will contain tables summarizing the most frequent n-grams for each length, along with their frequency and their calculated power-of-prime harmonic properties.

### 2.4. Action

After the analysis is complete, the Quine Engine appends this new cell to a designated report file (e.g., `ANALYSIS_REPORT.md`). This act is the first modification the system makes to its own state, based on a calculated understanding of its own content. It is the first step in the DFA loop.
