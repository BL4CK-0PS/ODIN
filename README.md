# ODIN

**Operational Defense Intelligence Network**

Every investigation strengthens tomorrow's defense.

---

## What is ODIN?

ODIN is an **Institutional Cyber Memory** platform. Instead of storing investigations as reports, ODIN transforms every investigation into reusable organizational intelligence. It is architected as an **operating system for cyber knowledge** — not a pipeline, but a kernel that processes observations into compounding memory.

---

## Features

- **Incident Reconstruction** — Upload logs, generate timeline, extract entities, map MITRE ATT&CK
- **Memory Engine** — Store investigations as knowledge objects with versioning and temporal weighting
- **Hybrid Similarity Search** — Structural + semantic + context-ranked retrieval
- **Investigation Diff** — Compare current and historical investigations
- **Cyber Knowledge Fabric** — Relationship visualization across entities, incidents, and techniques
- **Explainable AI** — Confidence propagation, provenance chains, policy-gated recommendations
- **Local AI** — Ollama-backed, no data leaves your infrastructure

---

## Architecture

```
Observations → Knowledge Kernel → Intelligence Engine → Memory Engine → Retrieval Engine → Decision Engine → Policy Gate → Analyst
```

Five core engines. One Knowledge Kernel (Canonical Incident, Evidence, Entity, Relationship, Memory Object). Everything communicates through domain events.

See [docs/02_ARCHITECTURE](docs/02_ARCHITECTURE) for full architecture.

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust (Axum) |
| Frontend | Next.js |
| Structured Store | PostgreSQL |
| Vector Store | Qdrant |
| Graph Store | Neo4j (Enterprise) / In-memory (Core) |
| AI | Ollama (Qwen 3, Nomic Embed) |

---

## Team & Ownership

| Domain | Owner | Reviewer |
|--------|-------|----------|
| System Architecture | Kishanth | Entire Team |
| Backend / Frontend | Kishanth | Dharshan |
| Infrastructure / DevOps | Dharshan | Kishanth |
| Databases / Deployment | Dharshan | Kishanth |
| Threat Intelligence / MITRE | Shreyanth | Kishanth |
| Cryptography / Security | Shreyanth | Entire Team |

Full team details and collaboration model in [docs/16_OPERATIONS/88_README.md](docs/16_OPERATIONS/88_README.md).

---

## Getting Started

```bash
# Prerequisites: Rust, Node.js, Docker

# Start infrastructure
docker compose up -d postgres qdrant ollama

# Start backend
cargo run

# Start frontend
cd apps/web && npm run dev
```

See [docs/15_ENGINEERING/76_DEVELOPER_GUIDE.md](docs/15_ENGINEERING/76_DEVELOPER_GUIDE.md) for full setup.

---

## Documentation

All documentation lives in [docs/](docs/).

| Section | Contents |
|---------|----------|
| 00_OVERVIEW | Vision, problem, competitive analysis |
| 01_PRODUCT | Personas, stories, requirements, metrics |
| 02_ARCHITECTURE | HLD, components, data flow, deployment |
| 03_DOMAIN | Canonical incident, entity, evidence models |
| 04_THREAT_MEMORY | Memory engine, similarity, diff, lifecycle |
| 05_AI | AI architecture, reasoning, narrative, evaluation |
| 06_BACKEND | Rust workspace, crate architecture, API, services |
| 07_FRONTEND | UI architecture, design system, pages, components |
| 08_DATABASE | PostgreSQL, Neo4j, Qdrant schemas, versioning |
| 09_SECURITY | Auth, RBAC, audit, encryption, evidence integrity |
| 10_DEPLOYMENT | Docker, CI/CD, monitoring, configuration |
| 11_API | Domain events, REST API, plugin system |
| 12_TESTING | Unit, integration, E2E, performance testing |
| 13_DEMO | Demo script, dataset, judge FAQ, presentation guide |
| 14_BUSINESS | Market, business model, GTM, roadmap |
| 15_ENGINEERING | Developer guide, coding standards, ADRs |
| 16_OPERATIONS | Runbook, IR, release, glossary, references |
| 17_APPENDIX | OpenAPI, datasets, backlog, implementation plan |

---

## License

Apache-2.0
