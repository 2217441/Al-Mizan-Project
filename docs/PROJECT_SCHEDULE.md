# Al-Mizan: Project Roadmap & Schedule

This document visualizes the execution timeline of the 25-Phase Master Plan using a Mermaid Gantt Chart.

## Global Execution Roadmap (2026)

```mermaid
gantt
    title Al-Mizan Sovereign Roadmap
    dateFormat  YYYY-MM-DD
    axisFormat  %b %Y
    
    section V1: Foundations
    Phase 1 (Ecosystem Stack)       :done,    p1, 2026-01-01, 7d
    Phase 2 (SurrealDB Schema)      :done,    p2, after p1, 7d
    Phase 3 (Istinbat Logic)        :active,  p3, after p2, 14d
    Phase 5 (Trust Metric Algebra)  :         p5, after p3, 7d
    Phase 11 (Abrogation/Naskh)     :         p11, after p5, 7d

    section V2: Intelligence
    Phase 7 (Retrieval/RAG)         :done,    p7, 2026-01-15, 14d
    Phase 4 (Synthesis Engine)      :active,  p4, after p7, 21d
    Phase 16 (Agent Topology)       :         p16, after p4, 21d
    Phase 17 (Ingestion Pipeline)   :         p17, after p4, 14d
    Phase 18 (API Tiers)            :         p18, after p16, 14d
    Phase 23 (Bias/Adl Monitor)     :         p23, after p18, 21d
    Phase 24 (Multilingual Bridge)  :         p24, after p23, 21d

    section V3: Operations
    Phase 6 (W3C Credentials)       :         p6, 2026-03-01, 14d
    Phase 8 (Auth & Scope)          :         p8, after p6, 14d
    Phase 12 (Audit Trails)         :         p12, after p8, 14d
    Phase 13 (Federation)           :         p13, after p12, 21d
    Phase 14 (HA Zones)             :         p14, after p13, 14d
    Phase 19 (Disaster Recovery)    :         p19, after p14, 14d
    Phase 20 (Blue-Green Deploy)    :         p20, after p19, 14d

    section V4: Governance
    Phase 9 (Consensus/Ijma)        :         p9, 2026-04-15, 30d
    Phase 10 (Stake & Slash)        :         p10, after p9, 21d
    Phase 22 (Scholar Onboarding)   :         p22, after p10, 14d

    section V5: Sovereignty
    Phase 21 (The Compact)          :active,  p21, 2026-06-01, 7d
    Phase 25 (Sov. Compute/TEE)     :         p25, after p21, 30d
```

## Status Legend
| Color | Meaning |
| :--- | :--- |
| **Grey (Done)** | Completed & Validated (e.g., Phase 1, 2, 7). |
| **Blue (Active)** | Currently under active development (e.g., Phase 4, 21). |
| **Pink (Future)** | Planned for future sprints. |
