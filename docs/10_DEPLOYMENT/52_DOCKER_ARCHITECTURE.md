# Docker Architecture

---

# Containers

frontend

Next.js

---

backend

Rust Axum

---

postgres

Primary Database

---

neo4j

Knowledge Graph

---

qdrant

Threat Memory

---

ollama

Local AI

---

nginx

Reverse Proxy

---

# Networks

frontend_network

backend_network

database_network

---

# Volumes

postgres_data

neo4j_data

qdrant_data

ollama_models

logs

backups

---

# Startup Order

1.

PostgreSQL

2.

Neo4j

3.

Qdrant

4.

Ollama

5.

Rust API

6.

Next.js
