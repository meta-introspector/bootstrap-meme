# Emojitape Interpreter

This project is a full toolchain for a novel, emoji-based programming language called **Emojitape**. It includes a parser, interpreter, and a multi-target compiler.

## 📜 Language Overview

Emojitape is a stack-oriented language that uses emojis as its primary tokens. While it can be used for simple arithmetic with Reverse Polish Notation (RPN), its core is built on a powerful foundation of formal logic and functional programming.

The language's design is heavily influenced by:
*   **Combinatory Logic**: It features a rich set of combinators, including the foundational **S, K, and I** combinators, as well as others like Y (the fixed-point combinator). This suggests a computational model similar to lambda calculus.
*   **Formal Logic**: The language includes tokens for propositional (`∧`, `∨`, `→`) and predicate (`∀`, `∃`) logic.

Source files are stored in the `tapes/` directory and use the `.emojitape` extension.

### File Structure

An Emojitape program is divided into several distinct sections, marked by special comments (e.g., `💬--- PRELUDE`). This structure allows for different modes of programming within a single file.

*   `PRELUDE`: The main functional and logical programming section.
*   `WASM COMPILER PRELUDE`: Holds specific definitions for WebAssembly compilation.
*   `RULES`: Defines patterns for a term-rewriting system.
*   `WORLD TAPE`: The primary tape for imperative, stack-based execution.
*   `GENERATED WAT BLOCK`: Contains compiled WebAssembly code generated from the tape.
*   `ZOS EXPORT`: Sections for defining and implementing interfaces to an external ontology system.
*   `SELF-REPRODUCING FOOTER`: A dedicated section for creating quines (self-reproducing programs).

### Rule Syntax

Rules are defined within the `RULES` section using the following structure:

```
➡️ <pattern> ↩️ <replacement>
```

*   `➡️` (RuleEntry): Marks the beginning of a new rule.
*   `<pattern>`: A sequence of Emojitape tokens that the interpreter will look for.
*   `↩️` (Return): Acts as a delimiter, separating the pattern from the replacement.
*   `<replacement>`: A sequence of Emojitape tokens that will replace the pattern if it matches.

For example, a rule to replace `1 1 ➕` with `2` might look like:

```
➡️ 1 1 ➕ ↩️ 2
```

### Example

Here is a simple Emojitape program from `tapes/simple.emojitape`:

```
💬--- PRELUDE
✅❌
💬--- RULES
➡️ 1 1 ➕ ↩️ 2

💬--- WORLD TAPE
1 2 ➕
```

This simple example demonstrates RPN by pushing `1` and `2` onto the stack, and then `➕` adds them together.

## ▶️ Usage

### Running the Interpreter

The interpreter is run from the command line, passing the path to a tape file as an argument.

```bash
cargo run -- tapes/simple.emojitape
```

### Generating Emojitape from Text

The `emojitape_generator` binary can convert a plain text file into an Emojitape program.

```bash
cargo run --bin emojitape_generator <input_text_file> <output_emojitape_file>
```

For example, to convert `bootstrap.txt` into `generated_emojitape.emojitape`:

```bash
cargo run --bin emojitape_generator bootstrap.txt tapes/generated_emojitape.emojitape
```

### Generating Rust Programs from Emojitape

The `rust_program_generator` binary takes a compiled Emojitape file (containing a `GENERATED WAT BLOCK`) and creates a Rust project that embeds and runs the WebAssembly.

```bash
cargo run --bin rust_program_generator <input_emojitape_file> <output_project_directory>
```

For example, to generate a Rust project from `tapes/my_program.emojitape` into a directory named `my_rust_app`:

```bash
cargo run --bin rust_program_generator tapes/my_program.emojitape my_rust_app
```

## ✨ Features

*   **Emoji-based Syntax**: All operations are represented by emojis.
*   **Multi-paradigm**:
    *   **Functional**: Based on **Combinatory Logic** (SKI calculus, Y combinator).
    *   **Imperative**: Stack-based execution on the `WORLD TAPE`.
    *   **Logic-based**: Includes primitives for propositional and predicate logic.
    *   **Term Rewriting**: A dedicated `RULES` section for defining transformation rules.
*   **Metaprogramming**: Designed for code generation and self-replication, with a dedicated `SELF-REPRODUCING FOOTER`.
*   **Interpreter**: Directly execute Emojitape code.
*   **Compiler/Transpiler**:
    *   Compiles Emojitape to standalone **Rust** programs.
    *   Compiles Emojitape to the **WebAssembly Text Format (`.wat`)**.

## 🏗️ Project Structure

The project is built in Rust and consists of several key components:

*   **Tokenizer**: Converts emoji streams into tokens based on a rich, categorized map.
*   **Parser**: Builds a structured representation of the program based on its section markers.
    *   `src/parser/tests.rs`: Contains unit tests for the parser.
*   **Interpreter**: Executes the parsed code.
*   **Generator**: Handles the compilation to Rust and WebAssembly.

## 🚧 Development Status

**This project is currently in the alpha stage. The language design and parsing are well-developed, but the core execution logic is still a work in progress.**

*   ✅ **Tokenizer**: Complete and robust.
*   ✅ **Parser**: Complete and correctly builds the program structure.
*   ❌ **Interpreter**: Currently a placeholder. It prints a representation of the program rather than executing it. The core logic for stack manipulation, applying rules, and functional evaluation is not yet implemented.
