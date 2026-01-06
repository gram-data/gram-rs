# Value Enum Contract

**Feature**: 019-gram-codec  
**Module**: `crates/gram-codec/src/value.rs`  
**Created**: 2026-01-06

## Overview

The `Value` enum represents all value types supported by gram notation property records. It provides type-safe representation of heterogeneous property values.

## Type Definition

```rust
/// Represents a value in gram notation property records.
/// Supports all value types defined in the tree-sitter-gram grammar.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// String value (quoted or unquoted symbol)
    /// Example: "Alice", hello
    String(String),
    
    /// Integer value with full i64 range
    /// Example: 42, -10, 0
    Integer(i64),
    
    /// Decimal/floating-point value
    /// Example: 3.14, -2.5, 0.0
    Decimal(f64),
    
    /// Boolean value
    /// Example: true, false
    Boolean(bool),
    
    /// Array of values (may be heterogeneous)
    /// Example: ["rust", 42, true]
    /// Note: Grammar allows mixed-type arrays
    Array(Vec<Value>),
    
    /// Numeric range with inclusive bounds
    /// Example: 1..10, 0..100
    Range { lower: i64, upper: i64 },
    
    /// Tagged string with format identifier
    /// Example: """markdown # Heading"""
    TaggedString { tag: String, content: String },
}
```

## Implementation

### Core Methods

```rust
impl Value {
    /// Parse value from tree-sitter node
    pub fn from_tree_sitter_node(node: &tree_sitter::Node, source: &str) -> Result<Self, ParseError> {
        match node.kind() {
            "symbol" => Ok(Value::String(node.utf8_text(source.as_bytes())?.to_string())),
            "string_literal" => {
                let content = extract_string_content(node, source)?;
                Ok(Value::String(content))
            }
            "integer" => {
                let text = node.utf8_text(source.as_bytes())?;
                let value = text.parse::<i64>()
                    .map_err(|e| ParseError::invalid_value(node, format!("Invalid integer: {}", e)))?;
                Ok(Value::Integer(value))
            }
            "decimal" => {
                let text = node.utf8_text(source.as_bytes())?;
                let value = text.parse::<f64>()
                    .map_err(|e| ParseError::invalid_value(node, format!("Invalid decimal: {}", e)))?;
                Ok(Value::Decimal(value))
            }
            "boolean_literal" => {
                let text = node.utf8_text(source.as_bytes())?;
                let value = text == "true";
                Ok(Value::Boolean(value))
            }
            "array" => {
                let mut values = Vec::new();
                for child in node.children(&mut node.walk()) {
                    if child.is_named() {
                        values.push(Value::from_tree_sitter_node(&child, source)?);
                    }
                }
                Ok(Value::Array(values))
            }
            "range" => {
                let lower = extract_range_bound(node, "lower", source)?;
                let upper = extract_range_bound(node, "upper", source)?;
                Ok(Value::Range { lower, upper })
            }
            "tagged_string" => {
                let tag = extract_tagged_string_tag(node, source)?;
                let content = extract_tagged_string_content(node, source)?;
                Ok(Value::TaggedString { tag, content })
            }
            _ => Err(ParseError::unsupported_value_type(node)),
        }
    }
    
    /// Serialize value to gram notation
    pub fn to_gram_notation(&self) -> String {
        match self {
            Value::String(s) => quote_if_needed(s),
            Value::Integer(i) => i.to_string(),
            Value::Decimal(f) => format_decimal(*f),
            Value::Boolean(b) => b.to_string(),
            Value::Array(values) => {
                let items: Vec<String> = values.iter()
                    .map(|v| v.to_gram_notation())
                    .collect();
                format!("[{}]", items.join(", "))
            }
            Value::Range { lower, upper } => format!("{}..{}", lower, upper),
            Value::TaggedString { tag, content } => {
                format!("\"\"\"{}{}\"\"\"", tag, content)
            }
        }
    }
    
    /// Get the type name of this value (for error messages)
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::String(_) => "string",
            Value::Integer(_) => "integer",
            Value::Decimal(_) => "decimal",
            Value::Boolean(_) => "boolean",
            Value::Array(_) => "array",
            Value::Range { .. } => "range",
            Value::TaggedString { .. } => "tagged string",
        }
    }
}
```

### Display Implementation

```rust
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_gram_notation())
    }
}
```

### Equality Implementation

```rust
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Decimal(a), Value::Decimal(b)) => {
                // Use epsilon comparison for floats
                (a - b).abs() < f64::EPSILON
            }
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Range { lower: l1, upper: u1 }, Value::Range { lower: l2, upper: u2 }) => {
                l1 == l2 && u1 == u2
            }
            (Value::TaggedString { tag: t1, content: c1 }, Value::TaggedString { tag: t2, content: c2 }) => {
                t1 == t2 && c1 == c2
            }
            _ => false,
        }
    }
}
```

## Value Type Examples

### String Values

```rust
// Unquoted symbol
Value::String("hello".to_string()) → hello

// Quoted string
Value::String("Hello World".to_string()) → "Hello World"

// Special characters
Value::String("a:b".to_string()) → "a:b"  // Needs quoting due to :

// Empty string
Value::String("".to_string()) → ""
```

### Numeric Values

```rust
// Integer
Value::Integer(42) → 42
Value::Integer(-10) → -10
Value::Integer(0) → 0

// Decimal
Value::Decimal(3.14) → 3.14
Value::Decimal(-2.5) → -2.5
Value::Decimal(0.0) → 0.0
```

### Boolean Values

```rust
Value::Boolean(true) → true
Value::Boolean(false) → false
```

