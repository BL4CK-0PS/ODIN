# Memory Engine Architecture

## Purpose

The Memory Engine is the persistence layer of the Knowledge Kernel.

Unlike traditional databases that store rows or documents, the Memory Engine stores **knowledge objects** — structured intelligence that compounds over time.

Every completed investigation permanently increases organizational memory.

---

# Core Principle

Old SOC

Incident → PDF → Archive → Forgotten

---

ODIN

Incident → Canonical Incident → Memory Object → Memory Engine → Future Investigation

---

# Components

Memory Engine consists of

1. **Memory Builder** — Convert Canonical Incident → Memory Object
2. **Memory Store** — PostgreSQL + Qdrant persistence
3. **Memory Versioning** — Append-only version history
4. **Knowledge Evolution** — Analyst feedback produces new versions
5. **Memory Decay** — Temporal weighting, not deletion

---

# Memory Object Schema

Every Memory Object carries:

- Metadata (id, title, severity, timestamps)
- Canonical Incident (immutable)
- Summary (generated, may evolve)
- Lessons Learned (append-only)
- Relationships (entity graph projection)
- Embedding (vector for similarity)
- Confidence (derived from evidence trust)
- Version (monotonic increment)
- Provenance (trace to source logs)

---

# Knowledge Evolution

Memory should evolve as understanding deepens.

```
Version 1  — Initial reconstruction
     ↓
Version 2  — Analyst adds notes
     ↓
Version 3  — Playbook linked
     ↓
Version 4  — Detection improved
     ↓
Version N  — Knowledge compounds
```

Never overwrite. Only append. Historical versions remain accessible.

---

# Memory Decay

Not all knowledge stays equally relevant.

```
Windows XP exploit (2015)  →  weight: 0.1
Cobalt Strike (2026)       →  weight: 0.95
```

- Older memories weighted lower in similarity ranking
- Never deleted — context may resurface (legacy attacks return)
- Feedback signals (reuse frequency) adjust weights up or down

---

# Workflow

Observation

↓

Canonical Incident

↓

Memory Builder

↓

Memory Object

│
├── PostgreSQL (structured data)
├── Qdrant (embedding + similarity)
└── In-Memory Graph (entity relationships)
│
↓

Retrieval Engine (context-weighted hybrid similarity)

↓

Decision Engine (recommendations with confidence)

---

# Responsibilities

- Store investigations
- Version knowledge
- Evolve memory from feedback
- Apply temporal decay
- Serve retrieval queries

---

# Design Goals

- Immutable evidence
- Append-only versions
- Compounding knowledge
- Temporal relevance
- Organization-specific weighting

---

# Non Goals

Memory Engine is NOT

- Log storage
- SIEM
- Case management
- Threat intelligence feed

It is organizational cyber memory.
