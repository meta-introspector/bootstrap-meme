### SOP: Refactoring `syn` Parsing Logic

**1. Purpose**

This SOP documents the process of refactoring the `syn` parsing logic in the `emojitape_interpreter` project. The goal of this refactoring is to improve the modularity and maintainability of the code by splitting a large, monolithic parsing function into smaller, more focused functions.

**2. Scope**

This SOP applies to the `declaration_splitter.rs` file and the new `src/parser/declarations` module.

**3. Procedure**

**Phase 1: Analysis and Planning**

**Objective:** To identify the code that needs to be refactored and plan the new structure.

**Steps:**

1.  **Identify the problem:** The `declaration_splitter.rs` file contained a large `item_to_declaration` function with a `match` statement that handled all the different types of Rust declarations (`ItemFn`, `ItemStruct`, etc.). This made the function difficult to read, maintain, and extend.
2.  **Plan the new structure:** The plan was to create a new `parser` module with a `declarations` submodule. Inside the `declarations` submodule, each declaration type would have its own file and a dedicated function for parsing it.

**Phase 2: Implementation**

**Objective:** To implement the new parsing structure.

**Steps:**

1.  **Create the directory structure:** A new directory was created at `src/parser/declarations`.
2.  **Create individual parsing functions:** For each declaration type (`Function`, `Struct`, `Enum`, `Trait`, `Impl`), a new file was created in the `src/parser/declarations` directory (e.g., `function.rs`, `struct.rs`). The parsing logic for each type was moved from the `item_to_declaration` function in `declaration_splitter.rs` to its corresponding new file.
3.  **Create a `types.rs` file:** The `Declaration` and `DeclarationType` structs were moved to their own file at `src/parser/declarations/types.rs` to improve code organization.
4.  **Create a `mod.rs` file:** A `mod.rs` file was created in `src/parser/declarations` to export all the new parsing functions and types.
5.  **Refactor the original file:** The `declaration_splitter.rs` file was updated to use the new functions from the `parser::declarations` module. The large `match` statement in `item_to_declaration` was replaced with calls to the new, specific parsing functions.
6.  **Update `code_analyzer.rs`:** The `code_analyzer.rs` file was updated to use the new `parser` module instead of the old `DeclarationSplitter`.
7.  **Cleanup:** The now-obsolete `declaration_splitter.rs` file was deleted.

**4. New Structure**

The new structure of the parser is as follows:

```
src/
├── parser/
│   ├── declarations/
│   │   ├── function.rs
│   │   ├── struct.rs
│   │   ├── enum.rs
│   │   ├── trait.rs
│   │   ├── impl.rs
│   │   ├── types.rs
│   │   └── mod.rs
│   └── mod.rs
└── ...
```

**5. Code Examples**

**Old Code (in `declaration_splitter.rs`):**

```rust
fn item_to_declaration(
    &self,
    item: &Item,
    lines: &[&str],
    file_path: Option<String>,
) -> Option<Declaration> {
    match item {
        Item::Fn(item_fn) => Some(self.function_to_declaration(item_fn, lines, file_path)),
        Item::Struct(item_struct) => {
            Some(self.struct_to_declaration(item_struct, lines, file_path))
        }
        Item::Enum(item_enum) => Some(self.enum_to_declaration(item_enum, lines, file_path)),
        Item::Trait(item_trait) => {
            Some(self.trait_to_declaration(item_trait, lines, file_path))
        }
        Item::Impl(item_impl) => Some(self.impl_to_declaration(item_impl, lines, file_path)),
        _ => None,
    }
}
```

**New Code (in `declaration_splitter.rs`):**

```rust
fn item_to_declaration(
    &self,
    item: &Item,
    lines: &[&str],
    file_path: Option<String>,
) -> Option<Declaration> {
    match item {
        Item::Fn(item_fn) => Some(declarations::function_to_declaration(item_fn, lines, file_path)),
        Item::Struct(item_struct) => {
            Some(declarations::struct_to_declaration(item_struct, lines, file_path))
        }
        Item::Enum(item_enum) => Some(declarations::enum_to_declaration(item_enum, lines, file_path)),
        Item::Trait(item_trait) => {
            Some(declarations::trait_to_declaration(item_trait, lines, file_path))
        }
        Item::Impl(item_impl) => Some(declarations::impl_to_declaration(item_impl, lines, file_path)),
        _ => None,
    }
}
```

**6. Expected Outcome**

*   Improved code modularity and maintainability.
*   Easier to add new declaration types in the future.
*   Clearer separation of concerns between parsing and other parts of the application.
