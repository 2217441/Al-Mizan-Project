# Al-Mizan Operational Runbooks

> **Maintained by**: DevOps Engineering Agent  
> **Last Updated**: 2026-01-04

---

## Table of Contents

1. [Daily Operations](#daily-operations)
2. [Deployment](#deployment)
3. [Backup & Recovery](#backup--recovery)
4. [Incident Response](#incident-response)
5. [Troubleshooting](#troubleshooting)

---

## Daily Operations

### Health Check

```bash
# Check deployment status
./deploy/switch.sh --status

# Check container health
docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" | grep almizan

# Check database connectivity
docker-compose exec almizan-db /surreal isready -e http://localhost:8000
```

### Verify Backups

```bash
# List recent backups
ls -lah database/backups/*.surql | head -5

# Verify latest backup is not empty
LATEST=$(ls -t database/backups/*.surql | head -1)
du -h "$LATEST"
```

---

## Deployment

### Normal Deployment (CI/CD)

Deployments are triggered automatically via GitHub Actions when:
- Code is pushed to `main` branch in `almizan-core/**` or `deploy/**`

### Manual Deployment

```bash
cd /opt/al-mizan-project

# Pull latest code
git pull origin main

# Deploy with blue-green switch
./deploy/switch.sh

# Or skip pull for local testing
./deploy/switch.sh --skip-pull
```

### Rollback

```bash
# Immediate rollback to previous slot
./deploy/switch.sh --rollback

# Verify rollback
./deploy/switch.sh --status
```

### Check Deployment Status

```bash
./deploy/switch.sh --status
```

---

## Backup & Recovery

### Manual Backup

```bash
# Using Makefile
make backup

# Direct script
./database/scripts/backup.sh
```

### Verify Backup

```bash
# Check file exists and has content
ls -lah database/backups/

# Verify not empty
head -20 database/backups/backup_YYYYMMDD_HHMMSS.surql
```

### Restore from Backup

```bash
# Using Makefile
make restore FILE=database/backups/backup_20260104_170918.surql

# Direct script (interactive)
./database/scripts/restore.sh database/backups/backup_20260104_170918.surql
```

### Disaster Recovery

```bash
# 1. Stop all application containers
docker-compose -f deploy/docker-compose.prod.yml stop almizan-core-blue almizan-core-green

# 2. Start database only
docker-compose up -d almizan-db

# 3. Wait for database to be healthy
docker-compose exec almizan-db /surreal isready -e http://localhost:8000

# 4. Restore from backup
./database/scripts/restore.sh database/backups/LATEST_BACKUP.surql

# 5. Restart application
docker-compose -f deploy/docker-compose.prod.yml up -d
```

---

## Incident Response

### Service Down

1. **Check container status**:
   ```bash
   docker ps -a | grep almizan
   ```

2. **Check container logs**:
   ```bash
   docker logs almizan-core-blue --tail 50
   docker logs almizan-core-green --tail 50
   ```

3. **Rollback if recently deployed**:
   ```bash
   ./deploy/switch.sh --rollback
   ```

4. **Restart containers**:
   ```bash
   docker-compose -f deploy/docker-compose.prod.yml restart
   ```

### Database Connection Errors

1. **Check database health**:
   ```bash
   docker-compose exec almizan-db /surreal isready -e http://localhost:8000
   ```

2. **Check database logs**:
   ```bash
   docker-compose logs almizan-db --tail 50
   ```

3. **Restart database**:
   ```bash
   docker-compose restart almizan-db
   # Wait for healthy
   sleep 10
   # Restart app
   docker-compose -f deploy/docker-compose.prod.yml restart almizan-core-blue almizan-core-green
   ```

### Port Already in Use (Development)

```bash
# Find process using port 3000
lsof -i :3000

# Kill if needed
kill -9 <PID>
```

---

## Troubleshooting

### Container Won't Start

```bash
# Check logs
docker logs almizan-core-blue --tail 100

# Common issues:
# - "Address already in use" → Another process on port 3000
# - "Connection refused" → Database not ready
# - "unwrap() on Err" → Check environment variables
```

### Database Not Responding

```bash
# Check if running
docker ps | grep almizan-db

# Check health
docker-compose exec almizan-db /surreal isready -e http://localhost:8000

# Check logs
docker-compose logs almizan-db --tail 50

# Restart database
docker-compose restart almizan-db
```

### Nginx 502 Bad Gateway

```bash
# Check which backend is active
cat deploy/nginx/active_upstream.conf

# Check if target container is running
docker ps | grep almizan-core

# Check container health
./deploy/healthcheck.sh 5 http://localhost:3000/health

# Switch to other slot if available
./deploy/switch.sh --rollback
```

### ETL Data Import Failed

```bash
# Check if database is running
docker-compose exec almizan-db /surreal isready -e http://localhost:8000

# Import manually
surreal import --conn http://localhost:8000 \
  -u root -p root \
  --ns idc --db main \
  almizan-etl/output/ingest.surql

# Check import
surreal sql --conn http://localhost:8000 \
  -u root -p root \
  --ns idc --db main \
  "SELECT count() FROM quran_verse"
```

---

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DB_USER` | SurrealDB username | `root` |
| `DB_PASS` | SurrealDB password | `root` |
| `DB_NS` | Database namespace | `idc` |
| `DB_NAME` | Database name | `main` |
| `DB_URL` | Database host:port | `almizan-db:8000` |
| `JWT_SECRET` | JWT signing secret | - |

---

## Useful Commands

### Docker

```bash
# Rebuild images
docker-compose build --no-cache

# View logs
docker-compose logs -f

# Enter container
docker exec -it almizan-core-blue /bin/sh

# Clean up
docker system prune -a
```

### Database

```bash
# Connect to SurrealDB CLI
docker-compose exec almizan-db /surreal sql \
  -e http://localhost:8000 \
  -u root -p root \
  --ns idc --db main

# Count records
SELECT count() FROM quran_verse GROUP ALL;
SELECT count() FROM hadith GROUP ALL;
```

### Development

```bash
# Run tests
make test

# Start local stack
make up

# View logs
make logs

# Stop stack
make down
```
