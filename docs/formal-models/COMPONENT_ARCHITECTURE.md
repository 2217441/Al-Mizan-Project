# Al-Mizan: Formal Component Architecture

This document formalizes the high-level ecosystem (Phase 1) and API topology (Phase 18) into a strict UML Component Diagram.

---

## 1. Component Diagram (Level 2 Abstraction)

```plantuml
@startuml
' ARCHITECTURE: Microservices + Serverless
' STANDARD: UML 2.x

package "Client Layer" {
    [Web UI (Askama SSR)]
    [Mobile App (Flutter)]
    [External API Consumer]
}

package "API Gateway" {
    [Kong / Envoy]
    [Auth & RBAC]
}

package "Intelligence Layer" {
    component "Synthesis Engine" as Synthesis
    component "Retrieval Engine" as RAG
    component "Bias Guardrail" as Adl
}

package "Core Domain" {
    component "Identity Service" as ID
    component "Consensus Engine" as BFT
    component "Abrogation Logic" as Naskh
}

package "Persistence Layer" {
    database "SurrealDB (Graph)" as DB
    database "Immutable Log (Merkle)" as Log
}

[Web UI (Askama SSR)] ..> [Kong / Envoy] : HTTPS / HTML
[External API Consumer] ..> [Kong / Envoy] : gRPC / REST

[Kong / Envoy] --> [Auth & RBAC] : Validate JWT
[Auth & RBAC] --> Synthesis : Authorized Req
[Auth & RBAC] --> RAG : Authorized Req

Synthesis --> Adl : Validate Output
Adl --> BFT : Submit for Consensus

BFT --> ID : Verify Stake
BFT --> Naskh : Check Constraints

BFT --> Log : Commit Final State
Synthesis --> DB : Read/Write Metadata
@enduml
```

---

## 2. Interface Definitions

*   **`ISynthesis`**: `generateFatwa(query: String): Artifact`
*   **`IConsensus`**: `proposeBlock(transactions: List<T>): Bool`
*   **`IAudit`**: `verifyMerkleProof(hash: SHA256): Path`
