# Services

## Service Architecture

All services are async workers communicating via:
- **Direct function calls** (within same process in monolith mode)
- **Redis message queue** (for background jobs across processes)
- **Shared database** (PostgreSQL, Neo4j, Qdrant)

## Core Services

### Incident Service
- Owns the incident lifecycle (CRUD, status transitions)
- Enforces state machine rules (e.g., can't go from Open to Closed without triaging)
- Publishes events on state changes

### Ingestion Service
- Listens on Redis queue for new alerts
- Runs normalization pipeline
- Triggers enrichment (async)
- Triggers memory indexing
- Publishes `incident.created` event

### Memory Service
- Listens for `incident.created` and `incident.updated` events
- Generates embeddings
- Stores/updates in Qdrant
- Runs similarity search queries

### Narrative Service
- Listens for `narrative.generate` events
- Collects incident data
- Calls LLM provider
- Validates output
- Stores narrative

### Graph Service
- Listens for entity extraction events
- Deduplicates and merges entities
- Creates relationships
- Handles graph query requests

### Search Service
- Maintains full-text search index (PostgreSQL GIN)
- Provides hybrid search (full-text + vector)
- Supports faceted filtering

## Event Bus

```rust
enum Event {
    IncidentCreated { id: Uuid },
    IncidentUpdated { id: Uuid, changed_fields: Vec<String> },
    IncidentClosed { id: Uuid },
    EvidenceAdded { incident_id: Uuid, evidence_id: Uuid },
    EvidenceEnriched { evidence_id: Uuid },
    EntityExtracted { entity_id: Uuid, incident_id: Uuid },
    TechniqueMapped { incident_id: Uuid, technique_id: String },
    NarrativeGenerated { incident_id: Uuid },
}
```

Published via Redis pub/sub. Services subscribe to relevant events.

## Service Dependencies

```
IncidentService ───► PostgreSQL
IngestionService ──► PostgreSQL, Redis, (external APIs)
MemoryService ─────► Qdrant
NarrativeService ──► PostgreSQL, LLM Provider
GraphService ──────► Neo4j
SearchService ─────► PostgreSQL, Qdrant
```
