# Security Architecture

Version: 1.0

---

# Philosophy

ODIN stores highly sensitive cybersecurity investigations.

Security is a product requirement, not an optional feature.

---

# Security Principles

Least Privilege

Defense in Depth

Local First

Immutable Evidence

Audit Everything

Human Approval

Explainability

---

# Security Layers

User

↓

Authentication

↓

Authorization

↓

API

↓

Business Logic

↓

Storage

↓

Audit

---

# Assets

Critical

- Canonical Incidents
- Evidence
- Investigation Notes
- Playbooks
- Knowledge Graph
- Threat Memory

---

# Trust Boundaries

Browser

↓

Rust API

↓

Databases

↓

Local AI Models

---

# Threats

Unauthorized Access

Evidence Tampering

Prompt Injection

Data Leakage

Privilege Escalation

Insider Threats

---

# Mitigations

RBAC

Immutable Evidence

Signed Audit Logs

Encryption

Local AI

Input Validation
