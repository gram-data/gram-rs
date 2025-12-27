# CI/CD Pipeline Configuration Contract

**Feature**: 002-workspace-setup  
**Date**: 2025-01-27

## Overview

This contract defines the structure and requirements for the GitHub Actions CI/CD pipeline configuration.

## Pipeline Structure

### File Location

```
.github/workflows/ci.yml
```

### Workflow Triggers

```yaml
on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]
```

**Requirements**:
- MUST run on all pushes (FR-015)
- MUST run on all pull requests (FR-015)
- SHOULD run on main and develop branches (configurable)

### Job Organization

The pipeline MUST include separate jobs for:
1. **Build**: Compile all workspace crates
2. **Test**: Run all workspace tests
3. **Lint**: Run clippy checks
4. **Format**: Verify code formatting

**Rationale**: Separate jobs enable parallel execution and clear failure reporting (FR-016).

### Build Job

```yaml
build:
  name: Build Workspace
  runs-on: ubuntu-latest
  strategy:
    matrix:
      target: [x86_64-unknown-linux-gnu, wasm32-unknown-unknown]
  steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: ${{ matrix.target }}
    - uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    - run: cargo build --workspace --target ${{ matrix.target }}
```

**Requirements**:
- MUST build all workspace crates (FR-014)
- MUST test both native and WASM targets (SC-003)
- MUST use caching to reduce build time
- MUST complete within time constraints (SC-004)

### Test Job

```yaml
test:
  name: Test Workspace
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    - run: cargo test --workspace
```

**Requirements**:
- MUST run all workspace tests (FR-014)
- MUST report failures with crate identification (FR-016)
- MUST complete within time constraints (SC-004)

### Lint Job

```yaml
lint:
  name: Lint Workspace
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: clippy
    - uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    - run: cargo clippy --workspace -- -D warnings
```

**Requirements**:
- MUST run clippy on all workspace crates
- MUST fail on warnings (strict mode)
- MUST report failures with crate identification (FR-016)

### Format Job

```yaml
format:
  name: Check Formatting
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt
    - run: cargo fmt --all -- --check
```

**Requirements**:
- MUST verify formatting for all workspace crates
- MUST fail if code is not formatted
- MUST report failures clearly (FR-016)

## Failure Reporting

**Requirements** (FR-016):
- Build failures MUST identify which crate failed
- Test failures MUST include crate name in output
- Lint/format failures MUST specify file and crate location
- Error messages MUST be actionable

## Performance Requirements

**Requirements** (SC-004):
- Full pipeline MUST complete within 10 minutes
- Individual jobs SHOULD complete within 5 minutes
- Caching MUST be used to reduce build time

## Example: Complete CI Configuration

```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

jobs:
  build:
    name: Build Workspace
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target: [x86_64-unknown-linux-gnu, wasm32-unknown-unknown]
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
      - uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      - run: cargo build --workspace --target ${{ matrix.target }}

  test:
    name: Test Workspace
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      - run: cargo test --workspace

  lint:
    name: Lint Workspace
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: clippy
      - uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      - run: cargo clippy --workspace -- -D warnings

  format:
    name: Check Formatting
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt
      - run: cargo fmt --all -- --check
```

