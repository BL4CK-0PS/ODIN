# Relationship Model

## Purpose

Relationships transform isolated entities into organizational knowledge.

---

# Node Types

Incident

Entity

Evidence

Technique

Playbook

Investigation

IOC

Host

User

Process

Registry

File

---

# Relationship Types

HOSTED_ON

EXECUTED

CREATED

MODIFIED

ACCESSED

CONNECTED_TO

USES

INITIATED

PART_OF

RELATED_TO

OBSERVED_IN

DETECTED_BY

INVESTIGATED_BY

MITIGATED_BY

REFERENCES

SIMILAR_TO

LEARNED_FROM

GENERATED

---

# Relationship Properties

Every relationship stores

- Timestamp

- Confidence

- Source

- Investigation

---

# Example

Incident

↓

OBSERVED_IN

↓

PowerShell

↓

EXECUTED

↓

EncodedCommand

↓

CONNECTED_TO

↓

C2 Server

↓

RELATED_TO

↓

Credential Dump

↓

MITIGATED_BY

↓

Playbook-07

---

# Design Principles

Relationships are directional.

Relationships are timestamped.

Relationships are explainable.

Relationships preserve evidence provenance.
