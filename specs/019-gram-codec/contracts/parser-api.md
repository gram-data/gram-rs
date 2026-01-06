# Parser API Contract

**Feature**: 019-gram-codec  
**Module**: `crates/gram-codec/src/parser.rs`  
**Created**: 2026-01-06

## Overview

The parser transforms gram notation (text) into Pattern data structures by leveraging the tree-sitter-gram parser and transforming the concrete syntax tree (CST) into Pattern AST.

## Public API

### Primary Parsing Function

```rust
/// Parse gram notation into a collection of patterns.
///
/// # Arguments
/// * `input` - Gram notation text (may contain multiple patterns, comments, root record)
///
/// # Returns
/// * `Ok(Vec<Pattern<Subject>>)` - Successfully parsed patterns (may be empty for empty/whitespace input)
/// * `Err(ParseError)` - Parse errors with location and messages
///
/// # Examples
/// ```
/// use gram_codec::parse_gram_notation;
///
/// // Simple node
/// let patterns = parse_gram_notation("(hello)").unwrap();
/// assert_eq!(patterns.len(), 1);
///
/// // Multiple patterns
/// let patterns = parse_gram_notation("(a) (b) (c)").unwrap();
/// assert_eq!(patterns.len(), 3);
///
/// // Empty input returns empty collection
/// let patterns = parse_gram_notation("   \n  ").unwrap();
/// assert_eq!(patterns.len(), 0);
///
/// // Comments are ignored
/// let patterns = parse_gram_notation("// comment\n(hello)").unwrap();
/// assert_eq!(patterns.len(), 1);
/// ```
pub fn parse_gram_notation(input: &str) -> Result<Vec<Pattern<Subject>>, ParseError>;
```

### Single Pattern Parsing (Convenience)

```rust
/// Parse gram notation that contains exactly one pattern.
///
/// # Arguments
/// * `input` - Gram notation text expected to contain one pattern
///
/// # Returns
/// * `Ok(Pattern<Subject>)` - Successfully parsed single pattern
/// * `Err(ParseError)` - Parse errors or multiple patterns found
///
/// # Errors
/// Returns error if:
/// - Input contains zero patterns (empty/whitespace only)
/// - Input contains multiple patterns (use `parse_gram_notation` instead)
/// - Syntax errors in gram notation
///
/// # Examples
/// ```
/// use gram_codec::parse_single_pattern;
///
/// // Valid single pattern
/// let pattern = parse_single_pattern("(hello)").unwrap();
///
/// // Error: empty input
/// let err = parse_single_pattern("   ").unwrap_err();
/// assert!(err.message.contains("no patterns"));
///
/// // Error: multiple patterns
/// let err = parse_single_pattern("(a) (b)").unwrap_err();
/// assert!(err.message.contains("multiple patterns"));
/// ```
pub fn parse_single_pattern(input: &str) -> Result<Pattern<Subject>, ParseError>;
```

### Parse with Options (Future Extension)

```rust
/// Parse gram notation with custom options.
///
/// # Arguments
/// * `input` - Gram notation text
/// * `options` - Parsing options (error recovery, validation level, etc.)
///
/// # Returns
/// * `Ok(ParseResult)` - Parse result with patterns and optional warnings
/// * `Err(ParseError)` - Fatal parse errors
///
/// Note: This is a future extension for advanced use cases.
/// Initial implementation will use default options.
pub fn parse_with_options(
    input: &str,
    options: &ParseOptions,
) -> Result<ParseResult, ParseError>;

pub struct ParseOptions {
    /// Whether to perform error recovery (report all errors vs fail on first)
    pub error_recovery: bool,
    
    /// Maximum nesting depth to prevent stack overflow
    pub max_depth: usize,
    
    /// Validate patterns during parsing (structure checks)
    pub validate: bool,
}

pub struct ParseResult {
    pub patterns: Vec<Pattern<Subject>>,
    pub warnings: Vec<ParseWarning>,
}
```

## Internal Functions (Not Public)

### Tree-Sitter Integration

```rust
/// Create tree-sitter parser configured for gram notation
fn create_parser() -> tree_sitter::Parser;

/// Parse input using tree-sitter-gram and return syntax tree
fn parse_to_tree(input: &str) -> Result<tree_sitter::Tree, ParseError>;

/// Extract parse errors from tree-sitter error nodes
fn extract_errors(tree: &tree_sitter::Tree, input: &str) -> Vec<ParseError>;
```

### CST → Pattern Transformation

```rust
/// Transform tree-sitter CST to Pattern AST
fn transform_tree(tree: &tree_sitter::Tree, input: &str) -> Result<Vec<Pattern<Subject>>, ParseError>;

/// Transform a gram_pattern node to Pattern
fn transform_gram_pattern(node: &tree_sitter::Node, input: &str) -> Result<Vec<Pattern<Subject>>, ParseError>;

