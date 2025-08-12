### Rust Code Editing Framework

#### Objective
Edit Rust code to split changes into reusable, parameterized modules, composed functionally, with quality and traceability (ISO 9001, GMP, Six Sigma, ITIL, C4, UML, SOP).

#### Principles
- **Modularity**: Split code into small, parameterized Rust modules/traits.
- **Functional Composition**: New modules use functional patterns (e.g., pure functions, immutability).
- **Quality**: Minimize defects (Six Sigma), ensure compliance (ISO 9001, GMP), and align with ITIL.
- **Visualization**: Use C4 and UML for architecture and documentation.
- **Standardization**: Follow SOPs for consistency.

#### Process

1. **Plan (ISO 9001, ITIL, SOP)**:
   - Log change request (ITIL).
   - Assess impact with C4 Context/Component diagrams.
   - Define SOP for edit process.

2. **Analyze & Split (Six Sigma, UML)**:
   - Use Six Sigma to analyze code (e.g., `cargo clippy` for issues).
   - Model with UML (Class/Sequence diagrams).
   - Split into parameterized modules:
     ```rust
     // Before: Monolithic function
     fn handle_request(req: Request) -> Response { /* ... */ }

     // After: Split modules
     pub mod parser {
         pub fn parse_request(req: Request, config: Config) -> ParsedData { /* ... */ }
     }
     pub mod processor {
         pub fn process_action(data: ParsedData, action: ActionType) -> Result { /* ... */ }
     }
     pub mod responder {
         pub fn send_response(result: Result, format: &str) -> Response { /* ... */ }
     }
     ```

3. **Rewrite & Compose (C4, UML)**:
   - Design new module with UML.
   - Update C4 diagrams (Component/Code levels).
   - Compose functionally:
     ```rust
     use parser::parse_request;
     use processor::process_action;
     use responder::send_response;

     pub fn handle_request_workflow(req: Request, config: Config, action: ActionType, format: &str) -> Response {
         let parsed = parse_request(req, config);
         let result = process_action(parsed, action);
         send_response(result, format)
     }
     ```

4. **Quality Check (ISO 9001, GMP, Six Sigma)**:
   - Write unit tests (`#[test]` for each module).
   - Run integration tests (`cargo test`).
   - Peer review for SOP compliance.
   - Update C4/UML docs and commit to version control.

5. **Deploy & Monitor (ITIL, Six Sigma)**:
   - Deploy via ITIL pipeline.
   - Monitor errors (e.g., logging metrics).
   - Track defects with Six Sigma tools.

6. **Improve (Six Sigma, ISO 9001)**:
   - Analyze defect rates.
   - Update SOPs based on feedback.
   - Audit for compliance.

---

### Example
**Task**: Add logging to `handle_request`.

1. **Plan**: Log change in ITIL, update C4 diagram, draft SOP.
2. **Split**:
   ```rust
   pub mod logger {
       pub fn log_action(result: &Result) { /* Log to file */ }
   }
   ```
3. **Compose**:
   ```rust
   pub fn handle_request_workflow(req: Request, config: Config, action: ActionType, format: &str) -> Response {
       let parsed = parse_request(req, config);
       let result = process_action(parsed, action);
       logger::log_action(&result); // New feature
       send_response(result, format)
   }
   ```
4. **Quality**: Test with `cargo test`, review, update docs.
5. **Deploy**: Roll out via pipeline, monitor logs.
6. **Improve**: Refine SOP for logging standards.

---

This framework ensures modular, high-quality Rust code edits with minimal future changes, leveraging Rust’s type safety and functional patterns.