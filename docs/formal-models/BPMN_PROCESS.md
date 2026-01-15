# Al-Mizan: Formal BPMN Process (Regulatory Workflow)

This document formally models the "Regulatory Fatwa Issuance" process using BPMN 2.0 semantics (approximated in PlantUML Activity Beta). It focuses on swimlanes and role-based handoffs.

---

## 1. Fatwa Issuance Workflow (BPMN)

**Scope**: Cross-functional process from public query to official Shariah Board ruling.

```plantuml
@startuml
|Public User|
start
:Submit Query (Mas'ala);
:Receive Tracking ID;

|Junior Scholar|
:Acknowledge Receipt;
:Preliminary Evidence Search;
if (Is Novel Issue?) then (yes)
    :Escalate to Council;
else (no)
    :Draft Response based on Precedent;
    :Submit for Peer Review;
endif

|Fatwa Council|
:Form Specialized Committee;
fork
    :Review Bioethics;
fork again
    :Review Finance;
end fork
:Draft Novel Ruling (Ijtihad);
:Vote on Draft;

|Shariah Supervisory Board (SSB)|
:Audit Compliance;
if (Approved?) then (yes)
    :Sign with Keys;
    :Publish to Ledger;
else (no)
    :Remand with Comments;
    stop
endif

|Public User|
:Receive Notification;
:View Verified Fatwa;

stop
@enduml
```
