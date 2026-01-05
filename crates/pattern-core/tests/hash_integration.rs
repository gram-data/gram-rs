//! Integration tests for Pattern hashing with collections
//!
//! Tests practical usage patterns including:
//! - Set operations (intersection, difference, union)
//! - Pattern indexing use cases
//! - Caching patterns
//! - Integration with other Pattern operations

use pattern_core::Pattern;
use std::collections::{HashMap, HashSet};

// ============================================================================
// Set Operations Tests
// ============================================================================

#[test]
fn test_hashset_intersection() {
    let p1 = Pattern::point("a".to_string());
    let p2 = Pattern::point("b".to_string());
    let p3 = Pattern::point("c".to_string());

    let mut set_a = HashSet::new();
    set_a.insert(p1.clone());
    set_a.insert(p2.clone());

    let mut set_b = HashSet::new();
    set_b.insert(p2.clone());
    set_b.insert(p3.clone());

    let intersection: HashSet<_> = set_a.intersection(&set_b).cloned().collect();

    assert_eq!(intersection.len(), 1);
    assert!(intersection.contains(&p2));
}

#[test]
fn test_hashset_difference() {
    let p1 = Pattern::point("a".to_string());
    let p2 = Pattern::point("b".to_string());
    let p3 = Pattern::point("c".to_string());

    let mut set_a = HashSet::new();
    set_a.insert(p1.clone());
    set_a.insert(p2.clone());

    let mut set_b = HashSet::new();
    set_b.insert(p2.clone());
    set_b.insert(p3.clone());

    let difference: HashSet<_> = set_a.difference(&set_b).cloned().collect();

    assert_eq!(difference.len(), 1);
    assert!(difference.contains(&p1));
    assert!(!difference.contains(&p2));
}

#[test]
fn test_hashset_union() {
    let p1 = Pattern::point("a".to_string());
    let p2 = Pattern::point("b".to_string());
    let p3 = Pattern::point("c".to_string());

    let mut set_a = HashSet::new();
    set_a.insert(p1.clone());
    set_a.insert(p2.clone());

    let mut set_b = HashSet::new();
    set_b.insert(p2.clone()); // Duplicate
    set_b.insert(p3.clone());

    let union: HashSet<_> = set_a.union(&set_b).cloned().collect();

    assert_eq!(union.len(), 3);
    assert!(union.contains(&p1));
    assert!(union.contains(&p2));
    assert!(union.contains(&p3));
}

#[test]
fn test_hashset_symmetric_difference() {
    let p1 = Pattern::point("a".to_string());
    let p2 = Pattern::point("b".to_string());
    let p3 = Pattern::point("c".to_string());

    let mut set_a = HashSet::new();
    set_a.insert(p1.clone());
    set_a.insert(p2.clone());

    let mut set_b = HashSet::new();
    set_b.insert(p2.clone());
    set_b.insert(p3.clone());

    let sym_diff: HashSet<_> = set_a.symmetric_difference(&set_b).cloned().collect();

    assert_eq!(sym_diff.len(), 2);
    assert!(sym_diff.contains(&p1));
    assert!(sym_diff.contains(&p3));
    assert!(!sym_diff.contains(&p2));
}

// ============================================================================
// Pattern Indexing Use Cases
// ============================================================================

#[test]
fn test_pattern_location_index() {
    // Simulate building a reverse index for pattern locations
    #[derive(Debug, Clone, PartialEq)]
    struct Location {
        file: String,
        line: usize,
    }

    let mut index: HashMap<Pattern<String>, Vec<Location>> = HashMap::new();

    let pattern1 = Pattern::point("constant".to_string());
    let pattern2 = Pattern::pattern(
        "function_call".to_string(),
        vec![Pattern::point("arg".to_string())],
    );

    // Add locations for pattern1
    index
        .entry(pattern1.clone())
        .or_insert_with(Vec::new)
        .push(Location {
            file: "main.rs".to_string(),
            line: 10,
        });
    index
        .entry(pattern1.clone())
        .or_insert_with(Vec::new)
        .push(Location {
            file: "lib.rs".to_string(),
            line: 42,
        });

    // Add location for pattern2
    index
        .entry(pattern2.clone())
        .or_insert_with(Vec::new)
        .push(Location {
            file: "main.rs".to_string(),
            line: 15,
        });

    // Verify lookups
    assert_eq!(index.get(&pattern1).unwrap().len(), 2);
    assert_eq!(index.get(&pattern2).unwrap().len(), 1);
}

#[test]
fn test_pattern_deduplication_pipeline() {
    // Simulate a pipeline that deduplicates patterns during processing
    let input_patterns = vec![
        Pattern::point("a".to_string()),
        Pattern::point("b".to_string()),
        Pattern::point("a".to_string()), // Duplicate
        Pattern::point("c".to_string()),
        Pattern::point("b".to_string()), // Duplicate
    ];

    let unique_patterns: HashSet<_> = input_patterns.into_iter().collect();

    assert_eq!(unique_patterns.len(), 3);
}

// ============================================================================
// Caching Use Cases
// ============================================================================

