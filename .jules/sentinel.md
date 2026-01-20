## 2024-10-24 - SQL Injection in SurrealDB Queries
**Vulnerability:** Found multiple instances of `format!` used to construct SurrealDB SQL queries with user input directly embedded.
**Learning:** Even with modern DBs like SurrealDB, raw query construction via string interpolation is vulnerable if not using the client's binding features.
**Prevention:** Always use `.bind((name, value))` and parameterized queries (e.g., ``) instead of `format!`.

## 2024-05-21 - Timing Attack in Admin Dashboard
**Vulnerability:** The admin dashboard used standard string equality (`==`) to verify the authentication token, allowing potential timing attacks to deduce the secret.
**Learning:** Even "internal" or "mock" auth checks in high-level handlers must use constant-time comparison to prevent side-channel leaks.
**Prevention:** Use a `constant_time_eq` helper for all token/secret comparisons, or a library like `subtle`.
