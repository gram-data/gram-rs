# Serializer API Contract

**Feature**: 019-gram-codec  
**Module**: `crates/gram-codec/src/serializer.rs`  
**Created**: 2026-01-06

## Overview

The serializer transforms Pattern data structures into valid gram notation text. All serialized output must pass `gram-lint` validation.

## Public API

### Primary Serialization Function

```rust
/// Serialize a Pattern to gram notation.
///
/// # Arguments
/// * `pattern` - Pattern to serialize
///
/// # Returns
/// * `Ok(String)` - Valid gram notation (passes `gram-lint`)
/// * `Err(SerializeError)` - Serialization errors
///
/// # Format Selection
/// - 0 elements → node notation: `(value)`
/// - 2 elements (both atomic) → relationship notation: `(left)-->(right)`
/// - Other cases → subject pattern notation: `[value | elements...]`
///
/// # Examples
/// ```
/// use gram_codec::to_gram_pattern;
/// use pattern_core::Pattern;
///
/// // Node pattern (0 elements)
/// let pattern = Pattern::point(Subject::with_identifier("hello"));
/// let gram = to_gram_pattern(&pattern).unwrap();
/// assert_eq!(gram, "(hello)");
///
/// // Relationship pattern (2 atomic elements)
/// let pattern = create_relationship("a", "b");
/// let gram = to_gram_pattern(&pattern).unwrap();
/// assert_eq!(gram, "(a)-->(b)");
///
/// // Subject pattern (multiple elements)
/// let pattern = create_subject_pattern("team", vec!["alice", "bob"]);
/// let gram = to_gram_pattern(&pattern).unwrap();
/// assert_eq!(gram, "[team | alice, bob]");
/// ```
pub fn to_gram_pattern(pattern: &Pattern<Subject>) -> Result<String, SerializeError>;
```

### Collection Serialization

```rust
/// Serialize multiple patterns to gram notation.
///
/// # Arguments
/// * `patterns` - Slice of patterns to serialize
///
/// # Returns
/// * `Ok(String)` - Valid gram notation with each pattern on a line
/// * `Err(SerializeError)` - Serialization errors
///
/// # Format
/// Patterns are separated by newlines for readability:
/// ```gram
/// (pattern1)
/// (pattern2)
/// (pattern3)
/// ```
///
/// # Examples
/// ```
/// use gram_codec::to_gram_patterns;
///
/// let patterns = vec![
///     Pattern::point(Subject::with_identifier("a")),
///     Pattern::point(Subject::with_identifier("b")),
/// ];
/// let gram = to_gram_patterns(&patterns).unwrap();
/// assert_eq!(gram, "(a)\n(b)");
/// ```
pub fn to_gram_patterns(patterns: &[Pattern<Subject>]) -> Result<String, SerializeError>;
```

### Serialization with Options (Future Extension)

```rust
/// Serialize with custom formatting options.
///
/// # Arguments
/// * `pattern` - Pattern to serialize
/// * `options` - Formatting options
///
/// # Returns
/// * `Ok(String)` - Formatted gram notation
/// * `Err(SerializeError)` - Serialization errors
///
/// Note: This is a future extension for custom formatting.
/// Initial implementation will use default formatting.
pub fn serialize_with_options(
    pattern: &Pattern<Subject>,
    options: &SerializeOptions,
) -> Result<String, SerializeError>;

pub struct SerializeOptions {
    /// Indentation for nested patterns (default: no indentation)
    pub indent: Option<String>,
    
    /// Include comments (future: if comments were preserved during parsing)
    pub include_comments: bool,
    
    /// Compact vs pretty formatting
    pub compact: bool,
    
    /// Maximum line length before wrapping (0 = no wrapping)
    pub max_line_length: usize,
}
```

## Internal Functions (Not Public)

### Format Selection Logic

```rust
/// Determine appropriate gram notation format for pattern
fn select_format(pattern: &Pattern<Subject>) -> GramFormat;

