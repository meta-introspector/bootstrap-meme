@rule@
@@
// src/main.rs
- fn main() {
-     println!("Hello, world!");
- }
+ mod ingestion;
+ mod analysis;
+ mod ooda_loop;
+ mod embedding;
+ mod replication; // New module for self-replication
+
+ fn main() {
+     // For testing self-replication
+     replication::replicate_self().expect("Failed to replicate self");
+     // Original OODA loop call
+     // ooda_loop::run_ooda_loop();
+ }
