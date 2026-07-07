# ODIN Architecture Overview

Version: 2.0 — Knowledge Operating System

---

# Core Metaphor

ODIN is not a pipeline.

ODIN is an **operating system for cyber knowledge**.

Logs, alerts, and investigations are inputs. Knowledge is the output. Memory compounds.

---

# Architecture

```
                          Presentation
                 Web • API • CLI • Reports
                               │
                               ▼
                    Application Layer
         Commands • Queries • Workflows • Policies
                               │
                               ▼
                   Knowledge Kernel (Core)
┌──────────────────────────────────────────────────────┐
│ Incident  │  Evidence  │  Entity  │  Relationship   │
│                     Memory Object                    │
└──────────────────────────────────────────────────────┘
    Nothing outside the kernel modifies these objects.
                               │
                    Knowledge Bus (Domain Events)
                               │
             ┌─────────────────┼─────────────────┐
             ▼                 ▼                 ▼
      Intelligence       Retrieval        Decision
         Engine           Engine            Engine
   (Rules + Context)  (Hybrid Similarity)  (Confidence + Policy)
             │                 │                 │
             └─────────────────┼─────────────────┘
                               ▼
                     Cyber Knowledge Fabric
                               │
                     ┌─────────┼─────────┐
                     ▼         ▼         ▼
               PostgreSQL   Qdrant    Neo4j (Enterprise)
             (Source of Truth) (Vector)  (Graph Projection)
                               │
                               ▼
                     Infrastructure Adapters
  Plugin Manager: Parsers  Storage  AI  Export  Search  Visualization
```

---

# Five Fundamental Principles

1. **Knowledge First** — Everything becomes knowledge, not logs or JSON
2. **Evidence Never Lies** — Immutable source truth
3. **Intelligence Is Derived** — Nothing "knows," everything derives
4. **Memory Compounds** — Append-only, never overwrite
5. **AI Never Owns Truth** — LLMs explain, evidence decides

---

# IntelligenceObject Trait

Every domain object inherits provenance, confidence, trust, and evidence traceability.

---

# ODIN Core (Hackathon)

- Knowledge Kernel
- Intelligence Engine
- Memory Engine
- Retrieval Engine
- Simple Decision Engine
- PostgreSQL + Qdrant
- In-memory graph projection
- Local LLM for explanation

---

# ODIN Enterprise (Roadmap)

- Cyber Knowledge Fabric (Neo4j)
- Policy Engine
- Context Engine
- Knowledge Evolution
- Memory Decay
- Trust Propagation
- Domain Event Bus (Kafka/NATS)
- Multi-tenancy
- Cross-organization federation

---

# Tagline

Every investigation strengthens tomorrow's defense.
