# Backup & Disaster Recovery Strategy

**Version**: 1.0  
**Last Updated**: 2026-01-07  
**Owner**: Database Administrator

---

## Recovery Objectives

| Metric | Target | Justification |
|--------|--------|---------------|
| **RPO** (Recovery Point Objective) | **< 24 hours** | Daily backups; acceptable for FYP demo/academic context |
| **RTO** (Recovery Time Objective) | **< 1 hour** | Manual restore from backup file |

> [!NOTE]
> For production/commercial deployment, consider RPO < 5 minutes with streaming replication.

---

## Backup Schedule

### Automated (Cron)

| Schedule | Type | Retention | Location |
|----------|------|-----------|----------|
| Daily 2:00 AM | Full export | 7 days | `database/backups/` |
| Weekly Sunday 3:00 AM | Verified backup | 30 days | `database/backups/` |

**Cron Config**: [`database/cron.d/almizan-backup`](file:///home/a/code/al-mizan-project/database/cron.d/almizan-backup)

### Manual Backup

```bash
# Quick backup
./scripts/backup_db.sh

# Or using docker directly
docker exec almizan-db /surreal export \
    --user root --pass root \
    --ns idc --db main > backup_$(date +%Y%m%d).surql
```

---

## Restore Procedure

### 1. Stop Application (Prevent Writes)

```bash
docker-compose stop almizan-core
```

### 2. Restore Database

```bash
# Import backup
cat backup_YYYYMMDD.surql | docker exec -i almizan-db \
    /surreal import --user root --pass root --ns idc --db main

# Or gunzip if compressed
gunzip -c backup_YYYYMMDD.surql.gz | docker exec -i almizan-db \
    /surreal import --user root --pass root --ns idc --db main
```

### 3. Verify Data Integrity

```bash
curl -s -X POST -u "root:root" \
    --header "surreal-ns: idc" --header "surreal-db: main" \
    -d "SELECT count() FROM quran_verse GROUP ALL;" \
    http://localhost:8000/sql
# Expected: 6236 records
```

### 4. Restart Application

```bash
docker-compose up -d almizan-core
```

---

## Backup Verification Checklist

- [ ] Backup file exists and is > 0 bytes
- [ ] Backup can be imported to test instance
- [ ] Record counts match production
- [ ] Application functions after restore

---

## Production Considerations

### Current State (FYP Demo)
- Local Docker deployment
- Manual backup verification
- 24-hour RPO acceptable

### Future State (Production)
- [ ] Offsite backup storage (S3/GCS)
- [ ] Automated backup verification
- [ ] Streaming replication for RPO < 5min
- [ ] Encrypted backups at rest
- [ ] Regular DR drills (quarterly)
