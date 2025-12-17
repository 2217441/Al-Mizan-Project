#!/bin/bash
set -e

echo "=== 🏛️  INITIALIZING DIGITAL CITADEL ==="

# 1. Start Infrastructure
echo "[1/4] Launching SurrealDB..."
docker-compose up -d almizan-db
sleep 3 # Wait for DB to wake up

# 2. Run Data Pipeline
echo "[2/4] Executing ETL Pipeline..."
cd almizan-etl
python3 generate_graph.py
cd ..

# 3. Ingest Data (Using the DB container to run the import)
# Assuming 'surreal' tool is available or running via docker exec
echo "[3/4] Ingesting 'Air-Gapped' Data..."
# NOTE: This uses the container's CLI to import the file we just generated.
# We mount the etl output to the container in a real setup, or just push it via API.
# For this demo, we assume the user might need to install 'surreal' CLI or we skip this step if not present.
if command -v surreal &> /dev/null; then
    surreal import --conn http://localhost:8000 --user root --pass root --ns idc --db main almizan-etl/output/ingest.surql
    echo "    > Ingestion Complete."
else
    echo "    > 'surreal' CLI not found. Skipping auto-import (Run manual import from UI)."
fi

# 4. Start Core Engine
echo "[4/4] Starting Al-Mizan Core..."
echo "    > Listening on http://localhost:3000"
cd almizan-core
cargo run
