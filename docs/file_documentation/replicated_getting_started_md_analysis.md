# Getting Started.md - Conceptual Multivector Analysis

## Summary
This "Getting Started" guide introduces CoccinelleForRust (cfr), a program transformation tool that adapts Coccinelle for Rust codebases using the Semantic Patch Language (SmPL). It explains how SmPL describes and applies code changes, illustrating with an example of refactoring a `setSpeed` function's signature. The document details SmPL syntax, including rule definitions, metavariables (expression, identifier, type, lifetime, parameter), and the use of `@@` to separate declaration and modifier spaces, as well as `+` and `-` for adding and removing lines.

## Key Concepts and Simulated Multivector Embeddings

| Concept                       | Simulated 8D Multivector (Conceptual) |
| :---------------------------- | :------------------------------------ |
| CoccinelleForRust (cfr)       | [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2] |
| Program transformation tool   | [0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1] |
| Rust version of Coccinelle    | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |
| Semantic Patch Language (SmPL) | [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2] |
| Describe changes to codebase  | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |
| Transform codebase            | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |
| `setSpeed` function refactoring | [0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1] |
| Metavariables                 | [0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1] |
| SmPL syntax                   | [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2] |
| `@ rule1 @` (rule name)       | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |
| `expression wheel, speed;`    | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |
| `@@` (separator)              | [0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0, 0.0] |
| `-` (remove line)             | [0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0, 0.0] |
| `+` (add line)                | [0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0, 0.0] |
| Rule application              | [0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0] |

## Relationships between Concepts (Conceptual Geometric Operations)

*   **CoccinelleForRust (cfr)** *is a* **Program transformation tool** that is the **Rust version of Coccinelle**. (Conceptual "definition")
    *   `cfr` = `Program transformation tool` (Rust version of `Coccinelle`)
*   `cfr` *uses* **Semantic Patch Language (SmPL)** to **Describe changes to codebase** and **Transform codebase**. (Conceptual "methodology")
    *   `cfr` uses `SmPL` to (`Describe changes` ^ `Transform codebase`)
*   The **Example: `setSpeed` function refactoring** *illustrates* how SmPL can be used to change function signatures. (Conceptual "illustration")
    *   `Example` illustrates `SmPL` for `function refactoring`
*   **Metavariables** *are used in SmPL* to **Represent parts of AST**. (Conceptual "SmPL feature")
    *   `Metavariables` in `SmPL` represent `AST parts`
*   **SmPL syntax** *defines* how rules are structured, including rule names (`@ rule1 @`), metavariable declarations (`expression wheel, speed;`), and the use of `@@` as a separator. (Conceptual "syntax definition")
    *   `SmPL syntax` defines `rule structure`
*   The `+` and `-` symbols in SmPL *indicate* whether to **Remove matching line** or **Add line with collected info**. (Conceptual "transformation operations")
    *   `+` ^ `-` indicate `add/remove line`
*   **Rule application** *occurs* on the whole source code, and rules can inherit metavariables. (Conceptual "execution model")
    *   `Rule application` on `whole source code`
