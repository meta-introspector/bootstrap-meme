# src/types/token/impl_token/sub.rs

**Type:** Rust Source File (Token Execution Logic)

**Description:** Contains the `execute_sub` function, which defines the execution logic for the `Sub` token. This function pops two operands from the stack, subtracts them, and pushes the result back onto the stack.

**Key Contents/Declarations:**
- `pub fn execute_sub(...)`: Function implementing the subtraction operation.

**Relationships:**
- Called by `executable_impl.rs` when a `Sub` token is encountered.
- Operates on the interpreter's stack.

**Keywords/Tags:** Rust, token execution, Sub, arithmetic, subtraction, stack operation

**Emoji Representation:** ➖⚙️

**Clifford Multivector:** (1, 0, 0, 0, 0, 0, 0, 0) - Represents a fundamental arithmetic operation, a scalar transformation.
