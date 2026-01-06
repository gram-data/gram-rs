# Feature Specification: Comonad Operations for Pattern

**Feature Branch**: `018-comonad-instance`  
**Created**: 2026-01-05  
**Status**: Draft  
**Input**: User description: "Comonad for Pattern as explored in specs/018-comonad-instance"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Extract Decorative Values (Priority: P1) 🎯 MVP

As a developer working with Pattern structures, I need to access the decorative value at any position so that I can retrieve metadata about the pattern without navigating the structure manually.

**Why this priority**: This is the foundational operation that enables all context-aware computations. The `extract` operation provides direct access to the value component of Pattern's "decorated sequence" semantics (value decorates elements). Without this, developers cannot access pattern metadata in a consistent, principled way.

**Independent Test**: Can be fully tested by calling `extract()` on patterns with various value types and verifying that it returns the correct decorative value. This delivers immediate value as a clean accessor for pattern metadata.

**Acceptance Scenarios**:

1. **Given** a pattern with string value "sonata" and elements ["A", "B", "A"], **When** I call `extract()`, **Then** I get "sonata" (the decorative value)
2. **Given** an atomic pattern (no elements) with value 42, **When** I call `extract()`, **Then** I get 42
3. **Given** a nested pattern structure, **When** I call `extract()` on any subpattern, **Then** I get that subpattern's value
4. **Given** patterns with different value types (String, Int, custom types), **When** I call `extract()`, **Then** I get the appropriate type for each

---

### User Story 2 - Compute Position-Aware Decorations (Priority: P1) 🎯 MVP

As a developer analyzing Pattern structures, I need to compute new decorative information at every position based on the subpattern context, so that I can annotate patterns with structural metadata (like depth, size, or custom metrics) for visualization, debugging, or analysis purposes.

**Why this priority**: The `extend` operation is the core capability that distinguishes Comonad from other pattern operations. It enables context-aware decoration where functions have access to the full subpattern at each position. This is essential for computing structural properties, enables visualization tools, and provides the foundation for all position-aware operations.

**Independent Test**: Can be fully tested by implementing `extend` and verifying that it correctly applies a context-aware function at every position in the pattern. Test with functions that compute depth, size, or other properties. This delivers the ability to transform patterns based on structural context.

**Acceptance Scenarios**:

1. **Given** a pattern and a function that computes depth, **When** I call `extend(|p| p.depth())`, **Then** I get a new pattern where each position's value is the depth of that subpattern
2. **Given** a pattern with nested structure, **When** I apply `extend` with a size function, **Then** each position is decorated with the count of nodes in that subtree
3. **Given** any pattern, **When** I apply `extend` with a function f, **Then** the root value equals f(original_pattern) and elements are recursively transformed
4. **Given** patterns with varying depths and element counts, **When** I apply `extend`, **Then** the function is applied at ALL positions (root and all nested levels)

---

### User Story 3 - Decorate with Depth at Each Position (Priority: P2)

As a developer visualizing or debugging Pattern structures, I need to see the depth (maximum nesting level) at every position, so that I can understand the structural complexity and identify deeply nested areas.

**Why this priority**: This is a concrete, useful application of `extend` that demonstrates position-aware decoration. Depth information is valuable for visualization, performance analysis, and structural validation. While not as fundamental as `extract` and `extend`, it provides immediate practical value.

**Independent Test**: Can be fully tested by calling `depth_at()` on patterns of known structure and verifying the depth values at each position. Delivers a ready-to-use utility for structural analysis.

**Acceptance Scenarios**:

1. **Given** an atomic pattern (no elements), **When** I call `depth_at()`, **Then** I get a pattern with value 0 (atomic patterns have depth 0)
2. **Given** a pattern with two atomic children, **When** I call `depth_at()`, **Then** root has depth 0, both children have depth 0
3. **Given** a nested structure root[a[x], b], **When** I call `depth_at()`, **Then** root has depth 2, a has depth 1, x has depth 0, b has depth 0
4. **Given** deeply nested patterns (10+ levels), **When** I call `depth_at()`, **Then** each position shows its correct nesting depth

---

### User Story 4 - Decorate with Subtree Size at Each Position (Priority: P2)

As a developer analyzing Pattern structures, I need to see the size (total node count) of the subtree at every position, so that I can identify large substructures, understand pattern distribution, and optimize operations on specific subtrees.

**Why this priority**: Size information complements depth for structural analysis. It's particularly useful for identifying "heavy" subtrees, understanding pattern complexity distribution, and making decisions about where to focus optimization efforts.

**Independent Test**: Can be fully tested by calling `size_at()` on patterns of known structure and verifying the node counts. Delivers a utility for complexity analysis.

