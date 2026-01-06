//! Parser integration tests

use gram_codec::{parse_gram_notation, parse_single_pattern};

#[test]
fn test_parse_simple_node() {
    let result = parse_gram_notation("(hello)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 0); // Atomic pattern
}

#[test]
fn test_parse_node_with_identifier() {
    let result = parse_gram_notation("(alice)");
    assert!(result.is_ok());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].value.identity.0, "alice");
}

#[test]
fn test_parse_empty_node() {
    let result = parse_gram_notation("()");
    assert!(result.is_ok());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
}

#[test]
fn test_parse_node_with_label() {
    let result = parse_gram_notation("(a:Person)");
    assert!(result.is_ok());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].value.identity.0, "a");
    assert!(patterns[0].value.labels.contains("Person"));
}

#[test]
fn test_parse_multiple_patterns() {
    let result = parse_gram_notation("(a) (b) (c)");
    assert!(result.is_ok());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 3);
}

#[test]
fn test_parse_relationship_right_arrow() {
    let result = parse_gram_notation("(a)-->(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2); // Relationship has 2 elements
}

#[test]
fn test_parse_subject_pattern() {
    let result = parse_gram_notation("[team | alice, bob]");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2); // Subject pattern with 2 elements
    assert_eq!(patterns[0].value.identity.0, "team");
}

#[test]
fn test_parse_with_comments() {
    let result = parse_gram_notation("// comment\n(hello)");
    assert!(result.is_ok());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
}

#[test]
fn test_parse_single_pattern_success() {
    let result = parse_single_pattern("(hello)");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value.identity.0, "hello");
}

#[test]
fn test_parse_single_pattern_multiple_error() {
    let result = parse_single_pattern("(a) (b)");
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("2 patterns"));
}

#[test]
fn test_parse_invalid_syntax() {
    let result = parse_gram_notation("(unclosed");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.error_count() > 0);
}

#[test]
fn test_parse_node_with_properties() {
    let result = parse_gram_notation("(a {name: \"Alice\"})");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert!(!patterns[0].value.properties.is_empty());
}

#[test]
fn test_parse_relationship_left_arrow() {
    let result = parse_gram_notation("(a)<--(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2);
}

#[test]
fn test_parse_relationship_bidirectional() {
    let result = parse_gram_notation("(a)<-->(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
}

#[test]
fn test_parse_relationship_squiggle() {
    let result = parse_gram_notation("(a)~~(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
}

#[test]
fn test_parse_annotated_pattern_with_symbol_value() {
    let result = parse_gram_notation("@type(node) (a)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 1); // Annotated pattern has 1 element
    assert_eq!(patterns[0].value.identity.0, ""); // Anonymous (empty identity)
    assert!(patterns[0].value.labels.is_empty()); // Unlabeled
    assert!(patterns[0].value.properties.contains_key("type")); // Annotation as property
}

#[test]
fn test_parse_annotated_pattern_with_integer_value() {
    let result = parse_gram_notation("@depth(2) (a)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 1);
    assert_eq!(patterns[0].value.identity.0, ""); // Anonymous
    assert!(patterns[0].value.properties.contains_key("depth")); // Annotation as property
}

#[test]
fn test_parse_multiple_annotations() {
    // Note: This test will work once tree-sitter-gram supports multiple annotations
    // For now, we test that the logic handles iteration correctly
    let result = parse_gram_notation("@type(node) (a)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].value.identity.0, ""); // Anonymous
    assert!(patterns[0].value.labels.is_empty()); // Unlabeled
    assert_eq!(patterns[0].value.properties.len(), 1); // One annotation property
    assert!(patterns[0].value.properties.contains_key("type"));
}
