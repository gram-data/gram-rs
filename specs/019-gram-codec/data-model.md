# Data Model: Basic Gram Codec

**Feature**: 019-gram-codec  
**Created**: 2026-01-06  
**Status**: Draft

## Overview

This document describes the data model for the gram codec, which provides bidirectional transformation between gram notation (text) and Pattern data structures.

## Core Entities

### Gram Notation (Input/Output Format)

Gram notation is a human-readable text format for representing patterns. It follows the grammar defined by tree-sitter-gram.

**Syntax Forms**:

1. **Node Pattern**: `(subject)` - represents a pattern with 0 elements
   - Example: `()`, `(hello)`, `(a:Person)`, `(a:Person {name: "Alice"})`

2. **Relationship Pattern**: `(left)-[edge]->(right)` - represents a pattern with 2 elements
   - Arrow types: `-->`, `<--`, `<-->`, `~~`, `~>`
   - Example: `(a)-->(b)`, `(a)-[:KNOWS]->(b)`, `(a)<--(b)`

3. **Subject Pattern**: `[subject | elements]` - represents a pattern with arbitrary elements
   - Example: `[team | alice, bob]`, `[outer | [inner | leaf]]`

4. **Annotated Pattern**: `@key(value) pattern` - represents a pattern with 1 element
   - Example: `@type(node) (a)`, `@depth(2) [x | y, z]`

5. **Comments**: `// comment text` - ignored during parsing
   - Example: `// This is a comment\n(hello)`

### Subject (Pattern Value Type)

A Subject represents the value/decoration of a pattern and can contain:

- **Identifier**: A name/reference for the pattern
  - Types: symbol (unquoted), string literal (quoted), integer
  - Example: `hello`, `"node-id"`, `42`

- **Labels**: One or more type labels
  - Syntax: `:Label` or `::Label`
  - Example: `:Person`, `:Person:Employee`

- **Record**: Key-value properties
  - Syntax: `{key1: value1, key2: value2}`
  - Example: `{name: "Alice", age: 30}`

A Subject can have any combination of these components:
- Identifier only: `(hello)`
- Labels only: `(:Person)`
- Record only: `({name: "Alice"})`
- Identifier + Labels: `(a:Person)`
- Identifier + Record: `(a {name: "Alice"})`
- Labels + Record: `(:Person {name: "Alice"})`
- All three: `(a:Person {name: "Alice"})`

### Property Values

Properties in records can have various types:

- **Strings**: Quoted strings or unquoted symbols
  - Example: `"Alice"`, `hello`

- **Numbers**: Integers or decimals
  - Example: `42`, `3.14`, `-10`

- **Booleans**: `true` or `false`
  - Example: `{active: true}`

- **Arrays**: List of scalar values
  - Example: `["rust", "wasm", "python"]`, `[1, 2, 3]`

- **Ranges**: Numeric ranges
  - Example: `1..10`, `0..100`

- **Tagged Strings**: Multi-line strings with format tags
  - Example: `"""markdown content"""`

### Pattern Structure

A Pattern is a recursive structure:

```
Pattern<V> {
  value: V,
  elements: Vec<Pattern<V>>
}
```

For gram notation, `V = Subject`.

**Element Count Semantics**:

- **0 elements**: Atomic/leaf pattern (node notation)
- **1 element**: Annotated pattern (annotation + element)
- **2 elements**: Relationship pattern (left node, right node)
- **N elements**: Subject pattern (general form)

### Path Patterns

Linear chains of relationships are flattened:

```gram
(a)-[r1]->(b)-[r2]->(c)
```

Is equivalent to:

```gram
[ | [r1 | (a), (b)], [r2 | (b), (c)] ]
```

## Mapping Rules

### Parsing (Gram Notation → Pattern)

1. **Node Pattern** `(subject)` → `Pattern { value: Subject, elements: [] }`
2. **Relationship Pattern** `(left)-[edge]->(right)` → `Pattern { value: Subject(edge), elements: [left_pattern, right_pattern] }`
3. **Subject Pattern** `[subject | e1, e2, ...]` → `Pattern { value: Subject, elements: [e1_pattern, e2_pattern, ...] }`
4. **Annotated Pattern** `@key(value) element` → `Pattern { value: Subject(annotation), elements: [element_pattern] }`

### Serialization (Pattern → Gram Notation)

1. **0 elements** → Node notation: `(subject)`
2. **1 element** → Annotation notation: `@key(value) element` or Subject pattern: `[subject | element]`
3. **2 elements** → Relationship notation: `(left)-->(right)` or Subject pattern: `[subject | left, right]`
4. **N elements (N > 2)** → Subject pattern notation: `[subject | e1, e2, ..., eN]`

## Round-Trip Equivalence

Round-trip correctness means:

```
parse(gram_text) → pattern1
serialize(pattern1) → gram_text2
parse(gram_text2) → pattern2
pattern1 ≅ pattern2  (structurally equivalent)
```

Structural equivalence means:
- Same value (Subject identifier, labels, properties)
- Same number of elements
- Elements are structurally equivalent (recursively)

Note: Formatting details (whitespace, comments, syntactic form choice) may differ.

## Grammar Authority

The tree-sitter-gram repository (`../tree-sitter-gram/grammar.js`) is the authoritative source for gram notation syntax rules. All parser behavior must conform to this grammar.

## Validation

All gram notation (input to parser, output from serializer) must pass validation by the `gram-lint` CLI tool, which uses the tree-sitter-gram parser to verify syntax correctness.
