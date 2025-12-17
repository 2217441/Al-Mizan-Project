# Al-Mizan: Semantic Knowledge Graph Framework

Al-Mizan is a research-driven backend infrastructure designed to address "Epistemological Incongruence" in Islamic digital resources. It shifts the paradigm from linear, keyword-based search to **Graph-based Semantic Traversal**, allowing for the preservation of complex theological relationships (Isnad, Tafsir, Istinbat).

## 🏗 System Architecture

The system operates on a Service-Oriented Architecture (SOA):

* **Core API (`almizan-core`)**: A high-performance Rust (Axum) server that exposes the Knowledge Graph via RESTful endpoints.
* **Graph Engine (`almizan-db`)**: SurrealDB instance storing the Ontology (Nodes & Edges).
* **Data Pipeline (`almizan-etl`)**: Python scripts for ingesting raw text from Tanzil and generating semantic edges via NLP.

## 🚀 Quick Start (Docker)

**Prerequisites**: Docker and Docker Compose.

### 1. Clone the repository

```bash
git clone https://github.com/your-repo/al-mizan.git
cd al-mizan
```

### 2. Start the Infrastructure (DB + API)

```bash
docker-compose up -d
```

### 3. Access the Dashboard

* **UI**: `http://localhost:8080`
* **API Docs**: `http://localhost:3000/docs`

## 📚 Documentation Index

* **[Architecture Overview](docs/ARCHITECTURE.md)**: High-level C4 diagrams and system boundaries.
* **[Ontology Specification](docs/specs/ONTOLOGY_SPEC.md)**: Definitions of Thabit (Immutable) and Zanni (Mutable) node types.
* **[Decision Logs (ADR)](docs/adr/)**: Engineering trade-offs and technology choices.

## 🛠 Tech Stack Justification

| Component | Technology | Why? |
| :--- | :--- | :--- |
| **Backend** | Rust (Axum) | Type safety, zero-cost abstractions, and concurrency for heavy graph queries. |
| **Database** | SurrealDB | Native graph support without the complexity of Neo4j; handles N:M relations efficiently. |
| **ETL** | Python | Rich ecosystem for NLP (LangChain) and data cleaning (Pandas). |

## 🧪 Research Hypothesis

This project validates the hypothesis that **Graph Models** offer superior query performance ($O(1)$) compared to **Relational Models** ($O(N^2)$) when traversing recursive Islamic genealogical structures (Isnad).

---

*Submitted as partial fulfillment for CSCI 4401 (FYP 1) at IIUM.*
