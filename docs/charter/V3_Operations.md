# Al-Mizan Charter: V3 - Operations (Infrastructure & Defense)

This volume specifies the security protocols, infrastructure high-availability, audit trails, and disaster recovery state machines that ensure the system's operational sovereignty.

---

## Phase 6: Enterprise Certification & VC Flow (W3C)

> **Status:** 🟡 `DEV` | **Implemented in:** [`enterprise/`](file:///home/a/code/al-mizan-project/almizan-core/src/enterprise)

The process of issuing Verifiable Credentials for Shariah-compliant contracts.

```mermaid
sequenceDiagram
    %% GLOBAL STANDARD: W3C Verifiable Credentials 2.0
    %% PROTOCOL: DID Core (Ed25519) + BBS+ (ZKP)

    participant I as Issuer (Al-Mizan Auth)
    participant H as Holder (Scholar Wallet)
    participant V as Verifier (Enterprise/Bank)

    Note over I,V: Phase 1: Issuance (JSON-LD + Context v2)
    I->>I: Canonicalize(JSON-LD)
    I->>I: Sign(BBS+ / Ed25519)
    I->>H: Issue VC (Credential + Proof)

    Note over H,V: Phase 2: Presentation (Selective Disclosure)
    H->>H: Generate ZKP Proof (Subset of Attributes)
    H->>V: Verifiable Presentation (VP)
    
    Note over V: Phase 3: Verification (DID Resolve)
    V->>V: DID Resolver (Resolve I.did)
    V->>V: Verify Signature (BBS+ Zero-Knowledge)
    V->>V: Validate Proof Purpose
    V-->>V: Success: Identity + Hukm Confirmed
    V-->>H: Access Granted (Privacy Intact)
```

---

## Phase 8: Security & Scope Authentication

> **Status:** 🟢 `PROD` | **Implemented in:** [`api/auth.rs`](file:///home/a/code/al-mizan-project/almizan-core/src/api/auth.rs)

Definitive Role-Based Access Control (RBAC) mapping.

```mermaid
flowchart TD
    %% GLOBAL STANDARD: NIST SP 800-207 (Zero Trust Architecture)
    %% MODEL: Attribute-Based Access Control (ABAC)

    User([Identity: DID + MFA]) --> PEP[Policy Enforcement Point]
    
    subgraph ABAC_ENGINE [Policy Decision Point]
        direction TB
        Attribs{Attribute Analysis}
        Attribs --> U[User: Role, Reputation]
        Attribs --> R[Resource: Sensitivity, Tier]
        Attribs --> E[Env: Time, Location]
        
        Policy{Policy Engine}
        U & R & E --> Policy
    end
    
    PEP --> Attribs
    
    subgraph ROOT_GUARD [Singleton Integrity: Raft/Strong Consistency]
        direction LR
        CheckRoot{Accessing Thabit?}
        CheckRoot -- Yes --> Strong{Enforce Strong Consistency}
        CheckRoot -- No --> Relaxed[Eventual Consistency]
    end
    
    Policy -- PERMIT --> CheckRoot
    Policy -- DENY --> Audit[Log: Security Incident]
    
    subgraph CONTINUOUS_TRUST [NIST Tenet: Re-evaluation]
        Verify[Per-Session Re-Verify]
        Pulse[Heartbeat: Trust Check]
    end
    
    ROOT_GUARD --> Verify
    Verify --> Access[/Authenticated Scoped Data/]
    
    style User fill:#fef,stroke:#333
    style ROOT_GUARD fill:#fff5f5,stroke:#e53e3e,stroke-width:2px
    style ABAC_ENGINE fill:#dfd,stroke:#383
```

---

## Phase 12: Audit & Liability Trail (ruling_history)

> **Status:** 🟡 `DEV` | **Implemented in:** [`domain/event.rs`](file:///home/a/code/al-mizan-project/almizan-core/src/domain/event.rs)

A complete, granular history of all interpretive data changes.

```mermaid
flowchart TD
    %% GLOBAL STANDARD: RFC 6962 (Certificate Transparency)
    %% MODEL: Merkle-Chained Transparent Log

    Payload[New Ruling / Hukm Artifact] --> Leaf[Pre-Hash: SHA-256]
    
    subgraph MERKLE_LOG [Append-Only Transparency Log]
        direction TB
        Leaf --> Insert[Push to Leaf Registry]
        Insert --> ReHash[Recompute Merkle Tree]
        ReHash --> STH[/"Signed Tree Head (STH)"/]
    end
    
    STH --> SCT[Issue: Signed Certificate Timestamp]
    
    subgraph VERIFICATION ["O(log n) Audit Flow"]
        direction LR
        SCT --> AuditProof{Inclusion Proof?}
        AuditProof -- "Path" --> RootCheck[Verify via Merkle Path]
        RootCheck -- "OK" --> Validated[/"State: Verified Immutable"/]
    end
    
    subgraph RECOVERY [Consistency Audit]
        OldLog[Previous STH] --> Sync{Consistency Proof?}
        Sync -- Verified --> AppendOnly[Confirmed: No Retroactive Edits]
    end
    
    Validated & AppendOnly --> Trust[Global Public Auditability]
    
    style STH fill:#dfd,stroke:#383
    style Validated fill:#dfd,stroke:#383
    style SCT fill:#fef,stroke:#333
```

---

## Phase 13: Network Federation & Snapshots

> **Status:** ⚪ `CONC` | **Implemented in:** [Federation Strategy]

Global synchronization for decentralized Al-Mizan nodes.

```mermaid
flowchart TD
    %% GLOBAL STANDARD: Kademlia (Libp2p) & ZFS Replication
    %% MODEL: Eventual Consistency + Strong Integrity Snapshots

    Node1[Al-Mizan Node: NY] <--> Node2[Al-Mizan Node: KL]
    Node2 <--> Node3[Al-Mizan Node: LDN]
    
    subgraph FEDERATION_DHT [Global Discovery: Kademlia]
        direction LR
        P2P[Find Peer: SHA-256 ID]
        Reputation[Verify Node Reputation]
    end
    
    Node1 & Node2 & Node3 <--> P2P
    
    subgraph DATA_REPLICATION [ZFS Block Streaming]
        direction TB
        Snap[Local Snapshot: T_0]
        Stream[Differential Send/Receive]
        Verify[Global Hash Agreement]
    end
    
    P2P -- Peer Found --> Stream
    Stream --> Verify
```

---

## Phase 14: High-Availability Zone Map

> **Status:** 🟡 `DEV` | **Implemented in:** [Infrastructure Config]

Definitive zero-downtime architecture for mission-critical regions.

```mermaid
flowchart TD
    %% GLOBAL STANDARD: Google SRE (Availability: 99.99%)
    %% MODEL: Geo-Distributed Raft Clusters
    
    West([Traffic: Americas]) --> GSLB{Global Server Load Balancer}
    East([Traffic: Asia/EU]) --> GSLB
    
    subgraph CLUSTER_HA [Highly Available Cluster: Region X]
        AZ1[Zone A: Primary]
        AZ2[Zone B: Standby]
        AZ3[Zone C: Witness]
        AZ1 --- AZ2 --- AZ3
    end
    
    West & East --> AZ1 & AZ2 & AZ3
    
    subgraph RAFT_CONSENSUS [Geo-Distributed RAFT]
        Leader[Leader: AZ1 + Lease]
        Quorum{Majority Check: N/2 + 1}
        Sync[Semi-Sync Replication: < 2ms]
    end
    
    AZ1 & AZ2 & AZ3 --> Leader
    
    subgraph FAILOVER ["Disaster Recovery: RTO < 30s"]
        Leader -- Failure --> Election[New Leader Selection]
        Election --> Standby[Promote Standby in AZ2]
    end
```

---

## Phase 19: Backup & Disaster Recovery Logic

> **Status:** 🟢 `PROD` | **Implemented in:** [Ops/Provisioning]

The 3-2-1-1-0 backup rule for theological data protection.

```mermaid
flowchart TD
    %% GLOBAL STANDARD: 3-2-1-1-0 Backup Rule
    %% TECHNIQUE: Immutable ZFS Snapshots & Air-Gapped Off-site

    subgraph PROD_DATA [Source: UnrealDB Active Nodes]
        Active[(Primary Database)] --> Snap[ZFS Block-Level Sync]
    Snap --> Copy1 & Copy2 & Copy3
    end
    
    subgraph BACKUP_STRATEGY [The 3-2-1-1-0 Protocol]
        direction TB
        Copy1[Copy 1: Local NVMe]
        Copy2[Copy 2: Cloud S3]
        Copy3[Copy 3: Off-site Tape / Deep Archive]
        AirGap{IMMUTABLE: Air-Gapped / Locked}
    Copy1 & Copy2 & Copy3 --> AirGap
    end
    
    subgraph RECOVERY_PIPELINE [ADR: Automated Restoration]
        direction LR
        Detect[Trigger: Region Failure] --> IaC[Terraform: Spin-up Cluster]
        IaC --> Pull[Stream: Latest Block Sync]
        Pull --> Validate{0-Error Verification}
    AirGap --> Detect
    end
    
    subgraph VERIFICATION [Integrity Post-Mortem]
        Validate -- Pass --> Live[/"Resume Traffic: RTO < 60s"/]
        Validate -- Fail --> Rollback[Selective Negation Cycle]
    end
```

---

## Phase 20: Blue-Green Switch State Machine

> **Status:** 🟡 `DEV` | **Implemented in:** [Infrastructure/Deployment]

Zero-downtime deployment for the Al-Mizan Core.

```mermaid
stateDiagram-v2
    %% GLOBAL STANDARD: Envoy/HAProxy Weight Shifting
    %% MODEL: Shadow Mirroring + Active Draining

    [*] --> Idle: Version Blue (Production)
    
    state STAGING_GREEN {
        direction TB
        Provision: Terraform Cluster
        Deploy: almizan-core v_next
        Validation: Health Checks + AI Audit
    }
    
    Idle --> STAGING_GREEN: Prepare Release
    
    state SWITCH_LOGIC {
        direction LR
        Mirror: Parallel Traffic (Shadow)
        Envoy: Shift Traffic Weight (0% -> 100%)
        Drain: Kill Blue Connections
    }
    
    STAGING_GREEN --> Mirror: Validation Passed
    Mirror --> Compare: Shadow Traffic Match
    Compare --> Envoy: Promote Green
    
    subgraph MONITORING [Post-Switch: DORA Guard]
        direction TB
        Error{CFR > 5%?}
        Latency{p99 > 200ms?}
        Rollback[State: Auto-Rollback to Blue]
        
        Error & Latency -- YES --> Rollback
    end
    
    Drain --> Error & Latency
    
    subgraph STATE_LIVE [State: Live Stability]
        Stable[/"Production: Version Green"/]
    end
    
    Error & Latency -- Clean --> Stable
    
    style STAGING_GREEN fill:#eef,stroke:#333
    style SWITCH_LOGIC fill:#dfd,stroke:#383
    style STATE_LIVE fill:#dfd,stroke:#383
```
