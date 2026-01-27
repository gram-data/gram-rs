#!/usr/bin/env python3
"""
Basic usage examples for pattern-core Python bindings.

Demonstrates:
- Creating atomic and nested patterns
- Working with Subjects (identity, labels, properties)
- Creating PatternSubject instances
- Basic pattern inspection
"""

import sys

try:
    import pattern_core
except ImportError:
    print("ERROR: pattern_core module not found.")
    print("Build it with: cd crates/pattern-core && maturin develop --uv --features python")
    sys.exit(1)


def example_atomic_pattern():
    """Create and inspect atomic patterns."""
    print("=" * 60)
    print("Example 1: Atomic Patterns")
    print("=" * 60)
    
    # Create atomic pattern
    atomic = pattern_core.Pattern.point("hello")
    
    print(f"Value: {atomic.value}")
    print(f"Elements: {atomic.elements}")
    print(f"Is atomic: {atomic.is_atomic()}")
    print(f"Length: {atomic.length()}")
    print(f"Size: {atomic.size()}")
    print(f"Depth: {atomic.depth()}")
    print()


def example_nested_pattern():
    """Create and inspect nested patterns."""
    print("=" * 60)
    print("Example 2: Nested Patterns")
    print("=" * 60)
    
    # Create children
    child1 = pattern_core.Pattern.point("child1")
    child2 = pattern_core.Pattern.point("child2")
    child3 = pattern_core.Pattern.point("child3")
    
    # Create parent with children
    parent = pattern_core.Pattern.pattern("parent", [child1, child2, child3])
    
    print(f"Parent value: {parent.value}")
    print(f"Number of children (length): {parent.length()}")
    print(f"Total nodes (size): {parent.size()}")
    print(f"Maximum depth: {parent.depth()}")
    print(f"Is atomic: {parent.is_atomic()}")
    
    # Access children
    print(f"\nChildren:")
    for i, child in enumerate(parent.elements):
        print(f"  Child {i}: {child.value}")
    print()


def example_pattern_from_list():
    """Create pattern from list of values."""
    print("=" * 60)
    print("Example 3: Pattern from List")
    print("=" * 60)
    
    # Create pattern from list (convenience method)
    pattern = pattern_core.Pattern.from_list("root", ["a", "b", "c", "d"])
    
    print(f"Root value: {pattern.value}")
    print(f"Length: {pattern.length()}")
    print(f"All values: {pattern.values()}")
    print()


def example_deeply_nested_pattern():
    """Create deeply nested pattern."""
    print("=" * 60)
    print("Example 4: Deeply Nested Pattern")
    print("=" * 60)
    
    # Build tree structure
    leaf = pattern_core.Pattern.point("leaf")
    level2 = pattern_core.Pattern.pattern("level2", [leaf])
    level1 = pattern_core.Pattern.pattern("level1", [level2])
    root = pattern_core.Pattern.pattern("root", [level1])
    
    print(f"Root value: {root.value}")
    print(f"Maximum depth: {root.depth()}")
    print(f"Total size: {root.size()}")
    print(f"All values: {root.values()}")
    print()


def example_value_types():
    """Create and work with Value types."""
    print("=" * 60)
    print("Example 5: Value Types")
    print("=" * 60)
    
    # Create different value types
    str_val = pattern_core.Value.string("hello")
    int_val = pattern_core.Value.int(42)
    decimal_val = pattern_core.Value.decimal(3.14)
    bool_val = pattern_core.Value.boolean(True)
    symbol_val = pattern_core.Value.symbol("alice")
    
    print(f"String value: {str_val.as_string()}")
    print(f"Integer value: {int_val.as_int()}")
    print(f"Decimal value: {decimal_val.as_decimal()}")
    print(f"Boolean value: {bool_val.as_boolean()}")
    print(f"Symbol value: {symbol_val.as_string()}")  # Symbols are strings
    
    # Array and map
    array_val = pattern_core.Value.array([
        pattern_core.Value.int(1),
        pattern_core.Value.int(2),
        pattern_core.Value.int(3)
    ])
    
    map_val = pattern_core.Value.map({
        "key1": pattern_core.Value.string("value1"),
        "key2": pattern_core.Value.int(42)
    })
    
    print(f"\nArray: {array_val.as_array()}")
    print(f"Map: {map_val.as_map()}")
    
    # Range and measurement
    range_val = pattern_core.Value.range(lower=0.0, upper=100.0)
    measurement_val = pattern_core.Value.measurement(42.5, "meters")
    
    print(f"Range: {range_val}")
    print(f"Measurement: {measurement_val}")
    print()


