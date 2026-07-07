# Crate Architecture

---

# DAG

shared

↓

kernel

↓

observation

↓

intelligence

↓

memory

↓

retrieval

↓

decision

↓

policy

↓

reporting

---

# Crates

## shared

IntelligenceObject trait

Domain types

Enums

Errors

Events

---

## kernel

Knowledge Kernel

Canonical Incident

Evidence

Entity

Relationship

Memory Object

No external dependencies on business logic.

---

## observation

Observation Layer

Log parsers

Normalization

Validation

Observation → Kernel objects

---

## intelligence

Intelligence Engine

Deterministic reasoning

Evidence validation

Context Engine

Confidence propagation

Trust scoring

---

## memory

Memory Engine

Memory Builder

Versioning

Evolution

Decay

---

## retrieval

Retrieval Engine

Hybrid similarity

Context-weighted ranking

Diff Engine

---

## decision

Decision Engine

Recommendation generation

Confidence computation

Evidence linking

---

## policy

Policy Engine

Governance rules

Compliance gates

Access control

---

## graph

Cyber Knowledge Fabric (abstraction)

In-memory graph (Core / hackathon)

Neo4j adapter (Enterprise)

---

## storage

PostgreSQL

Qdrant

Repositories

Provenance tracking

---

## reporting

Narrative generation

PDF

Markdown

JSON

STIX

---

## common

Utilities

Logging

Configuration

Security

---

# Rules

Crates communicate through traits.

No circular dependencies.

No crate accesses another crate's storage directly.

Kernel crate has zero business-logic dependencies.

Everything depends on traits, not concrete implementations.
