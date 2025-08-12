# src/types/token/impl_token/iff.rs

**Type:** Rust Source File (Token Execution Logic)

**Description:** Contains the `execute_iff` function, which defines the execution logic for the `Iff` token (If and only if). This function pops two boolean-like operands from the stack, performs a logical biconditional operation, and pushes the result onto the stack.

**Key Contents/Declarations:**
- `pub fn execute_iff(...)`: Function implementing the logical biconditional operation.

**Relationships:**
- Called by `executable_impl.rs` when an `Iff` token is encountered.
- Operates on the interpreter's stack.

**Keywords/Tags:** Rust, token execution, Iff, logical operation, biconditional, stack operation

**Emoji Representation:** ↔️⚙️

**Clifford Multivector:** (1, 1, 0, 0, 0, 0, 0, 0) - Represents a mutual implication, a symmetrical relationship.
