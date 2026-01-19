# Rust Usage Guide

This guide provides practical examples for using the `gram-rs` crates in your Rust projects.

## Adding Dependencies

To use `gram-rs`, add the following crates to your `Cargo.toml`. Since the library is organized as a workspace, you can reference them by path if you're working within the repo, or by version once published.

```toml
[dependencies]
pattern-core = { path = "crates/pattern-core" }
gram-codec = { path = "crates/gram-codec" }
```

## Programmatic Construction

The `pattern-core` crate provides the `Pattern` type and its core constructors.

### Creating an Atomic Pattern (Node)
Use `Pattern::point` to create a pattern with no elements.

```rust
use pattern_core::Pattern;

// Create a node with a string value
let node = Pattern::point("Person".to_string());

assert!(node.is_atomic());
assert_eq!(node.value(), &"Person".to_string());
```

### Creating a Nested Pattern
Use `Pattern::pattern` to create a pattern with elements.

```rust
use pattern_core::Pattern;

let alice = Pattern::point("Alice".to_string());
let bob = Pattern::point("Bob".to_string());

// Create a relationship pattern "Alice knows Bob"
let knows = Pattern::pattern(
    "KNOWS".to_string(),
    vec![alice, bob]
);

assert_eq!(knows.elements().len(), 2);
```

## Parsing and Serialization

The `gram-codec` crate handles conversion between Gram notation and Rust `Pattern` structures. For more details on Gram syntax, see the **[Gram Notation Reference](gram-notation.md)**.

### Parsing Gram Notation
Use `parse_gram` to create a `Pattern` from a Gram string.

```rust
use gram_codec::parse_gram;

let gram = "(a:Person)-[r:KNOWS]->(b:Person)";
let pattern = parse_gram(gram).expect("Failed to parse Gram notation");

println!("Relationship value: {}", pattern.value());
```

### Serializing to Gram Notation
*(Note: Ensure `to_gram` or equivalent is available in your version of `gram-codec`)*

```rust
use gram_codec::to_gram;
use pattern_core::Pattern;

let node = Pattern::point("Node".to_string());
let gram_string = to_gram(&node);

assert_eq!(gram_string, "(Node)");
```

## Basic Queries

`Pattern` provides several utilities for inspecting and querying its structure and values.

### Checking Values
You can check if any or all values in a pattern (including elements recursively) satisfy a predicate.

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("root".to_string(), vec![
    Pattern::point("child1".to_string()),
    Pattern::point("child2".to_string()),
]);

// Does any value contain "child"?
let has_child = pattern.any_value(|v| v.contains("child"));
assert!(has_child);

// Do all values have length > 3?
let all_long = pattern.all_values(|v| v.len() > 3);
assert!(all_long);
```

### Structural Inspection
```rust
let pattern = parse_gram("(a)-[r]->(b)").unwrap();

println!("Length: {}", pattern.length()); // Direct elements: 2
println!("Size: {}", pattern.size());     // Total nodes: 3
println!("Depth: {}", pattern.depth());   // Max nesting: 1
```
