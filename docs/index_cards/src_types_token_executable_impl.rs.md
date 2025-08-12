# src/types/token/executable_impl.rs

**Type:** Rust Source File (Trait Implementation)

**Description:** Implements the `ExecutableToken` trait for the `Token` enum. This file contains the central dispatch logic that maps each `Token` variant to its corresponding execution function located in the `src/types/token/impl_token/` directory.

**Key Contents/Declarations:**
- `impl ExecutableToken for Token`: The implementation block.
- `match self`: A large match statement that calls specific `execute_*` functions based on the `Token` variant.

**Relationships:**
- Implements the `ExecutableToken` trait defined in `executable.rs`.
- Calls functions from the `impl_token` module (e.g., `add::execute_add`, `mul::execute_mul`).
- Essential for the runtime behavior of Emojitape programs.

**Keywords/Tags:** Rust, trait implementation, Token, execution, dispatch, interpreter

**Emoji Representation:** ⚙️➡️
