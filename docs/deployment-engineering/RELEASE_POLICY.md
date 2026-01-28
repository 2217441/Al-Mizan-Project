# Release Policy: Al-Mizan

> **Author**: Deployment Engineer Agent
> **Date**: 2026-01-28
> **SFIA Alignment**: RELM Level 5 (Develops implementation plans)

---

## 1. Versioning Strategy
We follow **Semantic Versioning (SemVer 2.0.0)**:
*   `MAJOR`: Incompatible API/Ontology changes.
*   `MINOR`: New features (backward compatible).
*   `PATCH`: Bug fixes / Data corrections.

## 2. Release Gates
1.  **QA Sign-off**: All Level 1 tests passed (See `QA_ANALYSIS.md`).
2.  **Security Scan**: `cargo audit` clean.
3.  **Database Migration**: Idempotent SQL scripts verified in Staging.

## 3. Rollback Procedure
*   **Strategy**: "Blue-Green" Deployment (Phase 3 goal) -> Currently "Atomic Swap" for binary.
*   **Trigger**: API Error Rate > 1% or P99 Latency > 200ms.
*   **Action**: Revert `systemd` service to previous binary symlink and restart.

## 4. Environment Promotion
`Local` -> `Staging` (Automated) -> `Production` (Manual Approval).
