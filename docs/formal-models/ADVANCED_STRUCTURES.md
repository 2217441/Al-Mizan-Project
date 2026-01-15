# Al-Mizan: Formal Advanced Structures

This document contains the advanced structural UML diagrams required for a complete academic model: Object, Package, Profile, and Composite Structure diagrams.

---

## 1. Object Diagram (Instance Snapshot)

Models a specific runtime scenario: "Imam Ghazali Node" validating a "Bioethics Ruling" at `t=now`.

```plantuml
@startuml
object "node_ghazali: Scholar" as Scholar1 {
    did = "did:almizan:1234..."
    reputation = 9.8
    status = Active
}

object "ruling_789: FiqhRuling" as Ruling {
    id = "uuid-789"
    hukm = HARAM
    topic = "CRISPR Editing"
}

object "proof_verse: QuranVerse" as Verse {
    surah = 4
    ayah = 119
    text = "...change creation of Allah..."
}

Scholar1 ..> Ruling : signed
Ruling ..> Verse : cites (Dalil)
@enduml
```

---

## 2. Package Diagram (Namespace Hierarchy)

Formalizes the logical organization of the codebase and domain boundaries.

```plantuml
@startuml
package "io.almizan" {

    package "core" {
        [Domain Entity Models]
        [Theological Logic]
    }

    package "infra" {
        [SurrealDB Adapter]
        [Key Management (KM)]
        [TEE Driver]
    }

    package "intelligence" {
        [Synthesis Agent]
        [Bias Monitor]
        package "nlp" {
            [Arabic Tokenizer]
            [Embedding Model]
        }
    }

    package "api" {
        [GraphQL Resolver]
        [REST Controllers]
    }
}

api ..> core : uses
intelligence ..> core : uses
core ..> infra : persists
@enduml
```

---

## 3. Profile Diagram (Stereotypes)

Defines the Domain-Specific Language (DSL) extensions to UML for Islamic Finance.

```plantuml
@startuml
class "Class" as Metaclass

stereotype "Wahy" {
    invariant: Immutable
    invariant: Source=Divine
}

stereotype "Hukm" {
    tagged_value: CertaintyLevel
    tagged_value: SchoolOfThought
}

stereotype "Sovereign" {
    constraint: TEE_Only
}

Metaclass <|-- "Wahy"
Metaclass <|-- "Hukm"
Metaclass <|-- "Sovereign"
@enduml
```

---

## 4. Composite Structure Diagram (Internal Parts)

Models the internal structure of the `ConsensusEngine` component, showing ports and connectors.

```plantuml
@startuml
component "Consensus Engine (BFT)" as Engine {
    port "Submission" as InPort
    port "EventBus" as OutPort
    
    component "Validator" as Val
    component "Vote Ledger" as Ledger
    component "Stake Manager" as Stake
    
    InPort --> Val : propose()
    Val --> Ledger : record_vote()
    Val --> Stake : check_weight()
    Ledger --> OutPort : emit_finality()
}

interface "IProposal" as IP
IP - InPort
@enduml
```
