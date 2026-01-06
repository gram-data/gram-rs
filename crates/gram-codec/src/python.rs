//! Python bindings for Gram Codec
//!
//! This module provides Python bindings via PyO3 for the gram codec,
//! enabling use from Python code.
//!
//! # Usage
//!
//! ```python
//! from gram_codec import parse_gram, validate_gram, round_trip
//!
//! # Parse gram notation
//! result = parse_gram("(alice)-[:KNOWS]->(bob)")
//! print(f"Parsed {result['pattern_count']} patterns")
//! print(f"Identifiers: {result['identifiers']}")
//!
//! # Validate gram notation
//! is_valid = validate_gram("(hello)-->(world)")
//! print(f"Valid: {is_valid}")
//!
//! # Round-trip test
//! original = "(alice)-->(bob)"
//! serialized = round_trip(original)
//! print(f"Round-trip: {original} -> {serialized}")
//! ```

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

/// Parse gram notation and return a dictionary with pattern information
///
/// Args:
///     input (str): Gram notation text to parse
///
/// Returns:
///     dict: Dictionary with keys:
///         - 'pattern_count': Number of patterns parsed
///         - 'identifiers': List of root pattern identifiers
///
/// Raises:
///     ValueError: If parsing fails
#[pyfunction]
fn parse_gram(input: &str) -> PyResult<HashMap<String, PyObject>> {
    Python::with_gil(|py| {
        // Parse using the native parser
        let patterns = crate::parse_gram_notation(input)
            .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e.message)))?;

        // Extract identifiers
        let identifiers: Vec<String> = patterns
            .iter()
            .map(|p| p.value.identity.0.clone())
            .collect();

        // Build result dictionary
        let mut result = HashMap::new();
        result.insert("pattern_count".to_string(), patterns.len().into_py(py));
        result.insert("identifiers".to_string(), identifiers.into_py(py));

        Ok(result)
    })
}

/// Validate gram notation syntax
///
/// Args:
///     input (str): Gram notation text to validate
///
/// Returns:
///     bool: True if valid, False if invalid
#[pyfunction]
fn validate_gram(input: &str) -> bool {
    crate::parse_gram_notation(input).is_ok()
}

/// Round-trip test: parse and serialize back to gram notation
///
/// Args:
///     input (str): Original gram notation
///
/// Returns:
///     str: Serialized gram notation (may differ in formatting)
///
/// Raises:
///     ValueError: If parsing or serialization fails
#[pyfunction]
fn round_trip(input: &str) -> PyResult<String> {
    // Parse
    let patterns = crate::parse_gram_notation(input)
        .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e.message)))?;

    // Serialize all patterns
    crate::serialize_patterns(&patterns)
        .map_err(|e| PyValueError::new_err(format!("Serialize error: {}", e.reason)))
}

/// Get the version of the gram codec
///
/// Returns:
///     str: Version string
#[pyfunction]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Python module definition
#[pymodule]
fn gram_codec(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_gram, m)?)?;
    m.add_function(wrap_pyfunction!(validate_gram, m)?)?;
    m.add_function(wrap_pyfunction!(round_trip, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_gram() {
        assert!(validate_gram("(hello)"));
        assert!(validate_gram("(a)-->(b)"));
        assert!(!validate_gram("(unclosed"));
    }

    #[test]
    fn test_round_trip() {
        let input = "(hello)";
        let output = round_trip(input).unwrap();
        assert_eq!(output, "(hello)");
    }
}
