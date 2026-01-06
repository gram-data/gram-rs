# Clarification Summary: 019-gram-codec

**Date**: 2026-01-06  
**Clarification Request**: Explore `gram-lint` tool and validate all gram snippets  
**Status**: ✅ Complete

## What Was Clarified

The user requested comprehensive exploration of the `gram-lint` CLI tool and validation of all gram notation examples to ensure:

1. All gram snippets in specifications are valid
2. Parse tree output is documented for parser implementation
3. All future plans and tasks use only validated gram snippets

## Actions Taken

### 1. Explored gram-lint CLI Tool

**Tool Capabilities Documented**:
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
- Simple error message: `-e: parse error`
- No detailed location information in current version

### 2. Validated All Gram Syntax Forms

Tested and documented with parse trees:

#### Node Patterns (0 elements)
- ✅ Empty node: `()`
- ✅ Node with identifier: `(hello)`
- ✅ Node with label: `(a:Person)`
- ✅ Node with label and properties: `(a:Person {name: "Alice"})`
- ✅ Multiple labels: `(:Person:Employee)`
- ✅ Integer identifier: `(42)`

#### Relationship Patterns (2 elements)
- ✅ Right arrow: `(a)-->(b)`
- ✅ Left arrow: `(a)<--(b)`
- ✅ Bidirectional: `(a)<-->(b)`
- ✅ Undirected (squiggle): `(a)~~(b)`
- ✅ With label and properties: `(a)-[:KNOWS {since: 2020}]->(b)`
- ✅ Path pattern (chained): `(a)-[r1]->(b)-[r2]->(c)`

#### Subject Patterns (N elements)
- ✅ Simple: `[team | alice, bob, charlie]`
- ✅ With label and properties: `[team:Team {name: "DevRel"} | alice, bob, charlie]`
- ✅ Nested: `[outer | [inner | leaf]]`

#### Annotated Patterns (1 element)
- ✅ Annotation on node: `@type(node) (a)`

#### Property Value Types
- ✅ Numeric (integer, decimal): `({count: 42, ratio: 3.14})`
- ✅ Boolean: `({active: true})`
- ✅ Array: `({tags: ["rust", "wasm", "python"]})`
- ✅ Range: `({score: 1..10})`

#### Other Features
- ✅ Comments: `// This is a comment\n(hello)-->(world)`
- ✅ Root record: `{graph: "social"} (a)-->(b)`

#### Error Cases
- ✅ Unclosed parenthesis: `(unclosed` → parse error
- ✅ Missing property value: `({key: })` → parse error

**Total Validated Examples**: 40+ gram snippets with parse trees

### 3. Created Comprehensive Validation Documentation

**New File**: [VALIDATION.md](VALIDATION.md) (409 lines)

**Contents**:
- Complete list of validated gram snippets with parse trees
- Parse tree structure analysis and mapping
- Pattern type identification rules
- Subject component extraction guidelines
- Value type parsing guidelines
- Path pattern handling (right-associative nesting)
- Comment handling (discarded during parsing)
- Parser implementation guidelines based on parse tree structure
- Testing strategy recommendations
- Validation workflow for specifications

### 4. Updated Research Documentation

**Updated**: [research.md](research.md)

**Added Section 11**: Parse Tree Structure Analysis
- Complete parse tree structure documentation
- Pattern type identification rules
- Arrow type enumeration
- Subject component mapping
- Value type enumeration
- Path pattern handling implications
- Comment handling implications
- Design implications for parser implementation

**Updated References Section**:
- Added gram-lint usage details
- Added validation documentation reference
- Emphasized requirement: **All gram snippets in plans/tasks MUST be validated**

### 5. Updated Supporting Documentation

**Updated**: [quickstart.md](quickstart.md)
- Enhanced validation tool section
- Added parse tree output examples
- Added exit code documentation
- Added validation requirement note

**Updated**: [README.md](README.md)
- Added VALIDATION.md to documentation structure
- Enhanced validation section with parse tree examples
- Added comprehensive validation status

**Updated**: [checklists/requirements.md](checklists/requirements.md)
- Added validation complete checkmarks
- Added parse tree analysis complete note
- Added requirement for validated snippets in plans/tasks

