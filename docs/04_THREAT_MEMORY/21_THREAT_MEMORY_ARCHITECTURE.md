# Threat Memory Architecture

## Purpose

Threat Memory is the core innovation of ODIN.

Unlike traditional security platforms that store logs or alerts, Threat Memory stores reusable investigative knowledge.

Every completed investigation becomes searchable organizational memory.

---

# Core Principle

Old SOC

Incident

↓

PDF

↓

Archive

↓

Forgotten

---

ODIN

Incident

↓

Canonical Incident

↓

Memory Object

↓

Threat Memory

↓

Future Investigation

---

# Components

Threat Memory consists of

1. Canonical Incident
2. Similarity Engine
3. Explanation Engine
4. Investigation Diff
5. Knowledge Retrieval

---

# Workflow

Incident

↓

Canonical Incident

↓

Memory Builder

↓

Embedding

↓

Memory Store

↓

Similarity Search

↓

Historical Matches

↓

Investigation Support

---

# Responsibilities

Threat Memory

- Store investigations
- Search investigations
- Rank investigations
- Explain similarities
- Retrieve playbooks
- Retrieve lessons learned

---

# Design Goals

- Fast retrieval
- Explainable results
- Organization-specific
- Incremental learning
- Vendor agnostic

---

# Non Goals

Threat Memory is NOT

- SIEM
- SOAR
- Log Storage
- Case Management
- Threat Intelligence Feed

It is organizational cyber memory.
