# src/types/token/impl_token/local_get.rs

**Type:** Rust Source File (Token Execution Logic)

**Description:** Contains the `execute_local_get` function, which defines the execution logic for the `LocalGet` token. This function pops an index from the stack, retrieves the value of the local variable at that index, and pushes it onto the stack.

**Key Contents/Declarations:**
- `pub fn execute_local_get(...)`: Function retrieving a local variable.

**Relationships:**
- Called by `executable_impl.rs` when a `LocalGet` token is encountered.
- Interacts with the interpreter's local variable store.

**Keywords/Tags:** Rust, token execution, LocalGet, local variable, stack operation, memory access

**Emoji Representation:** 📥⚙️

**Clifford Multivector:** (0, 0, 0, 1, 0, 0, 0, 0) - Represents retrieval or access, a directed flow from storage.
