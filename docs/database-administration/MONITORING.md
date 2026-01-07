# Database Monitoring Guide

**Version**: 1.0  
**Last Updated**: 2026-01-07

---

## Quick Health Check

```bash
./scripts/db_health_check.sh
```

Sample output:
```
==================================
  Al-Mizan Database Health Check
  Tue Jan  7 10:50:00 +08 2026
==================================

Container: ● Running
Memory:    372MiB / 31.23GiB
CPU:       4.65%

Namespace: idc
Database:  main

Record Counts:
  quran_verse: 6236
  semantic_hadith: 22756
  narrator: 6624

Latest Backup: ./backups/backup_20260107_105010.surql.gz
==================================
```

---

## Current Baselines (2026-01-07)

| Metric | Value | Threshold |
|--------|-------|-----------|
| Memory Usage | 372 MiB | < 2 GiB |
| CPU Usage | 4.65% | < 50% |
| Quran Verses | 6,236 | Fixed |
| Semantic Hadith | 22,756 | Fixed |
| Narrators | 6,624 | Fixed |

---

## Manual Monitoring Commands

### Container Stats
```bash
# Real-time stats
docker stats al-mizan-project-almizan-db-1

# Snapshot
docker stats --no-stream al-mizan-project-almizan-db-1
```

### Disk Usage
```bash
# Volume size
docker system df -v | grep surreal

# Database file size  
docker exec al-mizan-project-almizan-db-1 du -sh /mydata/
```

### Database Health
```bash
# Quick connectivity test
curl -s http://localhost:8000/health

# Full info
curl -s -X POST -u "root:root" \
    --header "surreal-ns: idc" --header "surreal-db: main" \
    -d "INFO FOR DB;" http://localhost:8000/sql
```

---

## Alerting Thresholds

| Metric | Warning | Critical |
|--------|---------|----------|
| Memory | > 1 GiB | > 2 GiB |
| CPU | > 30% | > 70% |
| Disk | > 70% | > 90% |
| Backup Age | > 24h | > 48h |

---

## Automated Monitoring (Future)

For production, integrate with:
- **Prometheus**: Export metrics via SurrealDB exporter
- **Grafana**: Dashboard visualization
- **Alertmanager**: Threshold-based alerts

Example cron for daily report:
```cron
0 8 * * * /path/to/scripts/db_health_check.sh >> /var/log/db_health.log
```
