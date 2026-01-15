# Al-Mizan: Formal Interaction Overview

This document formally models the high-level interaction flow, combining Sequence and Activity logic to show the "Derivation-to-Consensus" macro process.

---

## 1. Interaction Overview Diagram

Models the macro-flow between subsystem interactions.

```plantuml
@startuml
start

:User Submits Query;

if (Semantic Cache Hit?) then (yes)
    :Return Cached Hukm;
    stop
else (no)
    ref over Scholar, System
        **Sequence: Istinbat Derivation**
        (See Behavioral Models 1.1)
    end ref
    
    if (Hukm Valid?) then (yes)
        ref over Network
            **Communication: Consensus Voting**
            (See Communication Links 1.0)
        end ref
    else (no)
        :Reject & Log Error;
        stop
    endif
endif

:Commit to Ledger;
stop
@enduml
```