**Acceptance Scenarios**:

1. **Given** an atomic pattern, **When** I call `size_at()`, **Then** I get a pattern with value 1 (one node)
2. **Given** a pattern with three atomic children, **When** I call `size_at()`, **Then** root has size 4 (root + 3 children), each child has size 1
3. **Given** a nested structure root[a[x], b], **When** I call `size_at()`, **Then** root has size 4, a has size 2, x has size 1, b has size 1
4. **Given** patterns with varying subtree sizes, **When** I call `size_at()`, **Then** each position shows the total count of nodes in that subtree

---

### User Story 5 - Decorate with Path Indices (Priority: P3)

As a developer navigating Pattern structures, I need to see the path from root to every position (as a sequence of element indices), so that I can reference specific positions programmatically, generate navigation links, or reconstruct the traversal path.

**Why this priority**: Path information is useful for navigation and reference but less critical than depth and size. It enables position addressing and path-based operations but is not essential for basic structural analysis.

**Independent Test**: Can be fully tested by calling `indices_at()` on patterns and verifying the path indices. Delivers a utility for position addressing.

**Acceptance Scenarios**:

1. **Given** an atomic pattern, **When** I call `indices_at()`, **Then** root has path [] (empty - it is the root)
2. **Given** a pattern with two children, **When** I call `indices_at()`, **Then** root has path [], first child has path [0], second child has path [1]
3. **Given** a nested structure root[a[x], b], **When** I call `indices_at()`, **Then** root has [], a has [0], x has [0, 0], b has [1]
4. **Given** deeply nested patterns, **When** I call `indices_at()`, **Then** each position has the correct path from root

---

### User Story 6 - Verify Comonad Laws (Priority: P1) 🎯 MVP

As a developer using Comonad operations, I need confidence that the operations satisfy the mathematical Comonad laws (extract-extend, extend-extract, and associativity), so that I can reason about transformations and compose operations safely without unexpected behavior.

**Why this priority**: Comonad laws are fundamental properties that ensure operations behave predictably and compose correctly. Without these guarantees, the feature would be unreliable and could produce inconsistent results, making it unsuitable for production use.

**Independent Test**: Can be fully tested through property-based testing that verifies all three laws hold for patterns of various structures. Delivers mathematical correctness guarantees.

**Acceptance Scenarios**:

1. **Given** any pattern p and function f, **When** I compute `extract(extend(f, p))`, **Then** it equals `f(p)` (extract-extend law)
2. **Given** any pattern p, **When** I compute `extend(extract, p)`, **Then** it equals p unchanged (extend-extract law / identity)
3. **Given** any pattern p and functions f and g, **When** I compute `extend(f, extend(g, p))`, **Then** it equals `extend(f ∘ extend(g), p)` (associativity law)
4. **Given** patterns of various structures (atomic, with elements, nested), **When** I verify all three laws, **Then** all laws hold for all structures

---

### Edge Cases

- What happens when calling `extract()` on patterns with custom value types?
  - Should work with any type V, returning &V
- What happens when calling `extend` with functions that access structural properties (depth, size, indices)?
  - Should correctly compute properties based on the subpattern at each position
- What happens when `extend` is called with functions that transform value types (e.g., Pattern<String> → Pattern<usize>)?
  - Should produce new pattern with transformed value type
- What happens with deeply nested patterns (100+ levels)?
  - Should handle recursion without stack overflow (same guarantees as existing Pattern methods)
- What happens with patterns containing many elements (1000+ elements)?
  - Should process efficiently (performance comparable to existing methods like map)
- What happens when helper functions (depth_at, size_at, indices_at) are called on empty patterns or atomic patterns?
  - Should return appropriate values: depth 0, size 1, path []
