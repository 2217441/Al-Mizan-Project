# System Architecture Runbooks

> **Author**: System Architect Agent
> **Date**: 2026-01-15
> **Status**: Draft

---

## 1. New Component Approval

**Trigger**: When a new microservice or module is proposed.

**Steps**:
1.  **Theological Check**: Does this component respect the Immutable/Mutable separation?
2.  **Resilience Check**: Can this run Offline? If not, is it marked "Optional"?
3.  **Performance Check**: Does it require >500MB RAM? (If yes, reject for RasPi compatibility).

## 2. Schema Modification

**Trigger**: Changing the Graph Structure.

**Steps**:
1.  Draft `schema.surql` change.
2.  Verify `Ontology Alignment` (Does it break the 3-Tier model?).
3.  Update `ontology.ttl` export definition.
