# Gram Notation Validation: 019-gram-codec

**Feature**: 019-gram-codec  
**Created**: 2026-01-06  
**Tool**: `gram-lint` CLI validator  
**Purpose**: Comprehensive validation of all gram notation examples used in specification

## About gram-lint

`gram-lint` is a CLI validator that uses the tree-sitter-gram parser to verify gram notation syntax.

**Usage**:
```bash
gram-lint [OPTIONS] [FILES]...

Options:
  -t, --tree                      Output the s-expression parse tree
  -e, --expression <EXPRESSIONS>  Lint a gram expression directly (can be used multiple times)
  -h, --help                      Print help
```

**Exit Codes**:
- `0`: Valid gram notation (no parse errors)
- `1`: Parse error detected

**Error Reporting**:
```bash
$ gram-lint -e "(unclosed"
-e: parse error
```

## Validated Gram Snippets

All examples below have been validated with `gram-lint` and include their parse tree output.

### 1. Node Patterns (0 elements)

#### Empty Node
```gram
()
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (node_pattern))
```

#### Node with Identifier
```gram
(hello)
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (node_pattern identifier: (symbol)))
```

#### Node with Label
```gram
(a:Person)
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (node_pattern identifier: (symbol) labels: (labels (symbol))))
```

#### Node with Label and Properties
```gram
(a:Person {name: "Alice"})
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (node_pattern identifier: (symbol) labels: (labels (symbol)) record: (record (record_property key: (symbol) value: (string_literal content: (string_content))))))
```

#### Node with Multiple Labels
```gram
(:Person:Employee)
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (node_pattern labels: (labels (symbol) (symbol))))
```

#### Node with Integer Identifier
```gram
(42)
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (node_pattern identifier: (integer)))
```

### 2. Relationship Patterns (2 elements)

#### Simple Right Arrow
```gram
(a)-->(b)
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (relationship_pattern left: (node_pattern identifier: (symbol)) kind: (right_arrow) right: (node_pattern identifier: (symbol))))
```

#### Left Arrow
```gram
(a)<--(b)
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (relationship_pattern left: (node_pattern identifier: (symbol)) kind: (left_arrow) right: (node_pattern identifier: (symbol))))
```

#### Bidirectional Arrow
```gram
(a)<-->(b)
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (relationship_pattern left: (node_pattern identifier: (symbol)) kind: (bidirectional_arrow) right: (node_pattern identifier: (symbol))))
```

#### Undirected Arrow (Squiggle)
```gram
(a)~~(b)
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (relationship_pattern left: (node_pattern identifier: (symbol)) kind: (undirected_arrow) right: (node_pattern identifier: (symbol))))
```

#### Relationship with Label and Properties
```gram
(a)-[:KNOWS {since: 2020}]->(b)
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (relationship_pattern left: (node_pattern identifier: (symbol)) kind: (right_arrow labels: (labels (symbol)) record: (record (record_property key: (symbol) value: (integer)))) right: (node_pattern identifier: (symbol))))
```

#### Path Pattern (Chained Relationships)
```gram
(a)-[r1]->(b)-[r2]->(c)
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (relationship_pattern left: (node_pattern identifier: (symbol)) kind: (right_arrow identifier: (symbol)) right: (relationship_pattern left: (node_pattern identifier: (symbol)) kind: (right_arrow identifier: (symbol)) right: (node_pattern identifier: (symbol)))))
```

**Note**: Path patterns are right-associative in the parse tree.

### 3. Subject Patterns (N elements)

#### Simple Subject Pattern
```gram
[team | alice, bob, charlie]
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (subject_pattern identifier: (symbol) elements: (subject_pattern_elements (pattern_reference identifier: (symbol)) (pattern_reference identifier: (symbol)) (pattern_reference identifier: (symbol)))))
```

#### Subject Pattern with Label and Properties
```gram
[team:Team {name: "DevRel"} | alice, bob, charlie]
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (subject_pattern identifier: (symbol) labels: (labels (symbol)) record: (record (record_property key: (symbol) value: (string_literal content: (string_content)))) elements: (subject_pattern_elements (pattern_reference identifier: (symbol)) (pattern_reference identifier: (symbol)) (pattern_reference identifier: (symbol)))))
```

#### Nested Subject Patterns
```gram
[outer | [inner | leaf]]
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (subject_pattern identifier: (symbol) elements: (subject_pattern_elements (subject_pattern identifier: (symbol) elements: (subject_pattern_elements (pattern_reference identifier: (symbol)))))))
```

### 4. Annotated Patterns (1 element)

#### Annotation on Node
```gram
@type(node) (a)
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (annotated_pattern annotations: (annotations (annotation key: (symbol) value: (symbol))) elements: (node_pattern identifier: (symbol))))
```

### 5. Property Value Types

#### Numeric Values (Integer and Decimal)
```gram
({count: 42, ratio: 3.14, active: true})
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (node_pattern record: (record (record_property key: (symbol) value: (integer)) (record_property key: (symbol) value: (decimal)) (record_property key: (symbol) value: (boolean_literal)))))
```

**Value Types Shown**:
- `integer`: 42
- `decimal`: 3.14
- `boolean_literal`: true

#### Array Values
```gram
({tags: ["rust", "wasm", "python"]})
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (node_pattern record: (record (record_property key: (symbol) value: (array (string_literal content: (string_content)) (string_literal content: (string_content)) (string_literal content: (string_content)))))))
```

