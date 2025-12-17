# ADR 0001: Selection of Core Technology Stack

Status: Accepted

Date: 2025-10-12

Deciders: Ammar Qasiem (System Architect), Firdaus (Data Scientist)

## Context and Problem Statement

We are building a Semantic Knowledge Graph for Islamic texts. The system requires:

1. **High-Performance Traversal**: The ability to query deep relationships (Isnad chains of 10+ depth) with minimal latency.
2. **Schema Flexibility**: The ability to model complex, irregular relationships (e.g., Abrogation, Conditionality) that do not fit neatly into SQL tables.
3. **Type Safety**: The domain deals with "Theological Truths"; data integrity is paramount.

## Decision Drivers

* **Performance**: Recursive queries in SQL (JOIN hell) are $O(N^2)$ or worse. We need $O(1)$ adjacency lookups.
* **Reliability**: The backend must be crash-resistant.
* **Developer Experience**: Separation of concerns between "System Logic" and "Data Analysis."

## Considered Options

### 1. Database

* **Option A: PostgreSQL (SQL)**: Standard, reliable, but slow for deep recursion.
* **Option B: Neo4j (Graph)**: Powerful, but heavy resource usage (Java-based) and proprietary query language (Cypher).
* **Option C: SurrealDB (Multi-Model)**: Lightweight, Written in Rust, supports Graph + Document models natively.

### 2. Backend Language

* **Option A: Python (FastAPI)**: Easy to write, but slower execution, Global Interpreter Lock (GIL) issues with concurrency.
* **Option B: Node.js (Express)**: Fast I/O, but weak type safety (JavaScript/TS).
* **Option C: Rust (Axum)**: Memory safe, blazing fast, prevents entire classes of bugs at compile time.

## Decision Outcome

We chose **SurrealDB** and **Rust (Axum)**.

### Justification

**SurrealDB over Postgres**:

* For Isnad chains, SurrealDB allows us to store pointers directly in the record (`narrated_by` -> `person`). Traversal is following a pointer (constant time), not computing a join (exponential time).
* SurrealDB's "Live Queries" allow us to push updates to the frontend in real-time.

**Rust over Python (for Backend)**:

* **Correctness**: Rust's borrow checker ensures we cannot have data races when serving multiple concurrent requests.
* **Performance**: Rust binaries are small and start instantly, making them ideal for the Dockerized/Containerized "Micro-Sovereign" architecture we are building.

**Python (for ETL)**:

* We retain Python only for the Data Ingestion pipeline (Member 2's role), leveraging libraries like pandas and langchain where they excel, without burdening the runtime performance of the API.

## Consequences

* **Positive**: System will handle massive query loads with minimal RAM.
* **Negative**: Higher learning curve for Rust.
* **Mitigation**: The architecture is split; the Data Scientist does not need to touch the Rust code, interacting only via JSON/SurQL scripts.
