# src/types/token/impl_token/not_equals.rs

**Type:** Rust Source File (Token Execution Logic)

**Description:** Contains the `execute_not_equals` function, which defines the execution logic for the `NotEquals` token. This function pops two operands from the stack, compares them for inequality, and pushes 1 (true) or 0 (false) onto the stack.

**Key Contents/Declarations:**
- `pub fn execute_not_equals(...)`: Function implementing the inequality comparison.

**Relationships:**
- Called by `executable_impl.rs` when a `NotEquals` token is encountered.
- Operates on the interpreter's stack.

**Keywords/Tags:** Rust, token execution, NotEquals, comparison, logical operation, stack operation

**Emoji Representation:** ≠⚙️

**Clifford Multivector:** (0, 0, 0, 1, 0, 0, 0, 0) - Represents a divergence or distinction, a decision point.
