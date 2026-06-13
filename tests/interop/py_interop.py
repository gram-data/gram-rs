#!/usr/bin/env python3
"""Python interop check: read gram from stdin, write JSON interchange to stdout.
Used by tests/interop/run.sh — must produce identical JSON to rs_interop and ts_interop.
"""
import json
import sys

lines = [l for l in sys.stdin if not l.lstrip().startswith("//")]
gram_input = "".join(lines)

try:
    from relateby.gram import parse_raw
    raw = parse_raw(gram_input)
    print(json.dumps(raw))
except Exception as e:
    sys.stderr.write(f"parse error: {e}\n")
    sys.exit(1)