/// Transform a node_pattern to Pattern (0 elements)
fn transform_node_pattern(node: &tree_sitter::Node, input: &str) -> Result<Pattern<Subject>, ParseError>;

/// Transform a relationship_pattern to Pattern (2 elements)
fn transform_relationship_pattern(node: &tree_sitter::Node, input: &str) -> Result<Pattern<Subject>, ParseError>;

/// Transform a subject_pattern to Pattern (N elements)
fn transform_subject_pattern(node: &tree_sitter::Node, input: &str) -> Result<Pattern<Subject>, ParseError>;

/// Transform an annotated_pattern to Pattern (1 element)
fn transform_annotated_pattern(node: &tree_sitter::Node, input: &str) -> Result<Pattern<Subject>, ParseError>;

/// Transform a subject (identifier, labels, record) to Subject
fn transform_subject(node: &tree_sitter::Node, input: &str) -> Result<Subject, ParseError>;

/// Transform a record to HashMap<String, Value>
fn transform_record(node: &tree_sitter::Node, input: &str) -> Result<HashMap<String, Value>, ParseError>;

/// Transform a value node to Value enum
fn transform_value(node: &tree_sitter::Node, input: &str) -> Result<Value, ParseError>;
```

### Arrow Type Handling

```rust
/// Determine arrow type and element ordering for relationship patterns
fn handle_arrow_type(
    kind_node: &tree_sitter::Node,
    left: Pattern<Subject>,
    right: Pattern<Subject>,
) -> (Pattern<Subject>, Pattern<Subject>);

// Arrow type mapping:
// right_arrow (-->): preserve order [left, right]
// left_arrow (<--): reverse order [right, left] to maintain semantic direction
// bidirectional_arrow (<-->): preserve order [left, right] (symmetric)
// undirected_arrow (~~): preserve order [left, right] (symmetric)
```

## Error Handling

### Error Types

See [error-types.md](error-types.md) for detailed error type definitions.

**Key Points**:
- Errors include location information (line, column)
- Error recovery enabled by default (report all errors found)
- Multiple errors collected in `ParseError.errors` vector
- Descriptive messages indicating expected vs found tokens

### Error Recovery Strategy

```rust
/// Error recovery attempts to:
/// 1. Continue parsing after syntax errors
/// 2. Collect all errors (not just first)
/// 3. Return all errors in ParseError.errors
///
/// Recovery rules:
/// - Skip invalid tokens until next valid pattern start
/// - Collect all error nodes from tree-sitter
/// - Preserve valid patterns that were successfully parsed
fn recover_from_errors(tree: &tree_sitter::Tree, input: &str) -> Vec<ParseError>;
```

## Performance Characteristics

- **Time Complexity**: O(n) for n characters of input
- **Space Complexity**: O(n) for parse tree + O(m) for m patterns
- **Target**: <100ms for 1000-node patterns (native), <200ms (WASM)

## Testing Strategy

### Unit Tests
- Each transform function tested independently
- Edge cases: empty input, deeply nested, all syntax forms
- Error cases: invalid syntax, unclosed brackets, etc.

### Integration Tests
- End-to-end parsing of gram notation
- Validation against `gram-lint` (all test inputs must pass gram-lint first)
- Round-trip correctness (parse → serialize → parse)

### Corpus Tests
- Parse all test cases from `../tree-sitter-gram/test/corpus/`
- Verify successful parsing for all valid gram notation
- 27 corpus files × multiple tests per file

## Dependencies

```toml
[dependencies]
tree-sitter = "0.20"
tree-sitter-gram = { path = "../../../tree-sitter-gram/bindings/rust" }
pattern-core = { path = "../pattern-core" }
```

## Example Usage

```rust
use gram_codec::{parse_gram_notation, parse_single_pattern};

// Parse multiple patterns
let input = r#"
    // Social graph
    (alice:Person {name: "Alice"})
    (bob:Person {name: "Bob"})
    (alice)-[:KNOWS]->(bob)
"#;

let patterns = parse_gram_notation(input)?;
assert_eq!(patterns.len(), 3);

// Parse single pattern
let pattern = parse_single_pattern("(hello)-->(world)")?;
assert_eq!(pattern.elements().len(), 2);

// Handle errors
match parse_single_pattern("(unclosed") {
    Ok(_) => panic!("Should have failed"),
    Err(e) => {
        println!("Parse error at {}:{}", e.location.line, e.location.column);
        println!("Message: {}", e.message);
    }
}
```

## Validation Requirements

All parsed patterns must:
1. Conform to tree-sitter-gram grammar (validated by tree-sitter parser)
2. Transform correctly to Pattern structures
3. Round-trip correctly (parse → serialize → parse produces equivalent)
4. Handle all gram syntax forms from VALIDATION.md

## Future Extensions

- Streaming/incremental parsing for large files
- Custom error handlers for specific error types
- Source location preservation for better error reporting
- Performance optimizations for deeply nested structures

