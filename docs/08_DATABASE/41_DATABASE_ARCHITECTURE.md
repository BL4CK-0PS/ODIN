# Database Architecture

Version: 1.0

---

# Philosophy

The Canonical Incident is the single source of truth.

Databases are projections of the Canonical Incident.

No database owns the business logic.

---

# Storage Strategy

Canonical Incident

↓

Projects Into

├── PostgreSQL
├── Neo4j
└── Qdrant

---

# Responsibilities

## PostgreSQL

Structured data

Metadata

Investigations

Playbooks

Evidence

Audit Logs

---

## Neo4j

Relationships

Knowledge Graph

Entity Connections

Incident Connections

---

## Qdrant

Semantic Search

Threat Memory

Similarity Retrieval

---

# Rules

Never duplicate business logic.

Only duplicate optimized views.

Canonical Incident always wins.
