# Feature 018: Comonad Instance - Port Evaluation

**Evaluation Date**: 2026-01-05  
**Status**: ⏸️ **DEFERRED - Reconsider if Users Request Position-Aware Operations**

## Quick Summary

After comprehensive analysis of the Haskell implementation and usage patterns, **I recommend DEFERRING this feature** but with a **stronger case than Applicative**. Comonad has concrete use cases (position-aware operations) but they can be implemented as direct methods without the Comonad abstraction.

## Key Findings

### 1. Has Concrete Use Cases ✅ (Unlike Applicative)

gram-hs provides three helper functions:
- **`depthAt`** - Depth at every position
- **`sizeAt`** - Size at every position  
- **`indicesAt`** - Path indices at every position

These are tested and documented, suggesting they could be useful for visualization and debugging.

### 2. But Comonad Not Necessary ⚠️

**Only 1 of 3 helpers uses Comonad operations:**

```haskell
-- Uses Comonad extend ✅
depthAt = extend depth

-- Direct implementation ❌
sizeAt (Pattern _ es) =
  let subResults = map sizeAt es
      mySize = 1 + sum (map value subResults)
  in Pattern mySize subResults

-- Direct implementation ❌
indicesAt = go []
  where go path (Pattern _ es) = Pattern path (zipWith (\i e -> go (path ++ [i]) e) [0..] es)
```

**This proves**: Position-aware operations can be implemented without Comonad abstraction.

### 3. No Production Usage ❌

- ✅ Defined in Pattern/Core.hs
- ✅ Tested extensively (law tests + helper tests)
- ❌ **NOT used** in Pattern/Graph.hs or any other production code
- ❌ **NOT used** in any application code

### 4. gram-rs Already Has Inspection Operations ✅

| Operation | gram-rs Status |
|-----------|---------------|
| Get root value | ✅ `p.value()` or `p.value` |
| Max depth | ✅ `p.depth()` |
| Total size | ✅ `p.size()` |
| Depth at each position | ❌ Not implemented |
| Size at each position | ❌ Not implemented |
| Path at each position | ❌ Not implemented |

**Missing**: Position-aware transformations (compute X at every position).

## Comparison with Applicative

| Aspect | Applicative | Comonad |
|--------|-------------|---------|
| **Production usage** | ❌ Zero | ❌ Zero |
| **Concrete use cases** | ❌ None | ✅ 3 helpers |
| **Helpers tested** | N/A | ✅ Yes |
| **Abstraction necessary** | ❌ No | ⚠️ Only for 1 of 3 |
| **User value** | None | Low (but exists) |
| **Recommendation** | Defer indefinitely | Defer, reconsider if requested |

**Verdict**: Comonad has a **stronger case** than Applicative because it has concrete, tested helper functions that could be useful.

## Detailed Documentation

For complete analysis, see:

1. **`ANALYSIS.md`** - Comprehensive technical analysis
   - Haskell implementation details
   - Usage patterns in gram-hs
   - Comparison with existing gram-rs features
   - Rust port options and challenges

2. **`RECOMMENDATION.md`** - Executive summary and decision guide
   - Quick facts table
   - What Rust users need to know
   - Specific recommendation
   - Implementation options if proceeding

## Recommendation

### ⏸️ DEFER, But Reconsider if Requested

**Do not implement Comonad at this time.**

Instead:
- ✅ Keep existing `depth()`, `size()`, `value()` methods
- ⏸️ Wait for user requests for position-aware operations
- ✅ If requested, add specific helpers (`depth_at`, `size_at`, `indices_at`) as direct methods
- ❌ Skip general `extend` and `duplicate` unless concrete use cases emerge

### If Users Request Position-Aware Operations

**Step 1**: Add specific helpers as direct methods:

```rust
impl<V> Pattern<V> {
    /// Returns a pattern where each position contains its depth.
    pub fn depth_at(&self) -> Pattern<usize> {
        Pattern {
            value: self.depth(),
            elements: self.elements.iter().map(|e| e.depth_at()).collect(),
        }
    }
    
    // Similar for size_at and indices_at
}
```

**Step 2**: Evaluate if general `extend` is needed:
- Do users need custom position-aware functions?
- Are there multiple use cases beyond the three helpers?
- Would `extend` simplify implementations?

**Step 3**: Only implement full Comonad if justified:
- Multiple use cases for general `extend`
- Users specifically request it
- Clear benefit over direct implementations

## Why Defer?

1. **No proven need**: Zero production usage in gram-hs
2. **Direct implementations clearer**: 2 of 3 helpers don't use Comonad
3. **All inspection operations exist**: gram-rs has depth(), size(), value()
4. **Can add helpers later**: Direct methods easy to add if requested
5. **Testing burden**: 3 comonad laws + helper tests for unclear benefit

## Why Stronger Than Applicative?

1. **Has concrete use cases**: 3 tested helper functions
2. **Could be useful**: Position-aware operations for visualization/debugging
3. **Documented in gram-hs**: Helpers have examples and tests
4. **Reasonable implementations**: Direct methods are straightforward

## Next Steps

1. ✅ Review this analysis
2. ✅ Updated TODO.md with DEFER status
3. ⏸️ Wait for user feedback on position-aware operations
4. 📋 Move to Phase 4 features (gram notation serialization)
5. 📝 Add helpers if users request them

## Questions for Users

Before implementing, ask:

1. **Do you need position-aware operations?**
   - Compute depth at every position?
   - Compute size at every position?
   - Compute path at every position?

2. **What would you use them for?**
   - Visualization?
   - Debugging?
   - Analysis?

3. **Do you need custom position-aware functions?**
   - Beyond depth, size, path?
   - General `extend` operation?

4. **Would direct methods suffice?**
   - Or do you need the Comonad abstraction?

If answers are unclear or "no", keep feature deferred.

## References

- Haskell Implementation: `../gram-hs/libs/pattern/src/Pattern/Core.hs:720-728, 1104-1138`
- Haskell Tests: `../gram-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs:4242-4400`
- Haskell Property Tests: `../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs:1287-1332`
- Feature Spec: `../gram-hs/specs/014-comonad-instance/spec.md`
- Porting Guide: `../../PORTING_GUIDE.md`
- TODO: `../../TODO.md` (updated with DEFER status)

