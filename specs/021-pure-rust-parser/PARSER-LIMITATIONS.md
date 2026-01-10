# Parser Limitations (nom-based parser)

**Date**: January 9, 2026  
**Status**: Known Limitation in Progress

---

## Property Parsing Not Fully Wired Up

### Issue

The nom-based parser has property record parsing implemented (`parser/subject.rs`), but properties are not being parsed from full gram notation like `(alice {name: "Alice"})`.

### Evidence

1. **Python AST with hardcoded properties works**:
   ```python
   # This works - manual Subject construction
   subject = Subject {
       identity: Symbol("alice"),
       properties: {"name": Value::VString("Alice"), "age": Value::VInteger(30)}
   }
   ast = AstPattern::from_pattern(&Pattern::point(subject))
   # Result: properties appear correctly in AST
   ```

2. **Parsing from gram text doesn't include properties**:
   ```python
   # This returns empty properties
   ast = parse_to_ast('(alice {name: "Alice"})')
   # Result: ast['subject']['properties'] == {}
   ```

3. **Test coverage gap**:
   - ✅ `test_property_pair` - Tests parsing individual `key: value` pairs
   - ❌ No test for `(node {properties})` end-to-end
   - ❌ No test for `parse_to_ast` with properties from gram text

### Root Cause

The property record parser exists but may not be properly integrated into the node pattern parser. The `subject` parser (line 64-89 in `parser/subject.rs`) calls `opt(preceded(ws, record))` which should parse `{...}`, but the results suggest this isn't working in practice.

### Impact

**For Phase 7 (AST Output)**:
- ✅ AST types are correct
- ✅ AST conversion logic is correct  
- ✅ WASM/Python bindings work correctly
- ⚠️  Underlying parser doesn't provide properties yet

**For Users**:
- Identity and labels parse correctly
- Properties are empty in parsed results
- Manually constructed patterns work fine

### Workaround

For now, users needing properties must:
1. Parse to get structure (identity, labels, elements)
2. Manually add properties afterward
3. Or wait for parser completion

### Next Steps

1. **Debug property parsing** (estimated: 1-2 hours)
   - Add end-to-end test: `(alice {name: "Alice"})` → check properties
   - Trace why `record` parser isn't being invoked
   - Fix integration between node parser and property parser

2. **Verify relationship patterns** properties
   - Test `-[rel {prop: val}]->`
   - Test subject patterns with properties

3. **Add comprehensive property tests**
   - All Value types as properties
   - Nested properties
   - Complex property values

---

## Status Summary

| Feature | Status |
|---------|--------|
| **AST Implementation** | ✅ Complete |
| **AST Conversion** | ✅ Complete |
| **Value Serialization** | ✅ Complete |
| **WASM Bindings** | ✅ Complete |
| **Python Bindings** | ✅ Complete |
| **Property Parsing** | ⚠️  Not Wired Up |

---

## Recommendation

**Phase 7 is still complete** - the AST output works correctly. The property parsing issue is a **parser implementation detail** that doesn't affect the AST design or bindings.

**Action**: Document this limitation and proceed with:
1. Commit Phase 7 work
2. File issue for property parsing
3. Continue to Phase 8 (gram-js) which doesn't depend on this

The parser can be fixed independently without changing the AST API.

---

**Created**: January 9, 2026  
**Issue**: Properties not parsed from gram text  
**Severity**: Medium (workaround available)  
**Blocks**: None (AST works, parser needs fixing)
