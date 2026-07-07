# Crate Architecture

---

shared

↓

ingestion

↓

reconstruction

↓

memory

↓

similarity

↓

reasoning

↓

reporting

---

Rules

Crates communicate through traits.

No circular dependencies.

No crate accesses another crate's database directly.

Storage is abstracted.

---

Traits

IncidentParser

SimilarityProvider

GraphProvider

Repository

ReasoningProvider

NarrativeProvider

StorageProvider

Every implementation depends on traits instead of concrete types.
