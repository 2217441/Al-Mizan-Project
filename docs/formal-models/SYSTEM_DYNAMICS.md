# Al-Mizan: System Dynamics (Causal Loops)

This document models the circular feedback loops that drive the long-term growth and stability of the Al-Mizan ecosystem, using **Causal Loop Diagram (CLD)** semantics.

---

## 1. The Trust-Adoption Flywheel

**Scope**: Analyzing how technical consistency drives scholarly participation.

```mermaid
flowchart TD
    A(Technical Consistency):::Positive
    B(Scholarly Trust):::Positive
    C(Fatwa Issuance Rate):::Positive
    D(Public User Base):::Positive
    E(Data Volume):::Positive
    
    A -->|Increases| B
    B -->|Encourages| C
    C -->|Attracts| D
    D -->|Generates| E
    E -->|Refines AI Models| A
    
    linkStyle 0,1,2,3,4 stroke:#383,stroke-width:2px;
    
    classDef Positive fill:#dfd,stroke:#383
```

**Feedback Analysis**:
*   **Result**: Reinforcing Loop (Positive).
*   **Key Driver**: Technical Consistency (The "Thabit" principle). If technical trust fails, the entire loop reverses into a collapse.

---

## 2. The Fitna (Chaos) Damping Loop

**Scope**: How the system self-corrects against misinformation.

```mermaid
flowchart TD
    X(Misinformation / Fitna):::Negative
    Y(Adl Guardrail Activation):::Positive
    Z(Correction Latency):::Negative
    
    X -->|Triggers| Y
    Y -->|Reduces| X
    
    X -->|Increases| Z
    Z -->|Delays| Y
    
    classDef Negative fill:#fdd,stroke:#f00
    classDef Positive fill:#dfd,stroke:#383
```

**Feedback Analysis**:
*   **Result**: Balancing Loop (Stabilizing).
*   **Constraint**: If `Correction Latency` (Z) becomes too high, the damping fails, leading to runaway Fitna.
