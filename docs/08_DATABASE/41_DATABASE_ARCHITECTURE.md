# Database Architecture

Version: 2.0

---

# Philosophy

The Knowledge Kernel is the single source of truth.

Databases are projections of the Kernel, not the Kernel itself.

No database owns business logic.

All objects implement IntelligenceObject trait for unified provenance and trust.

---

# Storage Strategy

```
Knowledge Kernel
     │
     ▼
  Domain Events
     │
 ┌───┼───┐
 ▼   ▼   ▼
PG  Qdrant  Neo4j (Enterprise)
```

Independent projection workers consume events and update each store asynchronously.

---

# Responsibilities

## PostgreSQL
- Source of truth for Kernel objects
- Structured data (incidents, evidence, entities, relationships)
- Memory versions and evolution history
- Audit logs
- Provenance chains
- Trust and confidence scores

## Qdrant
- Vector index over Memory Objects
- Semantic similarity retrieval
- Metadata filtering (severity, technique, context)

## Neo4j (Enterprise only)
- Cyber Knowledge Fabric
- Graph projection from Kernel objects
- Entity relationship traversal
- Path analysis across incidents
- **Not used in hackathon MVP** — in-memory graph suffices

---

# Cyber Knowledge Fabric

The term "Knowledge Graph" is replaced with **Cyber Knowledge Fabric**.

- Graph describes technology (Neo4j)
- Fabric describes architecture (woven connections across all knowledge)

Internally: Neo4j (Enterprise) or in-memory graph (Core).

Externally: The Fabric is how all knowledge connects.

---

# Provenance Tables

Every stored object carries provenance.

```
provenance_id
object_id
object_type
source_event_id
source_log_ref
parser_id
timestamp
confidence
trust
```

This enables full traceability: Recommendation → Evidence → Event → Log → Timestamp.

---

# IntelligenceObject Storage

Every table implements the IntelligenceObject contract.

```sql
CREATE TABLE intelligence_objects (
    id UUID PRIMARY KEY,
    object_type VARCHAR(64),
    provenance_id UUID REFERENCES provenance(id),
    confidence FLOAT,
    trust FLOAT,
    evidence_ids UUID[],
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ
);
```

Domain-specific tables (incidents, evidence, entities) inherit from this.

---

# Core vs. Enterprise

| Feature | Core (Hackathon) | Enterprise |
|---------|-----------------|------------|
| Primary store | PostgreSQL | PostgreSQL |
| Vector index | Qdrant | Qdrant cluster |
| Graph | In-memory (derived) | Neo4j cluster |
| Provenance | Basic traces | Full chain |
| Trust scores | Simple propagation | Weighted DAG |
| Projection model | Synchronous | Event-driven (Kafka) |

---

# Rules

- Never duplicate business logic in the database layer
- Only duplicate optimized views (indexes, projections)
- Canonical Incident in Kernel always wins
- Evidence is immutable — never UPDATE, only INSERT
- Confidence and trust are derived, never hardcoded
- Provenance must never be null
