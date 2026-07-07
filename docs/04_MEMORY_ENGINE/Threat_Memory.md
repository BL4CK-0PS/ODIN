# Threat Memory

## Concept

Threat Memory is ODIN's core differentiator — a **semantic vector store** of past incidents that enables similarity search across TTPs, IOCs, and narrative context.

## Architecture

```
┌─────────────────────────────┐
│      Incident Created        │
└─────────────┬───────────────┘
              │
              ▼
┌─────────────────────────────┐
│   Content Extraction         │
│   - Title, description       │
│   - TTPs (MITRE IDs + names) │
│   - IOC values               │
│   - Analyst notes            │
│   - Remediation steps        │
└─────────────┬───────────────┘
              │
              ▼
┌─────────────────────────────┐
│   Embedding Generation       │
│   - Combined text truncated  │
│     to 512 tokens             │
│   - Model: all-MiniLM-L6-v2  │
│     (384 dimensions)          │
│   - Normalized vector         │
└─────────────┬───────────────┘
              │
              ▼
┌─────────────────────────────┐
│   Qdrant Storage              │
│   - Collection per workspace │
│   - Payload: incident_id,    │
│     title, techniques, tags, │
│     severity, timestamp      │
│   - Indexed by: technique,   │
│     severity, timestamp      │
└─────────────────────────────┘
```

## Storage Strategy

| Parameter | Value |
|-----------|-------|
| Collection name | `incidents_{workspace_id}` |
| Vector size | 384 |
| Distance metric | Cosine |
| Replication factor | 2 |
| Optimizers | HNSW with ef_construct=200, M=16 |
| Payload indexes | technique[], severity, timestamp |
