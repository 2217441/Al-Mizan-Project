# Al-Mizan: Reliability & Safety Analysis (FTA)

This document formally models system failure modes using **Fault Tree Analysis (FTA)**. It identifies the root causes that could lead to a top-level critical failure, ensuring safety-critical design.

---

## 1. Consensus Liveness Failure (Fault Tree)

**Top Event**: The BFT Consensus Network halts (Safety Violation or Liveness Failure).

```plantuml
@startuml
object "Consensus Halted" as Top {
    Logic: OR
}

object "Network Partition" as Event1 {
    Logic: AND
}

object "ISP Outage" as Cause1_1
object "The Great Firewall" as Cause1_2

object "Byzantine Overlap" as Event2 {
    Logic: OR
}

object "Key Compromise" as Cause2_1
object "Malicious Majority (33% + 1)" as Cause2_2

object "Software Bug" as Event3 {
    Logic: OR
}

object "Panic in Rust TEE" as Cause3_1
object "Infinite Loop" as Cause3_2

Top <-- Event1
Top <-- Event2
Top <-- Event3

Event1 <-- Cause1_1
Event1 <-- Cause1_2

Event2 <-- Cause2_1
Event2 <-- Cause2_2

Event3 <-- Cause3_1
Event3 <-- Cause3_2
@enduml
```

**Risk Mitigation Strategy**:
1.  **Network Partition**: Multi-cloud sovereign hosting (Phase 21).
2.  **Byzantine Overlap**: Hardware Enclaves (SGX) + Threshold Signatures (Phase 25).
3.  **Software Bug**: Formal Verification + Rust Memory Safety (Phase 3).
