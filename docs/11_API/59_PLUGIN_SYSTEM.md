# Plugin System

Purpose

Allow ODIN to support new integrations without modifying the core.

---

Plugin Types

Parser

Storage

Embedding

Reasoning

Export

---

Parser Plugins

Sysmon

Zeek

Suricata

Windows Event Logs

Future

AWS CloudTrail

Azure Sentinel

Splunk

Elastic

---

Storage Plugins

PostgreSQL

Neo4j

Qdrant

Future

TigerGraph

Weaviate

---

Export Plugins

PDF

Markdown

JSON

STIX

CSV

---

Rust Design

Plugins implement traits.

Core depends only on traits.

No plugin modifies business logic.