#[test]
fn test_memoization_cache() {
    // Simulate memoizing expensive computations
    let mut cache: HashMap<Pattern<String>, String> = HashMap::new();

    let compute_expensive = |p: &Pattern<String>| -> String {
        // Simulate expensive computation
        format!("result_for_{}", p.value())
    };

    let pattern1 = Pattern::point("input1".to_string());
    let pattern2 = Pattern::point("input2".to_string());

    // First computation - cache miss
    let result1 = cache
        .entry(pattern1.clone())
        .or_insert_with(|| compute_expensive(&pattern1))
        .clone();
    assert_eq!(result1, "result_for_input1");

    // Second lookup - cache hit
    let result1_cached = cache.get(&pattern1).unwrap();
    assert_eq!(result1_cached, &result1);

    // Different pattern - cache miss
    let result2 = cache
        .entry(pattern2.clone())
        .or_insert_with(|| compute_expensive(&pattern2))
        .clone();
    assert_eq!(result2, "result_for_input2");

    assert_eq!(cache.len(), 2);
}

#[test]
fn test_pattern_compilation_cache() {
    // Simulate caching compiled patterns
    #[derive(Debug, Clone)]
    struct CompiledPattern {
        optimized: bool,
        instructions: Vec<String>,
    }

    let mut compilation_cache: HashMap<Pattern<String>, CompiledPattern> = HashMap::new();

    let pattern = Pattern::pattern(
        "match".to_string(),
        vec![Pattern::point("expr".to_string())],
    );

    // Compile once
    compilation_cache.insert(
        pattern.clone(),
        CompiledPattern {
            optimized: true,
            instructions: vec!["load".to_string(), "match".to_string()],
        },
    );

    // Reuse compilation
    assert!(compilation_cache.contains_key(&pattern));
    let compiled = compilation_cache.get(&pattern).unwrap();
    assert!(compiled.optimized);
}

// ============================================================================
// Integration with Pattern Operations
// ============================================================================

#[test]
fn test_hash_with_map_operation() {
    use pattern_core::Combinable;

    let patterns = vec![
        Pattern::point("hello".to_string()),
        Pattern::point("world".to_string()),
        Pattern::point("hello".to_string()), // Duplicate
    ];

    // Map then deduplicate
    let unique_upper: HashSet<_> = patterns
        .into_iter()
        .map(|p| p.map(|s| s.to_uppercase()))
        .collect();

    assert_eq!(unique_upper.len(), 2);
    assert!(unique_upper.contains(&Pattern::point("HELLO".to_string())));
    assert!(unique_upper.contains(&Pattern::point("WORLD".to_string())));
}

#[test]
fn test_hash_with_combine_operation() {
    use pattern_core::Combinable;

    let p1 = Pattern::point("a".to_string());
    let p2 = Pattern::point("b".to_string());

    let combined = p1.clone().combine(p2.clone());

    // Can use combined pattern as key
    let mut results: HashMap<Pattern<String>, &str> = HashMap::new();
    results.insert(combined.clone(), "combined");
    results.insert(p1.clone(), "p1");
    results.insert(p2.clone(), "p2");

    assert_eq!(results.len(), 3);
    assert_eq!(results.get(&combined), Some(&"combined"));
}

#[test]
fn test_hash_preserves_structure_after_operations() {
    // Verify that hash is consistent after various operations
    let original = Pattern::pattern(
        "root".to_string(),
        vec![
            Pattern::point("a".to_string()),
            Pattern::point("b".to_string()),
        ],
    );

    let cloned = original.clone();

    // Clone should produce equal pattern with same hash
    let mut set = HashSet::new();
    set.insert(original.clone());

    assert!(set.contains(&cloned));
    assert_eq!(set.len(), 1);
}

// ============================================================================
// Large-Scale Deduplication
// ============================================================================

#[test]
fn test_large_scale_deduplication() {
    let mut patterns = Vec::new();

    // Generate patterns with some duplicates
    for i in 0..1000 {
        let value = format!("pattern_{}", i % 100); // 10x duplication
        patterns.push(Pattern::point(value));
    }

    let unique: HashSet<_> = patterns.into_iter().collect();

    // Should deduplicate to 100 unique patterns
    assert_eq!(unique.len(), 100);
}

#[test]
fn test_nested_pattern_deduplication() {
    let mut patterns = Vec::new();

    // Generate nested patterns with duplicates
    for i in 0..100 {
        let root_value = format!("root_{}", i % 10); // 10 unique roots
        let child_value = format!("child_{}", i % 5); // 5 unique children
        let pattern = Pattern::pattern(root_value, vec![Pattern::point(child_value)]);
        patterns.push(pattern);
    }

    let unique: HashSet<_> = patterns.into_iter().collect();

    // Should have deduplicated based on structure
    // With i % 10 for root and i % 5 for child, we get 10 unique combinations:
    // (0,0), (1,1), (2,2), (3,3), (4,4), (5,0), (6,1), (7,2), (8,3), (9,4)
    // These 10 patterns repeat 10 times each
    assert_eq!(unique.len(), 10);
}
