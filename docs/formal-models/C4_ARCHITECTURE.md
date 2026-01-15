# Al-Mizan: Formal C4 Architecture (Strict C4-PlantUML)

This document models the system using the C4 model (Context, Containers, Components, Code), providing a high-level view of how Al-Mizan fits into the global Islamic ecosystem.

---

## 1. System Context Diagram (Level 1)

**Scope**: High-level interactions between the Al-Mizan Software System and external entities.

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Context.puml

Person(scholar, "Islamic Scholar", "Issues Fatwas and validates evidence.")
Person(user, "End User (Mukallaf)", "Queries rulings and verifiable credentials.")

System(almizan, "Al-Mizan Sovereign Cloud", "The decentralized operational backbone for Shariah governance.")

System_Ext(bank, "Islamic Bank", "Consumes Rulings for product compliance.")
System_Ext(gov, "Government Regulator", "Audits compliance via Zero-Knowledge proofs.")
System_Ext(ledger, "Hyperledger Indy", "Public permissioned ledger for DIDs.")

Rel(scholar, almizan, "Publishes Rulings", "HTTPS/TLS")
Rel(user, almizan, "Queries Fatwas", "HTTPS/TLS")

Rel(almizan, bank, "Issues Certificates", "Verifiable Credential")
Rel(almizan, gov, "Proves Solvency", "ZK-Proof")
Rel(almizan, ledger, "Anchors Roots", "DidDoc")
@enduml
```

---

## 2. Container Diagram (Level 2)

**Scope**: High-level deployment units (Web App, API, Database, TEE).

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml

Person(user, "User", "Accesses via Web/Mobile")

System_Boundary(c1, "Al-Mizan Cluster") {
    Container(web, "Web Application", "Rust/Askama", "Delivers SSR HTML content.")
    Container(mobile, "Mobile App", "Flutter", "Wallet and Identity management.")
    Container(api, "API Gateway", "Kong", "Routing, Rate Limiting, Auth.")
    
    Container(backend, "Core Services", "Rust/Axum", "Microservices for logic.")
    Container(tee, "Secure Enclave", "Intel SGX/WASM", "Executes Consensus & Signing.")
    
    ContainerDb(db, "SurrealDB", "Graph Store", "Stores Knowledge Graph.")
    ContainerDb(log, "Audit Log", "Merkle/File", "Immutable append-only log.")
}

Rel(user, web, "Visits", "HTTPS")
Rel(user, mobile, "Uses", "HTTPS")

Rel(web, api, "API Calls", "JSON/HTTPS")
Rel(mobile, api, "API Calls", "JSON/HTTPS")

Rel(api, backend, "Proxies", "gRPC")
Rel(backend, db, "Reads/Writes", "SurrealQL")
Rel(backend, tee, "Delegates Critical Ops", "Remote Attestation")
Rel(tee, log, "Commits Proofs", "File IO")
@enduml
```
