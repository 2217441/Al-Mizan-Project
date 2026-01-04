#!/bin/bash
set -e

# =============================================================================
# Al-Mizan Blue-Green Deployment Switch
# 
# Usage:
#   ./switch.sh              # Normal deployment
#   ./switch.sh --skip-pull  # Local simulation (skip docker pull)
#   ./switch.sh --rollback   # Rollback to previous color
#   ./switch.sh --status     # Show current deployment status
# =============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CONF_DIR="$SCRIPT_DIR/nginx"
COMPOSE_FILE="$SCRIPT_DIR/docker-compose.prod.yml"

# Dynamic container name discovery
get_nginx_container() {
    docker ps --format '{{.Names}}' | grep -E 'nginx' | head -1
}

# Argument Parsing
SKIP_PULL=false
ROLLBACK=false
STATUS_ONLY=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --skip-pull)
            SKIP_PULL=true
            shift
            ;;
        --rollback)
            ROLLBACK=true
            shift
            ;;
        --status)
            STATUS_ONLY=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--skip-pull] [--rollback] [--status]"
            exit 1
            ;;
    esac
done

# Determine Current Color
if cmp -s "$CONF_DIR/active_upstream.conf" "$CONF_DIR/active_blue.conf"; then
    CURRENT_COLOR="blue"
    OTHER_COLOR="green"
else
    CURRENT_COLOR="green"
    OTHER_COLOR="blue"
fi

# Status command
if [ "$STATUS_ONLY" = true ]; then
    echo "=== Deployment Status ==="
    echo "🔵 Active Color: $CURRENT_COLOR"
    echo "⚪ Standby Color: $OTHER_COLOR"
    echo ""
    echo "Container Status:"
    docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" | grep -E "almizan|nginx" || echo "No containers found"
    exit 0
fi

# Rollback command
if [ "$ROLLBACK" = true ]; then
    echo "🔄 Rolling back from $CURRENT_COLOR to $OTHER_COLOR..."
    
    # Check if the other container is running
    if ! docker ps | grep -q "almizan-core-$OTHER_COLOR"; then
        echo "❌ Cannot rollback: $OTHER_COLOR container is not running"
        exit 1
    fi
    
    # Switch traffic
    cp "$CONF_DIR/active_$OTHER_COLOR.conf" "$CONF_DIR/active_upstream.conf"
    
    NGINX_CONTAINER=$(get_nginx_container)
    if [ -n "$NGINX_CONTAINER" ]; then
        docker exec "$NGINX_CONTAINER" nginx -s reload
        echo "✅ Rolled back to $OTHER_COLOR"
    else
        echo "⚠️  Nginx container not found. Config updated but not reloaded."
    fi
    exit 0
fi

# Normal deployment
NEW_COLOR="$OTHER_COLOR"

echo "=== Al-Mizan Blue-Green Deployment ==="
echo "🔵 Current: $CURRENT_COLOR"
echo "🟢 Target:  $NEW_COLOR"
echo ""

# 1. Deploy New Version
echo "[1/4] Deploying to $NEW_COLOR slot..."
if [ "$SKIP_PULL" = false ]; then
    docker-compose -f "$COMPOSE_FILE" pull "almizan-core-$NEW_COLOR"
else
    echo "  ⚠️  Skipping pull (Local Simulation Mode)"
fi
docker-compose -f "$COMPOSE_FILE" up -d --no-deps "almizan-core-$NEW_COLOR"

# 2. Wait for startup
echo "[2/4] Waiting for container startup..."
sleep 5

# 3. HTTP Health Check
echo "[3/4] Performing health check..."
MAX_RETRIES=6
RETRY_INTERVAL=5
HEALTH_ENDPOINT="http://almizan-core-$NEW_COLOR:3000/health"

for i in $(seq 1 $MAX_RETRIES); do
    # Check via docker network
    if docker exec "almizan-core-$NEW_COLOR" curl -sf http://localhost:3000/health > /dev/null 2>&1; then
        echo "  ✅ Health check passed (attempt $i/$MAX_RETRIES)"
        break
    fi
    
    if [ $i -eq $MAX_RETRIES ]; then
        echo "  ❌ Health check failed after $MAX_RETRIES attempts"
        echo "  🔄 Keeping traffic on $CURRENT_COLOR"
        echo ""
        echo "Debug info:"
        docker logs "almizan-core-$NEW_COLOR" --tail 20
        exit 1
    fi
    
    echo "  ⏳ Retry $i/$MAX_RETRIES in ${RETRY_INTERVAL}s..."
    sleep $RETRY_INTERVAL
done

# 4. Switch Traffic
echo "[4/4] Switching traffic to $NEW_COLOR..."
cp "$CONF_DIR/active_$NEW_COLOR.conf" "$CONF_DIR/active_upstream.conf"

NGINX_CONTAINER=$(get_nginx_container)
if [ -n "$NGINX_CONTAINER" ]; then
    docker exec "$NGINX_CONTAINER" nginx -s reload
    echo "  ✅ Nginx reloaded"
else
    echo "  ⚠️  Nginx container not found. Config updated but not reloaded."
fi

echo ""
echo "=== ✅ Deployment Complete ==="
echo "🟢 Active: $NEW_COLOR"
echo "🔵 Standby: $CURRENT_COLOR (kept as hot spare)"
echo ""
echo "Rollback command: ./switch.sh --rollback"
