# src/types/token/executable.rs

**Type:** Rust Source File (Trait Definition)

**Description:** Defines the `ExecutableToken` trait, which specifies the `execute` method. This trait is implemented by the `Token` enum, allowing each token to define its own execution logic within the Emojitape interpreter.

**Key Contents/Declarations:**
- `pub trait ExecutableToken`: The trait definition.
- `fn execute(&self, stack: &mut Vec<i32>, locals: &mut HashMap<i32, i32>, tokens_iter: &mut Peekable<Iter<super::Token>>) -> Result<(), String>`: The signature of the execution method.

**Relationships:**
- Implemented by the `Token` enum (specifically in `executable_impl.rs`).
- Provides a standardized interface for token execution.

**Keywords/Tags:** Rust, trait, executable, token, interface, behavior

**Emoji Representation:** 🎯⚙️
