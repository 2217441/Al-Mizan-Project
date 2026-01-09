# Rust Engineering Runbooks

**Last Updated**: 2026-01-07

---

## Development Environment

### Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup component add clippy rustfmt

# Optional: Additional tools
cargo install cargo-watch cargo-audit cargo-criterion
```

### Build & Run

```bash
cd almizan-core

# Development build
cargo build

# Run with hot reload
cargo watch -x run

# Production build
cargo build --release
```

---

## Quality Checks

### Linting

```bash
# Standard clippy
cargo clippy

# Pedantic mode (strict)
cargo clippy -- -W clippy::pedantic

# Fix automatically
cargo clippy --fix
```

### Formatting

```bash
# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt
```

### Testing

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific module
cargo test api::
```

---

## Debugging

### Logging

```bash
# Enable debug logs
RUST_LOG=debug cargo run

# Trace level for specific module
RUST_LOG=almizan_core::api=trace cargo run
```

### Performance

```bash
# Flamegraph (requires installation)
cargo flamegraph

# Benchmarks
cargo bench
```

---

## Common Issues

### SurrealDB Connection

**Symptom**: Connection refused on startup

**Resolution**:
1. Verify SurrealDB is running: `surreal start`
2. Check `.env` for correct `DATABASE_URL`
3. Verify namespace/database exist

### Serialization Errors

**Symptom**: Failed to deserialize from SurrealDB

**Resolution**:
1. Check struct fields match DB schema
2. Verify `Option<T>` for nullable fields
3. Add `#[serde(default)]` where appropriate

---

## Emergency Procedures

### Hot Fix Deployment

1. Create fix branch: `git checkout -b hotfix/issue-X`
2. Make minimal change
3. Run full test suite: `cargo test`
4. Build release: `cargo build --release`
5. Follow deployment runbook

### Rollback

```bash
# Revert to previous working binary
# (Coordinate with deployment-engineering)
```

---

*Maintained by: Rust Engineer Agent*