def example_subject_basic():
    """Create and work with basic Subjects."""
    print("=" * 60)
    print("Example 6: Basic Subject")
    print("=" * 60)
    
    # Create Subject with identity only
    subject = pattern_core.Subject(identity="alice")
    
    print(f"Identity: {subject.identity}")
    print(f"Labels: {subject.get_labels()}")
    print(f"Properties: {subject.get_properties()}")
    print()


def example_subject_with_labels():
    """Create Subject with labels."""
    print("=" * 60)
    print("Example 7: Subject with Labels")
    print("=" * 60)
    
    # Create Subject with labels
    subject = pattern_core.Subject(
        identity="alice",
        labels={"Person", "Employee", "Developer"}
    )
    
    print(f"Identity: {subject.identity}")
    print(f"Labels: {subject.get_labels()}")
    
    # Add and remove labels
    subject.add_label("Manager")
    print(f"After adding Manager: {subject.get_labels()}")
    
    subject.remove_label("Developer")
    print(f"After removing Developer: {subject.get_labels()}")
    
    # Check label
    has_employee = subject.has_label("Employee")
    print(f"Has Employee label: {has_employee}")
    print()


def example_subject_with_properties():
    """Create Subject with properties."""
    print("=" * 60)
    print("Example 8: Subject with Properties")
    print("=" * 60)
    
    # Create Subject with properties
    subject = pattern_core.Subject(
        identity="alice",
        properties={
            "name": pattern_core.Value.string("Alice"),
            "age": pattern_core.Value.int(30),
            "email": pattern_core.Value.string("alice@example.com")
        }
    )
    
    print(f"Identity: {subject.identity}")
    print(f"Properties: {subject.get_properties()}")
    
    # Get specific property
    name_value = subject.get_property("name")
    if name_value:
        print(f"Name: {name_value.as_string()}")
    
    # Set and remove properties
    subject.set_property("department", pattern_core.Value.string("Engineering"))
    print(f"After adding department: {subject.get_properties()}")
    
    subject.remove_property("email")
    print(f"After removing email: {subject.get_properties()}")
    print()


def example_pattern_subject():
    """Create Pattern with Subject value."""
    print("=" * 60)
    print("Example 9: PatternSubject")
    print("=" * 60)
    
    # Create Subject
    subject = pattern_core.Subject(
        identity="alice",
        labels={"Person", "Employee"},
        properties={
            "name": pattern_core.Value.string("Alice"),
            "age": pattern_core.Value.int(30)
        }
    )
    
    # Create PatternSubject
    pattern = pattern_core.PatternSubject.point(subject)
    
    print(f"Pattern value (identity): {pattern.get_value().identity}")
    print(f"Pattern is atomic: {pattern.is_atomic()}")
    
    # Access Subject properties through pattern
    subject_value = pattern.get_value()
    print(f"Subject labels: {subject_value.get_labels()}")
    print(f"Subject properties: {subject_value.get_properties()}")
    print()


def example_pattern_subject_nested():
    """Create nested PatternSubject."""
    print("=" * 60)
    print("Example 10: Nested PatternSubject")
    print("=" * 60)
    
    # Create multiple Subjects
    alice = pattern_core.Subject(
        identity="alice",
        labels={"Person"},
        properties={"name": pattern_core.Value.string("Alice")}
    )
    
    bob = pattern_core.Subject(
        identity="bob",
        labels={"Person"},
        properties={"name": pattern_core.Value.string("Bob")}
    )
    
    charlie = pattern_core.Subject(
        identity="charlie",
        labels={"Person"},
        properties={"name": pattern_core.Value.string("Charlie")}
    )
    
    # Create PatternSubjects
    bob_pattern = pattern_core.PatternSubject.point(bob)
    charlie_pattern = pattern_core.PatternSubject.point(charlie)
    
    # Alice knows Bob and Charlie
    alice_pattern = pattern_core.PatternSubject.pattern(
        alice,
        [bob_pattern, charlie_pattern]
    )
    
    print(f"Alice identity: {alice_pattern.get_value().identity}")
    print(f"Alice knows {alice_pattern.length()} people")
    print(f"Total subjects: {alice_pattern.size()}")
    
    # Get all subjects in the pattern
    all_subjects = alice_pattern.values()
    print(f"All subjects: {[s.identity for s in all_subjects]}")
    print()


def main():
    """Run all examples."""
    print("\n" + "=" * 60)
    print("PATTERN-CORE PYTHON BINDINGS - BASIC USAGE EXAMPLES")
    print("=" * 60 + "\n")
    
    example_atomic_pattern()
    example_nested_pattern()
    example_pattern_from_list()
    example_deeply_nested_pattern()
    example_value_types()
    example_subject_basic()
    example_subject_with_labels()
    example_subject_with_properties()
    example_pattern_subject()
    example_pattern_subject_nested()
    
    print("=" * 60)
    print("All basic usage examples completed successfully!")
    print("=" * 60)


if __name__ == "__main__":
    main()
