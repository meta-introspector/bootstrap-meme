# src/types/token/mod.rs

**Type:** Rust Source File (Module Declaration)

**Description:** This file serves as the main module declaration for the `token` module. It defines the `Token` enum, which is the central data structure representing all possible tokens in the Emojitape language. It also declares and re-exports various submodules related to token functionalities.

**Key Contents/Declarations:**
- `pub enum Token`: The core enum defining all token variants (e.g., `Integer`, `Add`, `Sparkle`, `Comment`).
- `#[derive(Debug, PartialEq, Clone, EnumIter, EnumProperty)]`: Derivations for common traits.
- `pub mod emojis;`: Declares the `emojis` submodule.
- `pub mod executable;`: Declares the `executable` trait submodule.
- `pub mod executable_impl;`: Declares the `executable_impl` submodule (implementation of `ExecutableToken`).
- `pub mod impl_token;`: Declares the `impl_token` directory as a module.
- `pub mod impl_token_display_fromstr;`: Declares the submodule for `Display` and `FromStr` implementations.
- `pub mod tests;`: Declares the `tests` submodule.

**Relationships:**
- Defines the fundamental `Token` type used throughout the `emojitape_interpreter`.
- Acts as a central hub for all token-related submodules.
- `Token` variants are mapped to emojis in the `emojis` module.
- `executable_impl` depends on the `Token` enum and the `ExecutableToken` trait.

**Keywords/Tags:** Rust, module, enum, Token, Emojitape, data structure, core

**Emoji Representation:** 🧩📜⚙️
