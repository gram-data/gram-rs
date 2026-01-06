# Error Types Contract

**Feature**: 019-gram-codec  
**Module**: `crates/gram-codec/src/error.rs`  
**Created**: 2026-01-06

## Overview

Error types for gram codec parsing and serialization operations. All errors include descriptive messages and location information where applicable.

## Type Definitions

### ParseError

```rust
/// Error during gram notation parsing
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    /// Location where error occurred
    pub location: Location,
    
    /// Primary error message
    pub message: String,
    
    /// Additional errors (for error recovery)
    /// When error recovery is enabled, this contains all errors found
    pub errors: Vec<ParseError>,
}

impl ParseError {
    /// Create a new parse error at a specific location
    pub fn new(location: Location, message: String) -> Self {
        Self {
            location,
            message,
            errors: Vec::new(),
        }
    }
    
    /// Create parse error from tree-sitter node
    pub fn from_node(node: &tree_sitter::Node, message: String) -> Self {
        Self::new(
            Location::from_node(node),
            message,
        )
    }
    
    /// Add additional error (for error recovery)
    pub fn with_error(mut self, error: ParseError) -> Self {
        self.errors.push(error);
        self
    }
    
    /// Create error for unexpected token
    pub fn unexpected_token(node: &tree_sitter::Node, expected: &str) -> Self {
        Self::from_node(
            node,
            format!("Unexpected token '{}', expected {}", node.kind(), expected),
        )
    }
    
    /// Create error for missing required field
    pub fn missing_field(node: &tree_sitter::Node, field: &str) -> Self {
        Self::from_node(
            node,
            format!("Missing required field '{}'", field),
        )
    }
    
    /// Create error for invalid value
    pub fn invalid_value(node: &tree_sitter::Node, details: String) -> Self {
        Self::from_node(
            node,
            format!("Invalid value: {}", details),
        )
    }
    
    /// Create error for unsupported value type
    pub fn unsupported_value_type(node: &tree_sitter::Node) -> Self {
        Self::from_node(
            node,
            format!("Unsupported value type: {}", node.kind()),
        )
    }
    
    /// Create error for invalid pattern structure
    pub fn invalid_structure(location: Location, details: String) -> Self {
        Self::new(
            location,
            format!("Invalid pattern structure: {}", details),
        )
    }
    
    /// Total number of errors (including nested)
    pub fn error_count(&self) -> usize {
        1 + self.errors.len()
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at {}:{}: {}", 
            self.location.line, 
            self.location.column, 
            self.message
        )?;
        
        if !self.errors.is_empty() {
            write!(f, "\nAdditional errors:")?;
            for error in &self.errors {
                write!(f, "\n  - {}", error)?;
            }
        }
        
        Ok(())
    }
}

impl std::error::Error for ParseError {}
```

### SerializeError

```rust
/// Error during pattern serialization to gram notation
#[derive(Debug, Clone, PartialEq)]
pub enum SerializeError {
    /// Pattern structure cannot be represented in gram notation
    InvalidStructure {
        reason: String,
    },
    
    /// Invalid value that cannot be serialized
    InvalidValue {
        value_type: String,
        reason: String,
    },
    
    /// Invalid identifier (contains disallowed characters)
    InvalidIdentifier {
        identifier: String,
        reason: String,
    },
    
    /// Serialized output failed validation
    ValidationFailed {
        gram: String,
        reason: String,
    },
    
    /// I/O error during serialization (e.g., writing to file)
    IoError {
        message: String,
    },
}

impl SerializeError {
    /// Create error for invalid pattern structure
    pub fn invalid_structure(reason: impl Into<String>) -> Self {
        Self::InvalidStructure {
            reason: reason.into(),
        }
    }
    
    /// Create error for invalid value
    pub fn invalid_value(value_type: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidValue {
            value_type: value_type.into(),
            reason: reason.into(),
        }
    }
    
    /// Create error for invalid identifier
    pub fn invalid_identifier(identifier: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidIdentifier {
            identifier: identifier.into(),
            reason: reason.into(),
        }
    }
    
    /// Create error for validation failure
    pub fn validation_failed(gram: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::ValidationFailed {
            gram: gram.into(),
            reason: reason.into(),
        }
    }
}

impl std::fmt::Display for SerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidStructure { reason } => {
                write!(f, "Cannot serialize pattern structure: {}", reason)
            }
            Self::InvalidValue { value_type, reason } => {
                write!(f, "Cannot serialize {} value: {}", value_type, reason)
            }
            Self::InvalidIdentifier { identifier, reason } => {
                write!(f, "Invalid identifier '{}': {}", identifier, reason)
            }
            Self::ValidationFailed { gram, reason } => {
                write!(f, "Serialized gram notation failed validation: {}\n{}", reason, gram)
            }
            Self::IoError { message } => {
                write!(f, "I/O error: {}", message)
            }
        }
    }
}

impl std::error::Error for SerializeError {}

impl From<std::io::Error> for SerializeError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError {
            message: err.to_string(),
        }
    }
}
```

