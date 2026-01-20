## 2024-10-24 - SQL Injection in SurrealDB Queries
**Vulnerability:** Found multiple instances of `format!` used to construct SurrealDB SQL queries with user input directly embedded.
**Learning:** Even with modern DBs like SurrealDB, raw query construction via string interpolation is vulnerable if not using the client's binding features.
**Prevention:** Always use `.bind((name, value))` and parameterized queries (e.g., ``) instead of `format!`.

## 2026-01-20 - Timing Attack in Token Verification
**Vulnerability:** Found variable-time string comparison (`==`) used for checking sensitive auth tokens in `dashboard.rs`.
**Learning:** Standard equality operators short-circuit, allowing attackers to guess tokens via timing analysis.
**Prevention:** Use constant-time comparison (XOR accumulation) for all secret comparisons.
