# KitKat Break: Strategic Plan for Rust Program Generation

## 1. Current Situation

We are currently working on a `rust_program_generator` to create a Rust program from an `Emojitape`. The initial approach was to generate a new Cargo project that would then be compiled separately. However, this has led to complexities with string formatting and escaping within the generator, causing repeated compilation errors.

## 2. New Strategic Plan

To simplify the process and make incremental progress, we will adopt a more direct approach.

**Phase 1: Direct WASM Generation**

1.  **Refactor `rust_program_generator.rs`:**
    *   The generator will no longer create a new Cargo project.
    *   It will read and parse the `full_generated_emojitape.emojitape` file as before.
    *   It will extract the `generated_wat_block` string.
    *   It will use the `wat` crate (which is already a dependency) to directly parse this WAT string into a WASM binary.
    *   It will save the resulting WASM binary to a file named `module.wasm`.

**Phase 2: WASM Execution and Verification**

1.  **Add `wasmtime` Dependency:** Add the `wasmtime` crate to the `emojitape_interpreter/Cargo.toml` to serve as the runtime for the generated WASM.
2.  **Create a `wasm_runner` Binary:**
    *   Create a new binary within the existing project named `wasm_runner`.
    *   This binary will be responsible for loading and executing the `module.wasm` file.
3.  **Implement Runner Logic:**
    *   The `wasm_runner` will use `wasmtime` to load `module.wasm`.
    *   It will inspect the module to find the exported functions (e.g., the functions defined in the `generated_wat_block`).
    *   It will call these functions and print their results to verify that the generated WASM is correct.

## 3. Expected Outcome

This new plan will allow us to:

*   Immediately verify the correctness of the WAT to WASM compilation.
*   Avoid the complexities of generating and compiling a separate Rust project.
*   Have a clear, testable artifact (`module.wasm`).
*   Incrementally build towards a more complex code generation system in the future.

## 4. Next Steps

1.  Commit the current (non-compiling) state with a "KitKat break" message.
2.  Begin implementing Phase 1 by refactoring `rust_program_generator.rs`.
