# Architecture Decision Records Catalog

> **Author**: Technical Writing Agent  
> **Last Updated**: 2026-01-08  
> **Location**: `docs/adr/`

---

## Overview

Architecture Decision Records (ADRs) document significant technical decisions made during the Al-Mizan project. All ADRs follow the [MADR](https://adr.github.io/madr/) template format.

---

## ADR Index

| ID | Title | Status | Date | Key Decision |
|----|-------|--------|------|--------------|
| [0001](../adr/0001-tech-stack.md) | Core Technology Stack | ✅ Accepted | 2025-10-12 | SurrealDB + Rust (Axum) + Python ETL |
| [0002](../adr/0002-clean-architecture.md) | Clean Architecture Pattern | ✅ Accepted | 2025-11-26 | 4-layer architecture (Domain → Application → Infrastructure → Presentation) |
| [0003](../adr/0003-project-management.md) | Project Management Strategy | ✅ Accepted | 2025-12-05 | Agile + Domain-Driven Design |
| [0004](../adr/0004-data-verification-protocol.md) | Tiered Data Verification | ✅ Accepted | 2026-01-02 | Tier 1 (immutable) vs Tier 2 (semantic) verification |
| [0005](../adr/0005-public-api-security.md) | Public API Security | ✅ Accepted | 2026-01-02 | Read-only public API, admin-only writes |

---

## ADR Summaries

### ADR-0001: Core Technology Stack

**Decision**: Use **SurrealDB** as the database and **Rust (Axum)** for the backend API.

**Rationale**:
- SurrealDB provides O(1) graph traversal vs O(N²) SQL JOINs for Isnad chains
- Rust ensures memory safety and prevents data races in concurrent requests
- Python retained for ETL pipeline (pandas, langchain)

**Trade-offs**:
| Positive | Negative |
|----------|----------|
| High performance, low RAM | Higher Rust learning curve |
| Type safety at compile time | Team split between languages |
| Real-time Live Queries | Smaller SurrealDB ecosystem |

---

### ADR-0002: Clean Architecture Pattern

**Decision**: Adopt **Clean Architecture** (Onion/Hexagonal pattern).

**Layer Structure**:
```
┌─────────────────────────────────────┐
│  Presentation (API/Axum Handlers)   │
├─────────────────────────────────────┤
│  Application (Use Cases/Ports)      │
├─────────────────────────────────────┤
│  Infrastructure (DB/Adapters)       │
├─────────────────────────────────────┤
│  Domain (Entities/Business Rules)   │  ← Core (no dependencies)
└─────────────────────────────────────┘
```

**Rationale**:
- Core business logic (abrogation rules) is framework-independent
- Can swap database or API framework without touching domain logic
- Enables isolated unit testing with mocks

**Trade-offs**:
| Positive | Negative |
|----------|----------|
| Framework independence | More boilerplate code |
| High testability | DTO mapping overhead |
| Clear separation of concerns | Overkill for simple CRUD |

---

### ADR-0003: Project Management Strategy

**Decision**: Use **Agile (Scrum)** with **Domain-Driven Design (DDD)**.

**Key Elements**:
- **Ubiquitous Language**: Use Islamic terms in code (*Naskh*, *Mansukh*, *Nasikh*)
- **Bounded Contexts**: 
  - Text Context (Quran/Hadith integrity)
  - Abrogation Context (naskh relationships)
  - Identity Context (users/scholars)

**Roles**:
| Role | Responsibility |
|------|---------------|
| Product Owner | Defines "what" to build |
| Development Team | Defines "how" to build |
| Scrum Master | Facilitates process |

---

### ADR-0004: Tiered Data Verification Protocol

**Decision**: Implement a **Tiered Hybrid** verification strategy.

**The Two Tiers**:

| Tier | Scope | Verification | Status |
|------|-------|--------------|--------|
| **Tier 1** (Immutable) | Quran, Kutub Sittah | Checksum vs trusted sources | 100% ingested |
| **Tier 2** (Semantic) | Morphology, Cross-refs | Human audit required | Limited subsets |

**Rationale**:
- 100% accuracy required for theological texts (no LLM hallucinations allowed)
- Automated ingestion for text, human verification for semantic edges
- UI distinguishes verified edges (solid) from suggestions (dotted)

**Trade-offs**:
| Positive | Negative |
|----------|----------|
| Guaranteed Quran accuracy | Sparse graph initially |
| Safe growth strategy | Manual verification bottleneck |

---

### ADR-0005: Public API Security

**Decision**: Make the public API **strictly read-only** (GET only).

**Implementation**:
1. Public router (`/api/v1`) only registers `GET` handlers
2. SurrealDB user has `SELECT` permissions only
3. Write access reserved for admin via internal Docker network

**Rationale**:
- Prevents "Theological Data Injection" (fake hadiths, extremist views)
- API is authoritative source; cannot host unverified content
- Corrections submitted via GitHub Issues → verified → admin ETL ingestion

**Trade-offs**:
| Positive | Negative |
|----------|----------|
| Zero public data defacement | Users can't contribute directly |
| "Fitna Defense" enabled | Pull request model adds friction |

---

## Decision Status Legend

| Status | Meaning |
|--------|---------|
| ✅ **Accepted** | Decision currently in force |
| ⚠️ **Proposed** | Under review, not yet approved |
| ❌ **Deprecated** | Superseded or abandoned |

---

## How to Propose a New ADR

1. Copy the template from `docs/adr/0000-index.md` header
2. Create new file: `docs/adr/NNNN-short-title.md`
3. Fill in Context, Decision Drivers, Options, and Outcome
4. Submit PR for team review
5. Update index once accepted

---

## Cross-References

- [Architecture Overview](../ARCHITECTURE.md) - System design details
- [Glossary](./GLOSSARY.md) - Terminology definitions
- [API Reference](./API_REFERENCE.md) - Endpoint documentation

---

*This catalog is maintained by the Technical Writing Agent.*
