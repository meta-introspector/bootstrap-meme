# src/types/token/impl_token/lightning.rs

**Type:** Rust Source File (Token Execution Logic)

**Description:** Contains the `execute_lightning` function, which defines the execution logic for the `Lightning` token. This function expects the next token in the stream to be a `Float`, which it then pushes onto the stack (converted to an integer).

**Key Contents/Declarations:**
- `pub fn execute_lightning(...)`: Function handling float constants.

**Relationships:**
- Called by `executable_impl.rs` when a `Lightning` token is encountered.
- Interacts with the token stream to consume the next token.

**Keywords/Tags:** Rust, token execution, Lightning, float, constant, stack operation, token stream

**Emoji Representation:** ⚡⚙️

**Clifford Multivector:** (0, 0, 1, 0, 0, 0, 0, 0) - Represents a sudden introduction of a value, a dynamic insertion.
