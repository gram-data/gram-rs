# Relationship Creation Functions Added

**Date**: 2026-01-27  
**Feature**: Pattern relationship creation (zip3 and zip_with)

## Summary

Added two new functions for creating relationship patterns between nodes - essential for graph construction and bulk operations.

## What Was Added

### Rust Implementation (`pattern-core/src/pattern.rs`)

#### 1. `Pattern::zip3` - Combine Three Lists

```rust
pub fn zip3(
    left: Vec<Pattern<V>>,
    right: Vec<Pattern<V>>,
    values: Vec<V>,
) -> Vec<Pattern<V>>
```

**Purpose**: Create relationship patterns from three pre-computed lists.

**Use Cases**:
- Bulk import from CSV, JSON, database queries
- External API responses with relationship triples
- Static relationship configuration

**Example**:
```rust
let sources = vec![Pattern::point("Alice"), Pattern::point("Bob")];
let targets = vec![Pattern::point("Company"), Pattern::point("Project")];
let rel_types = vec!["WORKS_FOR", "MANAGES"];

let relationships = Pattern::zip3(sources, targets, rel_types);
// Creates: Pattern("WORKS_FOR", [Pattern("Alice"), Pattern("Company")])
//          Pattern("MANAGES", [Pattern("Bob"), Pattern("Project")])
```

#### 2. `Pattern::zip_with` - Compute Values from Pairs

```rust
pub fn zip_with<F>(
    left: Vec<Pattern<V>>,
    right: Vec<Pattern<V>>,
    value_fn: F,
) -> Vec<Pattern<V>>
where
    F: Fn(&Pattern<V>, &Pattern<V>) -> V,
    V: Clone,
```

**Purpose**: Create relationships with values computed from the node pairs.

**Use Cases**:
- Business logic for relationship determination
- Conditional relationship types based on node properties
- Dynamic graph generation with rules

**Example**:
```rust
let people = vec![Pattern::point("Alice"), Pattern::point("Bob")];
let companies = vec![Pattern::point("TechCorp"), Pattern::point("StartupInc")];

let relationships = Pattern::zip_with(people, companies, |person, company| {
    if company.value.contains("Startup") {
        "FOUNDED"
    } else {
        "WORKS_AT"
    }
});
```

### Python Bindings (`pattern-core/src/python.rs`)

Both functions exposed as static methods on the `Pattern` class:

```python
# zip3 - Pre-computed values
relationships = pattern_core.Pattern.zip3(
    sources,    # List[Pattern]
    targets,    # List[Pattern]
    rel_types   # List[Any]
)

# zip_with - Computed values
relationships = pattern_core.Pattern.zip_with(
    sources,    # List[Pattern]
    targets,    # List[Pattern]
    lambda s, t: compute_relationship(s, t)  # Callable[[Pattern, Pattern], Any]
)
```

### Type Stubs (`pattern_core/__init__.pyi`)

Added type annotations for both methods:

```python
@staticmethod
def zip3(
    left: List['Pattern'],
    right: List['Pattern'],
    values: List[Any]
) -> List['Pattern']: ...

@staticmethod
def zip_with(
    left: List['Pattern'],
    right: List['Pattern'],
    value_fn: Callable[['Pattern', 'Pattern'], Any]
) -> List['Pattern']: ...
```

### Examples (`examples/pattern-core-python/zip_relationships.py`)

Created comprehensive example file with 7 examples:

1. **Simple Relationships** - Basic zip3 usage
2. **Subject Relationships** - Using zip3 with Subject nodes
3. **Computed Relationships** - Using zip_with with business logic
4. **Conditional Relationships** - Access control example
5. **Bulk Import Pattern** - CSV/database import simulation
6. **Graph Queries** - Building and querying social networks
7. **Comparison** - When to use zip3 vs zip_with

### Documentation (`docs/python-usage.md`)

Added comprehensive section "Creating Graph Relationships" with:
- When to use which method (decision table)
- CSV import example
- Dynamic relationship creation example
- Knowledge graph construction example
- Performance considerations

## FP Background

These functions implement standard FP patterns:

