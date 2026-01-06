# Gram Codec WASM - Node.js Example

Node.js example demonstrating the gram-codec WASM bindings.

## Prerequisites

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Or via cargo
cargo install wasm-pack
```

## Building the WASM Module

```bash
# From the gram-rs root directory
cd crates/gram-codec

# Build for Node.js
wasm-pack build --target nodejs . -- --features wasm

# The output will be in crates/gram-codec/pkg/
```

## Installing and Running

```bash
# From gram-rs root
cd examples/gram-codec-wasm-node

# Install the WASM package
npm install ../../crates/gram-codec/pkg

# Run the example
node index.js

# Or use npm script
npm start
```

## What the Example Shows

The `index.js` example demonstrates:

1. **Parse Gram Notation** - Extract pattern count and identifiers
2. **Validate Syntax** - Quick validation of gram notation
3. **Round-Trip Testing** - Parse → serialize → parse validation
4. **Multiple Patterns** - Handling multiple patterns in one input
5. **Complex Patterns** - Various gram syntax forms
6. **Error Handling** - Catching and handling parse errors
7. **Version Information** - Getting codec version
8. **Batch Validation** - Validating multiple "files"

## API Usage in Node.js

```javascript
const { parse_gram, validate_gram, round_trip, version } = require('gram-codec');

// Parse gram notation
const result = parse_gram("(alice)-[:KNOWS]->(bob)");
console.log(`Patterns: ${result.pattern_count}`);
console.log(`Identifiers: ${result.identifiers}`);

// Validate
const isValid = validate_gram("(hello)");  // true

// Round-trip
const serialized = round_trip("(a)-->(b)");  // "(a)-->(b)"

// Version
console.log(version());  // "0.1.0"
```

## Using in Your Project

### Installation

```bash
# Install in your Node.js project
npm install /path/to/gram-rs/crates/gram-codec/pkg
```

### CommonJS

```javascript
const gramCodec = require('gram-codec');

const result = gramCodec.parse_gram("(hello)");
console.log(result);
```

### ES Modules

```javascript
import { parse_gram, validate_gram } from 'gram-codec';

const result = parse_gram("(hello)");
console.log(result);
```

## Common Patterns

### Read and Parse Files

```javascript
const fs = require('fs');
const { parse_gram } = require('gram-codec');

const gramFile = fs.readFileSync('data.gram', 'utf-8');
try {
    const result = parse_gram(gramFile);
    console.log(`Parsed ${result.pattern_count} patterns`);
} catch (e) {
    console.error('Parse error:', e);
}
```

### Batch Validation

```javascript
const { validate_gram } = require('gram-codec');
const glob = require('glob');
const fs = require('fs');

const files = glob.sync('**/*.gram');
const results = files.map(file => ({
    file,
    valid: validate_gram(fs.readFileSync(file, 'utf-8'))
}));

console.table(results);
```

### Stream Processing

```javascript
const { parse_gram } = require('gram-codec');
const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

rl.on('line', (line) => {
    try {
        const result = parse_gram(line);
        console.log(`✓ ${result.pattern_count} pattern(s)`);
    } catch (e) {
        console.log(`✗ Invalid: ${e.message}`);
    }
});
```

### Express.js API

```javascript
const express = require('express');
const { parse_gram, validate_gram } = require('gram-codec');

const app = express();
app.use(express.json());

app.post('/parse', (req, res) => {
    try {
        const result = parse_gram(req.body.gram);
        res.json({ success: true, result });
    } catch (e) {
        res.status(400).json({ success: false, error: e.message });
    }
});

app.post('/validate', (req, res) => {
    const isValid = validate_gram(req.body.gram);
    res.json({ valid: isValid });
});

app.listen(3000, () => console.log('Server running on port 3000'));
```

## Performance

WASM performance in Node.js:

- **Initialization**: ~10-20ms (one-time cost)
- **Parse**: Near-native speed (~95% of native Rust)
- **Memory**: Efficient, patterns are ephemeral
- **Throughput**: Suitable for high-volume processing

### Benchmarking

```javascript
const { parse_gram } = require('gram-codec');

const iterations = 10000;
const gram = "(alice)-[:KNOWS]->(bob)";

console.time('parse_gram');
for (let i = 0; i < iterations; i++) {
    parse_gram(gram);
}
console.timeEnd('parse_gram');
```

## Troubleshooting

### Module Not Found

If you get `Cannot find module 'gram-codec'`:

```bash
# Rebuild the WASM package
cd crates/gram-codec
wasm-pack build --target nodejs . -- --features wasm

# Reinstall in your project
cd examples/wasm-node
npm install ../../pkg
```

### WASM Loading Errors

If WASM fails to load:

```javascript
// Check Node.js version (requires 12+)
console.log(process.version);

// Verify WASM support
console.log(typeof WebAssembly);  // Should be 'object'
```

### Memory Errors

For large patterns:

```javascript
// Increase Node.js memory limit
node --max-old-space-size=4096 index.js
```

## TypeScript Support

The WASM package includes TypeScript definitions:

```typescript
import { parse_gram, validate_gram, round_trip, version } from 'gram-codec';

interface ParseResult {
    pattern_count: number;
    identifiers: string[];
}

const result: ParseResult = parse_gram("(hello)");
console.log(result.pattern_count);
```

## Testing

```javascript
const { parse_gram, validate_gram } = require('gram-codec');
const assert = require('assert');

describe('gram-codec', () => {
    it('should parse valid gram notation', () => {
        const result = parse_gram("(hello)");
        assert.strictEqual(result.pattern_count, 1);
    });
    
    it('should validate gram notation', () => {
        assert.strictEqual(validate_gram("(hello)"), true);
        assert.strictEqual(validate_gram("(unclosed"), false);
    });
    
    it('should handle round-trip', () => {
        const original = "(alice)-->(bob)";
        const serialized = round_trip(original);
        assert.strictEqual(serialized, original);
    });
});
```

## Integration Examples

### CLI Tool

```javascript
#!/usr/bin/env node
const { parse_gram, validate_gram } = require('gram-codec');
const fs = require('fs');

const [,, command, file] = process.argv;

if (command === 'parse' && file) {
    const content = fs.readFileSync(file, 'utf-8');
    try {
        const result = parse_gram(content);
        console.log(JSON.stringify(result, null, 2));
    } catch (e) {
        console.error('Error:', e.message);
        process.exit(1);
    }
} else if (command === 'validate' && file) {
    const content = fs.readFileSync(file, 'utf-8');
    const isValid = validate_gram(content);
    console.log(isValid ? 'Valid' : 'Invalid');
    process.exit(isValid ? 0 : 1);
} else {
    console.log('Usage: gram-tool parse|validate <file>');
}
```

## Next Steps

- See `../wasm-web/` for browser examples
- Check `../../README.md` for complete API reference
- Run benchmarks: `cargo bench --package gram-codec`
- Build for production: `wasm-pack build --target nodejs --release`

## Production Tips

1. **Cache the module**: Require once, reuse across requests
2. **Error handling**: Always wrap in try-catch for user input
3. **Validation first**: Validate before expensive parse operations
4. **Batch processing**: Process multiple patterns efficiently
5. **Memory management**: Node.js handles WASM memory automatically