enum GramFormat {
    Node,           // 0 elements: (value)
    Relationship,   // 2 atomic elements: (a)-->(b)
    SubjectPattern, // Other: [value | elements...]
    Annotation,     // 1 element (if annotation metadata available)
}

/// Check if pattern qualifies for relationship notation
fn is_relationship_pattern(pattern: &Pattern<Subject>) -> bool {
    // True if:
    // - Exactly 2 elements
    // - Both elements are atomic (0 elements each)
    pattern.elements().len() == 2
        && pattern.elements()[0].elements().is_empty()
        && pattern.elements()[1].elements().is_empty()
}
```

### Serialization Functions

```rust
/// Serialize as node pattern: (identifier:Label {props})
fn serialize_node_pattern(pattern: &Pattern<Subject>) -> Result<String, SerializeError>;

/// Serialize as relationship pattern: (left)-->(right)
fn serialize_relationship_pattern(pattern: &Pattern<Subject>) -> Result<String, SerializeError>;

/// Serialize as subject pattern: [value | elem1, elem2, ...]
fn serialize_subject_pattern(pattern: &Pattern<Subject>) -> Result<String, SerializeError>;

/// Serialize Subject (identifier, labels, properties)
fn serialize_subject(subject: &Subject) -> Result<String, SerializeError>;

/// Serialize property record: {key1: value1, key2: value2}
fn serialize_record(record: &HashMap<String, Value>) -> Result<String, SerializeError>;

/// Serialize a Value enum variant
fn serialize_value(value: &Value) -> Result<String, SerializeError>;
```

### Escaping and Quoting

```rust
/// Escape special characters in identifiers/strings
fn escape_string(s: &str) -> String;

/// Quote identifier if needed (contains spaces, special chars, or is keyword)
fn quote_identifier(s: &str) -> String;

/// Determine if identifier needs quoting
fn needs_quoting(s: &str) -> bool {
    // Needs quoting if:
    // - Contains whitespace
    // - Contains special characters: {}[]():,@#-~
    // - Starts with digit
    // - Is empty
}
```

## Value Serialization Rules

See [value-enum.md](value-enum.md) for Value enum definition.

**Serialization by Type**:
- `Value::String(s)` → `"s"` (quoted if contains special chars, else unquoted)
- `Value::Integer(i)` → `42` (decimal notation)
- `Value::Decimal(f)` → `3.14` (decimal notation with point)
- `Value::Boolean(b)` → `true` | `false`
- `Value::Array(values)` → `[v1, v2, v3]`
- `Value::Range{lower, upper}` → `1..10`
- `Value::TaggedString{tag, content}` → `"""tag content"""`

## Format Examples

### Node Patterns (0 elements)

```rust
// Empty node
Pattern { value: Subject::empty(), elements: [] }
→ "()"

// With identifier
Pattern { value: Subject::with_identifier("hello"), elements: [] }
→ "(hello)"

// With label
Pattern { value: Subject { identifier: Some("a"), labels: vec!["Person"], record: {} }, elements: [] }
→ "(a:Person)"

// With properties
Pattern { value: Subject { identifier: Some("a"), labels: [], record: {name: "Alice"} }, elements: [] }
→ "(a {name: \"Alice\"})"

// Complete
Pattern { value: Subject { identifier: Some("a"), labels: vec!["Person"], record: {name: "Alice"} }, elements: [] }
→ "(a:Person {name: \"Alice\"})"
```

### Relationship Patterns (2 atomic elements)

```rust
// Simple relationship
Pattern { 
    value: Subject::empty(), 
    elements: [
        Pattern { value: Subject::with_identifier("a"), elements: [] },
        Pattern { value: Subject::with_identifier("b"), elements: [] }
    ]
}
→ "(a)-->(b)"

