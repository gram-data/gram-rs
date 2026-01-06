#!/usr/bin/env python3
"""
Example usage of gram-codec Python bindings

To build and install:
    pip install maturin
    cd crates/gram-codec
    maturin develop --features python

To run this example:
    python examples/gram_codec.py
"""

# Uncomment after building the Python module
# from gram_codec import parse_gram, validate_gram, round_trip, version


def example1_parse():
    """Example 1: Parse gram notation"""
    print("=== Example 1: Parse Gram Notation ===")
    
    try:
        result = parse_gram("(alice)-[:KNOWS]->(bob)")
        print(f"Pattern count: {result['pattern_count']}")
        print(f"Identifiers: {result['identifiers']}")
    except ValueError as e:
        print(f"Parse error: {e}")


def example2_validate():
    """Example 2: Validate gram notation"""
    print("\n=== Example 2: Validate Gram Notation ===")
    
    examples = [
        "(hello)",
        "(a)-->(b)",
        "[team | alice, bob]",
        "(unclosed",  # Invalid
        "{missing_parens}",  # Invalid
    ]
    
    for gram in examples:
        is_valid = validate_gram(gram)
        print(f'"{gram}" is {"valid" if is_valid else "invalid"}')


def example3_roundtrip():
    """Example 3: Round-trip test"""
    print("\n=== Example 3: Round-Trip Test ===")
    
    original = '(alice:Person {name: "Alice"})-[:KNOWS]->(bob:Person {name: "Bob"})'
    print(f"Original:   {original}")
    
    try:
        serialized = round_trip(original)
        print(f"Serialized: {serialized}")
        
        # Verify it's still valid
        is_valid = validate_gram(serialized)
        print(f"Still valid: {is_valid}")
    except ValueError as e:
        print(f"Round-trip error: {e}")


def example4_multiple():
    """Example 4: Parse multiple patterns"""
    print("\n=== Example 4: Multiple Patterns ===")
    
    gram = "(alice) (bob) (charlie)"
    print(f"Input: {gram}")
    
    try:
        result = parse_gram(gram)
        print(f"Parsed {result['pattern_count']} patterns")
        print(f"Identifiers: {', '.join(result['identifiers'])}")
    except ValueError as e:
        print(f"Parse error: {e}")


def example5_complex():
    """Example 5: Parse complex patterns"""
    print("\n=== Example 5: Complex Patterns ===")
    
    examples = [
        # Node with label
        "(a:Person)",
        # Node with properties
        '(a {name: "Alice", age: 30})',
        # Relationship with label
        "(a)-[:KNOWS]->(b)",
        # Subject pattern
        '[team:Team {name: "DevRel"} | (alice), (bob), (charlie)]',
        # Nested pattern
        "[outer | [inner | (leaf)]]",
        # Annotated pattern
        "@type(node) (a)",
    ]
    
    for gram in examples:
        try:
            result = parse_gram(gram)
            print(f'✓ "{gram}" → {result["pattern_count"]} pattern(s)')
        except ValueError as e:
            print(f'✗ "{gram}" → Error: {e}')


def example6_errors():
    """Example 6: Error handling"""
    print("\n=== Example 6: Error Handling ===")
    
    invalid_examples = [
        "(unclosed",
        "{no_parens}",
        "(a {key: })",  # Empty property value
        "((nested))",  # Invalid nesting
    ]
    
    for gram in invalid_examples:
        try:
            parse_gram(gram)
            print(f'Unexpected success: "{gram}"')
        except ValueError as e:
            print(f'Expected error for "{gram}": {e}')


def example7_version():
    """Example 7: Version information"""
    print("\n=== Example 7: Version Information ===")
    print(f"Gram Codec version: {version()}")


def example8_filter_patterns():
    """Example 8: Filter patterns by identifier"""
    print("\n=== Example 8: Filter Patterns ===")
    
    gram = "(alice:Person) (bob:Person) (charlie:Person) (dave:Admin)"
    print(f"Input: {gram}")
    
    try:
        result = parse_gram(gram)
        print(f"Total patterns: {result['pattern_count']}")
        print(f"All identifiers: {result['identifiers']}")
        
        # Filter for specific identifiers
        target = "alice"
        if target in result['identifiers']:
            print(f"✓ Found pattern with identifier '{target}'")
        else:
            print(f"✗ No pattern with identifier '{target}'")
    except ValueError as e:
        print(f"Parse error: {e}")


def example9_batch_validation():
    """Example 9: Batch validation"""
    print("\n=== Example 9: Batch Validation ===")
    
    # Simulate reading multiple gram files
    gram_files = {
        "nodes.gram": "(alice) (bob) (charlie)",
        "relationships.gram": "(alice)-->(bob) (bob)-->(charlie)",
        "complex.gram": "[team | (alice), (bob)]",
        "invalid.gram": "(unclosed",
    }
    
    valid_count = 0
    invalid_count = 0
    
    for filename, content in gram_files.items():
        is_valid = validate_gram(content)
        status = "✓" if is_valid else "✗"
        print(f"{status} {filename}: {'valid' if is_valid else 'invalid'}")
        
        if is_valid:
            valid_count += 1
        else:
            invalid_count += 1
    
    print(f"\nSummary: {valid_count} valid, {invalid_count} invalid")


def example10_properties():
    """Example 10: Parse patterns with various property types"""
    print("\n=== Example 10: Property Types ===")
    
    examples = [
        '(a {name: "Alice"})',  # String
        "(a {age: 30})",  # Integer
        "(a {score: 95.5})",  # Decimal
        "(a {active: true})",  # Boolean
        '(a {tags: ["rust", "python"]})',  # Array
        "(a {range: 1..10})",  # Range
    ]
    
    for gram in examples:
        try:
            result = parse_gram(gram)
            print(f'✓ "{gram}"')
        except ValueError as e:
            print(f'✗ "{gram}" → {e}')


def main():
    """Run all examples"""
    print("Gram Codec Python Examples")
    print("===========================\n")
    
    # Uncomment after building the module
    # example1_parse()
    # example2_validate()
    # example3_roundtrip()
    # example4_multiple()
    # example5_complex()
    # example6_errors()
    # example7_version()
    # example8_filter_patterns()
    # example9_batch_validation()
    # example10_properties()
    
    print("\n=== Examples Complete ===")
    print("\nNote: Build the Python module first:")
    print("  pip install maturin")
    print("  cd crates/gram-codec")
    print("  maturin develop --features python")
    print("  python examples/gram_codec.py")


if __name__ == "__main__":
    # main()
    print("Gram Codec Python Example (template)")
    print("Build module first with: maturin develop --features python")

