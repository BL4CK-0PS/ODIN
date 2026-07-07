# High-Level Architecture

# Philosophy

ODIN is designed as an Institutional Cyber Memory platform.

It does not replace SIEM, SOAR, or DFIR platforms.

Instead, it sits above the existing cybersecurity ecosystem and converts investigations into reusable organizational knowledge.

---

# Architecture Layers

```
                    Existing Security Stack

Sysmon
Zeek
Suricata
Velociraptor
Wazuh
Elastic
Sentinel

              │

              ▼

============================

      Ingestion Layer

============================

Normalize

Validate

Parse

Correlate

              │

              ▼

============================

 Incident Reconstruction

============================

Timeline

Entities

Evidence

MITRE

Narrative

              │

              ▼

============================

 Memory Builder

============================

Canonical Incident

Knowledge Extraction

Relationships

Lessons Learned

              │

              ▼

============================

 Institutional Memory

============================

Neo4j

Qdrant

PostgreSQL

              │

              ▼

============================

 Threat Memory

============================

Similarity

Historical Incidents

Playbooks

Evidence

              │

              ▼

============================

 Presentation Layer

============================

Timeline

Knowledge Explorer

Incident Viewer

Search

```

---

# Architectural Principles

- Modular
- Explainable
- Event Driven
- Vendor Agnostic
- Local First
- AI Assisted
- Human Controlled

---

# Core Business Capabilities

1. Incident Reconstruction
2. Memory Builder
3. Threat Memory

Everything else supports these three capabilities.
