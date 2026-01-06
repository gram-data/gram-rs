// Example usage of gram-codec WASM bindings in JavaScript
// 
// To build WASM module:
//   wasm-pack build --target web crates/gram-codec -- --features wasm
//
// To run this example:
//   npm install
//   node examples/gram_codec_wasm.js

// Import WASM module (adjust path based on your build output)
// import init, { parse_gram, validate_gram, round_trip, version } from '../pkg/gram_codec.js';

// Example 1: Parse gram notation
async function example1_parse() {
    console.log("=== Example 1: Parse Gram Notation ===");
    
    try {
        const result = parse_gram("(alice)-[:KNOWS]->(bob)");
        console.log(`Pattern count: ${result.pattern_count}`);
        console.log(`Identifiers: ${result.identifiers}`);
    } catch (e) {
        console.error(`Parse error: ${e}`);
    }
}

// Example 2: Validate gram notation
async function example2_validate() {
    console.log("\n=== Example 2: Validate Gram Notation ===");
    
    const examples = [
        "(hello)",
        "(a)-->(b)",
        "[team | alice, bob]",
        "(unclosed",  // Invalid
        "{missing_parens}",  // Invalid
    ];
    
    for (const gram of examples) {
        const isValid = validate_gram(gram);
        console.log(`"${gram}" is ${isValid ? "valid" : "invalid"}`);
    }
}

// Example 3: Round-trip test
async function example3_roundtrip() {
    console.log("\n=== Example 3: Round-Trip Test ===");
    
    const original = "(alice:Person {name: \"Alice\"})-[:KNOWS]->(bob:Person {name: \"Bob\"})";
    console.log(`Original:   ${original}`);
    
    try {
        const serialized = round_trip(original);
        console.log(`Serialized: ${serialized}`);
        
        // Verify it's still valid
        const isValid = validate_gram(serialized);
        console.log(`Still valid: ${isValid}`);
    } catch (e) {
        console.error(`Round-trip error: ${e}`);
    }
}

// Example 4: Parse multiple patterns
async function example4_multiple() {
    console.log("\n=== Example 4: Multiple Patterns ===");
    
    const gram = "(alice) (bob) (charlie)";
    console.log(`Input: ${gram}`);
    
    try {
        const result = parse_gram(gram);
        console.log(`Parsed ${result.pattern_count} patterns`);
        console.log(`Identifiers: ${result.identifiers.join(", ")}`);
    } catch (e) {
        console.error(`Parse error: ${e}`);
    }
}

// Example 5: Parse complex patterns
async function example5_complex() {
    console.log("\n=== Example 5: Complex Patterns ===");
    
    const examples = [
        // Node with label
        "(a:Person)",
        // Node with properties
        "(a {name: \"Alice\", age: 30})",
        // Relationship with label
        "(a)-[:KNOWS]->(b)",
        // Subject pattern
        "[team:Team {name: \"DevRel\"} | (alice), (bob), (charlie)]",
        // Nested pattern
        "[outer | [inner | (leaf)]]",
        // Annotated pattern
        "@type(node) (a)",
    ];
    
    for (const gram of examples) {
        try {
            const result = parse_gram(gram);
            console.log(`✓ "${gram}" → ${result.pattern_count} pattern(s)`);
        } catch (e) {
            console.log(`✗ "${gram}" → Error: ${e}`);
        }
    }
}

// Example 6: Error handling
async function example6_errors() {
    console.log("\n=== Example 6: Error Handling ===");
    
    const invalidExamples = [
        "(unclosed",
        "{no_parens}",
        "(a {key: })",  // Empty property value
        "((nested))",   // Invalid nesting
    ];
    
    for (const gram of invalidExamples) {
        try {
            parse_gram(gram);
            console.log(`Unexpected success: "${gram}"`);
        } catch (e) {
            console.log(`Expected error for "${gram}": ${e}`);
        }
    }
}

// Example 7: Version information
async function example7_version() {
    console.log("\n=== Example 7: Version Information ===");
    console.log(`Gram Codec version: ${version()}`);
}

// Main function to run all examples
async function main() {
    console.log("Gram Codec WASM Examples");
    console.log("========================\n");
    
    // Initialize WASM module
    // await init();
    
    // Run examples
    // await example1_parse();
    // await example2_validate();
    // await example3_roundtrip();
    // await example4_multiple();
    // await example5_complex();
    // await example6_errors();
    // await example7_version();
    
    console.log("\n=== Examples Complete ===");
    console.log("\nNote: Uncomment the example calls after building the WASM module:");
    console.log("  wasm-pack build --target web crates/gram-codec -- --features wasm");
}

// Run if executed as main module
// if (import.meta.url === `file://${process.argv[1]}`) {
//     main().catch(console.error);
// }

// Export for use as module
// export { main };

console.log("Gram Codec WASM Example (template)");
console.log("Build WASM first with: wasm-pack build --target web crates/gram-codec -- --features wasm");

