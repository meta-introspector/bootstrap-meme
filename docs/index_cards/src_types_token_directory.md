# src/types/token/ Directory

**Type:** Directory

**Description:** Contains the definitions and implementations related to the `Token` enum, which represents the fundamental units of the Emojitape language. This directory is crucial for understanding how Emojitape programs are parsed, interpreted, and translated.

**Key Contents/Declarations:**
- `mod.rs`: Main module file, declaring submodules.
- `emojis/`: Contains emoji mappings for various tokens.
- `executable.rs`: Defines the `ExecutableToken` trait.
- `executable_impl.rs`: Implements the `ExecutableToken` trait for the `Token` enum.
- `impl_token/`: Contains individual execution logic for each token.
- `impl_token_display_fromstr.rs`: Implements `Display` and `FromStr` traits for the `Token` enum.
- `tests.rs`: Unit tests for token-related functionalities.

**Relationships:**
- Central to the parsing and interpretation of Emojitape.
- `Token` enum is used throughout the interpreter and generator components.

**Keywords/Tags:** Token, Emojitape, parsing, interpretation, enum, data structure

**Emoji Representation:** 🧩📜
