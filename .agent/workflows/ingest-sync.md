---
description: Coordinates the ETL ingestion and SurrealDB synchronization process.
---

# Al-Mizan: Ingest Sync

Use this workflow to ingest new data into the Islamic Digital Citadel.

## Instructions
1.  **Source Check**: Verify the status of source data (crawlers, APIs).
2.  **ETL Execution**: Run the ingestion scripts to transform and load data into the staging area.
3.  **SurrealDB Sync**: Synchronize the staging data with the production Knowledge Graph.
4.  **Verify Ingestion**: Run a sample query to ensure the new data is searchable and correctly linked.
