// src/main.rs
mod ingestion_refactored;
mod analysis;
mod ooda_loop_refactored;
mod embedding;

fn main() {
    ooda_loop_refactored::run_ooda_loop();
}
