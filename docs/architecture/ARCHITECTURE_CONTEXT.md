# Architecture Context

## System Overview
**Name**: Al-Mizan
**Purpose**: Semantic Middleware between Raw Islamic Text Sources and End-User Applications.
**Pattern**: Service-Oriented Architecture (SOA) with a unified Graph Persistency Layer.
**Primary Constraint**: Read-Heavy workloads (99% Reads) for high-performance Reference API.

## Core Components
1. **Backend Core (`almizan-core`)**: Rust/Axum web server.
2. **Database Layer (`almizan-db`)**: SurrealDB (Graph database).
3. **Data Pipeline (`almizan-etl`)**: Python ETL for ingestion.
4. **Frontend (`almizan-web`)**: HTML/CSS/JS with D3.js visualization.

## Key Decisions (ADRs)
- **Database**: SurrealDB for graph capabilities and flexibility.
- **Backend**: Rust for performance and type safety.
- **Architecture**: SOA for modularity.
- **Security**: No public write access (Sandbox Protocol).

## Context Source
Derived from `docs/ARCHITECTURE.md`.
