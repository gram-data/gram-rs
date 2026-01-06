# Gram Codec Python Examples

Python examples demonstrating the gram-codec library usage.

## Prerequisites

```bash
# Install maturin (Python package builder for Rust)
pip install maturin
```

## Building the Python Module

```bash
# From the gram-rs root directory
cd crates/gram-codec

# Build and install in development mode
maturin develop --features python

# Or build a wheel for distribution
maturin build --release --features python
```

## Examples

### 1. Quick Start (`quickstart.py`)

Basic usage demonstrating the core functions:

```bash
python examples/gram-codec-python/quickstart.py
```

**What it shows:**
- Parse gram notation
- Validate syntax
- Round-trip testing
- Error handling

### 2. Interactive Demo (`demo.py`)

Comprehensive interactive demo with multiple examples:

```bash
python examples/gram-codec-python/demo.py
```

**What it shows:**
- Parse various gram notation forms
- Batch validation
- Complex patterns
- Interactive REPL mode
- Error handling

### 3. Original Template (`gram_codec.py`)

Extended template with 10 example functions (requires uncommenting):

```bash
python examples/gram-codec-python/gram_codec.py
```

## API Reference

### `parse_gram(input: str) -> dict`

Parse gram notation and return pattern information.

```python
from gram_codec import parse_gram

result = parse_gram("(alice)-[:KNOWS]->(bob)")
print(result['pattern_count'])  # 1
print(result['identifiers'])    # ['', 'alice', 'bob']
```

**Returns:** Dictionary with keys:
- `pattern_count`: Number of patterns parsed
- `identifiers`: List of root pattern identifiers

**Raises:** `ValueError` if parsing fails

### `validate_gram(input: str) -> bool`

Quick validation check without parsing details.

```python
from gram_codec import validate_gram

is_valid = validate_gram("(hello)")  # True
is_invalid = validate_gram("(unclosed")  # False
```

**Returns:** `True` if valid, `False` if invalid

### `round_trip(input: str) -> str`

Parse and serialize back to gram notation.

```python
from gram_codec import round_trip

original = "(alice)-->(bob)"
serialized = round_trip(original)  # "(alice)-->(bob)"
```

**Returns:** Serialized gram notation

**Raises:** `ValueError` if parsing or serialization fails

### `version() -> str`

Get the codec version.

```python
from gram_codec import version

print(version())  # "0.1.0"
```

## Common Patterns

### Validate Multiple Files

```python
from gram_codec import validate_gram
import glob

gram_files = glob.glob("data/*.gram")
for file_path in gram_files:
    with open(file_path) as f:
        content = f.read()
        if validate_gram(content):
            print(f"✓ {file_path}")
        else:
            print(f"✗ {file_path}")
```

### Parse and Extract Information

```python
from gram_codec import parse_gram

gram = "[team:Team | (alice), (bob), (charlie)]"
result = parse_gram(gram)

print(f"Team: {result['identifiers'][0]}")
print(f"Member count: {result['pattern_count'] - 1}")
```

### Error Handling

```python
from gram_codec import parse_gram

try:
    result = parse_gram(user_input)
    # Process valid gram notation
    process(result)
except ValueError as e:
    print(f"Invalid gram notation: {e}")
    # Handle error
```

## Troubleshooting

### Module not found

If you get `ModuleNotFoundError: No module named 'gram_codec'`:

```bash
# Make sure you're in the right directory
cd crates/gram-codec

# Rebuild and install
maturin develop --features python
```

### Building on macOS

If you encounter signing issues on macOS:

```bash
# Add this to your shell profile
export MACOSX_DEPLOYMENT_TARGET=10.9
```

### Virtual Environment

To install in a virtual environment:

```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install maturin
cd crates/gram-codec
maturin develop --features python
```

## Performance

The Python bindings use PyO3 for efficient Rust↔Python interop. Performance characteristics:

- Parsing: O(n) where n is input length
- Validation: Same as parsing but returns early on error
- Round-trip: Combined parse + serialize cost

For batch processing, consider validating first before parsing to avoid expensive parse operations on invalid input.

## Next Steps

- See `crates/gram-codec/README.md` for the full Rust API
- Check `specs/019-gram-codec/` for complete specification
- Run benchmarks: `cargo bench --package gram-codec`
