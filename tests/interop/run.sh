#!/usr/bin/env bash
# Cross-language interop test
#
# Parses the same gram fixture in Rust, TypeScript, and Python using the JSON
# interchange format, then verifies all three produce identical output.
#
# Requirements: cargo, node, python3 (with relateby installed), jq
#
# Usage:
#   ./tests/interop/run.sh
#   ./tests/interop/run.sh path/to/other.gram

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="${1:-$REPO_ROOT/tests/interop/quatrain.gram}"

# Prefer the local dev venv when it exists; fall back to system python3.
VENV_PYTHON="$REPO_ROOT/python/packages/relateby/.venv/bin/python"
if [[ -x "$VENV_PYTHON" ]]; then
  PYTHON="${PYTHON:-$VENV_PYTHON}"
else
  PYTHON="${PYTHON:-python3}"
fi

# In local dev (no installed wheel), point Python at the source tree so the
# pure-Python modules are importable. In CI the wheel is installed globally
# so PYTHONPATH is not needed (and would shadow the native .so files).
if [[ -z "${CI:-}" ]]; then
  export PYTHONPATH="$REPO_ROOT/python/packages/relateby${PYTHONPATH:+:$PYTHONPATH}"
fi
TMPDIR_LOCAL="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_LOCAL"' EXIT

normalize() {
  # Sort object keys and array items deterministically so label ordering
  # differences between languages don't cause false failures.
  jq --sort-keys '[.[] | walk(if type == "array" and (map(type) | unique == ["string"]) then sort else . end)]'
}

echo "==> Fixture: $FIXTURE"
echo ""

echo "--- Rust ---"
RS_JSON=$(cargo run -q -p relateby-gram --example interop_check 2>/dev/null < "$FIXTURE")
echo "$RS_JSON" | jq '.[0].subject' 2>/dev/null || echo "$RS_JSON"

echo ""
echo "--- TypeScript ---"
TS_JSON=$(node "$REPO_ROOT/tests/interop/ts_interop.mjs" < "$FIXTURE")
echo "$TS_JSON" | jq '.[0].subject' 2>/dev/null || echo "$TS_JSON"

echo ""
echo "--- Python ---"
PY_JSON=$("$PYTHON" "$REPO_ROOT/tests/interop/py_interop.py" < "$FIXTURE")
echo "$PY_JSON" | jq '.[0].subject' 2>/dev/null || echo "$PY_JSON"

echo ""
echo "==> Comparing..."

RS_NORM=$(echo "$RS_JSON" | normalize)
TS_NORM=$(echo "$TS_JSON" | normalize)
PY_NORM=$(echo "$PY_JSON" | normalize)

RS_FILE="$TMPDIR_LOCAL/rs.json"
TS_FILE="$TMPDIR_LOCAL/ts.json"
PY_FILE="$TMPDIR_LOCAL/py.json"

echo "$RS_NORM" > "$RS_FILE"
echo "$TS_NORM" > "$TS_FILE"
echo "$PY_NORM" > "$PY_FILE"

PASS=true

if ! diff -q "$RS_FILE" "$TS_FILE" > /dev/null; then
  echo "FAIL: Rust and TypeScript outputs differ"
  diff "$RS_FILE" "$TS_FILE" || true
  PASS=false
fi

if ! diff -q "$RS_FILE" "$PY_FILE" > /dev/null; then
  echo "FAIL: Rust and Python outputs differ"
  diff "$RS_FILE" "$PY_FILE" || true
  PASS=false
fi

if $PASS; then
  echo "PASS: Rust, TypeScript, and Python all agree on the interchange format"
else
  exit 1
fi
