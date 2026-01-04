#!/bin/bash
# Healthcheck script for production deployments
# Used by: switch.sh, monitoring systems

set -e

# Configuration
TIMEOUT=${1:-5}
ENDPOINT=${2:-"http://localhost:3000/health"}

# Perform health check
HTTP_CODE=$(curl -sf -o /dev/null -w "%{http_code}" --max-time "$TIMEOUT" "$ENDPOINT" 2>/dev/null || echo "000")

if [ "$HTTP_CODE" = "200" ]; then
    echo "✅ Health check passed ($ENDPOINT)"
    exit 0
else
    echo "❌ Health check failed: HTTP $HTTP_CODE ($ENDPOINT)"
    exit 1
fi
