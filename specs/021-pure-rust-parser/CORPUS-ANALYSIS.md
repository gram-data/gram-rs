# Corpus Test Analysis

**Date**: January 9, 2026
**Baseline**: 41/134 tests passing (30.6%)

## 📊 Test Results Summary

| Status | Count | Percentage |
|--------|-------|------------|
| **Passed** | 41 | 30.6% |
| **Failed** | 93 | 69.4% |
| **Total** | 134 | 100% |

---

## 🔍 Failure Analysis by Category

### Category 1: Subject Patterns with Brackets `[]` (HIGH PRIORITY)
**Impact**: 5+ failures in `brackets.txt`, affects core syntax

**Issues**:
- `[]` - Empty subject pattern not parsed
- `[name]` - Named subject pattern not parsed  
- `[name:Label]` - Labeled subject pattern not parsed
- `[name | elem]` - Subject pattern with elements not parsed correctly

**Root Cause**: Parser likely doesn't handle `[]` syntax at all, or handles it incorrectly.

**Fix**: Implement proper subject pattern parsing in `parser/subject.rs` or create new `parser/subject_pattern.rs`.

### Category 2: Comments (HIGH PRIORITY)
**Impact**: 4 failures in `comments.txt`

**Issues**:
- Comments before top-level patterns
- Comments before records
- End-of-line comments after subjects
- Multiple end-of-line comments

**Root Cause**: Comment parser in `parser/combinators.rs` may not handle all comment positions.

**Fix**: Enhance `comment` combinator to handle inline comments and all positions.

### Category 3: Arrow Types (HIGH PRIORITY)
**Impact**: 15+ failures across `double_arrows.txt`, `single_arrows.txt`, `squiggle_arrows.txt`

**Issues**:
- `--` undirected arrow not implemented
- `<--` left arrow not implemented  
- `<-->` bidirectional arrow not implemented
- `==>` double right arrow not implemented
- `~~>` squiggle arrow not implemented

**Root Cause**: Only `-->` implemented in `parser/relationship.rs`.

**Fix**: Add all arrow types to `ArrowType` enum and `arrow` parser.

### Category 4: Labeled Relationships `-[:LABEL]->` (HIGH PRIORITY)
**Impact**: 10+ failures in `labeled_relationships.txt`

**Issues**:
- `-[:KNOWS]->` syntax not parsed
- `-[r:KNOWS]->` syntax not parsed
- `-[{props}]->` syntax not parsed

**Root Cause**: Relationship parser only handles anonymous arrows.

**Fix**: Enhance `relationship` parser to handle edge subjects in `[]`.

### Category 5: Range Values (MEDIUM PRIORITY)
**Impact**: 5+ failures in `range_values.txt`

**Issues**:
- `5..` partial range (lower bound only)
- `..10` partial range (upper bound only)
- `..` unbounded range

**Root Cause**: Range parser in `parser/value.rs` only handles `0..10` (both bounds).

**Fix**: Update `range` parser to handle `Option<i64>` bounds.

### Category 6: Array Values (MEDIUM PRIORITY)
**Impact**: 1 failure in `array_properties.txt`

**Issues**:
- Arrays in properties may have parsing issues

**Root Cause**: Likely whitespace or delimiter issues in array parser.

**Fix**: Review and test `array` parser in `parser/value.rs`.

### Category 7: Annotation Values (MEDIUM PRIORITY)
**Impact**: 2+ failures in `annotation.txt`

**Issues**:
- `@key(value)` syntax not parsed (only `@key` works)
- Multiple annotations on same node

**Root Cause**: Annotation parser in `parser/annotation.rs` doesn't parse values.

**Fix**: Enhance annotation parser to handle `@key(value)` syntax.

### Category 8: Path Validation (LOW PRIORITY)
**Impact**: 2+ failures related to path representation

**Issues**:
- Tree-sitter represents paths as nested `relationship_pattern`
- Our parser represents paths as nested `Pattern` with 2 elements
- Validator doesn't recognize structural equivalence

**Root Cause**: Validator is too strict about structure matching.

**Fix**: Enhance validator to recognize equivalent representations.

### Category 9: Graph-Level Syntax (LOW PRIORITY)
**Impact**: 2+ failures in `graph_global.txt`

