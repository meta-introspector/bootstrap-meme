# src/types/token/impl_token/not.rs

**Type:** Rust Source File (Token Execution Logic)

**Description:** Contains the `execute_not` function, which defines the execution logic for the `Not` token. This function pops a boolean-like operand from the stack, performs a logical NOT operation, and pushes the result back onto the stack.

**Key Contents/Declarations:**
- `pub fn execute_not(...)`: Function implementing the logical NOT operation.

**Relationships:**
- Called by `executable_impl.rs` when a `Not` token is encountered.
- Operates on the interpreter's stack.

**Keywords/Tags:** Rust, token execution, Not, logical operation, boolean, stack operation

**Emoji Representation:** ¬⚙️

**Clifford Multivector:** (0, 1, 0, 0, 0, 0, 0, 0) - Represents an inversion or negation, a directional flip.
