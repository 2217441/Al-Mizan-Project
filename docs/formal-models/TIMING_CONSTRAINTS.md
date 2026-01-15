# Al-Mizan: Formal Timing Constraints

This document formally models the strict real-time latency constraints for the BFT Consensus and Network heartbeat using a strict UML Timing Diagram.

---

## 1. Timing Diagram: Real-Time BFT Constraints

Models the state changes of a Validator Node over time, enforcing the 500ms block time budget.

```plantuml
@startuml
robust "Validator Node" as VN
robust "Network State" as Net

scale 100 as 50 pixels

@0
VN is Idle
Net is "Block N"

@100
VN is Proposing
Net is "Block N+1 Pending"

@250
VN is Voting
Net is "Propagating Votes"

@400
VN is Committed
Net is "Finalizing"

@500
VN is Idle
Net is "Block N+1 Final"

highlight 0 to 500 #Gold;line:DimGrey : Block Time Budget (500ms)

@enduml
```
