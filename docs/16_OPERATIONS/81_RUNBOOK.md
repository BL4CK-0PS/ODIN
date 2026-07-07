# Operations Runbook

Version 1.0

---

# Purpose

This runbook describes how to operate, maintain, and troubleshoot ODIN.

---

## Startup

1. Start PostgreSQL
2. Start Neo4j
3. Start Qdrant
4. Start Ollama
5. Start Rust API
6. Start Next.js

---

## Health Checks

GET /system/health

Verify

- API
- PostgreSQL
- Neo4j
- Qdrant
- Ollama

---

## Daily Tasks

Review logs

Verify backups

Monitor storage

Check failed investigations

Review audit logs

---

## Recovery

If Qdrant fails

↓

Similarity disabled

↓

System continues

If Neo4j fails

↓

Graph disabled

↓

Investigations continue

If Ollama fails

↓

Fallback deterministic reasoning

↓

No service outage
