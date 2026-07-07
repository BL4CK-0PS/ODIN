# Deployment Architecture

Version: 1.0

---

# Philosophy

ODIN is designed as a local-first enterprise platform.

Organizations should be able to deploy ODIN entirely within their own infrastructure.

No investigation data is required to leave the organization.

---

# Local Deployment

Browser

↓

Next.js Frontend

↓

Rust API (Axum)

↓

PostgreSQL

Neo4j

Qdrant

↓

Ollama

---

# Enterprise Deployment

Users

↓

Load Balancer

↓

Rust API Cluster

↓

PostgreSQL HA

Neo4j Cluster

Qdrant Cluster

↓

Monitoring

↓

Backup

---

# Cloud Deployment

Supported

AWS

Azure

GCP

Private Cloud

On-Premises

---

# Deployment Principles

- Stateless API
- Persistent databases
- Local AI
- Secure networking
- Horizontal scalability