#### Range Values
```gram
({score: 1..10})
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (node_pattern record: (record (record_property key: (symbol) value: (range lower: (integer) upper: (integer))))))
```

### 6. Comments

#### Comment with Content
```gram
// This is a comment
(hello)-->(world)
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern (comment) (relationship_pattern left: (node_pattern identifier: (symbol)) kind: (right_arrow) right: (node_pattern identifier: (symbol))))
```

**Note**: Comments appear as `(comment)` nodes in the parse tree but contain no semantic content.

### 7. Root Record

#### Root Record at Beginning
```gram
{graph: "social"} (a)-->(b)
```
**Status**: ✅ Valid  
**Parse Tree**:
```
(gram_pattern root: (record (record_property key: (symbol) value: (string_literal content: (string_content)))) (relationship_pattern left: (node_pattern identifier: (symbol)) kind: (right_arrow) right: (node_pattern identifier: (symbol))))
```

**Note**: Root record is marked with `root:` field in the parse tree.

## Parse Tree Structure Summary

### Top-Level Structure
All valid gram notation produces a `gram_pattern` root node:
```
(gram_pattern [root: (record)]? [comment]* pattern*)
```

### Pattern Types
- `node_pattern`: 0 elements, represented with `()`
- `relationship_pattern`: 2 elements, represented with arrows
- `subject_pattern`: N elements, represented with `[ | ]`
- `annotated_pattern`: 1 element, represented with `@key(value)`

### Arrow Types (relationship_pattern kind field)
- `right_arrow`: `-->`
- `left_arrow`: `<--`
- `bidirectional_arrow`: `<-->`
- `undirected_arrow`: `~~`

### Subject Components
- `identifier`: symbol, string_literal, or integer
- `labels`: One or more symbols prefixed with `:` or `::`
- `record`: Key-value properties in `{}`

### Value Types (record_property value)
- `symbol`: Unquoted identifier
- `string_literal`: Quoted string with `string_content`
- `integer`: Whole number
- `decimal`: Floating-point number
- `boolean_literal`: `true` or `false`
- `array`: List of scalar values in `[]`
- `range`: Numeric range with `lower` and `upper` bounds

## Invalid Gram Notation Examples

### Unclosed Parenthesis
```gram
(unclosed
```
**Status**: ❌ Invalid  
**Error**: `-e: parse error`  
**Exit Code**: 1

### Missing Property Value
```gram
({key: })
```
**Status**: ❌ Invalid  
**Error**: `-e: parse error`  
**Exit Code**: 1

## Validation Workflow for Specifications

All gram notation examples in specifications, plans, and tasks must be validated:

### 1. Validate Individual Snippets
```bash
gram-lint -e "(hello)-->(world)"
```

### 2. Validate with Parse Tree
```bash
gram-lint -e "(hello)-->(world)" --tree
```

### 3. Validate Files
```bash
gram-lint examples/data/social.gram
```

### 4. Validate Multiple Expressions
```bash
gram-lint -e "(a)-->(b)" -e "[team | alice, bob]" -e "@type(node) (x)"
```

## Parser Implementation Guidelines

Based on parse tree analysis, parser implementation should:

1. **Start with gram_pattern root**: Every valid input produces a `gram_pattern` node
2. **Handle optional root record**: Check for `root:` field at beginning
3. **Process comments separately**: Comments are separate nodes, not attached to patterns
4. **Distinguish pattern types**: Use element count to determine pattern type
   - No children elements → node_pattern
   - Annotations field → annotated_pattern  
   - Kind field with arrows → relationship_pattern
   - Elements field with `|` → subject_pattern
5. **Extract subject components**: Identifier, labels, record are optional named fields
6. **Parse value types**: Property values have specific types (integer, decimal, boolean_literal, array, range, etc.)
7. **Handle path patterns**: Relationship patterns can nest (right-associative)

## Testing Strategy

### Unit Tests (Per Syntax Form)
- Test each pattern type independently
- Test each arrow type independently
- Test each value type independently
- Verify parse tree structure matches expected

### Integration Tests (Combined Forms)
- Test nested patterns
- Test path patterns (chained relationships)
- Test patterns with all subject components
- Test root records with patterns

### Round-Trip Tests
- Parse → serialize → parse → verify equivalence
- Test with all validated examples above

### Error Tests
- Test invalid syntax
- Verify error detection and reporting
- Test edge cases (empty input, whitespace only, comment only)

## References

- **gram-lint source**: `../tree-sitter-gram/tools/gram-lint/`
- **Grammar definition**: `../tree-sitter-gram/grammar.js`
- **Test corpus**: `../tree-sitter-gram/test/corpus/`
- **Example files**: `../tree-sitter-gram/examples/data/`

## Validation Checklist

- ✅ All node pattern forms validated
- ✅ All relationship arrow types validated
- ✅ All subject pattern forms validated
- ✅ Annotated pattern validated
- ✅ All property value types validated
- ✅ Comment handling validated
- ✅ Root record validated
- ✅ Nested patterns validated
- ✅ Path patterns validated
- ✅ Error cases tested
- ✅ Parse tree structures documented

**All gram notation examples in this specification have been validated with gram-lint ✅**

