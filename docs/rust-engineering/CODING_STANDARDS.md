# Rust Coding Standards: Al-Mizan

> **Author**: Rust Engineer Agent
> **Date**: 2026-01-28
> **SFIA Alignment**: PROG Level 5 (Sets standards)

---

## 1. Safety & Unsafe Policy
*   **Default**: `#![forbid(unsafe_code)]` in all business logic crates.
*   **Exception**: FFI and Low-level hardware access (Tier 3 Only).
*   **Requirement**: Every `unsafe` block must have a `// SAFETY:` comment explaining invariants.
*   **Verification**: All unsafe code must pass `cargo miri test`.

## 2. Error Handling
*   **Application**: Use `anyhow::Result` for top-level binaries.
*   **Libraries**: Use `thiserror` for library-level errors.
*   **Panics**: `unwrap()` and `expect()` are **FORBIDDEN** in production code. Use `?` propagation or `unwrap_or_else`.

## 3. Async Runtime
*   **Standard**: `tokio` (latest stable).
*   **Guidelines**:
    *   Avoid blocking threads in async context.
    *   Use `tokio::select!` for cancellation safety.
    *   Prefer `Auditor` pattern for shared state over `Arc<Mutex<T>>` where possible.

## 4. Performance
*   **Allocation**: Minimize heap allocation in hot loops. Use `SmallVec` or `Cow`.
*   **Zero-Copy**: Prefer referencing data slices over cloning (`&str` vs `String`).
