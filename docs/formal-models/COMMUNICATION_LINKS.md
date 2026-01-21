# Al-Mizan: Formal Communication Links

This document formally models the object relationships and message passing during the Consensus Voting process using a strict UML Communication Diagram.

---

## 1. Communication Diagram: Consensus Voting

Focuses on the structural relationships between objects involved in the voting mechanism.

```plantuml
@startuml
agent ":ScholarNode" as Scholar
agent ":ConsensusProposal" as Proposal
agent ":VoteLedger" as Ledger
agent ":VerificationService" as Verify

Scholar -down-> Proposal : 1: vote(Accept)
Proposal -right-> Verify : 2: validateSignature()
Verify -left-> Proposal : 3: return(Valid)
Proposal -down-> Ledger : 4: appendVote()
Ledger -up-> Proposal : 5: notifyThresholdUpdate()
@enduml
```
