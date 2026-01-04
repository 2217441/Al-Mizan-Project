# Deployment Engineering Runbooks

> **Agent**: Deployment Engineering  
> **Last Updated**: 2026-01-05

---

## Table of Contents

1. [Standard Deployment](#standard-deployment)
2. [Emergency Rollback](#emergency-rollback)
3. [Pipeline Troubleshooting](#pipeline-troubleshooting)
4. [Release Management](#release-management)

---

## Standard Deployment

### Automated (Recommended)

Deployments are triggered automatically on push to `main`:

```bash
# Push to main branch triggers:
# 1. backend.yml - Build & test
# 2. deploy.yml - Build image → GHCR → Blue-green switch
```

### Manual Deployment

```bash
# SSH to production
ssh user@$PROD_HOST

# Navigate to project
cd /opt/al-mizan-project

# Check current status
./deploy/switch.sh --status

# Execute deployment
./deploy/switch.sh

# Verify deployment
curl -f http://localhost:3000/health
```

### Local Testing (Simulate Production)

```bash
cd /home/a/code/al-mizan-project

# Build locally
docker-compose -f deploy/docker-compose.prod.yml build

# Deploy with skip-pull
./deploy/switch.sh --skip-pull

# Verify
./deploy/switch.sh --status
```

---

## Emergency Rollback

### Instant Rollback

```bash
# Rollback to previous color
./deploy/switch.sh --rollback

# Verify rollback
./deploy/switch.sh --status

# Check application health
curl -f http://localhost:3000/health
```

### If Rollback Fails

```bash
# 1. Check if standby container exists
docker ps -a | grep almizan-core

# 2. Manually start the standby container
docker-compose -f deploy/docker-compose.prod.yml up -d almizan-core-blue

# 3. Wait for health
sleep 10

# 4. Force switch traffic
cp deploy/nginx/active_blue.conf deploy/nginx/active_upstream.conf
docker exec $(docker ps -q -f name=nginx) nginx -s reload
```

### Complete Stack Recovery

```bash
# Stop everything
docker-compose -f deploy/docker-compose.prod.yml down

# Start fresh
docker-compose -f deploy/docker-compose.prod.yml up -d

# Check all services
docker ps
```

---

## Pipeline Troubleshooting

### Build Failures

#### Cargo Build Fails

```bash
# Check Cargo.lock is committed
git status Cargo.lock

# Clear CI cache
# Go to GitHub Actions → Workflow → Re-run with "clear cache"

# Local reproduction
cd almizan-core
cargo build --release 2>&1 | tee build.log
```

#### Docker Build Fails

```bash
# Build locally with verbose
docker build -t test --progress=plain ./almizan-core

# Check Dockerfile syntax
docker build --check ./almizan-core/Dockerfile
```

### Deploy Job Fails

#### SSH Connection Issues

```bash
# Verify secrets are set
# GitHub → Settings → Secrets → Actions

# Required secrets:
# - PROD_HOST
# - PROD_USER  
# - PROD_SSH_KEY

# Test SSH manually
ssh -i key.pem $PROD_USER@$PROD_HOST "echo 'connected'"
```

#### Container Pull Fails

```bash
# On VPS, verify registry login
echo $GITHUB_TOKEN | docker login ghcr.io -u $GITHUB_USER --password-stdin

# Verify image exists
docker pull ghcr.io/firdaushisyam/islamic-digital-citadel/almizan-core:latest
```

### Health Check Failures

```bash
# Check container logs
docker logs almizan-core-blue --tail 50
docker logs almizan-core-green --tail 50

# Check if port is listening
docker exec almizan-core-blue netstat -tlnp | grep 3000

# Test health endpoint locally
docker exec almizan-core-blue curl -f http://localhost:3000/health
```

---

## Release Management

### Create Manual Release Tag

```bash
# Tag current commit
git tag -a v1.0.0 -m "Release v1.0.0"

# Push tag
git push origin v1.0.0

# This doesn't trigger deployment, just creates release
```

### Hotfix Deployment

```bash
# 1. Create hotfix branch
git checkout -b hotfix/critical-fix main

# 2. Make fix
# ... edit files ...

# 3. Commit and push
git add .
git commit -m "fix: critical issue description"
git push origin hotfix/critical-fix

# 4. Create PR and merge to main
# CI will auto-deploy on merge
```

### Feature Flag Deployment

```bash
# If feature flags are implemented:
# 1. Deploy with flag disabled by default
# 2. Gradually enable for users
# 3. Monitor metrics
# 4. Full rollout or rollback
```

---

## Monitoring Checkpoints

### Pre-Deployment

- [ ] All CI checks passing
- [ ] No critical security alerts
- [ ] Database migrations applied (if any)
- [ ] Feature flags configured

### Post-Deployment

- [ ] Health endpoint responding
- [ ] Application logs clean
- [ ] No error rate spike
- [ ] Response times normal

### Daily Checks

```bash
# Deployment status
./deploy/switch.sh --status

# Container health
docker ps --format "table {{.Names}}\t{{.Status}}"

# Recent deployments (check git log)
git log --oneline -5
```

---

## Quick Reference

| Action | Command |
|--------|---------|
| Deploy | `./deploy/switch.sh` |
| Rollback | `./deploy/switch.sh --rollback` |
| Status | `./deploy/switch.sh --status` |
| Logs (blue) | `docker logs almizan-core-blue` |
| Logs (green) | `docker logs almizan-core-green` |
| Health | `curl localhost:3000/health` |

---

*Last updated: 2026-01-05T00:07:21+08:00*
