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
pub fn parse_gram_notation(input: &str) -> Result<Vec<Pattern<Subject>>, ParseError> {
    // Handle empty/whitespace-only input
    if input.trim().is_empty() {
        return Ok(vec![]);
    }

    // Parse input to tree-sitter tree
    let tree = parse_to_tree(input)?;

    // Check for parse errors
    let errors = extract_errors(&tree, input);
    if !errors.is_empty() {
        let mut primary = ParseError::new(
            Location::start(),
            format!("Parse failed with {} error(s)", errors.len()),
        );
        for error in errors {
            primary = primary.with_error(error);
        }
        return Err(primary);
    }

    // Transform tree to patterns
    crate::transform::transform_tree(&tree, input)
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
pub fn parse_single_pattern(input: &str) -> Result<Pattern<Subject>, ParseError> {
    let patterns = parse_gram_notation(input)?;

    match patterns.len() {
        0 => Err(ParseError::new(
            Location::start(),
            "Input contains no patterns".to_string(),
        )),
        1 => Ok(patterns.into_iter().next().unwrap()),
        n => Err(ParseError::new(
            Location::start(),
            format!("Input contains {} patterns, expected exactly 1", n),
        )),
    }
}

/// Create tree-sitter parser configured for gram notation
fn create_parser() -> Result<tree_sitter::Parser, ParseError> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_gram::LANGUAGE.into())
        .map_err(|e| {
            ParseError::new(Location::start(), format!("Failed to set language: {}", e))
        })?;
    Ok(parser)
}

/// Parse input using tree-sitter-gram and return syntax tree
fn parse_to_tree(input: &str) -> Result<tree_sitter::Tree, ParseError> {
    let mut parser = create_parser()?;
    parser.parse(input, None).ok_or_else(|| {
        ParseError::new(
            Location::start(),
            "Failed to parse input (tree-sitter returned None)".to_string(),
        )
    })
}

/// Extract parse errors from tree-sitter error nodes
fn extract_errors(tree: &tree_sitter::Tree, _input: &str) -> Vec<ParseError> {
    let mut errors = Vec::new();
    let root_node = tree.root_node();

    // Traverse tree to find error and missing nodes
    fn traverse(node: tree_sitter::Node, errors: &mut Vec<ParseError>) {
        if node.is_error() {
            errors.push(ParseError::from_node(
                &node,
                "Syntax error at this location".to_string(),
            ));
        } else if node.is_missing() {
            errors.push(ParseError::from_node(
                &node,
                format!("Missing required syntax: {}", node.kind()),
            ));
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            traverse(child, errors);
        }
    }

    traverse(root_node, &mut errors);
    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_input() {
        let result = parse_gram_notation("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_parse_whitespace_only() {
        let result = parse_gram_notation("   \n  \t  ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_parse_single_pattern_empty() {
        let result = parse_single_pattern("");
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("no patterns"));
    }

    #[test]
    fn test_create_parser() {
        let result = create_parser();
        assert!(result.is_ok());
    }
}
