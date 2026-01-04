#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <backup_file.surql>"
    exit 1
fi

# Configuration
if [ -f ../../.env ]; then
    set -a
    source ../../.env
    set +a
fi

DB_USER=${DB_USER:-root}
DB_PASS=${DB_PASS:-root}
DB_NS=${DB_NS:-idc}
DB_NAME=${DB_NAME:-main}
# Resolve absolute path for backup file
BACKUP_FILE="$(realpath "$1")"

if [ ! -f "$BACKUP_FILE" ]; then
    echo "Error: File $BACKUP_FILE not found."
    exit 1
fi

echo "WARNING: This will overwrite data in NS: $DB_NS, DB: $DB_NAME."
read -p "Are you sure? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
fi

echo "Restoring from $BACKUP_FILE..."

# Import data using docker compose exec
cd "$(dirname "$0")/../.."

cat "$BACKUP_FILE" | docker-compose exec -T -i almizan-db /surreal import \
    --user "$DB_USER" \
    --pass "$DB_PASS" \
    --ns "$DB_NS" \
    --db "$DB_NAME"

if [ $? -eq 0 ]; then
    echo "Restore completed successfully."
else
    echo "Restore failed!"
    exit 1
fi
