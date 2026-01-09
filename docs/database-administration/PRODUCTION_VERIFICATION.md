# Production Database Verification

**Date**: 2026-01-07  
**Status**: ⚠️ Partial (Local Dev Verified, VPS Pending)

---

## Deployment Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    GitHub Actions                        │
│                    (deploy.yml)                          │
└─────────────────────┬───────────────────────────────────┘
                      │ SSH Deploy
                      ▼
┌─────────────────────────────────────────────────────────┐
│                 Production VPS                           │
│                /opt/al-mizan-project                     │
├─────────────────────────────────────────────────────────┤
│  ┌─────────┐                                            │
│  │  NGINX  │ :80                                        │
│  └────┬────┘                                            │
│       │ Blue-Green Switch                               │
│  ┌────┴────┐                                            │
│  ▼         ▼                                            │
│  [blue]   [green]  (almizan-core containers)            │
│  └────┬────┘                                            │
│       │                                                 │
│       ▼                                                 │
│  ┌──────────────┐                                       │
│  │  SurrealDB   │  (via DB_URL env var)                 │
│  └──────────────┘                                       │
└─────────────────────────────────────────────────────────┘
```

---

## Required Secrets

| Secret | Purpose | Configured |
|--------|---------|------------|
| `PROD_HOST` | VPS IP/hostname | ❓ Check GitHub |
| `PROD_USER` | SSH username | ❓ Check GitHub |
| `PROD_SSH_KEY` | SSH private key | ❓ Check GitHub |
| `DB_URL` | Database connection | Set in VPS `.env` |
| `DB_USER` | Database username | Set in VPS `.env` |
| `DB_PASS` | Database password | Set in VPS `.env` |
| `JWT_SECRET` | Auth token secret | Set in VPS `.env` |

---

## Local Development ✅

| Check | Status | Notes |
|-------|--------|-------|
| Docker running | ✅ | SurrealDB v2.4.0 |
| Port 8000 bound | ✅ | Verified |
| Data persistence | ✅ | `surreal_data` volume |
| Backups configured | ✅ | 2 backups exist |
| Schema applied | ✅ | 47 tables |
| Indexes applied | ✅ | Performance optimized |

---

## Production Verification Checklist

> [!IMPORTANT]
> These checks require VPS access. Complete when production deployment is configured.

### Access & Connectivity
- [ ] SSH access to PROD_HOST working
- [ ] Database reachable from application containers
- [ ] TLS/SSL configured for DB connection

### Security
- [ ] DB credentials rotated from defaults
- [ ] Firewall rules restrict DB port
- [ ] JWT_SECRET is unique and secure

### Backups
- [ ] Cron job installed on VPS
- [ ] Backup script executable
- [ ] Backup storage has adequate space
- [ ] Test restore completed successfully

### Monitoring
- [ ] Health endpoint responding
- [ ] Container restart policy set
- [ ] Log aggregation configured

---

## Deployment Trigger

Production deployment requires manual trigger:

```yaml
# From GitHub Actions UI:
# Actions → Backend CI/CD → Run workflow
# Check: "Deploy to VPS"
```

Or via CLI:
```bash
gh workflow run deploy.yml -f deploy_to_vps=true
```
