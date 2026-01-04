# DevOps Task Checklist

## Phase 1: Critical Fixes ✅

- [x] Fix Dockerfile healthcheck (install curl)
- [x] Add backup verification to backup.sh
- [x] Create cron configuration for automated backups
- [x] Fix Makefile path error (backend → almizan-core)
- [x] Add make backup/restore commands

## Phase 2: Deployment Hardening ✅

- [x] Add HTTP health check to switch.sh
- [x] Add rollback command to switch.sh (--rollback)
- [x] Add status command to switch.sh (--status)
- [x] Remove hardcoded container name (dynamic discovery)
- [x] Create reusable healthcheck.sh script

## Phase 3: Documentation ✅

- [x] Create DEVOPS_ANALYSIS.md
- [x] Create RUNBOOKS.md
- [x] Create TASKS.md (this file)

## Phase 4: Observability (Future)

- [ ] Add health endpoint metrics to Rust app
- [ ] Create docker-compose.monitoring.yml
- [ ] Add Prometheus configuration
- [ ] Create basic Grafana dashboard
- [ ] Configure alerting rules

## Phase 5: Security Hardening (Future)

- [ ] Implement secrets management (SOPS/Vault)
- [ ] Add TLS termination to Nginx
- [ ] Configure rate limiting
- [ ] Add security headers

---

## Summary of Changes Made

### Files Modified

| File | Change |
|------|--------|
| `almizan-core/Dockerfile` | Added `curl` for healthcheck |
| `Makefile` | Fixed path, added `backup`/`restore` targets |
| `database/scripts/backup.sh` | Added verification + size logging |
| `deploy/switch.sh` | Complete rewrite with rollback/status/health |

### Files Created

| File | Purpose |
|------|---------|
| `deploy/healthcheck.sh` | Reusable HTTP health check script |
| `database/cron.d/almizan-backup` | Cron configuration for daily backups |
| `docs/devops/DEVOPS_ANALYSIS.md` | Infrastructure assessment |
| `docs/devops/RUNBOOKS.md` | Operational procedures |
| `docs/devops/TASKS.md` | This checklist |

---

*Last updated: 2026-01-04T20:07:26+08:00*
