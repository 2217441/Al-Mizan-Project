#!/bin/bash
set -e

# Configuration
REPO_OWNER="firdaushisyam" # Lowercase for compatibility
IMAGE_NAME="ghcr.io/$REPO_OWNER/almizan-core:latest"

echo "=== 🧪 Initializing Local Production Simulation ==="

# 1. Export Dummy Secrets
export DB_URL="almizan-db:8000"
export DB_USER="root"
export DB_PASS="root"
export DB_NS="almizan"
export DB_DB="main"
export JWT_SECRET="dev_secret_local_simulation"
export repo_owner_lower=$REPO_OWNER

# 2. Build the Docker Image Locally
echo "📦 Building Docker Image ($IMAGE_NAME)..."
docker build -t $IMAGE_NAME -f almizan-core/Dockerfile ./almizan-core

# 3. Setup Nginx Config if missing
if [ ! -f "deploy/nginx/active_upstream.conf" ]; then
    echo "⚙️  Initializing Nginx Config..."
    cp deploy/nginx/active_blue.conf deploy/nginx/active_upstream.conf
fi

# 4. Start Infrastructure (Nginx + DB if needed)
# In prod, DB is external, but for sim we might need it. 
# However, prod compose doesn't include DB.
# Let's assume the user runs the local dev DB or we add one.
# For now, let's just treat the app stack.
echo "🏛️  Starting Production Stack..."
docker-compose -f deploy/docker-compose.prod.yml up -d nginx

# 5. Execute Deployment Switch
echo "🔄 Executing Blue/Green Deployment..."
./deploy/switch.sh --skip-pull

echo "=== ✅ Simulation Complete ==="
echo "🌍 App should be accessible at http://localhost:80"
echo "   (Note: Ensure port 80 is free, or edit docker-compose.prod.yml)"
