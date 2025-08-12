// src/cocci_parser/mod.rs
pub mod cocci_element;
pub use cocci_element::CocciElement;
pub mod cocci_parser_function;
pub use cocci_parser_function::parse_cocci_content;
pub mod tests; // Import the new module



