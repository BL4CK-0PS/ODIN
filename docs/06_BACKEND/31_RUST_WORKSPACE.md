# Rust Workspace

Version 1.0

---

# Philosophy

ODIN follows a modular Rust workspace architecture.

Each crate represents one business capability.

No business logic is shared through the API crate.

Business logic remains isolated.

---

Workspace

```
odin/

Cargo.toml

crates/

api/

shared/

ingestion/

reconstruction/

memory/

graph/

similarity/

reasoning/

storage/

reporting/

common/
```

---

# Crates

## api

REST API

Authentication

Routing

Request Validation

---

## shared

Shared models

Enums

Traits

Events

Errors

---

## ingestion

Log parsers

Normalization

Validation

---

## reconstruction

Timeline generation

MITRE mapping

Entity extraction

Canonical Incident

---

## memory

Memory Builder

Memory Objects

Versioning

---

## graph

Neo4j

Relationship projection

Graph queries

---

## similarity

Threat Memory

Similarity Engine

Ranking

Diff Engine

---

## reasoning

Deterministic reasoning

Narrative generation

Explanation generation

---

## storage

PostgreSQL

Neo4j

Qdrant

Repositories

---

## reporting

Report generation

PDF

Markdown

JSON

---

## common

Utilities

Logging

Configuration

Security