### Location

```rust
/// Location in source code (1-indexed line and column)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    /// Line number (1-indexed)
    pub line: usize,
    
    /// Column number (1-indexed)
    pub column: usize,
}

impl Location {
    /// Create a new location
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
    
    /// Create location from tree-sitter node
    pub fn from_node(node: &tree_sitter::Node) -> Self {
        let start = node.start_position();
        Self {
            line: start.row + 1,      // tree-sitter is 0-indexed
            column: start.column + 1,  // tree-sitter is 0-indexed
        }
    }
    
    /// Location at start of file
    pub fn start() -> Self {
        Self { line: 1, column: 1 }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::start()
    }
}
```

## Error Scenarios

### Parse Errors

```rust
// Syntax error
ParseError::new(
    Location::new(1, 5),
    "Unexpected token ')', expected identifier".to_string(),
)

// Multiple errors (error recovery)
ParseError::new(
    Location::new(1, 1),
    "Parse failed with 3 errors".to_string(),
)
.with_error(ParseError::new(Location::new(1, 5), "Unclosed parenthesis".to_string()))
.with_error(ParseError::new(Location::new(2, 10), "Missing comma".to_string()))
.with_error(ParseError::new(Location::new(3, 1), "Invalid identifier".to_string()))

// Missing field
ParseError::missing_field(node, "identifier")

// Invalid value
ParseError::invalid_value(node, "Integer overflow: value exceeds i64::MAX".to_string())
```

### Serialize Errors

```rust
// Invalid structure
SerializeError::invalid_structure(
    "Pattern with 1 element cannot be serialized (use annotation or subject pattern)"
)

// Invalid value
SerializeError::invalid_value(
    "CustomType",
    "Cannot serialize custom value types to gram notation"
)

// Invalid identifier
SerializeError::invalid_identifier(
    "hello world",
    "Identifier contains whitespace (use quotes or subject pattern notation)"
)

// Validation failed
SerializeError::validation_failed(
    "(unclosed",
    "gram-lint validation failed with exit code 1"
)
```

## Error Conversion

### From tree-sitter Errors

```rust
impl From<tree_sitter::TreeSitterError> for ParseError {
    fn from(err: tree_sitter::TreeSitterError) -> Self {
        ParseError::new(
            Location::start(),
            format!("Tree-sitter error: {}", err),
        )
    }
}
```

### Error Propagation

```rust
// Parser returns Result with ParseError
pub fn parse_gram_notation(input: &str) -> Result<Vec<Pattern<Subject>>, ParseError> {
    let tree = parse_to_tree(input)?;  // Propagate parse error
    let patterns = transform_tree(&tree, input)?;  // Propagate transformation error
    Ok(patterns)
}

// Serializer returns Result with SerializeError
pub fn serialize_pattern(pattern: &Pattern<Subject>) -> Result<String, SerializeError> {
    let gram = match pattern.elements().len() {
        0 => serialize_node_pattern(pattern)?,  // Propagate serialization error
        2 if is_relationship_pattern(pattern) => serialize_relationship_pattern(pattern)?,
        _ => serialize_subject_pattern(pattern)?,
    };
    
    validate_output(&gram)?;  // Propagate validation error
    Ok(gram)
}
```

