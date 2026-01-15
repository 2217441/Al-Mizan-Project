# Al-Mizan: Formal Behavioral Models

This document aggregates the strict temporal and state-based logic of the system, adhering to UML 2.5 Sequence and State Machine standards.

---

## 1. Sequence Diagrams (Interactions)

### 1.1 Verifiable Credential Issuance (Phase 6 Formalized)

```plantuml
@startuml
autonumber
participant "Al-Mizan Authority" as Issuer
participant "Scholar Wallet" as Holder
participant "Islamic Bank" as Verifier
participant "Hyperledger Indy" as Ledger

note over Issuer, Holder: Pre-condition: Scholar passes Phase 22

Issuer->Issuer: Create Schema Definition
Issuer->Ledger: Register Schema ID

Issuer->Holder: Offer Credential (Hukm_Signer)
Holder-->>Issuer: Request Credential

Issuer->Holder: Issue Credential (CL-Signatures)
note right of Holder: Stored in Secure Enclave

Verifier->Holder: Request Proof (Predicate: Rank > 5.0)
Holder->Holder: Generate Zero-Knowledge Proof
Holder->Verifier: Presentation (ZKP)

Verifier->Ledger: Get Schema Definition
Verifier->Verifier: Verify Proof
Verifier-->>Holder: Access Granted
@enduml
```

---

## 2. State Machine Diagrams (Lifecycle)

### 2.1 Consensus Lifecycle (Phase 9 Formalized)

```plantuml
@startuml
[*] --> Draft : Ijtihad Proposed

state "Scholar Deliberation" as Deliberation {
    Draft --> PeerReview : Submitted
    PeerReview --> Revision : Feedback Received
    Revision --> PeerReview : Updated
    PeerReview --> Voting : Threshold Met
}

Deliberation --> Consensus : Supermajority Vote
Deliberation --> Rejected : Consistently Failed

state "Consensus States" as Consensus {
    state Mashhur : Majority (Zanni)
    state Ijma : Unanimous (Qat'i)
    
    Mashhur --> Ijma : Extended Validation
}

Consensus --> Canonized : Merkle Commitment
Canonized --> [*]
@enduml
```

### 2.2 Blue-Green Deployment State (Phase 20 Formalized)

```plantuml
@startuml
[*] --> Blue_Active

state "Deployment Transition" as Deploy {
    Blue_Active --> Green_Staging : Deploy v(n+1)
    Green_Staging --> Green_Testing : Run CI/CD
    Green_Testing --> Green_Ready : Tests Passed
    
    state fork_state <<fork>>
    Green_Ready --> fork_state
    fork_state --> Traffic_Mirrored
    fork_state --> Traffic_Shifting
    
    Traffic_Shifting --> Green_Active : 100% Traffic
}

Green_Active --> [*]

Traffic_Shifting --> Rollback : Error Rate > 0.1%
Rollback --> Blue_Active : Revert
@enduml
```
