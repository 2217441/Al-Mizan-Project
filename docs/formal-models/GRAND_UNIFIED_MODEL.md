# Al-Mizan: The Grand Unified Model (The Omni-Graph)

This diagram represents the **complete architectural teleology** of the Al-Mizan project, connecting all 25 Phases into a single unified system view.

---

## The Omni-Graph

```mermaid
flowchart TD
    %% ==========================================
    %% 1. THEOLOGICAL INPUTS (EPISTEMOLOGY)
    %% ==========================================
    subgraph V1 ["V1: Foundations (Epistemology)"]
        direction TB
        P1(Phase 1: Ecosystem Stack)
        P2[(Phase 2: SurrealDB Schema)]
        P3[[Phase 3: Istinbat Logic]]
        P5{{Phase 5: Trust Metric}}
        P11((Phase 11: Naskh Logic))
        
        Wahy[Divine Sources] --> P1
        P1 --> P2
        P2 --> P3
        P3 --> P5
        P3 --> P11
    end

    %% ==========================================
    %% 2. INTELLIGENCE & PROCESS (RUNTIME)
    %% ==========================================
    subgraph V2 ["V2: Intelligence (The Factory)"]
        direction TB
        P17[Phase 17: Ingestion Pipeline]
        P7>Phase 7: Vector Retrieval]
        P4{Phase 4: Synthesis Engine}
        P16((Phase 16: Agent Topology))
        P24[Phase 24: Multilingual Bridge]
        P23{Phase 23: Adl Bias Monitor}
        P18(Phase 18: API Gateway)

        P2 --> P17
        P17 --> P7
        P7 --> P4
        P3 --> P16
        P16 --> P4
        P4 --> P24
        P4 --> P23
        P4 --> P18
    end

    %% ==========================================
    %% 3. OPERATIONS & RESILIENCE (INFRA)
    %% ==========================================
    subgraph V3 ["V3: Operations (The Fortress)"]
        direction TB
        P14[Phase 14: HA Zones]
        P13((Phase 13: Federation))
        P19[Phase 19: Disaster Recovery]
        P20(Phase 20: Blue-Green Deploy)
        P12[Phase 12: Audit Merkle Log]
        P6[Phase 6: W3C Credentials]
        P8{Phase 8: Security RBAC}

        P18 --> P14
        P14 --> P13
        P14 -- Backup --> P19
        P20 --> P14
        P18 --> P8
        P8 --> P6
        P8 --> P12
    end

    %% ==========================================
    %% 4. GOVERNANCE & CONSENSUS (HUMAN)
    %% ==========================================
    subgraph V4 ["V4: Governance (The Shura)"]
        direction TB
        P9{Phase 9: Ijma Consensus}
        P10[Phase 10: Stake & Slash]
        P22((Phase 22: Scholar Onboarding))
        
        P5 --> P22
        P22 --> P9
        P9 --> P10
        P10 -- Updates --> P3
    end

    %% ==========================================
    %% 5. SOVEREIGNTY (THE COMPACT)
    %% ==========================================
    subgraph V5 ["V5: Sovereignty (The Compact)"]
        direction TB
        P25[Phase 25: TEE Enclaves]
        P21{{Phase 21: The Compact}}
        
        P8 --> P25
        P9 --> P25
        P25 --> P21
    end

    %% ==========================================
    %% CROSS-LINKS (THE MESH)
    %% ==========================================
    %% Intelligence -> Governance (The Adl Feedback)
    P23 -- "Bias Detected" --> P10
    P4 -. "Proposed Fatwa" .-> P3
    
    %% Governance -> Operations (The Control Plane)
    P9 -- "Consensus Reached" --> P12
    P10 -- "Slashing Event" --> P8
    
    %% Operations -> Intelligence (The Data Plane)
    P13 -. "Federated Knowledge" .-> P7
    P14 -- "Zone Failover" --> P16
    
    %% Sovereignty -> Foundations (The Root Power)
    P25 == "Encrypted State" ==> P2
    P21 -. "Compact Breach" .-> P10
    
    %% Foundations -> Intelligence (The Semantics)
    P11 -- "Naskh Logic" --> P4
    P5 -- "Trust Scores" --> P4

    classDef foundation fill:#f9f9fa,stroke:#333,stroke-width:2px;
    classDef intel fill:#e3f2fd,stroke:#1565c0,stroke-width:2px;
    classDef ops fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px;
    classDef gov fill:#fff3e0,stroke:#ef6c00,stroke-width:2px;
    classDef sov fill:#f3e5f5,stroke:#7b1fa2,stroke-width:2px;

    class P1,P2,P3,P5,P11 foundation;
    class P17,P7,P4,P16,P24,P23,P18 intel;
    class P14,P13,P19,P20,P12,P6,P8 ops;
    class P9,P10,P22 gov;
    class P25,P21 sov;
```

---

## Legend

| Zone | Focus | Key Tech |
| :--- | :--- | :--- |
| **V1: Foundations** | Epistemology | Wahy, Ontologies, SurrealDB |
| **V2: Intelligence** | Logic & AI | RAG, Agents, Vector Search |
| **V3: Operations** | Resilience | Geo-HA, DR, Merkle Logs |
| **V4: Governance** | Human Consensus | BFT, Staking, DID |
| **V5: Sovereignty** | Hardware Trust | SGX Enclaves, Legal Compacts |