// With relationship label/properties
Pattern { 
    value: Subject { identifier: None, labels: vec!["KNOWS"], record: {since: 2020} },
    elements: [
        Pattern { value: Subject::with_identifier("a"), elements: [] },
        Pattern { value: Subject::with_identifier("b"), elements: [] }
    ]
}
→ "(a)-[:KNOWS {since: 2020}]->(b)"
```

### Subject Patterns (N elements)

```rust
// Multiple elements
Pattern { 
    value: Subject::with_identifier("team"),
    elements: [
        Pattern { value: Subject::with_identifier("alice"), elements: [] },
        Pattern { value: Subject::with_identifier("bob"), elements: [] },
        Pattern { value: Subject::with_identifier("charlie"), elements: [] }
    ]
}
→ "[team | alice, bob, charlie]"

// Nested
Pattern {
    value: Subject::with_identifier("outer"),
    elements: [
        Pattern {
            value: Subject::with_identifier("inner"),
            elements: [
                Pattern { value: Subject::with_identifier("leaf"), elements: [] }
            ]
        }
    ]
}
→ "[outer | [inner | leaf]]"

// 2 elements but non-atomic (uses subject pattern, not relationship)
Pattern {
    value: Subject::with_identifier("root"),
    elements: [
        Pattern { value: Subject::with_identifier("a"), elements: [nested1] },
        Pattern { value: Subject::with_identifier("b"), elements: [] }
    ]
}
→ "[root | [a | nested1], b]"  // Not (a)-->(b) because first element is non-atomic
```

## Validation

All serialized output MUST:
1. Pass `gram-lint` validation (exit code 0)
2. Round-trip correctly (parse → serialize → parse produces equivalent)
3. Use canonical formatting (consistent spacing, no comments)

**Validation Process**:
```rust
// Validate serialized output
fn validate_output(gram: &str) -> Result<(), SerializeError> {
    // 1. Check gram-lint validation
    let lint_result = std::process::Command::new("gram-lint")
        .args(&["-e", gram])
        .output()?;
    
    if !lint_result.status.success() {
        return Err(SerializeError::InvalidGrammar { gram: gram.to_string() });
    }
    
    // 2. Check round-trip correctness
    let reparsed = parse_gram_notation(gram)?;
    // Compare reparsed with original pattern (structural equivalence)
    
    Ok(())
}
```

## Error Handling

See [error-types.md](error-types.md) for detailed error type definitions.

**Common Errors**:
- Cannot serialize pattern structure to gram notation
- Invalid property value (unsupported type)
- Invalid identifier (contains disallowed characters)

## Performance Characteristics

- **Time Complexity**: O(n) for n nodes in pattern
- **Space Complexity**: O(n) for output string
- **Target**: <100ms for 1000-node patterns (native), <200ms (WASM)

## Testing Strategy

### Unit Tests
- Each serialization function tested independently
- Format selection logic for all element counts
- Value serialization for all Value variants
- Escaping and quoting edge cases

### Integration Tests
- End-to-end serialization of patterns
- Validation with `gram-lint` for all output
- Round-trip correctness (serialize → parse → compare)

### Property-Based Tests
- Generate random patterns, serialize, validate, round-trip
- Ensure all serialized output passes `gram-lint`

## Dependencies

```toml
[dependencies]
pattern-core = { path = "../pattern-core" }

[dev-dependencies]
gram_codec = { path = ".", features = ["parser"] }  # For round-trip tests
```

## Example Usage

```rust
use gram_codec::to_gram_pattern;
use pattern_core::Pattern;

// Serialize a simple pattern
let pattern = Pattern::point(Subject::with_identifier("hello"));
let gram = to_gram_pattern(&pattern)?;
println!("{}", gram);  // Output: (hello)

// Validate output
assert!(validate_with_gram_lint(&gram));

// Round-trip test
let reparsed = parse_gram_notation(&gram)?;
assert_eq!(pattern, reparsed[0]);
```

## Future Extensions

- Pretty-printing with indentation
- Comment preservation (if parsing preserves comments)
- Configurable formatting styles
- Streaming serialization for very large patterns

