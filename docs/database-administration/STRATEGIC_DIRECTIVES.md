# Strategic Directives: Database Administration
> **Source**: Master Strategic Plan (2026-01-15)

## 1. Scaling Strategy
*   **Directive**: Utilize SurrealDB's separation of Compute and Storage.
*   **Action**: Plan for **TiKV** backend integration for Petabyte-scale growth.

## 2. The 3-Tier Ontology
*   **Directive**: Implement the "Tagged Reality" Schema.
*   **Structure**:
    *   **Tier 1**: Immutable (`SourceText`).
    *   **Tier 2**: Interpretive (`Ruling`).
    *   **Tier 3**: Operational (`Object` with `Tags`).
*   **Logic**: Tier 3 nodes must handle massive write velocity (Real-Time updates) without locking Tier 1/2.

## 3. Cryptographic Verification
*   **Directive**: "Cyborg Isnad" implementation.
*   **Action**: Schema must support `signature` fields on Edge writes.
    *   `RELATION: { signed_by: "scholar_id", signature: "0x..." }`
