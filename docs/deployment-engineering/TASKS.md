# Deployment Engineering Tasks

> **Agent**: Deployment Engineering  
> **Last Updated**: 2026-01-05

## recently Completed
- [x] Define Release Policy (Versioning/Rollback) (See `RELEASE_POLICY.md`) <!-- id: sfia-1 -->
- [ ] **Fixed deploy.yml 'missing server host' error**
  - VPS deployment is now opt-in (manual trigger only)
  - Docker build runs on every push
  - Won't fail when VPS secrets aren't configured

## Backlog

### 🔴 P0 - Critical

- [ ] **Establish Deployment Metrics Baseline**
  - ✅ Added deployment timing to switch.sh
  - ✅ Created deploy_history.log tracking
  - ✅ Created deploy_metrics.json for latest status

### 🟠 P1 - High Priority

- [ ] **Add GitHub Deployment Environments**
  - ✅ Added `production` environment to deploy.yml
  - ✅ Ready for protection rules configuration

- [ ] **Implement Deployment Notifications**
  - ✅ Added Slack webhook template (commented, ready to enable)
  - ✅ Added deployment summary to workflow

- [ ] **Optimize Pipeline Performance**
  - ✅ Enabled Docker BuildKit caching  
  - ✅ Parallel jobs in backend.yml
  - ✅ Post-deployment verification stage

### 🟡 P2 - Medium Priority

- [ ] **Automated Release Tagging**
  - ✅ Created `release.yml` with release-please
  - ✅ Auto-generates changelogs on merge
  - ✅ Tags container images with version

- [ ] **Container Image Scanning**
  - ✅ Added Trivy scanning to deploy.yml
  - ✅ Scans for HIGH/CRITICAL CVEs
  - ✅ Results in GitHub Security tab

- [ ] **Discord Notifications**
  - ✅ Created `notify-discord.yml` reusable workflow
  - ✅ Ready to enable with `DISCORD_WEBHOOK_URL` secret

### 🟢 P3 - Enhancements

- [ ] **Canary Deployment Support**
  - Design traffic splitting architecture
  - Implement percentage-based rollout
  - Add metric comparison automation

- [ ] **ETL Pipeline Integration**
  - Add ETL validation workflow
  - Test data transformations in CI
  - Version ETL outputs

---

## In Progress

- [/] Initial workspace setup and analysis

---

## Completed

- [ ] Created deployment-engineering workspace
- [ ] Initial deployment analysis (`DEPLOYMENT_ANALYSIS.md`)
- [ ] Task backlog created (`TASKS.md`)
- [ ] Runbooks created (`RUNBOOKS.md`)

---

## Notes

### Dependencies on Other Agents

| Task | Depends On |
|------|------------|
| Monitoring integration | DevOps Engineer |
| Test automation | QA Expert |
| DB migration deployment | Database Admin |

### Quick Reference

- **Deployment Location**: `/opt/al-mizan-project` (VPS)
- **Container Registry**: `ghcr.io/firdaushisyam/islamic-digital-citadel/almizan-core`
- **Deploy Script**: `deploy/switch.sh`
- **Rollback**: `deploy/switch.sh --rollback`
- **Status**: `deploy/switch.sh --status`

---

*Last updated: 2026-01-05T00:07:21+08:00*
