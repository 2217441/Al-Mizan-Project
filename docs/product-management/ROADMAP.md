# Product Roadmap: Al-Mizan

## Mission
To build a semantic knowledge graph that immunizes Islamic knowledge against misinformation by tracing every concept back to its source (Isnad).

## Legend
- 🟢 **Live**: Verified and operational.
- 🟡 **In Progress**: Active development or optimization.
- 🔴 **Planned**: Scheduled for future phases.

---

## Phase 1: Foundation (Completed)
**Goal:** Prove the Hypothesis (Graph > Relational Isnad)
> **Status**: 🟢 Core Architecture Live
- [x] **Core Ontology**: Define Thabit vs Zanni nodes.
- [x] **ETL Pipeline**: Ingest Tanzil/Sunnah.com data.
- [x] **API v1**: Read-only Graph Traversal.
- [x] **Architecture**: Dockerized SOA with Rust/SurrealDB.

## Phase 1b: Optimization & Rescue (URGENT)
**Goal:** Stabilize Foundation & Rescue Frontend
> **Timeline**: Immediate (Next 2 Sprints)
- [ ] **Data Ingestion Reliability**: Fix ETL pipeline flakiness.
- [ ] **Frontend Rescue**:
    - [ ] Audit current UI state.
    - [ ] Implement visual overhaul requirements.
- [ ] **Documentation**: Consolidate fragmented roadmaps.

## Phase 2: Cyborg Isnad (Next Focus)
**Goal:** Scalable Verification & Ingestion
> **Timeline**: Q1-Q2 2026
- [ ] **AI-Assisted Extraction**:
    - [ ] OCR pipeline for scanned PDF books (Classical Fiqh).
    - [ ] LLM Agent for edge drafting (Concept extraction).
- [ ] **Verification Interface**:
    - [ ] "Tinder for Hadith" UI (Swipe to Verify Edge).
    - [ ] Scholar "God Mode" dashboard.
- [ ] **Gamification System**:
    - [ ] Reputation system for student verifiers.
    - [ ] "Micro-Waqf" contribution tracking.

## Phase 3: Analysis & Ecosystem (Future)
**Goal:** Commercialization & Advanced Insight
> **Timeline**: Q3 2026+
- [ ] **API Monetization**:
    - [ ] API Keys & Rate Limiting tiers.
    - [ ] SDKs for Python/JS.
- [ ] **Advanced Analytics**:
    - [ ] "Contradiction Detection" algorithms.
    - [ ] Automated Isnad Grading (Algorithmically checked against validation).
- [ ] **Mobile App**: Consumer-facing "Answer Engine".
