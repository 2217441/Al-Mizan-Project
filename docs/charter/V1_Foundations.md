# Al-Mizan Charter: V1 - Foundations (The Epistemological Engine)

This volume specifies the core epistemological layers, data structures, and foundational logic that ensure the integrity of the Al-Mizan system.

---

## Phase 1: High-Level Ecosystem (The Tawhidic Stack)

> **Status:** 🟡 `DEV` | **Implemented in:** [`almizan-core`](file:///home/a/code/al-mizan-project/almizan-core)

A non-negotiable zero-trust architecture optimized for epistemological integrity.

```mermaid
graph TB
    subgraph ZONE_SOURCES [1. Epistemological Foundations]
        direction LR
        S1["Primary (Wahy)"]
        S2["Secondary (Aql)"]
        S3["Linguistic (Lughah)"]
    end

    subgraph ZONE_ETL [2. Neuro-Symbolic Pipeline]
        direction TB
        E1["ETL (Normalisation)"]
        E2["Semantic Drafting (Ollama)"]
        E3["Integrity Check (SHACL/OWL)"]
    end

    subgraph ZONE_CORE ["3. The Tawhidic Stack (Persistence)"]
        direction LR
        subgraph T1 [Tier 1: Thabit - Immutable]
            Q[Quran/Sahih]
        end
        subgraph T2 ["Tier 2: Zanni - Analytic"]
            F[Fiqh / Ijma]
        end
        subgraph T3 ["Tier 3: Contextual - Support"]
            C[Bio / Lng / History]
        end
        DB[(SurrealDB Graph)]
    end

    subgraph ZONE_DELIVERY [4. Authoritative Delivery]
        API["Axum Gateway (Rust)"]
        Guard["Ethical Guardrails (Adl/Rahma)"]
        D3["Graph Visualization UI"]
    end

    S1 & S2 & S3 --> E1
    E1 & E2 & E3 --> Q & F & C
    DB <--> API
    API <--> Guard
    API --> D3

    classDef zone fill:#ffffff,stroke:#000,stroke-width:2px;
    class ZONE_SOURCES,ZONE_ETL,ZONE_CORE,ZONE_DELIVERY zone;
    classDef t1Style fill:#f2f2f2,stroke:#333,stroke-dasharray: 5 5;
    class T1 t1Style;
    classDef guardStyle fill:#fff5f5,stroke:#e53e3e,stroke-width:2px;
    class Guard guardStyle;
```

---

## Phase 2: Physical SurrealDB Schema (Definitive ERD)

> **Status:** 🟢 `PROD` | **Implemented in:** [`database/schema`](file:///home/a/code/al-mizan-project/database/schema)

Strict `SCHEMAFULL` enforcement of nodes and relations with specific theological primitives.

```mermaid
erDiagram
    %% GLOBAL STANDARD: schema.org/CreativeWork extensions
    %% ENCODING: Unicode UTF-8 (U+0600-06FF)
    
    allah ||--o{ quran_verse : "REVEALS (Primary)"
    allah ||--o{ hadith : "REVEALS (Qudsi)"
    scholar ||--o{ fiqh_ruling : "DERIVES (Ijtihad)"
    fiqh_ruling }o--o{ quran_verse : "EVIDENCE (Dalil)"
    fiqh_ruling }o--o{ hadith : "EVIDENCE (Sunnah)"
    hadith ||--o{ quran_verse : "EXPLAINS (Tafsir)"
    quran_verse ||--o{ quran_verse : "ABROGATES (Naskh)"
    ijma }o--o{ scholar : "CONSENSUS (Unanimous)"
    
    allah {
        string id PK "allah:tawhid (Immutable)"
        string name "Unicode: 99 Names"
    }

    quran_verse {
        string id PK "verse:1:1"
        int surah "ISO: Surah Code"
        int ayah "ISO: Ayah Code"
        string text "UTF-8: Uthmani Script"
        string abstract "schema.org/abstract"
    }

    hadith {
        string id PK "hadith:id"
        string text "UTF-8: Matn"
        string collection "schema.org/provider"
        string grade "Sahih | Hasan | .."
    }

    scholar {
        string id PK "scholar:name"
        float reputation "BFT Score (0.0-10.0)"
        bool ban "Sovereign Lockdown Status"
    }

    fiqh_ruling {
        string id PK "ruling:uuid"
        string hukm "Wajib | Haram | Mubah | .."
        string status "probationary | canonical | suspended"
        datetime datePublished "ISO 8601"
    }
```

---

## Phase 3: Istinbat (Legal Derivation) & Mawdu Defense

> **Status:** 🟡 `DEV` | **Implemented in:** [`domain/validation.rs`](file:///home/a/code/al-mizan-project/almizan-core/src/domain/validation.rs)

The strict logical flow for deriving rulings, including automated evidence validation.

```mermaid
flowchart TD
    %% GLOBAL STANDARD: Deontic Logic (Wajib | Haram | Mubah)
    %% VALIDATION: LegalBench / LEXam Reasoning Chains

    Input([Legal Query / Mas'ala]) --> Knowledge[Retrieve Relevant Wahy]
    
    subgraph MAWDU_DEFENSE [Dorar-Standard Forensic Check]
        direction TB
        Isnad{Isnad Scan} -- Discontinuous --> Reject[REJECT: Fabricated]
        Narrator{Narrator Audit} -- Unreliable --> Reject
        Matn{Matn Variance} -- Contra-Quran --> Reject
    end
    
    Knowledge --> MAWDU_DEFENSE
    MAWDU_DEFENSE -- PASSED --> Istinbat[Usul-al-Fiqh Derivation]
    
    subgraph LOGIC_ENGINE [Deontic Reasoning]
        direction LR
        Ijtihad[Juridical Reasoning] --> Hukm{Assign Norm}
        Hukm --> W[Wajib] & H[Haram] & M[Mubah] & ND[Mandub/Makruh]
    end
    
    Istinbat --> LOGIC_ENGINE
    
    subgraph AUDIT_VAL [Benchmark: LegalBench / LEXam]
        Logic{Consistency Check} -- Failed --> Loop[Refine Ijtihad]
        Cite{Citation Integrity} -- Failed --> Loop
    end
    
    LOGIC_ENGINE --> AUDIT_VAL
    AUDIT_VAL -- VERIFIED --> Output[/"Final Hukm Artifact"/]
    
    style Reject fill:#fba,stroke:#f00
    style AUDIT_VAL fill:#dfd,stroke:#383
```

---

## Phase 5: Trust Metric Algebra (Scoring Math)

> **Status:** 🟡 `DEV` | **Implemented in:** [`domain/opinion.rs`](file:///home/a/code/al-mizan-project/almizan-core/src/domain/opinion.rs)

How the system calculates the "Certainty Score" for scholarly evidence.

```mermaid
flowchart TB
    %% GLOBAL STANDARD: EigenTrust (Spectral Graph) & Bayesian Inference
    %% NORMALIZATION: Robust Scaling (Median/IQR)
    
    Data[Raw Scholarly interactions] --> Normalize[Robust Scaling Layer]
    
    subgraph SPECTRAL_RANKING [Global Influence: EigenTrust / HITS]
        direction LR
        Hubs[Hub Score: Citing Experts]
        Auth[Authority Score: Cited Experts]
        GlobalRep[Principal Eigenvector T]
    end
    
    Normalize --> SPECTRAL_RANKING
    
    subgraph EVIDENCE_ENGINE ["Local Trust: Bayesian Update"]
        direction TB
        Prior["Prior Belief P(T)"] --> Update{Bayes' Theorem}
        Interaction[New Evidence E] --> Update
        Posterior["Posterior P(T|E)"]
    end
    
    GlobalRep --> Prior
    Update --> Posterior
    
    subgraph FUZZY_LOGIC [Qualitative Guardrails]
        direction LR
        Adl[Moral Integrity]
        Knowledge[Academic Depth]
        FuzzyJoin{Fuzzy Logic Synthesis}
    end
    
    Posterior --> FuzzyJoin
    Adl & Knowledge --> FuzzyJoin
    
    FuzzyJoin --> Metric[/Final Scholarly Trust Coefficient/]
    
    style Metric fill:#dfd,stroke:#383
    style SPECTRAL_RANKING fill:#fef,stroke:#333
```

---

## Phase 11: Abrogation (Naskh) Logic State Machine

> **Status:** 🟡 `DEV` | **Implemented in:** [`domain/verse.rs`](file:///home/a/code/al-mizan-project/almizan-core/src/domain/verse.rs)

Ensuring theological consistency across revelation periods.

```mermaid
stateDiagram-v2
    %% GLOBAL STANDARD: Temporal Logic in AI & Law (TLAI)
    %% MODEL: Validity Intervals [T_start, T_end]

    [*] --> Active: Hukm In-Force (T_start)
    
    state Active {
        [*] --> Valid_Efficacious: Efficacy Check
        Valid_Efficacious --> Valid_Efficacious: Periodic Re-verify
    }

    state ABROGATION_EVENT <<choice>>
    Active --> ABROGATION_EVENT: New Nasikh Detected

    ABROGATION_EVENT --> Superseded: Chronology Verified (Later)
    ABROGATION_EVENT --> Conflict: Chronology Invalid (Earlier/Equal)

    state Superseded {
        direction TB
        Mansukh: Ruling Terminated (T_end)
        Preserved: Text Remains (Tilawa)
        HistoricRecord: Provenance Locked
    }

    Superseded --> [*]
    Conflict --> Active: Maintain Status Quo
    
    state Nasikh_Action {
        direction LR
        NewHukm: Established Link
        Terminator: Termination Command
    }

    Active --> Nasikh_Action: Triggering Naskh
    Nasikh_Action --> Superseded: Causal Link Established
    
    style Active fill:#dfd,stroke:#383
    style Superseded fill:#eee,stroke:#999
    style Conflict fill:#fba,stroke:#f00
```
