# Al-Mizan Charter: V2 - Intelligence (The Truth Factory)

This volume specifies the AI synthesis engines, knowledge retrieval lifecycles, and autonomous agent topologies that drive the system's analytical capabilities.

---

## Phase 4: Synthesis Engine: Score & Strategy

> **Status:** 🟡 `DEV` | **Implemented in:** [`api/v1/synthesis.rs`](file:///home/a/code/al-mizan-project/almizan-core/src/api/v1/synthesis.rs)

Advisory logic mapping strictness and seniority levels to final responses.

```mermaid
flowchart TB
    %% GLOBAL STANDARD: AAOIFI Shariah Governance
    %% ARCHITECTURE: Explainable AI (XAI) Decision Tree

    Query([Complex Contract / Mas'ala]) --> Analyze[NLP Feature Extraction]
    
    subgraph XAI_ENGINE [Explainable Decision Tree]
        direction TB
        Pillar1{Riba Check} -- Detected --> Flag[FLAG: Non-Compliant]
        Pillar2{Gharar Check} -- Excessive --> Flag
        Pillar3{Maisir Check} -- Found --> Flag
        Pillar4{Other} -- No Violations --> Synthesis
    end
    
    Analyze --> Pillar1
    
    subgraph STRICTNESS_TRIAGE [Dynamic Compliance Tiers]
        direction LR
        S1[Permissive: Majority Opinions]
        S2[Balanced: AAOIFI Standard]
        S3[Strict: Al-Hazm / Precaution]
    end
    
    Synthesis[Multi-Criteria Synthesis] --> S1 & S2 & S3
    S1 & S2 & S3 --> Scholar
    
    subgraph HIL_GATE [Human-in-the-Loop]
        Scholar[Scholar Audit & Signing]
        Explain[XAI: Generate Evidence Logs]
    end
    
    Scholar & Explain --> Final[/"Authoritative Response"/]
    Scholar -.->|Traceability| Explain
    
    style Flag fill:#fba,stroke:#f00
    style HIL_GATE fill:#dfd,stroke:#383
    
    subgraph LOGS [Zone: Liability Monitoring]
        direction LR
        L1[Retail Audit]
        L2[Commercial Audit]
    end
    Scholar -.-> L1 & L2
```

---

## Phase 7: Knowledge Retrieval Lifecycle (UI to DB)

> **Status:** 🟢 `PROD` | **Implemented in:** [`api/v1/graph.rs`](file:///home/a/code/al-mizan-project/almizan-core/src/api/v1/graph.rs)

The definitive journey of a query through the zero-trust gateway.

```mermaid
flowchart TD
    %% GLOBAL STANDARD: Sub-100ms Latency Benchmark
    %% TECHNIQUE: Hybrid Pivot-Traversal (Node-First + Edge-First)

    Query([User Natural Language Query]) --> Cache{Semantic Cache?}
    
    Cache -- Hit (Sub-10ms) --> Result[/Cached Response/]
    
    Cache -- Miss --> Pivot[HNSW Vector Pivot]
    
    subgraph RETRIEVAL_CORE [Graph Search Engine]
        direction TB
        Pivot --> Connect[Identify Pivot Nodes]
        Connect --> MultiHop{Parallel Multi-Hop}
        MultiHop -- 1st Order --> Adj[Retrieve Adjacency List]
        MultiHop -- 2nd Order --> Subgraph[Build Contextual Subgraph]
    end
    
    RETRIEVAL_CORE --> Rank[Re-ranking: EigenTrust Weighting]
    
    subgraph PERFORMANCE_GUARD ["SLO: < 100ms"]
        Rank --> Verify{Latency Check}
        Verify -- OK --> Output[/Structured Knowledge Artifact/]
        Verify -- Slow --> Optimize[Trigger: Subgraph Pruning]
    end
    
    style Result fill:#dfd,stroke:#383
    style PERFORMANCE_GUARD fill:#fef,stroke:#333
```

---

## Phase 16: Agent Collaboration Topology

> **Status:** 🟡 `DEV` | **Implemented in:** [`api/v1/agents.rs`](file:///home/a/code/al-mizan-project/almizan-core/src/api/v1/agents.rs)

The "Blackboard" architecture for multi-agent theological synthesis.

```mermaid
flowchart TD
    %% GLOBAL STANDARD: IEEE FIPA & Blackboard MAS
    %% PROTOCOL: gRPC / Protobuf bi-directional streaming

    Agenda([Global Synthesis Agenda]) --> Blackboard[(Theological Blackboard)]
    
    subgraph AGENT_SWARM [The Shura Council: Autonomous Agents]
        direction LR
        A1[Research Agent]
        A2[Validation Agent]
        A3[Synthesis Agent]
    end
    
    Blackboard <--> A1 & A2 & A3
    
    subgraph COLLABORATION_HUB [Coordination Layer]
        direction TB
        Poll[Polling: New Knowledge]
        Consensus{Behavioral Coherence?}
    end
    
    SharedState --> Poll
    
    subgraph SUPERVISOR [Orchestrator: Quality Guard]
        direction TB
        Audit[Post-Task Audit]
        Refine[Refinement Cycle]
    end
    
    Consensus -- Result --> Audit
    Refine -- Feedback --> Agenda
```

---

## Phase 17: Ingestion Pipeline (The Truth Factory)

> **Status:** 🟡 `DEV` | **Implemented in:** [`ingestion/`](file:///home/a/code/al-mizan-project/almizan-core/src/ingestion)

High-throughput processing of scholarly texts and revelation data.

### Data Lineage: The Lifecycle of a Verse
Tracing the path from Raw Text to Retrieval-Ready Vector.

```mermaid
flowchart LR
    Scan[Crawler: Scan PDFs] --> OCR[OCR: Extract Text]
    OCR --> Clean[Normalizer: Remove Diacritics]
    Clean --> Meta[Enrichment: Add Metadata (Surah/Ayah)]
    Meta --> Chunk[Chunking: Semantic Split]
    Chunk --> Embed[Embedding: Vectorize (OpenAI/Cohere)]
    Embed --> Upsert[SurrealDB: Upsert Vector Node]
    Upsert --> Index[HNSW Index: Ready for Search]
```

```mermaid
flowchart TD
    %% GLOBAL STANDARD: Apache Kafka / Redpanda (605MB/s)
    %% VALIDATION: TruthFinder & Source Integrity Check

    Sources([Unstructured Texts / PDF / API]) --> Gateway{Ingestion Gateway}
    
    subgraph TRUTH_FACTORY [Validation & Transformation]
        direction TB
        Parse[NLP Parsing: Matn Extraction]
        CrossCheck{Source Verification}
        GraphInsert[Convert to SurQL Edges]
    end
    
    Gateway --> Parse
    
    subgraph VALIDATION_EDGE [Integrity Monitoring]
        direction LR
        Checksum[SHA-256 Check]
        IsnadAud[Isnad Audit: Chain Integrity]
    end
    
    CrossCheck --> Checksum & IsnadAud
    GraphInsert --> DB[(SurrealDB: Global Hub)]
    
    style VALIDATION_EDGE fill:#fef,stroke:#333
    style TRUTH_FACTORY fill:#dfd,stroke:#383
    style LOAD fill:#f5f5ff,stroke:#333

    %% SCHEMAFULL Constraint Check enforced at DB layer
```

---

## Phase 18: API Topology: Endpoint Tiers

> **Status:** 🟡 `DEV` | **Implemented in:** [`api/`](file:///home/a/code/al-mizan-project/almizan-core/src/api)

Definitive distribution tiers for public and private access.

```mermaid
flowchart TD
    %% GLOBAL STANDARD: Kong/Envoy Gateway (50k+ RPS)
    %% MODEL: gRPC Internal Mesh + GraphQL Public Tier

    Entry --> Auth
    
    subgraph SECURITY_OWASP [Edge Hardening]
        direction TB
        Auth[mTLS / JWT Auth]
        Headers[Security Headers: HSTS/CSP]
        WAF[Web Application Firewall]
    end
    
    Auth & Headers & WAF --> Tier1 & Tier2 & Tier3
    
    subgraph THROTTLING [Distribution Tiers: Token Bucket]
        direction LR
        Tier1[Scholar: Unlimited Burst]
        Tier2[Public: 100 req/min]
        Tier3[Corporate: High-Volume SLA]
    end
    
    Tier1 & Tier2 & Tier3 --> gRPC & GQL & REST
    
    subgraph PROTOCOLS [Omni-Protocol Access]
        direction TB
        gRPC[Internal: gRPC / Protobuf - 25ms]
        GQL[Public: GraphQL / JSON - 180ms]
        REST[Legacy: REST / JSON - 250ms]
    end
    
    subgraph BACKEND [The Al-Mizan Core]
        gRPC --> Mesh[Service Mesh: Istio/Linkerd]
        GQL & REST --> App[Backend Engine]
    end

---

## Phase 23: Bias Monitoring & "Adl" (Justice) Guardrails

> **Status:** ⚪ `CONC` | **Implemented in:** [`domain/validation.rs`](file:///home/a/code/al-mizan-project/almizan-core/src/domain/validation.rs) (Bias Hooks)

Continuous monitoring for AI "Fitna" (hallucinations) and theological bias.

```mermaid
flowchart TB
    %% GLOBAL STANDARD: NIST AI 600-1 (Generative AI Governance)
    %% MODEL: Adversarial Red-Teaming + Theological Validation

    Output[/Candidate LLM Response/] --> Monitor{Adl Guardrail Engine}
    
    subgraph BIAS_DETECTION [Theological Red-Teaming]
        direction TB
        Hallucination[Hallucination Check: Wahy Alignment]
        Bias[Bias Check: Sectarian / Extreme Skew]
        Safety[Safety Check: Ethical Boundaries]
    end
    
    Monitor --> Hallucination & Bias & Safety
    
    Hallucination & Bias & Safety -- Threshold Met --> Approve[Final Authoritative Hukm]
    Hallucination & Bias & Safety -- Violation --> Quarantine[Isolate Response & Flag for Review]
    
    Quarantine --> Scholar[Human Scholar Audit]
    Scholar -- Corrected --> Approve
```

---

## Phase 24: Multilingual Semantics Bridge

> **Status:** ⚪ `CONC` | **Implemented in:** [`ingestion/semantics.rs`](file:///home/a/code/al-mizan-project/almizan-core/src/ingestion/semantics.rs)

Preserving sacred nuances across linguistic transitions (Arabic to Global Languages).

```mermaid
flowchart LR
    %% GLOBAL STANDARD: ISO 639-3 & Theological WordNet
    %% MODEL: Semantic Space Mapping (Arabic Hub-and-Spoke)

    Arabic[Source: Classical Arabic / Wahy] --> Embed[Sacred Semantic Embedding]
    
    subgraph SEMANTIC_MAPPING [The Nuance Bridge]
        direction TB
        Context[Contextual Intent: Maqasid]
        Technical[Legal Technicality: Istilah]
        Linguistic[Linguistic Root: Ishtiqaq]
    end
    
    Embed --> Context & Technical & Linguistic
    
    Context & Technical & Linguistic --> TargetEng[Target: English / Scholarly]
    Context & Technical & Linguistic --> TargetMalay[Target: Malay / Regional]
    Context & Technical & Linguistic --> TargetOther[Target: Global Languages]
    
    style Arabic fill:#dfd,stroke:#383
    style targetEng fill:#fef,stroke:#333
```
```
