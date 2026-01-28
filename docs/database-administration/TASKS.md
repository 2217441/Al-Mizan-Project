# Database Administration Tasks

## Backlog

### High Importance
- [ ] **Establish Backup Strategy** ✅ COMPLETE (2026-01-07)
  - Define RPO/RTO requirements. ✅ RPO < 24h, RTO < 1h
  - Create a script for `surreal export`. ✅ Done (see `scripts/backup_db.sh`)
  - Schedule automated backups (cron/systemd/job scheduler). ✅ Done (see `database/cron.d/`)
  - See `BACKUP_STRATEGY.md` for full documentation.
- [ ] **Production Database Verification** ✅ DOCUMENTED (2026-01-07)
  - Identify where the production database is hosted. ✅ VPS via DB_URL env var
  - Verify access controls and security groups. ⚠️ Checklist created (pending VPS access)
  - Ensure backups are enabled for production. ⚠️ Checklist created (pending VPS access)
  - See `PRODUCTION_VERIFICATION.md` for deployment architecture and checklists.
- [ ] **Schema Documentation**
  - Reverse engineer the full schema from `.surql` files.
  - Document field types and relationships in `SCHEMA.md`.
  - Canonical schema: `database/schema/schema.surql` (861 lines, v1.1)

### Medium Importance
- [ ] **Performance Tuning** (Audit Complete - 2026-01-07)
  - Monitor query performance for common queries (`almizan-core`). ✅ Done
  - Add indexes if missing for high-frequency lookups (e.g., searching hadith by narrator).
  - See `PERFORMANCE_AUDIT.md` for full report.
- [ ] **P0: Add Missing Indexes** ✅ COMPLETE (2026-01-07)
  - `semantic_hadith`: Added `collection`, `ref_no` indexes
  - `narrator`: Added `name_ar`, `generation` indexes
  - Result: 79ms → 12ms (85% improvement)
- [ ] **Security Hardening**
  - Specific audit of `DB_USER` roles/permissions. ✅ Done
  - Ensure SSL/TLS is enforced in production. (Needs verification)
- [ ] **Namespace Unification** ✅ COMPLETE (2026-01-07)
  - Migrated from `idc` → `almizan`
  - 35,668 records migrated successfully
  - Updated `db.rs` to use `DB_NS` env var
  - Created `scripts/migrate_namespace.sh` for automation

### Ongoing
- [ ] **Monitoring** ✅ COMPLETE (2026-01-07)
    - Track disk usage of `surreal_data`. ✅ Baseline captured
    - Monitor memory usage of `almizan-db` container. ✅ 372 MiB
    - Created `scripts/db_health_check.sh` script
    - See `MONITORING.md` for full guide

---

## New Backlog

### Phase 2: Data Quality & Integrity

- [ ] **Ingest Missing Data**
  - `divine_name`: Currently 0 records (99 names expected)
  - Run ETL for `divine_names.surql`
  - Verify graph connections (Allah → manifests_as → divine_name)

- [ ] **Data Integrity Validation**
  - Create `scripts/validate_data_integrity.sh`
  - Check orphaned edges (narrator without hadith)
  - Verify referential integrity across tables
  - Run weekly via cron

- [ ] **Cleanup Legacy Namespace**
  - Archive `idc` namespace data (already backed up)
  - Remove `idc` namespace after 30-day grace period
  - Document deprecation in CHANGELOG

### Phase 3: Advanced Performance

- [ ] **Full-Text Search Optimization**
  - Add Arabic analyzer indexes for Quran text search
  - Implement `text_uthmani` full-text index
  - Benchmark search performance (target: < 50ms)

- [ ] **API Query Optimization**
  - Profile slow endpoints with tracing
  - Identify N+1 query patterns
  - Implement query batching where applicable

- [ ] **Connection Pooling**
  - Evaluate SurrealDB connection pooling options
  - Implement connection reuse in `db.rs`
  - Benchmark concurrent request handling

### Phase 4: High Availability (Production)

- [ ] **Replication Setup**
  - Design primary-replica topology
  - Configure SurrealDB clustering
  - Document failover procedures

- [ ] **Disaster Recovery Drill**
  - Schedule quarterly DR test
  - Restore from backup to fresh instance
  - Measure actual RTO vs target (< 1h)

- [ ] **SSL/TLS Enforcement**
  - Generate certificates for production DB
  - Configure TLS in SurrealDB
  - Update connection strings in deploy configs

### Phase 5: Documentation & Automation

- [ ] **Update All Documentation for `almizan` Namespace**
  - `README.md` - import commands
  - `docs/data_ingestion_strategy.md`
  - `docs/cloud_testing_guide.md`
  - `docs/devops/RUNBOOKS.md`

- [ ] **Create Schema Migration System**
  - Version control for schema changes
  - Automated migration scripts
  - Rollback support

## Completed
- [ ] Initial Database Analysis (`DB_ANALYSIS.md`).
- [ ] Agent Setup and Directory creation.
- [ ] Documentation audit and corrections (2026-01-07).
- [ ] Backup scripts verified working.
- [ ] Cron automation configured.
- [ ] Backup Strategy defined (`BACKUP_STRATEGY.md`).
- [ ] Production Verification documented (`PRODUCTION_VERIFICATION.md`).
- [ ] Performance Audit completed (`PERFORMANCE_AUDIT.md`).
- [ ] P0 Indexes applied (85% query improvement).
- [ ] Monitoring setup complete (`MONITORING.md`, `scripts/db_health_check.sh`).
- [ ] Namespace Unification: `idc` → `almizan` (35,668 records migrated).
