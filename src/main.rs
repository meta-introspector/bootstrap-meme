// src/main.rs
mod ingestion;
mod ingestion_refactored;
mod analysis;
mod ooda_loop;
mod embedding;

fn main() {
    ooda_loop::run_ooda_loop();
}
