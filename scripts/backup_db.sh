#!/bin/bash
set -e

# Configuration
BACKUP_DIR="./backups"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
CONTAINER_NAME=${DB_CONTAINER:-almizan-db}
DB_USER=${DB_USER:-root}
DB_PASS=${DB_PASS:-root}
NS=${DB_NS:-almizan}
DB=${DB_DB:-main}
RETENTION_DAYS=7

# Ensure backup directory exists
mkdir -p "$BACKUP_DIR"

echo "Starting backup for NS:$NS DB:$DB at $TIMESTAMP..."

# Perform Export
# Note: We execute the export command inside the container and pipe the output to a local file
docker exec "$CONTAINER_NAME" /surreal export --user "$DB_USER" --pass "$DB_PASS" --ns "$NS" --db "$DB" > "$BACKUP_DIR/backup_$TIMESTAMP.surql"

if [ $? -eq 0 ]; then
  echo "Backup created successfully: $BACKUP_DIR/backup_$TIMESTAMP.surql"
  
  # Compression (Optional but recommended)
  gzip "$BACKUP_DIR/backup_$TIMESTAMP.surql"
  echo "Backup compressed: $BACKUP_DIR/backup_$TIMESTAMP.surql.gz"
  
  # Cleanup old backups
  echo "Cleaning up backups older than $RETENTION_DAYS days..."
  find "$BACKUP_DIR" -name "backup_*.surql.gz" -mtime +$RETENTION_DAYS -delete
  
  echo "Backup process completed."
else
  echo "Backup failed!"
  exit 1
fi
