# Standard Operating Procedure: Emoji Refactoring

## 1. Purpose
This SOP outlines the process for refactoring emoji token execution logic into dedicated functions within their respective modules and ensuring consistency across the `emojitape_interpreter` project.

## 2. Scope
This SOP applies to all emoji token implementations within `src/types/token/emojis/` and their corresponding usage in `src/interpreter.rs`.

## 3. Procedure

### 3.1. Identify Target Emoji Token
Select an emoji token from `src/types/token/emojis/mod.rs` that needs its execution logic extracted from `src/interpreter.rs`.

### 3.2. Create/Update `execute_` Function in Emoji Module
1.  Navigate to the corresponding emoji module file (e.g., `src/types/token/emojis/add_token.rs`).
2.  If it doesn't exist, create a new public function named `execute_<token_name>` (e.g., `execute_add`).
3.  The function signature should generally be:
    ```rust
pub fn execute_<token_name>(
    stack: &mut Vec<i32>,
    locals: &mut HashMap<i32, i32>,
    tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    // Token-specific logic here
    Ok(())
}
    ```
    *   **Note:** If the token carries a value (e.g., `Token::Integer(i)`, `Token::Comment(s)`), include that value as the first parameter (e.g., `i: &i32`, `s: &String`).
4.  Move the relevant execution logic from the `match` arm in `src/interpreter.rs` into this new `execute_` function.
5.  Ensure all necessary `use` statements (e.g., `std::collections::HashMap`, `std::iter::Peekable`, `std::slice::Iter`) are present in the emoji module file.

### 3.3. Update `src/interpreter.rs`
1.  **Add `use` statement:** If not already present, add a `use` statement for the emoji module at the top of `src/interpreter.rs` (e.g., `use crate::types::token::emojis::add_token;`).
2.  **Refactor `match` arm:** Replace the original logic in the `match` arm for the target token with a call to the new `execute_` function:
    ```rust
            Token::<TokenName> => {
                <token_name>::execute_<token_name>(&mut stack, &mut locals, &mut tokens_iter)?;
            },
    ```
    *   **Note:** If the token carries a value, pass it as the first argument (e.g., `Token::Integer(i) => { i32_const_token::execute_i32_const(i, ...)?; }`).

### 3.4. Update `src/types/token/emojis/mod.rs`
1.  Ensure that the emoji module is publicly declared in `src/types/token/emojis/mod.rs` (e.g., `pub mod add_token;`).

### 3.5. Verification
1.  Run `cargo check` to ensure there are no compilation errors.
2.  Run existing tests (if any) to verify the refactored logic behaves as expected.

## 4. Best Practices
*   **Atomic Changes:** Perform refactoring for one emoji token at a time to isolate issues.
*   **Exact Matching:** When using `replace` tool, ensure `old_string` precisely matches the content in the file, including whitespace and newlines.
*   **Error Handling:** Ensure `execute_` functions return `Result<(), String>` and propagate errors using `?` where appropriate.
*   **Documentation:** Add comments to the `execute_` functions explaining their purpose and any specific behavior.
