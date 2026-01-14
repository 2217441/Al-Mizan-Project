# Architecture Runbooks

## Architecture Review Process
1. **Context Gathering**: Read `docs/ARCHITECTURE.md` and related source code.
2. **Analysis**: Use `ARCHITECTURE_ANALYSIS.md` to document findings.
3. **Recommendation**: Create/Update ADRs in `docs/adr/` if significant changes are proposed.
4. **Verification**: Verify implementation against architectural guidelines.

## Creating an ADR
1. Use the template in `docs/adr/template.md` (if exists) or standard ADR format.
2. Place in `docs/adr/XXXX-title.md`.
3. Update `docs/adr/README.md` index.

## System Health Check
1. Review `cargo clippy` output for code health.
2. Check `surrealdb` query performance.