- **zip3**: The classic `zipWith3` function from Haskell
- **zip_with**: Based on `zipWith` but specialized for pattern creation

```haskell
-- Haskell equivalent
zipWith3 :: (a -> b -> c -> d) -> [a] -> [b] -> [c] -> [d]

-- Our specialization
zip3 :: [Pattern V] -> [Pattern V] -> [V] -> [Pattern V]
zip_with :: (Pattern V -> Pattern V -> V) -> [Pattern V] -> [Pattern V] -> [Pattern V]
```

## Why Both?

The two functions serve complementary purposes:

### zip3 (List-based)
✅ **Better for**:
- Data already computed (CSV, database, API)
- Parallel data sources
- Explicit length control
- Simple, clear intent

### zip_with (Function-based)  
✅ **Better for**:
- Values depend on patterns being connected
- Business logic determines relationship type
- Lazy/on-demand computation
- Avoiding intermediate allocations

### Real-World Example

```python
# Data import scenario (use zip3)
import pandas as pd
df = pd.read_csv('relationships.csv')
sources = [pattern_core.Pattern.point(s) for s in df['source']]
targets = [pattern_core.Pattern.point(t) for t in df['target']]
rel_types = df['type'].tolist()
relationships = pattern_core.Pattern.zip3(sources, targets, rel_types)

# Access control scenario (use zip_with)
def determine_access(user_pattern, resource_pattern):
    user_role = user_pattern.value.split('_')[0]
    if user_role == 'admin':
        return 'FULL_ACCESS'
    elif resource_pattern.value == 'public':
        return 'READ_ONLY'
    else:
        return 'NO_ACCESS'

users = [pattern_core.Pattern.point("admin_user"), pattern_core.Pattern.point("guest_user")]
resources = [pattern_core.Pattern.point("db"), pattern_core.Pattern.point("public")]
access = pattern_core.Pattern.zip_with(users, resources, determine_access)
```

## Tests

All 7 examples in `zip_relationships.py` pass successfully:

```bash
$ python examples/pattern-core-python/zip_relationships.py
============================================================
PATTERN-CORE: RELATIONSHIP CREATION (ZIP3 & ZIP_WITH)
============================================================
[All 7 examples pass]
============================================================
All relationship creation examples completed!
============================================================
```

## Performance

Both functions have:
- **Time Complexity**: O(n) where n = min(len(left), len(right))
- **Space Complexity**: O(n) for result vector
- **zip3**: Slightly faster (no callback overhead)
- **zip_with**: Minimal overhead for function calls

Both stop at shortest list length (safe default behavior).

## Files Modified

1. **`crates/pattern-core/src/pattern.rs`**
   - Added `Pattern::zip3` (46 lines with docs)
   - Added `Pattern::zip_with` (44 lines with docs)

2. **`crates/pattern-core/src/python.rs`**
   - Added Python binding for `zip3` (35 lines)
   - Added Python binding for `zip_with` (35 lines)
   - Fixed `Value::as_string()` to handle symbols

3. **`crates/pattern-core/pattern_core/__init__.pyi`**
   - Added type hints for `zip3`
   - Added type hints for `zip_with`

4. **`examples/pattern-core-python/zip_relationships.py`**
   - New file with 7 comprehensive examples (270 lines)

5. **`docs/python-usage.md`**
   - Added "Creating Graph Relationships" section
   - Updated API reference table
   - Added use case examples

6. **`examples/pattern-core-python/README.md`**
   - Updated examples list
   - Added relationship creation examples

## Next Steps

These functions enable:
- **Graph databases**: Efficient bulk relationship creation
- **Knowledge graphs**: Entity and relationship extraction
- **Social networks**: Friend/connection modeling
- **Access control**: User-resource permission graphs
- **Data pipelines**: Transform relational data to graph structures

## Category Theory Note

These functions are related to:
- **Product**: Combining independent data (zip3 as cartesian product)
- **Reader/Environment**: Computing in context (zip_with as reader pattern)
- **Applicative**: `zip3` ≈ `liftA3`, `zip_with` ≈ `liftA2`

The design provides both **explicit** (zip3) and **computed** (zip_with) approaches, giving developers flexibility based on their use case.