## Key Findings

### Parse Tree Structure

All valid gram notation produces a `gram_pattern` root node:
```
(gram_pattern [root: (record)]? [comment]* pattern*)
```

### Pattern Type Identification

Pattern types can be distinguished by parse tree fields:
- `node_pattern`: No `kind` or `annotations` fields → 0 elements
- `relationship_pattern`: Has `kind` field (arrow type) → 2 elements
- `subject_pattern`: Has `elements` field → N elements
- `annotated_pattern`: Has `annotations` field → 1 element

### Arrow Types

Relationship patterns use these arrow types in `kind` field:
- `right_arrow`: `-->`
- `left_arrow`: `<--`
- `bidirectional_arrow`: `<-->`
- `undirected_arrow`: `~~`

### Path Pattern Handling

Chained relationships are right-associative:
```gram
(a)-[r1]->(b)-[r2]->(c)
```
Parses as:
```
relationship_pattern(
  left: node_a,
  right: relationship_pattern(
    left: node_b,
    right: node_c
  )
)
```

This requires recursive handling or flattening during parsing.

## Impact on Planning Phase

### Parser Library Selection

The parse tree analysis provides concrete data for evaluating parser options:

1. **Direct tree-sitter use**: Parse tree structure is well-documented, transformation to Pattern is straightforward
2. **Manual implementation**: Would need to replicate this exact parse tree structure for compatibility
3. **Hybrid approach**: Use tree-sitter for parsing, custom logic for Pattern transformation

### Parser Implementation

Parse tree analysis provides clear implementation guidelines:
- Pattern type determination logic
- Subject component extraction
- Value type parsing
- Path pattern flattening strategy
- Comment handling (discard)

### Testing Strategy

Validation documentation provides:
- 40+ test cases with expected parse trees
- Error cases for negative testing
- Round-trip testing examples
- Integration testing scenarios

## Validation Requirements for Future Work

### All Plans and Tasks Must:

1. **Use validated gram snippets**: Reference [VALIDATION.md](VALIDATION.md) for examples
2. **Validate new examples**: Run `gram-lint -e "<gram>" --tree` before inclusion
3. **Document parse trees**: Include parse tree output for non-trivial examples
4. **Test error cases**: Include invalid gram notation for negative testing

### Validation Workflow

```bash
# 1. Validate snippet
gram-lint -e "(your)-->(gram)"

# 2. View parse tree
gram-lint -e "(your)-->(gram)" --tree

# 3. Include in documentation if valid (exit code 0)
```

## Documentation Metrics

**Total Documentation**: 1,535 lines across 6 markdown files

| File | Lines | Purpose |
|------|-------|---------|
| VALIDATION.md | 409 | Validated examples with parse trees |
| research.md | 371 | Research questions and parse tree analysis |
| README.md | 231 | Feature overview and navigation |
| quickstart.md | 203 | Quick reference guide |
| spec.md | 165 | Complete specification |
| data-model.md | 156 | Data model and mapping rules |

**Plus**:
- checklists/requirements.md: Quality checklist (updated)
- CLARIFICATION_SUMMARY.md: This document

## Next Steps

The specification is now enhanced with comprehensive validation and ready for:

1. **Planning Phase** (`/speckit.plan`):
   - Use validated examples from VALIDATION.md
   - Reference parse tree structure analysis for parser design
   - Evaluate parser library options with concrete parse tree data
   - Design Pattern transformation logic based on documented structure

2. **Implementation**:
   - Follow parser implementation guidelines from VALIDATION.md
   - Use validated test cases for unit/integration testing
   - Validate all serializer output with `gram-lint`

3. **Testing**:
   - Use 40+ validated examples as test cases
   - Include parse tree verification in tests
   - Test error cases (invalid gram notation)
   - Validate round-trip correctness

## Conclusion

✅ **Clarification Complete**: The specification now includes:
- Comprehensive `gram-lint` tool documentation
- 40+ validated gram snippets with parse trees
- Complete parse tree structure analysis
- Parser implementation guidelines
- Testing strategy recommendations
- Clear requirements for future plans and tasks

All gram notation examples are validated and ready for use in planning and implementation phases.

