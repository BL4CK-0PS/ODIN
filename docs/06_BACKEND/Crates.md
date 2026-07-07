# Crates

## odin-api

**Purpose:** REST API routes and HTTP handlers.

```
src/
├── routes/
│   ├── mod.rs
│   ├── incidents.rs       # CRUD + status transitions
│   ├── evidence.rs        # Evidence upload/manage
│   ├── memory.rs          # Similarity search endpoint
│   ├── narrative.rs       # Narrative generation trigger
│   ├── graph.rs           # Graph query endpoints
│   ├── search.rs          # Hybrid search endpoint
│   ├── auth.rs            # Login, token refresh
│   └── health.rs          # Health check
├── middleware/
│   ├── auth.rs            # JWT verification
│   ├── logging.rs         # Request/response logging
│   └── rate_limit.rs      # Token bucket rate limiting
├── error.rs               # API error types → HTTP status mapping
└── lib.rs
```

**Framework:** Actix-web 4
**OpenAPI:** utoipa for auto-generated spec at `/openapi.json`

## odin-ingestion

**Purpose:** Process incoming alerts from external sources.

```
src/
├── webhook.rs             # Generic webhook receiver
├── normalizer.rs          # Alert format → internal model
├── enricher.rs            # IOC enrichment (VT, DNS, WHOIS)
├── mitre_mapper.rs        # Evidence → MITRE technique mapping
├── pipeline.rs            # Orchestration of ingestion steps
└── sources/               # Source-specific normalizers
    ├── sentinel.rs
    ├── elastic.rs
    └── generic.rs
```

**Key behavior:** Fire-and-forget via Redis queue. Returns 202 immediately.

## odin-memory

**Purpose:** Embedding generation and vector similarity search.

```
src/
├── embedder.rs            # Text → embedding vector
├── search.rs              # Similarity query + scoring
├── indexer.rs             # Bulk indexing operations
└── lifecycle.rs           # Memory stage management
```

## odin-narrative

**Purpose:** LLM prompt construction and narrative generation.

```
src/
├── generator.rs           # Main generation orchestrator
├── prompts/               # Prompt template loading
│   ├── narrative.md
│   ├── similarity.md
│   └── mitre_suggest.md
├── validator.rs           # Output validation
├── cache.rs               # Response caching
└── feedback.rs            # Analyst feedback collection
```

## odin-graph

**Purpose:** Neo4j operations.

```
src/
├── client.rs              # Connection pool
├── entities.rs            # Entity CRUD
├── relationships.rs       # Relationship CRUD
├── queries.rs             # Common graph queries
└── migrations.rs          # Schema/index management
```

## odin-search

**Purpose:** Hybrid search (full-text + vector).

```
src/
├── fulltext.rs            # PostgreSQL full-text search
├── hybrid.rs              # Combined search logic
└── ranking.rs             # Result ranking
```

## odin-models

**Purpose:** Shared types used across crates. Zero dependencies on other odin crates.

```
src/
├── incident.rs
├── evidence.rs
├── entity.rs
├── ioc.rs
├── technique.rs
├── playbook.rs
├── narrative.rs
├── user.rs
├── workspace.rs
├── timeline.rs
├── pagination.rs
└── error.rs
```

## odin-db

**Purpose:** Database migrations and query functions.

```
src/
├── migrations/            # SQL migration files
│   ├── 001_initial.sql
│   └── 002_add_timeline.sql
├── incident_queries.rs
├── evidence_queries.rs
└── connection.rs         # Pool setup and config
```

## odin-common

**Purpose:** Utilities shared by all crates.

```
src/
├── config.rs             # Environment/config loading
├── logging.rs            # Structured logging setup
├── error.rs              # Base error types
├── idgen.rs              # ULID generation
├── crypto.rs             # Hashing, encryption helpers
├── time.rs               # Time utilities
└── retry.rs              # Retry with backoff
```
