# Fitna Defense Protocol (FDP)

> **Definition**: A graph traversal algorithm that validates node authenticity by recursively proving that a complete, unbroken chain of trust exists from a given Node to a Thabit (Immutable) Root Source (Quran/Sahih Hadith).

## 1. Context
Current implementation (`domain/validation.rs`) only checks:
1. **Scholar Status**: Is the scholar Active or Slashed?
2. **Immediate Evidence**: `find_evidence` checks one hop.

**The Gap**: A malicious actor could insert a Ruling derived from a "Fabricated Hadith" which itself is linked to a "Fake Scholar". Without recursive checking, the Ruling might appear valid because the immediate parent (Fake Scholar) isn't marked "Slashed" yet.

## 2. The Verification Algorithm

### Function Signature
```rust
async fn verify_chain_recursive(&self, node_id: &str, depth: u8) -> VerificationResult
```

### Logic Flow
1. **Base Case (Success)**: 
   - Node is `quran_verse` OR `hadith` (Sahih Collection).
   - Return `VerificationResult::Proven(Tier1)`.

2. **Base Case (Failure)**:
   - Node is `scholar` AND status is `Slashed/Suspended`.
   - Return `VerificationResult::Rejected("Invalid Author")`.
   - `depth` > `MAX_DEPTH` (Circuit Breaker).

3. **Recursive Step**:
   - Find all incoming edges of type `DERIVED_FROM` or `NARRATED_BY`.
   - Recursively call `verify_chain_recursive` on the source node.
   - **Thabit Rule**: At least ONE path must resolve to `Proven(Tier1)`.

## 3. Implementation Plan
- **Location**: `almizan-core/src/domain/graph.rs`
- **New Struct**: `VerificationReport` containing the trace of the proof.
- **API**: Expose `/api/v1/evidence/{id}/verify` endpoint.

## 4. Constraint Checklist
- [ ] Must handle cycles (visited set).
- [ ] Must time out or limit depth (DoS prevention).
- [ ] Must be performant (cache results of Thabit nodes?).
