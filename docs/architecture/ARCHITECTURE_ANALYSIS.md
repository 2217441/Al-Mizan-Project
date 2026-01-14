# Architecture Analysis

## 1. System Context
**Project**: Al-Mizan
**Type**: Digital Islamic Library & Knowledge Graph
**Stage**: FYP (Final Year Project) / MVP
**Key Goals**: Accuracy (Tier 1 compliance), Performance (Rust), Visualization (Tawhidic Graph).

## 2. Component Analysis

### Backend (`almizan-core`)
- **Pattern**: Pragmatic Layered Architecture.
- **Compliance with ADR-0002 (Clean Architecture)**: **Partial**.
  - **Domain Layer**: Exists (`src/domain`, `src/enterprise`). Contains pure business logic.
  - **Application/Infra Layer**: API handlers often bypass the repo layer (`db.client.query`) for performance/simplicity.
- **Security Audit**:
  - **Headers/CORS**: 🔴 **CRITICAL FAIL**. `tower_http` dependencies exist in `Cargo.toml`, but `main.rs` does not attach `CorsLayer` or `TraceLayer`. API is unprotected against cross-origin attacks.
  - **Auth**: JWT implementation exists in `src/identity`, but needs verification of usage in handlers.

### Database (`almizan-db`)
- **Technology**: SurrealDB v2.x.
- **Schema**: Graph-Relational.
  - Nodes: `quran_verse`, `hadith`, `scholar`, `fiqh_ruling`.
  - Edges: `derived_from`, `narrated_by`, `issued_by`.
- **Scalability**: ✅ **PASS**.
  - `DEFINE INDEX ...` covers all graph traversal paths (Section 10 of schema).
  - `arabic_analyzer` is defined for text search.
- **Security**: ✅ **PASS**. Events (`DEFINE EVENT`) enforce "Scholar Slashing" and preventing "Mawdu" derivations at the DB level.

### ETL Pipeline (`almizan-etl`)
- **Role**: Implements "Sandbox Protocol" (ADR-0004).
- **Structure**:
  - `transform_tanzil.py`: Tier 1 (Immutable) ingestion.
  - `verify_ingestion.py`: Integrity verification.
- **Security**: Write-access limited to offline pipeline.

### Frontend (`almizan-web`)
- **Architecture**: Server-Side Rendering (SSR).
- **Core Tech**: Rust `askama` + Vanilla JS + `vis-network` (Graph).
- **Styling**: Modern CSS Variables.
- **Observations**:
  - `graph.html`: Visualizes reliability, but logic is duplicated from backend.

## 3. Analysis Findings

### ✅ Strengths
1.  **Type Safety**: Rust enforces strong constraints effectively.
2.  **Database Design**: Schema is well-optimized for graph traversals ($O(1)$) and enforcing business rules (Events).
3.  **Security (Data)**: "Scholar Slashing" and "Immutable Thabit Tier" are correctly enforced in DB.

### ⚠️ Gaps & Risks
1.  **API Security**: Missing CORS/Security Headers in `main.rs` exposes the app to browser-based attacks.
2.  **Fitna Defense Protocol**: Currently checks Scholar Status (Active/Slashed) but lacks the **recursive root-to-leaf verification** (Chain of Transmission validation).
3.  **Documentation Drift**: `ARCHITECTURE.md` incorrectly lists D3.js.

## 4. Strategic Recommendations
1.  **Immediate Fix (Security)**: Add `TraceLayer` and `CorsLayer` to `main.rs`.
2.  **Priority Feature (Architecture)**: Implement `verify_chain_recursive()` in `domain/graph.rs` (The "Fitna Defense").
3.  **Refactor**: Move raw SQL from `api/v1/verse.rs` to a dedicated `repository` module to improve testability.
