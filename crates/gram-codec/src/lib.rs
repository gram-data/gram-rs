//! # Gram Codec
//!
//! Bidirectional codec between Gram notation (human-readable text format) and Pattern data structures.
//!
//! This crate provides:
//! - **Parsing**: Transform Gram notation text into Pattern structures
//! - **Serialization**: Transform Pattern structures into valid Gram notation
//!
//! ## Features
//!
//! - Full support for all Gram syntax forms (nodes, relationships, subject patterns, annotations)
//! - Round-trip correctness (parse → serialize → parse produces equivalent pattern)
//! - Error recovery (reports all syntax errors, not just the first)
//! - Multi-platform support (native Rust, WebAssembly, Python)
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use gram_codec::{parse_gram_notation, serialize_pattern};
//!
//! // Parse gram notation into patterns
//! let gram_text = "(alice:Person {name: \"Alice\"})-[:KNOWS]->(bob:Person {name: \"Bob\"})";
//! let patterns = parse_gram_notation(gram_text)?;
//!
//! // Serialize patterns back to gram notation
//! for pattern in &patterns {
//!     let output = serialize_pattern(pattern)?;
//!     println!("{}", output);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Grammar Authority
//!
//! This codec uses [`tree-sitter-gram`](https://github.com/gram-data/tree-sitter-gram) as the
//! authoritative grammar specification. All Gram notation is validated against this grammar.

// Module declarations
mod error;
mod parser;
mod serializer;
pub(crate) mod transform;
mod value;

// Optional platform-specific modules
#[cfg(feature = "wasm")]
mod wasm;

#[cfg(feature = "python")]
mod python;

// Public API exports
pub use error::{Location, ParseError, SerializeError};
pub use parser::{parse_gram_notation, parse_single_pattern};
pub use serializer::{serialize_pattern, serialize_patterns};
pub use value::Value;

// Re-export Pattern and Subject from pattern-core for convenience
pub use pattern_core::{Pattern, Subject};
