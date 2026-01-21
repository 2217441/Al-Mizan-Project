# Strategic Directives: Rust Engineering
> **Source**: Master Strategic Plan (2026-01-15)

## 1. "Compile-Time Theology"
*   **Directive**: Leverage Rust's Type System to enforce theological safety.
*   **Action**: Ensure `Result<Ruling, Error>` types strictly handle "Undefined" states. An unhandled theological state is a compile error.

## 2. Performance Benchmark
*   **Directive**: Prove the "Hybrid Graph" hypothesis.
*   **Action**: Execute **Phase 3 Benchmark**:
    *   Compare `al-mizan` (SurrealDB) vs `postgres` (Recursive CTE).
    *   Task: Traverse a 10-depth Isnad chain for 10,000 nodes.
    *   Success Metric: Al-Mizan must be 10x faster or significantly more memory efficient.

## 3. The Digital Ark (Resilience)
*   **Directive**: Enable "Zero-Dependency Operation".
*   **Action**:
    *   Investigate `cross-compilation` targets for `aarch64` (Raspberry Pi) and `x86_64` (Legacy Laptops).
    *   Goal: Produce a single static binary (`almizan-core`) that includes the database engine (embedded) and the UI, requiring no external cloud services.
