//! Parser for Gram notation

use crate::{error::Location, ParseError};
use pattern_core::{Pattern, Subject};

/// Parse Gram notation text into a collection of Pattern structures
///
/// # Arguments
///
/// * `input` - Gram notation text to parse
///
/// # Returns
///
/// * `Ok(Vec<Pattern<Subject>>)` - Successfully parsed patterns
/// * `Err(ParseError)` - Parse errors with location information
///
/// # Example
///
/// ```rust,no_run
/// use gram_codec::parse_gram_notation;
///
/// let patterns = parse_gram_notation("(alice)-[:KNOWS]->(bob)")?;
/// # Ok::<(), gram_codec::ParseError>(())
/// ```
pub fn parse_gram_notation(_input: &str) -> Result<Vec<Pattern<Subject>>, ParseError> {
    // TODO: Implement in Phase 3
    Err(ParseError {
        location: Location::start(),
        message: "Not yet implemented".to_string(),
        errors: vec![],
    })
}

/// Parse a single Gram pattern from text
///
/// Convenience function that expects exactly one pattern in the input.
///
/// # Arguments
///
/// * `input` - Gram notation text containing a single pattern
///
/// # Returns
///
/// * `Ok(Pattern<Subject>)` - Successfully parsed pattern
/// * `Err(ParseError)` - Parse error or multiple patterns found
pub fn parse_single_pattern(_input: &str) -> Result<Pattern<Subject>, ParseError> {
    // TODO: Implement in Phase 3
    Err(ParseError {
        location: Location::start(),
        message: "Not yet implemented".to_string(),
        errors: vec![],
    })
}
