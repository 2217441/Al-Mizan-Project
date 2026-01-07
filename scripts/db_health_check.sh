#!/bin/bash
# Database Health Check Script
# Usage: ./scripts/db_health_check.sh [--json]

set -e

OUTPUT_FORMAT="${1:-text}"

# Configuration
DB_CONTAINER="al-mizan-project-almizan-db-1"
DB_USER="${DB_USER:-root}"
DB_PASS="${DB_PASS:-root}"
DB_NS="${DB_NS:-almizan}"
DB_DB="${DB_DB:-main}"

# Colors for terminal output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

check_container() {
    if docker ps --format '{{.Names}}' | grep -q "$DB_CONTAINER"; then
        echo "running"
    else
        echo "stopped"
    fi
}

get_memory_usage() {
    docker stats --no-stream --format "{{.MemUsage}}" "$DB_CONTAINER" 2>/dev/null | head -1
}

get_cpu_usage() {
    docker stats --no-stream --format "{{.CPUPerc}}" "$DB_CONTAINER" 2>/dev/null | head -1
}

get_record_counts() {
    curl -s -X POST -u "$DB_USER:$DB_PASS" \
        --header "surreal-ns: $DB_NS" \
        --header "surreal-db: $DB_DB" \
        -H "Accept: application/json" \
        -d "SELECT count() FROM quran_verse GROUP ALL; SELECT count() FROM semantic_hadith GROUP ALL; SELECT count() FROM narrator GROUP ALL;" \
        http://localhost:8000/sql 2>/dev/null
}

get_backup_info() {
    if [ -d "./backups" ]; then
        ls -t ./backups/*.surql.gz 2>/dev/null | head -1
    else
        echo "none"
    fi
}

# Main execution
CONTAINER_STATUS=$(check_container)
MEMORY=$(get_memory_usage)
CPU=$(get_cpu_usage)
RECORDS=$(get_record_counts)
LATEST_BACKUP=$(get_backup_info)

if [ "$OUTPUT_FORMAT" == "--json" ]; then
    # JSON output for automated monitoring
    cat <<EOF
{
    "timestamp": "$(date -Iseconds)",
    "container": {
        "name": "$DB_CONTAINER",
        "status": "$CONTAINER_STATUS",
        "memory": "$MEMORY",
        "cpu": "$CPU"
    },
    "database": {
        "namespace": "$DB_NS",
        "database": "$DB_DB"
    },
    "records": $RECORDS,
    "backup": {
        "latest": "$LATEST_BACKUP"
    }
}
EOF
else
    # Human-readable output
    echo "=================================="
    echo "  Al-Mizan Database Health Check"
    echo "  $(date)"
    echo "=================================="
    echo ""
    
    if [ "$CONTAINER_STATUS" == "running" ]; then
        echo -e "Container: ${GREEN}● Running${NC}"
    else
        echo -e "Container: ${RED}● Stopped${NC}"
    fi
    
    echo "Memory:    $MEMORY"
    echo "CPU:       $CPU"
    echo ""
    echo "Namespace: $DB_NS"
    echo "Database:  $DB_DB"
    echo ""
    echo "Record Counts:"
    echo "$RECORDS" | python3 -c "
import sys, json
data = json.load(sys.stdin)
for i, item in enumerate(data):
    if item['status'] == 'OK':
        tables = ['quran_verse', 'semantic_hadith', 'narrator']
        count = item['result'][0]['count'] if item['result'] else 0
        print(f'  {tables[i]}: {count}')
" 2>/dev/null || echo "  (query failed)"
    echo ""
    echo "Latest Backup: $LATEST_BACKUP"
    echo "=================================="
fi
