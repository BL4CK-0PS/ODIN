# Memory Lifecycle

## Stages

```
Ingested ──► Embedded ──► Indexed ──► Queryable ──► Archived ──► Expired
```

### 1. Ingested
- Incident created in PostgreSQL
- Entities extracted and stored in Neo4j
- Not yet searchable via threat memory

### 2. Embedded
- Async worker generates embedding vector
- Vector stored in Qdrant with payload
- Typically completes within seconds of ingestion

### 3. Indexed
- Qdrant optimizes HNSW index (background)
- Full-text search index updated
- Available for similarity queries

### 4. Queryable
- Visible in similarity search results
- Contributes to technique frequency stats
- Edges in knowledge graph traversable

### 5. Archived
- Incident closed > 90 days ago (configurable)
- Moved to cold storage in Qdrant (lower priority collection)
- Still searchable but excluded from default queries
- Full data retained in PostgreSQL + Neo4j

### 6. Expired
- Based on data retention policy (default: 365 days post-closure)
- Vector deleted from Qdrant
- Aggregated statistics retained
- Raw evidence files deleted from object store

## Retention Configuration

| Tier | Duration | Action |
|------|----------|--------|
| Hot | 0–90 days | Full search + graph |
| Warm | 91–365 days | Searchable, cold Qdrant |
| Cold | 366+ days | Aggregate only, full delete |

## Re-indexing Triggers

- Incident update (title, description, TTPs, tags)
- Bulk re-index (nightly cron for stale incidents)
- Model change (full workspace re-index)
- Manual (analyst-initiated per incident)
