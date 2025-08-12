# src/ Directory

**Type:** Directory

**Description:** Contains the core Rust source code for the `emojitape_interpreter` project. This directory is organized into submodules representing different functionalities of the interpreter, generator, and related tools.

**Key Contents/Declarations:**
- `bin/`: Executable binaries.
- `types/`: Data structures and enums, including `Token` definitions and implementations.
- `interpreter/`: Logic for interpreting emojitape programs.
- `generator/`: Logic for generating Rust programs from emojitape.
- `parser/`: Handles parsing of emojitape syntax.
- `tokenizer/`: Breaks down emojitape into individual tokens.
- `rust_to_emoji/`: Converts Rust code to emojitape.
- `cocci_parser/`: Related to Coccinelle parsing.
- `cocci_converter.rs`: Converts Coccinelle output.
- `emoji_to_rust_standalone.rs`: Standalone emoji to Rust conversion.
- `lib.rs`: Library entry point.
- `main_conversion_demo.rs`: Demonstration of conversion.
- `rust_to_emoji_standalone.rs`: Standalone Rust to emoji conversion.

**Relationships:**
- The central hub for all application logic.
- Modules within `src/` depend on each other to form the complete application.

**Keywords/Tags:** Source code, Rust, modules, application logic

**Emoji Representation:** 💻📂
