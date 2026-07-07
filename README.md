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

## Team

ODIN is developed by a multidisciplinary team focused on cybersecurity, software engineering, and emerging technologies.

---

### Kishanth R
**Project Lead · Full Stack Engineer · Cybersecurity Engineer**

**Primary Responsibilities**
- Product Architecture, Technical Leadership, Rust Backend, Next.js Frontend
- API Design, Threat Memory Engine, Similarity Engine, Incident Reconstruction
- AI Integration, System Integration, Final Demo & Presentation

**Ownership:** Intelligence Engine, Memory Engine, Retrieval Engine, Decision Engine, Frontend, Overall System Architecture

---

### Dharshan
**DevOps Engineer · Cybersecurity Engineer**

**Primary Responsibilities**
- Infrastructure Architecture, Docker & Containerization, CI/CD Pipelines
- Deployment Automation, PostgreSQL Management, Qdrant & Neo4j Deployment
- Monitoring & Observability, Backend Testing, Infrastructure Security

**Ownership:** Infrastructure Layer, Deployment, Storage Layer, Monitoring, Security Operations

---

### Shreyanth
**Cybersecurity Engineer · Cryptography & Quantum Security Researcher**

**Primary Responsibilities**
- Threat Intelligence, MITRE ATT&CK Mapping, Threat Modeling
- Detection Engineering, Cryptographic Design, Quantum-Safe Security Research
- Security Validation, Investigation Methodology, Security Documentation, Dataset Validation

**Ownership:** Threat Intelligence, Security Domain, Investigation Models, Detection Logic, Security Research, Cryptographic Components

---

### Collaboration Model

| Area | Owner | Reviewer |
|-------|--------|----------|
| System Architecture | Kishanth | Entire Team |
| Backend | Kishanth | Dharshan |
| Frontend | Kishanth | Dharshan |
| Infrastructure | Dharshan | Kishanth |
| DevOps | Dharshan | Kishanth |
| Database | Dharshan | Kishanth |
| Threat Intelligence | Shreyanth | Kishanth |
| MITRE Mapping | Shreyanth | Kishanth |
| Cryptography | Shreyanth | Entire Team |
| Documentation | Entire Team | Project Lead |
| Final Integration | Kishanth | Entire Team |

---

### Decision Ownership

| Area | Owner | Reviewers |
|------|-------|-----------|
| Product Decisions | Kishanth | — |
| Architecture Decisions | Kishanth | Entire Team |
| Infrastructure Decisions | Dharshan | Kishanth |
| Security Decisions | Shreyanth | Kishanth |
| Final Release Approval | — | All Team Members |

---

### Development Workflow

```
Feature Branch → Development → Peer Review → Integration Testing → Main Branch → Demo Build
```

---

### Communication

**Daily Sync:** Progress, blockers, integration status.

**Critical Changes:** Architecture changes require team discussion. API changes must be communicated before implementation. Database schema changes require coordination with all owners.

---

### Guiding Principle

Every team member owns a domain, but the product belongs to the entire team.

Individual ownership improves velocity. Shared responsibility ensures quality.

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
