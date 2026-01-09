#!/bin/bash
# =============================================================================
# NAMESPACE UNIFICATION MIGRATION SCRIPT
# Migrates from 'idc' namespace to 'almizan' namespace
# =============================================================================
# Usage:
#   ./scripts/migrate_namespace.sh backup    - Create pre-migration backup
#   ./scripts/migrate_namespace.sh migrate   - Perform data migration
#   ./scripts/migrate_namespace.sh config    - Update configuration files
#   ./scripts/migrate_namespace.sh verify    - Verify migration success
#   ./scripts/migrate_namespace.sh rollback  - Rollback to previous namespace
#   ./scripts/migrate_namespace.sh all       - Run complete migration
# =============================================================================

set -e

# Configuration
OLD_NS="idc"
NEW_NS="almizan"
DB_NAME="main"
DB_USER="${DB_USER:-root}"
DB_PASS="${DB_PASS:-root}"
DB_URL="http://localhost:8000"
BACKUP_DIR="./backups/migration"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Banner
banner() {
    echo ""
    echo -e "${CYAN}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║${NC}  ${BLUE}⚖️  AL-MIZAN NAMESPACE UNIFICATION${NC}                          ${CYAN}║${NC}"
    echo -e "${CYAN}║${NC}     Migrating: ${YELLOW}$OLD_NS${NC} → ${GREEN}$NEW_NS${NC}                              ${CYAN}║${NC}"
    echo -e "${CYAN}╚══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

# Logging
log_step() { echo -e "\n${BLUE}[STEP]${NC} $1"; }
log_ok() { echo -e "${GREEN}[✓]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[!]${NC} $1"; }
log_error() { echo -e "${RED}[✗]${NC} $1"; }
log_info() { echo -e "${CYAN}[i]${NC} $1"; }

# Execute SurrealQL
surql() {
    local ns="${2:-$NEW_NS}"
    curl -s -X POST -u "$DB_USER:$DB_PASS" \
        --header "surreal-ns: $ns" \
        --header "surreal-db: $DB_NAME" \
        -H "Accept: application/json" \
        -d "$1" "$DB_URL/sql"
}

# Get record count
get_count() {
    local ns="$1"
    local table="$2"
    local result=$(surql "SELECT count() FROM $table GROUP ALL;" "$ns")
    echo "$result" | grep -o '"count":[0-9]*' | head -1 | cut -d: -f2
}

# =============================================================================
# BACKUP PHASE
# =============================================================================
do_backup() {
    log_step "Creating pre-migration backup..."
    
    mkdir -p "$BACKUP_DIR"
    
    # Export old namespace
    log_info "Exporting $OLD_NS namespace..."
    docker-compose exec -T almizan-db /surreal export \
        --user "$DB_USER" --pass "$DB_PASS" \
        --ns "$OLD_NS" --db "$DB_NAME" \
        > "$BACKUP_DIR/pre_migration_${OLD_NS}_${TIMESTAMP}.surql"
    
    if [ -s "$BACKUP_DIR/pre_migration_${OLD_NS}_${TIMESTAMP}.surql" ]; then
        local size=$(du -h "$BACKUP_DIR/pre_migration_${OLD_NS}_${TIMESTAMP}.surql" | cut -f1)
        log_ok "Backup created: $BACKUP_DIR/pre_migration_${OLD_NS}_${TIMESTAMP}.surql ($size)"
    else
        log_error "Backup failed - file is empty!"
        exit 1
    fi
    
    # Save current counts for verification
    log_info "Saving record counts..."
    echo "# Pre-migration counts - $TIMESTAMP" > "$BACKUP_DIR/counts_before.txt"
    for table in quran_verse semantic_hadith narrator prophet divine_name topic; do
        count=$(get_count "$OLD_NS" "$table")
        echo "$table:$count" >> "$BACKUP_DIR/counts_before.txt"
        log_info "$table: $count"
    done
    
    log_ok "Backup phase complete!"
}

# =============================================================================
# MIGRATE PHASE
# =============================================================================
do_migrate() {
    log_step "Migrating data to new namespace..."
    
    # Check if new namespace already has data
    local test_count=$(get_count "$NEW_NS" "quran_verse" 2>/dev/null || echo "0")
    if [ -n "$test_count" ] && [ "$test_count" != "0" ] && [ "$test_count" != "null" ]; then
        log_warn "Target namespace '$NEW_NS' already contains data ($test_count quran_verse records)"
        read -p "Continue and overwrite? [y/N] " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_info "Migration cancelled."
            exit 0
        fi
    fi
    
    # Find latest backup
    LATEST_BACKUP=$(ls -t "$BACKUP_DIR"/pre_migration_${OLD_NS}_*.surql 2>/dev/null | head -1)
    if [ -z "$LATEST_BACKUP" ]; then
        log_error "No backup found! Run './scripts/migrate_namespace.sh backup' first."
        exit 1
    fi
    
    log_info "Using backup: $LATEST_BACKUP"
    
    # Copy backup to container
    log_info "Copying backup to container..."
    docker cp "$LATEST_BACKUP" al-mizan-project-almizan-db-1:/tmp/migration_backup.surql
    
    # Import to new namespace
    log_info "Importing data to $NEW_NS namespace..."
    docker exec al-mizan-project-almizan-db-1 /surreal import \
        --user "$DB_USER" --pass "$DB_PASS" \
        --ns "$NEW_NS" --db "$DB_NAME" \
        --endpoint http://localhost:8000 \
        /tmp/migration_backup.surql
    
    # Cleanup
    docker exec al-mizan-project-almizan-db-1 rm -f /tmp/migration_backup.surql
    
    log_ok "Data migration complete!"
}

# =============================================================================
# CONFIG PHASE
# =============================================================================
do_config() {
    log_step "Updating configuration files..."
    
    # Update db.rs - make namespace configurable
    log_info "Updating Rust code (db.rs)..."
    sed -i 's/client.use_ns("idc")/let ns = std::env::var("DB_NS").unwrap_or("almizan".to_string());\n        client.use_ns(\&ns)/' \
        almizan-core/src/repository/db.rs 2>/dev/null || {
        log_warn "Could not auto-update db.rs - manual edit required"
    }
    
    # Update .env files
    log_info "Updating .env files..."
    sed -i "s/DB_NS=$OLD_NS/DB_NS=$NEW_NS/" .env
    sed -i "s/DB_NS=$OLD_NS/DB_NS=$NEW_NS/" .env.example
    log_ok "Updated .env and .env.example"
    
    # Update scripts
    log_info "Updating shell scripts..."
    for script in scripts/backup_db.sh scripts/db_health_check.sh database/scripts/backup.sh database/scripts/restore.sh; do
        if [ -f "$script" ]; then
            sed -i "s/:-$OLD_NS/:-$NEW_NS/g" "$script"
            sed -i "s/=\"$OLD_NS\"/=\"$NEW_NS\"/g" "$script"
            log_ok "Updated $script"
        fi
    done
    
    # Update start.sh
    sed -i "s/--ns $OLD_NS/--ns $NEW_NS/g" start.sh
    log_ok "Updated start.sh"
    
    # Update simulate_prod.sh
    sed -i "s/DB_NS=\"$OLD_NS\"/DB_NS=\"$NEW_NS\"/g" deploy/simulate_prod.sh
    log_ok "Updated deploy/simulate_prod.sh"
    
    log_ok "Configuration update complete!"
    log_warn "NOTE: Application rebuild required (cargo build)"
}

# =============================================================================
# VERIFY PHASE
# =============================================================================
do_verify() {
    log_step "Verifying migration..."
    
    local all_ok=true
    
    # Load expected counts
    if [ ! -f "$BACKUP_DIR/counts_before.txt" ]; then
        log_error "No pre-migration counts found. Cannot verify."
        exit 1
    fi
    
    log_info "Comparing record counts..."
    echo ""
    printf "${CYAN}%-20s %10s %10s %10s${NC}\n" "Table" "Before" "After" "Status"
    echo "─────────────────────────────────────────────────────"
    
    while IFS=: read -r table expected; do
        [[ "$table" =~ ^#.* ]] && continue
        [ -z "$table" ] && continue
        
        actual=$(get_count "$NEW_NS" "$table" 2>/dev/null || echo "0")
        
        if [ "$expected" == "$actual" ]; then
            printf "%-20s %10s %10s ${GREEN}%10s${NC}\n" "$table" "$expected" "$actual" "✓ Match"
        else
            printf "%-20s %10s %10s ${RED}%10s${NC}\n" "$table" "$expected" "$actual" "✗ MISMATCH"
            all_ok=false
        fi
    done < "$BACKUP_DIR/counts_before.txt"
    
    echo ""
    
    if [ "$all_ok" = true ]; then
        log_ok "All record counts match! Migration verified."
        echo ""
        echo -e "${GREEN}╔══════════════════════════════════════════════════════════════╗${NC}"
        echo -e "${GREEN}║${NC}  ${GREEN}✓ MIGRATION SUCCESSFUL${NC}                                     ${GREEN}║${NC}"
        echo -e "${GREEN}║${NC}                                                              ${GREEN}║${NC}"
        echo -e "${GREEN}║${NC}  Namespace: ${YELLOW}$OLD_NS${NC} → ${GREEN}$NEW_NS${NC}                              ${GREEN}║${NC}"
        echo -e "${GREEN}║${NC}  Database:  $DB_NAME                                          ${GREEN}║${NC}"
        echo -e "${GREEN}╚══════════════════════════════════════════════════════════════╝${NC}"
        echo ""
        log_info "Next steps:"
        echo "  1. Rebuild application: cd almizan-core && cargo build --release"
        echo "  2. Restart services:    docker-compose down && docker-compose up -d"
        echo "  3. Test endpoints:      curl http://localhost:3000/api/v1/graph"
    else
        log_error "Migration verification FAILED! Some counts don't match."
        log_warn "Consider running: ./scripts/migrate_namespace.sh rollback"
        exit 1
    fi
}

# =============================================================================
# ROLLBACK PHASE
# =============================================================================
do_rollback() {
    log_step "Rolling back to $OLD_NS namespace..."
    
    # Restore .env
    sed -i "s/DB_NS=$NEW_NS/DB_NS=$OLD_NS/" .env
    sed -i "s/DB_NS=$NEW_NS/DB_NS=$OLD_NS/" .env.example
    
    # Restore scripts
    for script in scripts/backup_db.sh scripts/db_health_check.sh database/scripts/backup.sh database/scripts/restore.sh; do
        if [ -f "$script" ]; then
            sed -i "s/:-$NEW_NS/:-$OLD_NS/g" "$script"
            sed -i "s/=\"$NEW_NS\"/=\"$OLD_NS\"/g" "$script"
        fi
    done
    
    sed -i "s/--ns $NEW_NS/--ns $OLD_NS/g" start.sh
    sed -i "s/DB_NS=\"$NEW_NS\"/DB_NS=\"$OLD_NS\"/g" deploy/simulate_prod.sh
    
    log_ok "Configuration rolled back to $OLD_NS"
    log_warn "NOTE: If you modified db.rs, restore it manually or rebuild with DB_NS=$OLD_NS"
}

# =============================================================================
# RUN ALL
# =============================================================================
do_all() {
    banner
    do_backup
    do_migrate
    do_config
    do_verify
}

# =============================================================================
# MAIN
# =============================================================================
case "${1:-help}" in
    backup)
        banner
        do_backup
        ;;
    migrate)
        banner
        do_migrate
        ;;
    config)
        banner
        do_config
        ;;
    verify)
        banner
        do_verify
        ;;
    rollback)
        banner
        do_rollback
        ;;
    all)
        do_all
        ;;
    *)
        banner
        echo "Usage: $0 {backup|migrate|config|verify|rollback|all}"
        echo ""
        echo "Commands:"
        echo "  backup   - Create pre-migration backup"
        echo "  migrate  - Perform data migration to new namespace"
        echo "  config   - Update configuration files"
        echo "  verify   - Verify migration success"
        echo "  rollback - Rollback to previous namespace"
        echo "  all      - Run complete migration (backup → migrate → config → verify)"
        ;;
esac
