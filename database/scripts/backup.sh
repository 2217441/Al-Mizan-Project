#!/bin/bash

# Configuration
# If .env exists, source it
if [ -f ../../.env ]; then
    set -a
    source ../../.env
    set +a
fi

DB_USER=${DB_USER:-root}
DB_PASS=${DB_PASS:-root}
DB_NS=${DB_NS:-almizan}
DB_NAME=${DB_NAME:-main}
# Get absolute path to script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BACKUP_DIR="$SCRIPT_DIR/../backups"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
FILENAME="$BACKUP_DIR/backup_$TIMESTAMP.surql"

# Ensure backup directory exists
mkdir -p "$BACKUP_DIR"

echo "Starting backup for NS: $DB_NS, DB: $DB_NAME..."

# Export data using docker compose exec
# Navigate to project root to find docker-compose.yml
cd "$(dirname "$0")/../.."

docker-compose exec -T almizan-db /surreal export \
    --user "$DB_USER" \
    --pass "$DB_PASS" \
    --ns "$DB_NS" \
    --db "$DB_NAME" \
    > "$FILENAME"

if [ $? -eq 0 ]; then
    # Verify backup is not empty
    if [ ! -s "$FILENAME" ]; then
        echo "❌ Backup file is empty! Check database connection."
        rm -f "$FILENAME"
        exit 1
    fi
    
    # Log success with file size
    SIZE=$(du -h "$FILENAME" | cut -f1)
    echo "✅ Backup completed successfully: $FILENAME ($SIZE)"
    
    # Keep only last 7 backups
    ls -t "$BACKUP_DIR"/*.surql | tail -n +8 | xargs -r rm
else
    echo "❌ Backup failed!"
    exit 1
fi
