# Workspace

## Repository Structure

```
odin/
├── Cargo.toml              # Workspace root
├── Cargo.lock
├── rust-toolchain.toml     # MSRV: 1.78
├── .env.example
├── docker-compose.yml
├── Dockerfile
├── Makefile
│
├── crates/
│   ├── odin-api/           # REST API routes + handlers
│   ├── odin-ingestion/     # Alert intake + enrichment
│   ├── odin-memory/        # Embedding + vector search
│   ├── odin-narrative/     # LLM narrative generation
│   ├── odin-graph/         # Neo4j knowledge graph
│   ├── odin-search/        # Full-text + vector search
│   ├── odin-models/        # Domain models + types
│   ├── odin-db/            # PostgreSQL + migrations
│   └── odin-common/        # Shared utilities
│
├── frontend/               # SvelteKit application
│   ├── src/
│   ├── package.json
│   ├── svelte.config.js
│   └── vite.config.ts
│
├── data/                   # Bundled data files
│   ├── mitre/
│   └── sigma/
│
├── docs/                   # Documentation
├── scripts/                # Build/deploy scripts
├── tests/                  # Integration + E2E tests
└── k8s/                    # Kubernetes manifests
```

## Rust Version Policy

- MSRV: 1.78 (tracked in `rust-toolchain.toml`)
- Edition: 2021
- Lint: `clippy::all`, `clippy::pedantic`, `rust-2024-compatibility`
- CI: lint + test on MSRV and stable

## Makefile Targets

```
make dev          # Start all services (docker-compose)
make build        # Build production binary
make test         # Run all tests
make lint         # Run clippy
make db-migrate   # Run PostgreSQL migrations
make db-reset     # Reset dev database
make docs         # Generate documentation
```
