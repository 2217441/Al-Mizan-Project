.PHONY: up down build logs test

# Docker Compose shortcuts
up:
	docker-compose up -d

down:
	docker-compose down

build:
	docker-compose build

logs:
	docker-compose logs -f

# Local development shortcuts
test:
	cd almizan-core && cargo test

# Database operations
backup:
	./database/scripts/backup.sh

restore:
	@echo "Usage: make restore FILE=database/backups/backup_YYYYMMDD_HHMMSS.surql"
	@test -n "$(FILE)" || (echo "ERROR: FILE not set" && exit 1)
	./database/scripts/restore.sh $(FILE)
