# Al-Mizan: Formal Use Case Model

This document defines the strict UML Use Case model for the Al-Mizan system, identifying primary actors and their permissible interactions with the system boundary.

---

## 1. System Actors

| Actor | Type | Description |
| :--- | :--- | :--- |
| **Public User** | `Consumer` | Layperson seeking religiously valid answers (Fatwa). |
| **Scholar Node** | `Provider` | Authenticated human scholar authorized to derive and sign rulings. |
| **Student** | `Learner` | Academic user accessing raw sources for research. |
| **Sovereign Admin** | `System` | Operator of the physical node (restricted access via TEE). |
| **Compliance Officer** | `Auditor` | Independent verifier of system integrity and logs. |

---

## 2. High-Level Use Case Diagram

```plantuml
@startuml
left to right direction
actor "Public User" as Public
actor "Scholar Node" as Scholar
actor "Student (Talib)" as Student
actor "Sovereign Admin" as Admin
actor "Compliance Officer" as Auditor

package "Al-Mizan System Boundary" {
    usecase "Seek Knowledge (Query)" as UC1
    usecase "Derive Hukm (Ijtihad)" as UC2
    usecase "Sign Artifact (Witness)" as UC3
    usecase "Verify Chain (Isnad)" as UC4
    usecase "Audit Log (Liability)" as UC5
    usecase "Manage Infrastructure" as UC6
}

Public --> UC1
Student --> UC1
Student --> UC4

Scholar --> UC2
Scholar --> UC3

UC2 ..> UC4 : <<include>>
UC2 ..> UC3 : <<include>>

Auditor --> UC5
Admin --> UC6

note right of Admin : Restricted by TEE/Enclave
@enduml
```

---

## 3. Detailed Use Case Specifications

### UC1: Seek Knowledge
*   **Actor**: Public, Student
*   **Goal**: Retrieve a religiously valid answer with evidence.
*   **Preconditions**: None (Public Access).
*   **Postconditions**: Answer delivered with "Certainty Score".

### UC2: Derive Hukm (Ijtihad)
*   **Actor**: Scholar Node
*   **Goal**: Synthesize a new ruling based on primary sources.
*   **Preconditions**: Authenticated via DID + Reputation > Threshold.
*   **Postconditions**: New `FiqhRuling` artifact committed to SurrealDB.

### UC5: Audit Log
*   **Actor**: Compliance Officer
*   **Goal**: Verify that no historical records have been mutated.
*   **Preconditions**: Access to Merkle Tree Head (STH).
*   **Postconditions**: Integrity Report generated.
