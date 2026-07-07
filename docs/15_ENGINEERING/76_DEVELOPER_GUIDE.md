# Developer Guide

Version: 1.0

---

# Welcome

Welcome to ODIN.

ODIN is an Institutional Cyber Memory platform built using Rust, Next.js, PostgreSQL, Neo4j, and Qdrant.

The objective is simple:

Transform every cybersecurity investigation into reusable organizational intelligence.

---

# Development Philosophy

- Business Logic First
- Domain Driven Design
- Test Driven Development where practical
- Explainability over AI magic
- Composition over inheritance
- Traits over concrete implementations

---

# Local Setup

Requirements

Rust Stable

Node.js LTS

Docker

Docker Compose

Git

---

# Startup

cargo run

↓

API

npm run dev

↓

Frontend

docker compose up

↓

Databases

---

# Workspace

api

shared

ingestion

reconstruction

memory

similarity

reasoning

graph

storage

reporting

common

---

# Development Workflow

1. Pull latest changes

2. Create feature branch

3. Implement

4. Write tests

5. cargo fmt

6. cargo clippy

7. cargo test

8. Open Pull Request

---

# Principles

No crate owns another crate.

Business logic belongs in domain crates.

Database logic belongs in storage.

API only orchestrates.
