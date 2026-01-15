# Al-Mizan: Formal Class Structure

This document transforms the operational Entity-Relationship Diagram (ERD) into a strict UML Class Diagram for academic modeling.

---

## 1. Domain Entities (Static Structure)

The system is composed of five primary immutable classes representing the theological primitives.

```plantuml
@startuml
' PERSISTENCE: SurrealDB Schemaless+Schemafull Hybrid
' NOTATION: UML 2.5 Standard

abstract class Wahy {
    +String id
    +String text_content
    +verifyIntegrity() : Boolean
}

class QuranVerse {
    +Int surah_idx
    +Int ayah_idx
    +String uthmani_script
    +isMutant() : Boolean
}

class Hadith {
    +String collection
    +String grade
    +String isnad_chain
    +verifyChain() : Float
}

class Scholar {
    +String did_public_key
    +Float reputation_score
    +Boolean is_jailed
    +issueRuling(Query) : FiqhRuling
    +stakeTokens(Amount) : Void
}

class FiqhRuling {
    +UUID ruling_id
    +DeonticStatus hukm
    +DateTime timestamp
    +String[] evidence_links
    +getConfidence() : Float
}

class IjmaConcensus {
    +UUID consensus_id
    +Float threshold_met
    +finalize() : Void
}

Wahy <|-- QuranVerse
Wahy <|-- Hadith

Scholar "1" --> "*" FiqhRuling : Derives
FiqhRuling "*" --> "*" Wahy : Evidences
Scholar "*" --o "1" IjmaConcensus : Votes_On
IjmaConcensus "1" *-- "*" FiqhRuling : Canonizes
@enduml
```

---

## 2. Object Constraints (OCL)

Formal constraints affecting the classes above:

*   **Scholar Integrity**: `context Scholar inv: self.reputation_score >= 5.0 implies self.issueRuling()`
*   **Abrogation Logic**: `context QuranVerse inv: self.isAbrogated implies self.efficacy == false`
*   **Consensus Validity**: `context IjmaConcensus inv: self.votes->size() > TotalScholars * 0.66`
