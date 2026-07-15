# =============================================================================
# ODIN - Development Makefile
# =============================================================================
# Usage: make [target]
# Run `make help` to see all available targets.
# =============================================================================

.PHONY: help build run test lint fmt clean docker-up docker-down docker-logs \
        backup restore smoke-test env

# Default target
help: ## Show this help message
	@echo "ODIN - Operational Defense Intelligence Network"
	@echo "=============================================="
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

# ---------------------------------------------------------------------------
# Environment
# ---------------------------------------------------------------------------

env: ## Copy .env.example to .env (will not overwrite)
	@test -f .env || (cp .env.example .env && echo "Created .env from .env.example") || true
	@test -f .env && echo ".env exists" || echo "No .env file found"

# ---------------------------------------------------------------------------
# Build & Run
# ---------------------------------------------------------------------------

build: ## Build the Rust workspace
	cargo build --workspace

build-release: ## Build the Rust workspace in release mode
	cargo build --workspace --release

run: ## Start the API server locally
	cargo run -p odin-api

run-web: ## Start the Next.js frontend locally
	cd apps/web && npm run dev

# ---------------------------------------------------------------------------
# Testing
# ---------------------------------------------------------------------------

test: ## Run all Rust tests
	cargo test --workspace

test-verbose: ## Run all Rust tests with verbose output
	cargo test --workspace -- --nocapture

test-integration: env ## Run full integration test with Docker
	docker compose up -d --build --wait
	@echo "Waiting for API to be healthy..."
	@for i in $$(seq 1 60); do \
		curl -sf http://localhost:3001/api/v1/system/health > /dev/null 2>&1 && break; \
		sleep 2; \
	done
	bash deploy/scripts/smoke-test.sh http://localhost
	docker compose down

# ---------------------------------------------------------------------------
# Code Quality
# ---------------------------------------------------------------------------

fmt: ## Format all Rust code
	cargo fmt --all

fmt-check: ## Check Rust formatting without modifying
	cargo fmt --all -- --check

lint: ## Run Clippy lints on all crates
	cargo clippy --workspace --all-targets -- -D warnings

audit: ## Run cargo-audit for security vulnerabilities
	cargo audit || (cargo install cargo-audit && cargo audit)

check: fmt-check lint ## Run format check and linting

# ---------------------------------------------------------------------------
# Docker
# ---------------------------------------------------------------------------

docker-up: env ## Start all services with Docker Compose
	docker compose up -d --build

docker-down: ## Stop all services
	docker compose down

docker-down-clean: ## Stop all services and remove volumes
	docker compose down -v

docker-logs: ## Tail logs from all services
	docker compose logs -f

docker-logs-api: ## Tail API logs
	docker compose logs -f odin-api

docker-ps: ## Show running containers
	docker compose ps

docker-health: ## Check health status of all services
	@echo "=== Service Health ==="
	@docker compose ps --format "table {{.Name}}\t{{.Status}}"

# ---------------------------------------------------------------------------
# Monitoring
# ---------------------------------------------------------------------------

monitoring-up: ## Start monitoring stack only
	docker compose up -d prometheus grafana loki postgres-exporter redis-exporter

open-grafana: ## Open Grafana in browser
	@echo "Grafana: http://localhost:3002 (admin/odin-grafana)"
	@start http://localhost:3002 2>/dev/null || open http://localhost:3002 2>/dev/null || echo "Navigate to http://localhost:3002"

open-prometheus: ## Open Prometheus in browser
	@echo "Prometheus: http://localhost:9090"
	@start http://localhost:9090 2>/dev/null || open http://localhost:9090 2>/dev/null || echo "Navigate to http://localhost:9090"

# ---------------------------------------------------------------------------
# Database
# ---------------------------------------------------------------------------

backup: ## Backup the PostgreSQL database
	bash deploy/scripts/backup-db.sh

restore: ## Restore from latest backup (set BACKUP_FILE=path)
	bash deploy/scripts/restore-db.sh $(BACKUP_FILE)

psql: ## Open psql shell to the database
	docker compose exec postgres psql -U odin -d odin

# ---------------------------------------------------------------------------
# Utilities
# ---------------------------------------------------------------------------

smoke-test: ## Run post-deploy smoke tests
	bash deploy/scripts/smoke-test.sh http://localhost

clean: ## Clean build artifacts
	cargo clean
	cd apps/web && rm -rf .next node_modules/.cache 2>/dev/null || true

login: ## Quick login test (admin/odin-dev-password)
	@curl -s -X POST http://localhost:3001/api/v1/auth/login \
		-H "Content-Type: application/json" \
		-d '{"username":"admin","password":"odin-dev-password"}' | python3 -m json.tool 2>/dev/null || \
	curl -s -X POST http://localhost:3001/api/v1/auth/login \
		-H "Content-Type: application/json" \
		-d '{"username":"admin","password":"odin-dev-password"}'