- How do comonad operations interact with existing Pattern operations (map, fold, traverse)?
  - Should compose naturally: can map decorations, fold over decorations, traverse decorated patterns

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide `extract` operation that returns a reference to the pattern's decorative value (the value component)
- **FR-002**: System MUST provide `extend` operation that takes a context-aware function and applies it at every position in the pattern
- **FR-003**: `extend` MUST pass the full subpattern (not just the value) to the function at each position, enabling context-aware computation
- **FR-004**: `extend` MUST preserve the pattern structure (same nesting depth and element count), replacing values with computed decorations
- **FR-005**: System MUST satisfy the extract-extend Comonad law: `extract(extend(f, p)) = f(p)` for all patterns p and functions f
- **FR-006**: System MUST satisfy the extend-extract Comonad law: `extend(extract, p) = p` for all patterns p (identity)
- **FR-007**: System MUST satisfy the extend associativity law: `extend(f, extend(g, p)) = extend(f ∘ extend(g), p)` for all patterns p and functions f, g
- **FR-008**: System MUST provide `depth_at` helper that decorates each position with its depth (maximum nesting level of subpattern)
- **FR-009**: System MUST provide `size_at` helper that decorates each position with the size of its subtree (total node count)
- **FR-010**: System MUST provide `indices_at` helper that decorates each position with its path from root (sequence of element indices)
- **FR-011**: Operations MUST work with patterns containing values of any type (generic over value type V)
- **FR-012**: Operations MUST handle atomic patterns (patterns with no elements) correctly
- **FR-013**: Operations MUST handle patterns with elements correctly (including nested structures)
- **FR-014**: Operations MUST handle deeply nested patterns (100+ levels) without stack overflow
- **FR-015**: Operations MUST handle patterns with many elements (1000+ elements) efficiently
- **FR-016**: `depth_at` SHOULD be implemented using `extend` to demonstrate conceptual consistency
- **FR-017**: `size_at` SHOULD be implemented using `extend` to demonstrate conceptual consistency
- **FR-018**: `indices_at` MAY use direct implementation (requires path tracking that extend doesn't provide)

### Key Entities

- **Pattern**: A recursive data structure representing a decorated sequence. Contains a value (decoration) and a list of pattern elements (the actual content). The value provides information ABOUT the elements. This "decorated sequence" semantic is what makes Pattern a natural Comonad.

- **Decorative Value**: The value component of a Pattern that provides metadata or information about the elements. This is what `extract` accesses and what `extend` recomputes.

- **Context-Aware Function**: A function that takes a Pattern (not just a value) and computes a result based on the full subpattern structure. Examples: computing depth, size, indices, or any custom structural metric. This is the type of function that `extend` applies.

- **Position**: A specific node in the pattern tree. Each position has a subpattern (the tree rooted at that position) and can be decorated with information about that subpattern.

- **Subpattern**: The pattern structure rooted at a specific position. `extend` gives functions access to the full subpattern at each position, enabling context-aware computation.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can extract decorative values from patterns in a single method call, eliminating manual structure navigation
- **SC-002**: Developers can compute position-aware decorations using `extend` with any context-aware function
- **SC-003**: All three Comonad laws verified to hold for 100% of test cases across diverse pattern structures (atomic, with elements, deeply nested)
- **SC-004**: Helper functions (`depth_at`, `size_at`, `indices_at`) produce correct results for 100% of test patterns
- **SC-005**: Operations handle deeply nested patterns (100+ levels) without stack overflow
- **SC-006**: Operations process patterns with 1000+ elements in under 100ms on standard hardware
- **SC-007**: `extend` operations complete in time proportional to pattern size (O(n) where n is node count)
- **SC-008**: Developers can compose `extend` operations with existing Pattern methods (`map`, `fold`, `traverse`) without errors
- **SC-009**: Documentation clearly explains Pattern's "decorated sequence" semantics and why Comonad is the conceptually correct abstraction
- **SC-010**: Code examples demonstrate practical use cases (depth analysis, size analysis, path tracking)

## Assumptions

- Pattern's "decorated sequence" semantics (value decorates elements) are well-understood and documented
- The value component provides information ABOUT the elements, not additional data alongside them
- Context-aware functions need access to the full subpattern structure, not just individual values
- Position-aware decorations are useful for visualization, debugging, analysis, and optimization
- Comonad laws provide valuable correctness guarantees even with limited production usage in gram-hs
- Implementation will pass functions by reference (no Clone bound needed)
- `duplicate` operation (Pattern<Pattern<V>>) is deferred until concrete use cases emerge
- Property-based testing will verify laws across many pattern structures
- Implementation will use idiomatic Rust patterns (passing function by reference, consuming self where appropriate)

## Dependencies

- **Prerequisites**: Pattern data type with value and elements fields (✅ Complete - Feature 004)
- **Prerequisites**: Pattern must have depth(), size(), and value() methods (✅ Complete - Feature 005)
- **Prerequisites**: Pattern must have Eq instance for testing (✅ Complete)
- **Prerequisites**: Pattern must have Debug/Display for debugging (✅ Complete)
- **No blocking dependencies**: This feature can be implemented with existing Pattern functionality

## Out of Scope

- `duplicate` operation (Pattern<Pattern<V>>) - Deferred until concrete use cases emerge
- Custom Comonad trait abstraction - Using direct methods for idiomatic Rust
- Position-aware operations beyond depth, size, indices - Can be added later as helpers if needed
- Integration with external visualization libraries - Users can build on top of the provided operations
- Performance optimization for specific use cases - Basic implementation sufficient
- Async/concurrent versions of operations - Not needed for current use cases
