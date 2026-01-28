# System Landscape: Al-Mizan

> **Author**: System Architect Agent
> **Date**: 2026-01-28
> **Status**: Active
> **SFIA Alignment**: ARCH Level 6 (Initiate, Influence)

---

## 1. Architectural Vision
To construct a **Sovereign, Graph-Based Islamic Knowledge Engine** that guarantees:
1.  **Traceability**: Every node (Fact) links to a Root Authority (Verse/Hadith).
2.  **Immutability**: Tier 1 data (Quran) is read-only and cryptographically verified.
3.  **Federation**: The graph can sync across disjoint nodes (Offline-First).

## 2. Solution Components (The "Stack")

### Core Engine (The "Brain")
*   **Database**: SurrealDB (Multi-model: Graph + Document).
*   **Language**: Rust (Zero-cost abstractions, Memory safety).
*   **Query Layer**: GraphQL (Federated) + SurrealQL.

### Integration (The "Nervous System")
*   **ETL Pipeline**: Python-driven ingestion (Crawl4AI -> JSON -> Surreal).
*   **Agent Interface**: `mcp-server` (Model Context Protocol) for AI interaction.
*   **SFIA Integration**: Agents consume `data/sfia` directly (Markdown-First RAG).

## 3. Key Constraints & Standards
*   **Zero-Hallucination**: The UI must flag any data points *derived* by AI but not *verified* by a Scholar Node.
*   **Performance**: Graph traversals must complete < 50ms for depth-3 queries.
*   **Offline Capability**: The full dataset (~2GB) must be packable into a single SQLite/SurrealDB binary file.

## 4. Decision Log (ADR)
All major technical shifts are recorded in `docs/adr/`.
*   [0001-tech-stack](file:///home/a/code/al-mizan-project/docs/adr/0001-tech-stack.md)
*   [0002-graph-db-selection](file:///home/a/code/al-mizan-project/docs/adr/0002-graph-db-selection.md)

## 5. Strategic Gaps (Attention Needed)
*   **Federation Protocol**: We lack a formal spec for ActivityPub extensions ("IslamicGraphPub").
*   **Security Model**: RBAC for "Scholar" vs "Student" needs formal definition in `docs/security/`.
