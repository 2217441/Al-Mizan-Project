# Architectural Improvement Plan

> **Goal**: Evolve Al-Mizan from a "Standalone App" to a "Federated Kernel" for the Islamic Digital Ecosystem.  
> **Strategy**: Decouple Core Logic, Harden Integrity, Enable Federation.

## 1. Move from "Pragmatic" to "Strict" Hexagonal Architecture
**Problem**: Current `api -> db` coupling prevents federation. You can't federate logic if it's trapped in a Controller calling SQL directly.
**Solution**: Enforce strict **Ports & Adapters**.
*   **Domain**: Defines `VerificationPort` (Interface).
*   **Adapter A (Local)**: `SurrealDbAdapter` implements `VerificationPort`.
*   **Adapter B (Remote)**: `RemoteScholarAdapter` implements `VerificationPort` (calls external Fatwa API).
*   **Benefit**: The core "Fitna Defense" logic remains pure and can verify chains across *both* local DB and remote APIs seamlessly.

## 2. Adopt "Event-Driven Integrity" (CQRS-Lite)
**Problem**: Validating a chain recursively on every *Write* is slow. Validating on every *Read* is slow.
**Solution**: Split the model.
*   **Command (Write)**: When a Ruling is added, emit an event `RulingProposed`.
*   **Event Processor**: An async worker catches `RulingProposed`, runs the `verify_chain_recursive()` (heavy lifting), and stamps the result.
*   **Query (Read)**: API simply reads the pre-computed `integrity_status`.
*   **Benefit**: Instant API reads, near-instant writes, decoupled verification scaling.

## 3. Implement "Semantic Caching"
**Problem**: Traversing 10+ hops of Isnad for 10,000 requests/sec is expensive.
**Solution**: Don't cache by URL (`/api/verse/1`). Cache by **Semantic Graph Subgraph**.
*   **Mechanism**: Cache the *Tawhidic Path* (Allah -> Prophet -> Verse). Invalidating one node (e.g., Scholar Slashed) invalidates only the subgraphs dependent on them.
*   **Benefit**: Massive performance gain while maintaining strict "Theological Correctness" (no stale invalid data).

## 4. Federated Identity (DID) Layer
**Problem**: "Who is Scholar X?" is currently just a string in your DB.
**Solution**: Upgrade `scholar` table to support DIDs (`did:key:...`).
*   **Action**: Add `public_key` field to Scholar. Require cryptographic signatures for `canonical` rulings.
*   **Benefit**: Lay the groundwork for the "Ummah Grid." Other apps can cryptographically verify your data without trusting your database rights.

## Summary Checklist
- [ ] Refactor `api` to use `Repository` traits (Hexagonal).
- [ ] Introduce `async` event handler for Verification (CQRS).
- [ ] Add `signature` fields to schema (Federation).
