# Deployment Architecture

# Local Deployment

```
Browser

↓

Next.js

↓

Rust API

↓

PostgreSQL

Neo4j

Qdrant

↓

Ollama
```

---

# Containers

Frontend

- Next.js

Backend

- Rust API

AI

- Ollama

Graph

- Neo4j

Vector

- Qdrant

Database

- PostgreSQL

---

# Production

```
Load Balancer

↓

Rust API Cluster

↓

Redis

↓

PostgreSQL

↓

Neo4j Cluster

↓

Qdrant Cluster

↓

Object Storage

↓

Monitoring

Prometheus

Grafana

Loki
```

---

# Scaling Strategy

Horizontal

- Rust API

Vertical

- Ollama

Distributed

- Neo4j

Sharded

- Qdrant

---

# Deployment Goals

Startup Time

<30 sec

Recovery Time

<60 sec

Availability

99.9%

Local First

Internet Optional