### Array Values

```rust
// Homogeneous array
Value::Array(vec![
    Value::String("rust".to_string()),
    Value::String("wasm".to_string()),
]) → ["rust", "wasm"]

// Heterogeneous array (allowed by grammar)
Value::Array(vec![
    Value::String("name".to_string()),
    Value::Integer(42),
    Value::Boolean(true),
]) → ["name", 42, true]

// Nested arrays (if grammar allows)
Value::Array(vec![
    Value::Array(vec![Value::Integer(1), Value::Integer(2)]),
    Value::Array(vec![Value::Integer(3), Value::Integer(4)]),
]) → [[1, 2], [3, 4]]

// Empty array
Value::Array(vec![]) → []
```

### Range Values

```rust
Value::Range { lower: 1, upper: 10 } → 1..10
Value::Range { lower: 0, upper: 100 } → 0..100
Value::Range { lower: -5, upper: 5 } → -5..5
```

### Tagged String Values

```rust
Value::TaggedString { 
    tag: "markdown".to_string(), 
    content: "# Heading\n\nContent".to_string() 
} → """markdown# Heading

Content"""

Value::TaggedString { 
    tag: "".to_string(), 
    content: "Plain text".to_string() 
} → """Plain text"""
```

## Helper Functions

### String Quoting

```rust
/// Quote string if it contains special characters
fn quote_if_needed(s: &str) -> String {
    if needs_quoting(s) {
        format!("\"{}\"", escape_string(s))
    } else {
        s.to_string()
    }
}

/// Check if string needs quoting
fn needs_quoting(s: &str) -> bool {
    s.is_empty()
        || s.contains(char::is_whitespace)
        || s.chars().any(|c| matches!(c, '(' | ')' | '[' | ']' | '{' | '}' | ':' | ',' | '@' | '#' | '-' | '~'))
        || s.chars().next().map_or(false, |c| c.is_ascii_digit())
}

/// Escape special characters in strings
fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
}
```

### Decimal Formatting

```rust
/// Format decimal to avoid unnecessary trailing zeros
fn format_decimal(f: f64) -> String {
    if f.fract() == 0.0 && f.is_finite() {
        format!("{:.1}", f)  // Always include .0 to distinguish from integer
    } else {
        f.to_string()
    }
}
```

### Range Extraction

```rust
/// Extract range bound from tree-sitter node
fn extract_range_bound(node: &tree_sitter::Node, field: &str, source: &str) -> Result<i64, ParseError> {
    let bound_node = node.child_by_field_name(field)
        .ok_or_else(|| ParseError::missing_field(node, field))?;
    
    let text = bound_node.utf8_text(source.as_bytes())?;
    text.parse::<i64>()
        .map_err(|e| ParseError::invalid_value(&bound_node, format!("Invalid range bound: {}", e)))
}
```

### Tagged String Extraction

```rust
/// Extract tag from tagged string
fn extract_tagged_string_tag(node: &tree_sitter::Node, source: &str) -> Result<String, ParseError> {
    // Tagged strings use """ syntax with optional tag
    // """tag content""" or """content"""
    // Tag is text between """ and first whitespace/newline
    // Implementation details based on grammar
}

/// Extract content from tagged string
fn extract_tagged_string_content(node: &tree_sitter::Node, source: &str) -> Result<String, ParseError> {
    // Content is everything after tag (or after """ if no tag)
    // Until closing """
}
```

## Integration with Subject

```rust
pub struct Subject {
    pub identifier: Option<String>,
    pub labels: Vec<String>,
    pub record: HashMap<String, Value>,  // Properties use Value enum
}

impl Subject {
    /// Create subject with properties
    pub fn with_properties(mut self, properties: HashMap<String, Value>) -> Self {
        self.record = properties;
        self
    }
    
    /// Get property value
    pub fn get_property(&self, key: &str) -> Option<&Value> {
        self.record.get(key)
    }
    
    /// Set property value
    pub fn set_property(&mut self, key: String, value: Value) {
        self.record.insert(key, value);
    }
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_string_values() {
        let v = Value::String("hello".to_string());
        assert_eq!(v.to_gram_notation(), "hello");
        assert_eq!(v.type_name(), "string");
    }
    
    #[test]
    fn test_integer_values() {
        assert_eq!(Value::Integer(42).to_gram_notation(), "42");
        assert_eq!(Value::Integer(-10).to_gram_notation(), "-10");
    }
    
    #[test]
    fn test_array_values() {
        let v = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        assert_eq!(v.to_gram_notation(), "[1, 2, 3]");
    }
    
    #[test]
    fn test_range_values() {
        let v = Value::Range { lower: 1, upper: 10 };
        assert_eq!(v.to_gram_notation(), "1..10");
    }
    
    #[test]
    fn test_value_equality() {
        assert_eq!(Value::Integer(42), Value::Integer(42));
        assert_ne!(Value::Integer(42), Value::String("42".to_string()));
    }
}
```

### Property-Based Tests

```rust
#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn value_round_trip(value: Value) {
            let gram = value.to_gram_notation();
            // Parse and compare (would need parser integration)
        }
        
        #[test]
        fn string_quoting_is_consistent(s: String) {
            let value = Value::String(s.clone());
            let gram = value.to_gram_notation();
            // Verify gram is valid and parses back to same string
        }
    }
}
```

## Dependencies

```toml
[dependencies]
tree-sitter = "0.20"

[dev-dependencies]
proptest = "1.0"
```

## Future Extensions

- Serde integration for JSON/TOML interop
- Custom value types (user-defined)
- Value schema validation
- Type coercion (e.g., integer → decimal)

