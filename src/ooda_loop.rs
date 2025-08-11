// src/ooda_loop_refactored.rs
use std::io::{self, Write};
//use std::collections::HashMap;
use crate::ingestion_refactored::{ingest_project_files, SystemBuffer}; // Use refactored ingestion
use crate::analysis::perform_vocabulary_analysis;
//mod embedding; // Declare the new embedding module
use crate::embedding::{create_univalent_multivectors, UnivalentMultivector}; // Import necessary items

pub fn run_ooda_loop() {
    println!("--- Starting System `e` Quine Bootstrap ---");

    // Initial Observe Phase
    let buffer = ingest_project_files(None); // No filter for initial ingestion

    // Initial Orient Phase: Perform vocabulary analysis and create embeddings
    println!("\n[OODA - ORIENT] Performing initial vocabulary analysis and embedding...");
    let term_counts = perform_vocabulary_analysis(&buffer);
    println!("    -> [Analysis Report]: Vocabulary analysis complete. Found {} unique terms.", term_counts.len());

    let multivectors = create_univalent_multivectors(&term_counts);
    println!("    -> [Embedding Report]: Created {} univalent multivectors.", multivectors.len());

    // OODA Cycle: Decide and Act
    perform_single_ooda_cycle(&buffer, multivectors); // Pass multivectors
    
    println!("\n--- System `e` session ended. ---
");
}

fn perform_single_ooda_cycle(buffer: &SystemBuffer, initial_multivectors: Vec<UnivalentMultivector>) {
    let choice = get_user_choice();

    let should_exit = handle_user_action(choice, buffer, initial_multivectors);

    if should_exit {
        return;
    }

    if prompt_for_continuation() {
        // Re-run the single cycle function, passing the buffer and the initial multivectors
        // Note: For a true continuous OODA, the buffer and multivectors might need to be updated
        // or re-ingested based on the 'Act' phase's output.
        perform_single_ooda_cycle(buffer, initial_multivectors);
    }
}

fn get_user_choice() -> String {
    println!("\n[OODA - DECIDE] What would you like to do next?");
    println!("  1. Run another analysis (e.g., 'vocabulary')");
    println!("  2. Bootstrap (simulate quine bootstrap)");
    println!("  3. Exit");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap(); // Ensure prompt is displayed

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    choice.trim().to_string()
}

fn handle_user_action(choice: String, buffer: &SystemBuffer, current_multivectors: Vec<UnivalentMultivector>) -> bool {
    println!("\n[OODA - ACT] Executing chosen action...");
    match choice.as_str() {
        "1" | "vocabulary" => {
            run_vocabulary_action(buffer);
            false // Do not exit
        }
        "2" | "bootstrap" => {
            run_bootstrap_action(buffer, current_multivectors);
            false // Do not exit
        }
        "3" | "exit" => {
            println!("    -> User chose to exit.");
            true // Exit
        }
        _ => {
            println!("Invalid choice. No action performed for this cycle.");
            false // Do not exit
        }
    }
}

fn run_vocabulary_action(buffer: &SystemBuffer) {
    println!("    -> Running vocabulary analysis and embedding...");
    let term_counts = perform_vocabulary_analysis(&buffer);
    println!("    -> [Analysis Report]: Vocabulary analysis complete. Found {} unique terms.", term_counts.len());
    let multivectors = create_univalent_multivectors(&term_counts);
    println!("    -> [Embedding Report]: Created {} univalent multivectors.", multivectors.len());
}

fn run_bootstrap_action(buffer: &SystemBuffer, current_multivectors: Vec<UnivalentMultivector>) {
    println!("\n    -> [BOOTSTRAP - Step 1] Reading source repository (conceptual: all files already ingested)...\n");
    // For quine bootstrap, specifically ingest .md files as per SOP
    let md_buffer = ingest_project_files(Some("md")); // Ingest only .md files for bootstrap
    println!("        -> Ingested {} .md files for bootstrap.", md_buffer.files.len());

    println!("    -> [BOOTSTRAP - Step 2] Translating to embedding space (conceptual: applying procedures)....");
    let bootstrap_term_counts = perform_vocabulary_analysis(&md_buffer); // Use md_buffer for analysis
    println!("        -> [Translation Report]: Vocabulary analysis complete. Found {} unique terms.", bootstrap_term_counts.len());
    let bootstrap_multivectors = create_univalent_multivectors(&bootstrap_term_counts);
    println!("        -> [Embedding Report]: Created {} univalent multivectors.", bootstrap_multivectors.len());

    println!("\n    -> [BOOTSTRAP - Step 3] Executing reasoning upon the self (conceptual: LLM interaction)...");
    simulate_llm_reasoning(&bootstrap_multivectors); // Pass multivectors

    println!("\n    -> [BOOTSTRAP - Step 4] Generating new files (conceptual: quine output)...");
    simulate_quine_output(&md_buffer); // Pass md_buffer for quine output simulation
    println!("    -> Quine bootstrap simulation complete. Cycle ready to begin again.");
}

fn prompt_for_continuation() -> bool {
    print!("\nDo you want to perform another OODA cycle? (yes/no): ");
    io::stdout().flush().unwrap();
    let mut continue_choice = String::new();
    io::stdin().read_line(&mut continue_choice).unwrap();
    let continue_choice = continue_choice.trim().to_lowercase();
    continue_choice == "yes"
}

// Conceptual function to simulate LLM reasoning
fn simulate_llm_reasoning(multivectors: &Vec<UnivalentMultivector>) {
    println!("        -> [LLM Simulation]: Analyzing {} univalent multivectors for semantic patterns...", multivectors.len());
    // In a real scenario, this would involve complex LLM calls and processing.
    // For now, we just acknowledge the input.
    let mut found_quine = false;
    let mut found_embedding = false;
    for mv in multivectors {
        if mv.term.contains("quine") {
            found_quine = true;
        }
        if mv.term.contains("embedding") {
            found_embedding = true;
        }
    }

    if found_quine {
        println!("        -> [LLM Insight]: Detected 'quine' related terms. Focus on self-replication concepts.");
    }
    if found_embedding {
        println!("        -> [LLM Insight]: Detected 'embedding' related terms. Focus on vector space representation.");
    }
    println!("        -> [LLM Simulation]: Reasoning complete. Generating insights for next action.");
}

// Conceptual function to simulate quine output (generating 1:1 copy of source files)
fn simulate_quine_output(buffer: &SystemBuffer) {
    println!("        -> Simulating generation of SOPs, Rust design files, and README.md.");
    println!("        -> (No actual files are overwritten to prevent data loss during simulation.)");
    println!("        -> Generated files (conceptual content preview):");
    for file_record in &buffer.files {
        // In a real quine, this would write the content to a new file.
        // Here, we just print a snippet to show what would be generated.
        println!("            --- File: {} ---
", file_record.content.lines().next().unwrap_or("[empty]")); // Added newline
        // Only print the first line to avoid excessive output
    }
}