## Error Handling Best Practices

### Parser Error Handling

```rust
// Collect all errors during error recovery
fn parse_with_recovery(input: &str) -> Result<Vec<Pattern<Subject>>, ParseError> {
    let tree = parse_to_tree(input)?;
    
    // Check for parse errors in tree
    let errors = extract_errors(&tree, input);
    
    if !errors.is_empty() {
        // Return primary error with all collected errors
        let mut primary = ParseError::new(
            Location::start(),
            format!("Parse failed with {} errors", errors.len()),
        );
        
        for error in errors {
            primary = primary.with_error(error);
        }
        
        return Err(primary);
    }
    
    // Continue with transformation
    transform_tree(&tree, input)
}
```

### Serializer Error Handling

```rust
// Provide helpful error messages
fn serialize_value(value: &Value) -> Result<String, SerializeError> {
    match value {
        Value::String(s) => Ok(quote_if_needed(s)),
        Value::Integer(i) => Ok(i.to_string()),
        // ... other cases ...
        _ => Err(SerializeError::invalid_value(
            value.type_name(),
            "This value type is not supported in the current gram notation version"
        )),
    }
}
```

### Error Recovery Strategy

```rust
// Continue parsing after errors to find all issues
fn extract_errors(tree: &tree_sitter::Tree, input: &str) -> Vec<ParseError> {
    let mut errors = Vec::new();
    let mut cursor = tree.walk();
    
    fn traverse(cursor: &mut tree_sitter::TreeCursor, input: &str, errors: &mut Vec<ParseError>) {
        let node = cursor.node();
        
        if node.is_error() || node.is_missing() {
            errors.push(ParseError::from_node(
                &node,
                format!("Syntax error at this location"),
            ));
        }
        
        if cursor.goto_first_child() {
            loop {
                traverse(cursor, input, errors);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }
    
    traverse(&mut cursor, input, &mut errors);
    errors
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_error_display() {
        let error = ParseError::new(
            Location::new(1, 5),
            "Unexpected token".to_string(),
        );
        
        assert_eq!(
            error.to_string(),
            "Parse error at 1:5: Unexpected token"
        );
    }
    
    #[test]
    fn test_serialize_error_variants() {
        let error = SerializeError::invalid_structure("Test reason");
        assert!(error.to_string().contains("Test reason"));
        
        let error = SerializeError::invalid_value("CustomType", "Not supported");
        assert!(error.to_string().contains("CustomType"));
    }
    
    #[test]
    fn test_error_recovery() {
        let mut primary = ParseError::new(Location::start(), "Multiple errors".to_string());
        primary = primary.with_error(ParseError::new(Location::new(1, 1), "Error 1".to_string()));
        primary = primary.with_error(ParseError::new(Location::new(2, 1), "Error 2".to_string()));
        
        assert_eq!(primary.error_count(), 3);  // 1 primary + 2 additional
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_error_propagation() {
        let result = parse_gram_notation("(unclosed");
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.location.line >= 1);
        assert!(!error.message.is_empty());
    }
    
    #[test]
    fn test_serialization_validation() {
        // Create an invalid pattern that cannot be serialized
        let pattern = /* ... */;
        let result = serialize_pattern(&pattern);
        assert!(result.is_err());
    }
}
```

## Dependencies

```toml
[dependencies]
tree-sitter = "0.20"
```

## Future Extensions

- Structured error codes (e.g., `E0001: Unclosed parenthesis`)
- Error recovery suggestions ("Did you mean ...?")
- Colored error output for terminal display
- Integration with diagnostic protocols (LSP, etc.)
- Severity levels (error, warning, info)

