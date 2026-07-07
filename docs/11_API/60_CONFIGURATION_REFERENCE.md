# Configuration Reference

Environment Variables

DATABASE_URL

QDRANT_URL

NEO4J_URL

OLLAMA_URL

JWT_SECRET

HOST

PORT

RUST_LOG

---

Feature Flags

graph

similarity

reasoning

reporting

audit

metrics

---

Config Files

config/

development.toml

testing.toml

production.toml

---

Configuration Priority

CLI

↓

Environment

↓

Configuration File

↓

Defaults

---

Validation

Configuration is validated during startup.

Application fails fast on invalid configuration.
