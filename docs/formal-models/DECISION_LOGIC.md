# Al-Mizan: Formal Decision Logic (DMN)

This document formalizes the strict Shariah computational rules using **Decision Model and Notation (DMN)** semantics. It focuses on deterministic logic tables for financial and inheritance calculations.

---

## 1. Zakat Liability Decision Service

**Scope**: Determining if a user is liable for Zakat based on asset thresholds (Nisab) and holding period (Hawl).

```plantuml
@startmockup
salt
{
  **Decision: Is Zakat Payable?**
  |+ *Input: TotalAssets* | *Input: NisabThreshold* | *Input: HawlPassed* || *Output: Payable* | *Output: Rate* |
  |+ > Threshold | Any | True || **YES** | **2.5%** |
  |+ < Threshold | Any | Any || **NO** | 0% |
  |+ Any | Any | False || **NO** | 0% |
  |+ > Threshold | Gold | True || **YES** | **2.5%** |
}
@endmockup
```

*Note: Rendered as a truth table using PlantUML Salt, representing a DMN Decision Table.*

---

## 2. Inheritance (Fara'id) Calculation

**Scope**: Logic for distributing shares to primary heirs (Ashab al-Furud).

```plantuml
@startuml
object "Deceased Profile" as Deceased {
    gender = Male
    children = [Son, Daughter]
    spouse = Wife
}

map "Share Logic" as Rules {
    Wife => 1/8 (if children exist)
    Wife => 1/4 (if no children)
    Son => Remainder (Asaba)
    Daughter => 1/2 of Son (if concurrent)
}

Deceased ..> Rules : Applied To
@enduml
```
