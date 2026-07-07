# Data Flow

---

# Flow

```
Raw Logs

↓

Upload API

↓

Normalization

↓

Incident Reconstruction

↓

Canonical Incident

↓

Memory Builder

↓

Institutional Memory

├── PostgreSQL

├── Neo4j

└── Qdrant

↓

Threat Memory

↓

Similarity Search

↓

Knowledge Explorer

↓

Analyst
```

---

# Detailed Flow

Step 1

Receive logs.

---

Step 2

Normalize events.

---

Step 3

Extract

- Entities
- Evidence
- IOCs
- MITRE

---

Step 4

Generate Canonical Incident.

---

Step 5

Store

Metadata

Graph

Embedding

---

Step 6

Receive new incident.

---

Step 7

Generate canonical representation.

---

Step 8

Similarity Search.

---

Step 9

Return

Historical Incidents

Similarity

Playbooks

Evidence

Differences
