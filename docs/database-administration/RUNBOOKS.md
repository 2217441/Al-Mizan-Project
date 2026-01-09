# Database Administration Runbooks

## Accessing the Database

### Local Development
To access the SurrealDB instance running in Docker:

1.  **Via CLI (inside container)**:
    ```bash
    docker exec -it almizan-db /surreal sql --endpoint http://localhost:8000 --user root --pass root
    # Note: Replace root/root with actual DB_USER/DB_PASS if different in .env
    ```

2.  **Via Curl**:
    ```bash
    curl -X POST -u "root:root" -H "NS: idc" -H "DB: main" -d "INFO FOR DB;" http://localhost:8000/sql
    ```

## Backup & Restore

### Manual Export (Backup)
To export the current database state to a file:

```bash

# Export specific namespace/database
docker exec -it almizan-db /surreal export --user root --pass root --ns idc --db main output.surql
```

### Automated Backup
We have a script located at `scripts/backup_db.sh` that automates this process.

**Usage:**
```bash
./scripts/backup_db.sh
```

**Configuration:**
The script uses environment variables but defaults to `root/root` and `almizan/main`.
To override:
```bash
export DB_USER=myuser
export DB_PASS=mypass
./scripts/backup_db.sh
```

**Automation (Cron):**
To run daily at 2 AM:
```cron
0 2 * * * /path/to/al-mizan-project/scripts/backup_db.sh >> /var/log/db_backup.log 2>&1
```


### Manual Import (Restore)
To import a `.surql` file:

```bash
cat backup.surql | docker exec -i almizan-db /surreal import --user root --pass root --ns idc --db main
```

## Troubleshooting

### Database Container Unhealthy
If `almizan-db` is unhealthy:
1. Check logs: `docker logs almizan-db`.
2. Verify volume permissions: Ensure the user ID in the container can write to the mapped volume.
3. Check disk space: `df -h`.

### Connection Refused
1. Ensure the port `8000` is mapped in `docker-compose.yml`.
2. Check if another service is using port 8000.
3. Verify SurrealDB is actually running: `ss -tlnp | grep 8000`.

### Port 8000 Not Listening
If using the local binary via `start.sh`:
1. Kill any existing process: `pkill -f "surreal start"`
2. Restart with Docker: `docker-compose up -d almizan-db`
3. Wait for healthcheck: `docker-compose ps` should show "healthy"
