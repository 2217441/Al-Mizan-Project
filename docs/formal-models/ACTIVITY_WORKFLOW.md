# Al-Mizan: Formal Activity Workflow

This document formally models the "Istinbat" (Legal Derivation) process using a strict UML Activity Diagram.

---

## 1. Activity Diagram: The Istinbat Process

Models the control flow, decision points, and parallel activities involved in deriving a ruling.

```plantuml
@startuml
start

:Receive Legal Query (Mas'ala);

partition "Knowledge Retrieval" {
    fork
        :Search Quran (Primary);
    fork again
        :Search Hadith (Sunnah);
    fork again
        :Search Fiqh Corpus (Ijma);
    end fork
}

:Validate Evidence (Isnad/Matn);

if (Evidence Found?) then (yes)
    :Analyze Context (Maqasid);
    if (Clear Text Exists?) then (yes)
        :Apply Qat'i Ruling;
    else (no)
        :Perform Ijtihad (Analogical Deduction);
    endif
else (no)
    :Suspend Judgment (Tawaqquf);
    stop
endif

:Draft Hukm Artifact;

partition "Peer Review" {
    :Submit to Consensus Network;
    while (Objections > 0?) is (yes)
        :Refine Argument;
        :Respond to Critiques;
    endwhile (no)
}

:Sign & Canonize Hukm;

stop
@enduml
```