**Issues**:
- Graph-level records
- Graph-type declarations

**Root Cause**: May be optional/advanced features not in core spec.

**Fix**: Evaluate if these are required for conformance.

---

## 🎯 Prioritized Fix Plan

### Phase 1: Core Syntax (Target: 70% pass rate)

**Priority 1A - Brackets/Subject Patterns** (Est: +5 tests)
- [ ] Implement `[]` empty subject pattern parsing
- [ ] Implement `[name]` named subject pattern
- [ ] Implement `[name:Label]` labeled subject pattern  
- [ ] Fix `[name | elements]` parsing

**Priority 1B - Arrow Types** (Est: +15 tests)
- [ ] Add `--` undirected arrow
- [ ] Add `<--` left arrow
- [ ] Add `<-->` bidirectional arrow
- [ ] Add `==>`, `<==`, `<==>` double arrows
- [ ] Add `~~>`, `<~~`, `<~~>` squiggle arrows

**Priority 1C - Comments** (Est: +4 tests)
- [ ] Fix comment before pattern
- [ ] Fix comment before record
- [ ] Fix end-of-line comments
- [ ] Fix multiple comments

### Phase 2: Relationship Enhancements (Target: 85% pass rate)

**Priority 2A - Labeled Relationships** (Est: +10 tests)
- [ ] Implement `-[:LABEL]->` syntax
- [ ] Implement `-[id:LABEL]->` syntax
- [ ] Implement `-[{props}]->` syntax
- [ ] Implement `-[id:LABEL {props}]->` syntax

### Phase 3: Value Types (Target: 95% pass rate)

**Priority 3A - Range Values** (Est: +5 tests)
- [ ] Implement `5..` partial range
- [ ] Implement `..10` partial range
- [ ] Implement `..` unbounded range

**Priority 3B - Annotation Values** (Est: +2 tests)
- [ ] Implement `@key(value)` syntax
- [ ] Test multiple annotations

**Priority 3C - Arrays** (Est: +1 test)
- [ ] Debug array property parsing

### Phase 4: Validator & Edge Cases (Target: 100% pass rate)

**Priority 4A - Validator Enhancements** (Est: +2 tests)
- [ ] Recognize path representation equivalence
- [ ] Improve structural matching logic

**Priority 4B - Graph Syntax** (Evaluate)
- [ ] Determine if graph-level features are required
- [ ] Implement if in core spec

---

## 📈 Estimated Progress

| Phase | Target Pass Rate | Tests Added | Cumulative |
|-------|-----------------|-------------|------------|
| **Baseline** | 30.6% | - | 41/134 |
| **Phase 1** | 70% | +24 | 65/134 |
| **Phase 2** | 85% | +10 | 75/134 |
| **Phase 3** | 95% | +8 | 83/134 |
| **Phase 4** | 100% | +2 | 85/134 |

Note: Some tests may pass automatically when related features are fixed, so actual progress may be faster.

---

## 🔬 Next Steps

1. **Run detailed failure analysis**:
   ```bash
   cargo test --package gram-codec --test corpus_integration test_corpus_conformance -- --nocapture 2>&1 | tee corpus-failures.log
   ```

2. **Start with Priority 1A** (brackets/subject patterns):
   - Most fundamental syntax issue
   - Affects many downstream features
   - Clear implementation path

3. **Iterate through priorities**:
   - Fix → Test → Measure progress
   - Update this document with actual results
   - Adjust priorities based on findings

4. **Track progress**:
   - Re-run corpus tests after each fix
   - Record pass rate improvements
   - Identify any new issues revealed

---

## 📝 Notes

- **Structural Equivalence**: Some "failures" may be due to different (but equivalent) representations. Our parser uses nested `Pattern` structures while tree-sitter uses flat `relationship_pattern` nodes.

- **Serializer Impact**: As we add syntax support to the parser, we'll need to update the serializer to output these forms correctly.

- **Round-Trip Tests**: New syntax forms should be added to `round_trip_tests.rs` to ensure bidirectional correctness.

- **Documentation**: Track any intentional differences from tree-sitter in `CONFORMANCE.md`.

---

**Status**: Ready to begin Phase 1 implementation.
