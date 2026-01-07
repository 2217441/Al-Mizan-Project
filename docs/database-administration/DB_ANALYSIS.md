# Database Analysis

## Executive Summary
The Al-Mizan project utilizes **SurrealDB** as its primary data store. The database architecture is closely tied to an ETL pipeline (`almizan-etl`) that generates SurrealQL (`.surql`) scripts for data ingestion. Current orchestration is handled via Docker Compose for local development, with a specific production deployment strategy that appears to decouple the database or use an external instance (based on `deploy/docker-compose.prod.yml`).

## Inventory

| Component | Technology | Version | Status | Notes |
|-----------|------------|---------|--------|-------|
| **Core Database** | SurrealDB | `v2.4.0` | Active | Defined in `docker-compose.yml`. |
| **ETL Pipeline** | Python (Implied) | - | Active | Outputting `.surql` files to `almizan-etl/output`. |
| **Persistence** | Docker Volume | - | Verified | Volume `surreal_data` mapped to `/mydata`. |

## Infrastructure & Configuration

### Local Development
- **Service Name**: `almizan-db`
- **Port**: 8000 (Exposed)
- **Namespace**: `idc` (via `DB_NS` in `.env`)
- **Database**: `main` (via `DB_NAME` in `.env`)
- **Credentials**: Injected via `DB_USER` and `DB_PASS`.
- **Healthcheck**: Configured to test `/surreal isready`.
- **Persistence**: `surreal_data` volume ensures data survives container restarts.

> **⚠️ Warning**: When using `start.sh` without Docker, SurrealDB runs in **in-memory mode** (`memory`). All data is lost on restart. Use Docker for persistent development.

### Production
- **Configuration**: `deploy/docker-compose.prod.yml` defines the core application (`almizan-core`) but **does not** explicitly define the database service.
- **Implication**: Production database is likely an external managed instance or deployed via a separate manifest/workflow.
- **Connection**: `DB_URL` environment variable is used to point the core application to the database.

## Data Model & Schema
The data model appears to be "Schema-on-Write" or defined via ingestion scripts (`.surql`). Key data entities identified from ETL outputs include:
- **Quran**: `quran_full`, `cloud_juz30`
- **Hadith**: `hadith_nawawi`, `bukhari_part1`, `hadith40`
- **Metadata**: `narrators`, `chains`, `topics`, `mentions`, `divine_names`
- **Morphology**: `morphology.surql`

## Current Risks & Observations
1.  **Backup Strategy**: No automated backup strategy detected in the repo.
2.  **Schema Management**: Schema appears to be derived from ETL outputs rather than a centralized schema registry or migration tool (valid for SurrealDB but requires discipline).
3.  **Production Clarity**: The location and backup policy of the production database needs confirmation.
4.  **ETL Dependency**: DB state is highly dependent on the successful execution of ETL scripts.

## Recommendations
1.  **Implement Backup**: Create a periodic backup job using `surreal export`.
2.  **Schema Validation**: Document the expected schema for core entities.
3.  **Production Hardening**: Ensure production `DB_URL` uses TLS and proper authentication.
4.  **Monitoring**: Add SurrealDB metrics to any monitoring stack (e.g., Prometheus exporter if available, or basic health pings).
